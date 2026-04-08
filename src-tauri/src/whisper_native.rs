//! Native Whisper — whisper-rs 0.16 binding với DTW timestamp alignment.
//!
//! Model priority: large-v3 (primary) > large-v2 (VRAM fallback) > large-v3-turbo (speed)
//! Platform: Mac Intel 2019 (AMD 555X 4GB VRAM) + Mac M-series

use crate::error::{AutoSubError, Result};
use crate::subtitle::Segment;
use log::{info, warn};
use std::sync::Arc;
use tokio::sync::mpsc;
use whisper_rs::{
    DtwMode, DtwModelPreset, FullParams, SamplingStrategy,
    WhisperContext, WhisperContextParameters,
};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct WhisperNativeProgress {
    pub percent: f32,
    pub segment_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModelVariant {
    LargeV2,
    LargeV3,
    LargeV3Turbo,
    Unknown,
}

impl ModelVariant {
    /// Detect từ đường dẫn file model.
    /// QUAN TRỌNG: kiểm tra "turbo" TRƯỚC "v3" vì "large-v3-turbo" chứa "large-v3".
    pub fn from_path(path: &str) -> Self {
        let p = path.to_lowercase();
        if p.contains("large-v3-turbo") || p.contains("large_v3_turbo") || p.contains("largev3turbo") {
            Self::LargeV3Turbo
        } else if p.contains("large-v3") || p.contains("large_v3") || p.contains("largev3") {
            Self::LargeV3
        } else if p.contains("large-v2") || p.contains("large_v2") || p.contains("largev2") {
            Self::LargeV2
        } else {
            Self::Unknown
        }
    }

    pub fn has_dtw(&self) -> bool {
        !matches!(self, Self::Unknown)
    }

    pub fn recommended_threads() -> usize {
        if is_apple_silicon() {
            num_cpus::get_physical().max(1)
        } else {
            4 // Intel Mac + AMD discrete: CPU chỉ là coordinator
        }
    }
}

// ── Engine ────────────────────────────────────────────────────────────────────

pub struct WhisperEngine {
    ctx: Arc<WhisperContext>,
    model_path: String,
    pub variant: ModelVariant,
}

impl WhisperEngine {
    pub fn load(model_path: &str) -> Result<Self> {
        let variant = ModelVariant::from_path(model_path);
        info!("whisper_native: loading {:?} from {}", variant, model_path);

        if !is_apple_silicon() {
            if let Ok(meta) = std::fs::metadata(model_path) {
                let size_mb = meta.len() / 1_048_576;
                info!("whisper_native: model size = {}MB", size_mb);
                if size_mb > 1500 {
                    warn!(
                        "whisper_native: {}MB model — AMD 555X 4GB VRAM, macOS ~1GB overhead. \
                         Nếu OOM hãy dùng bản q5_0 (~800MB).",
                        size_mb
                    );
                }
            }
        }

        let mut ctx_params = WhisperContextParameters::default();
        ctx_params.use_gpu = true; // Metal: Intel AMD + Apple Silicon

        // DTW alignment — cải thiện timestamp chính xác theo từng từ.
        // BUG whisper.cpp: WHISPER_ASSERT filter_width < a->ne[2] khi batch audio quá ngắn
        // → fix bằng cách pad audio lên MIN_BATCH_SECS trong transcribe(), không tắt DTW.
        match variant {
            ModelVariant::LargeV2 => {
                ctx_params.dtw_parameters.mode = DtwMode::ModelPreset {
                    model_preset: DtwModelPreset::LargeV2,
                };
                info!("whisper_native: DTW preset LargeV2 enabled");
            }
            ModelVariant::LargeV3 => {
                ctx_params.dtw_parameters.mode = DtwMode::ModelPreset {
                    model_preset: DtwModelPreset::LargeV3,
                };
                info!("whisper_native: DTW preset LargeV3 enabled");
            }
            ModelVariant::LargeV3Turbo => {
                ctx_params.dtw_parameters.mode = DtwMode::ModelPreset {
                    model_preset: DtwModelPreset::LargeV3Turbo,
                };
                info!("whisper_native: DTW preset LargeV3Turbo enabled");
            }
            ModelVariant::Unknown => {
                ctx_params.dtw_parameters.mode = DtwMode::None;
                warn!(
                    "whisper_native: không có DTW preset cho '{}'. \
                     Dùng attention-based timestamp.",
                    model_path
                );
            }
        }

        let ctx = WhisperContext::new_with_params(model_path, ctx_params).map_err(|e| {
            AutoSubError::WhisperDecode(format!("Load model '{}' thất bại: {}", model_path, e))
        })?;

        info!("whisper_native: model loaded");

        Ok(Self {
            ctx: Arc::new(ctx),
            model_path: model_path.to_string(),
            variant,
        })
    }

    pub async fn transcribe(
        &self,
        samples: Vec<f32>,
        language: &str,
        threads: Option<usize>,
        progress_tx: Option<mpsc::Sender<WhisperNativeProgress>>,
        abort_flag: Option<Arc<AtomicBool>>,
    ) -> Result<Vec<Segment>> {
        let ctx = self.ctx.clone();
        let language = language.to_string();
        let n_threads = threads.unwrap_or_else(ModelVariant::recommended_threads);
        let has_dtw = self.variant.has_dtw();
        let abort_flag_cb = abort_flag.clone();

        let result = tokio::task::spawn_blocking(move || {
            let mut state = ctx.create_state().map_err(|e| {
                AutoSubError::WhisperDecode(format!("create_state failed: {}", e))
            })?;

            let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 2 });
            params.set_n_threads(n_threads as i32);

            if language == "auto" || language.is_empty() {
                params.set_language(None);
            } else {
                params.set_language(Some(&language));
            }

            params.set_token_timestamps(true);
            params.set_split_on_word(true);
            params.set_max_len(42);
            params.set_print_special(false);
            params.set_print_progress(false);
            params.set_print_realtime(false);
            params.set_print_timestamps(false);
            // Skip segment toàn nhạc/silence — tránh decode nhiễu thành text giả
            params.set_no_speech_thold(0.6);
            params.set_logprob_thold(-1.0);

            let progress_tx_cb = progress_tx.clone();
            params.set_progress_callback_safe(move |pct| {
                if let Some(ref tx) = progress_tx_cb {
                    let _ = tx.blocking_send(WhisperNativeProgress {
                        percent: pct as f32,
                        segment_count: 0,
                    });
                }
            });

            if let Some(ref abort) = abort_flag_cb {
                unsafe {
                    params.set_abort_callback(Some(whisper_abort_callback));
                    params.set_abort_callback_user_data(Arc::as_ptr(abort) as *mut c_void);
                }
            }

            // Fix WHISPER_ASSERT: filter_width < a->ne[2]
            //
            // Khi DTW enabled, whisper.cpp gọi whisper_exp_compute_token_level_timestamps_dtw()
            // với medfilt_width=7 (hardcoded). Assertion yêu cầu: 7 < n_audio_tokens.
            // n_audio_tokens = n_frames của batch SAU khi encode, tỉ lệ thuận với độ dài audio.
            //
            // Nếu batch quá ngắn (xảy ra sau "single timestamp ending - skip entire chunk"),
            // whisper nội bộ nhảy seek đến cuối chunk và batch tiếp theo có thể chỉ còn
            // vài frames → n_audio_tokens < 7 → crash.
            //
            // Fix: pad lên đúng 30 giây (WHISPER_CHUNK_SIZE = 30s @ 16kHz = 480000 samples).
            // Với 30s audio, n_audio_tokens = 1500 (n_audio_ctx của model) >> 7, an toàn.
            const WHISPER_CHUNK_SAMPLES: usize = 30 * 16000; // 480000
            let padded: Vec<f32>;
            let input = if samples.len() < WHISPER_CHUNK_SAMPLES {
                padded = {
                    let mut v = samples.clone();
                    v.resize(WHISPER_CHUNK_SAMPLES, 0.0);
                    v
                };
                &padded
            } else {
                &samples
            };

            state.full(params, input).map_err(|e| {
                AutoSubError::WhisperDecode(format!("Whisper inference failed: {}", e))
            })?;

            let n_segs = state.full_n_segments();
            let mut segments: Vec<Segment> = Vec::with_capacity(n_segs as usize);

            for i in 0..n_segs {
                let segment = match state.get_segment(i) {
                    Some(s) => s,
                    None => continue,
                };

                let text = segment.to_str_lossy().unwrap_or_default().trim().to_string();
                if text.is_empty() {
                    continue;
                }

                // Centiseconds từ segment (attention-based)
                let t0_cs = segment.start_timestamp();
                let t1_cs = segment.end_timestamp();

                // DTW token timestamps — chính xác hơn, dùng khi có preset
                let n_tokens = segment.n_tokens();
                let (start_secs, end_secs) = if has_dtw && n_tokens > 0 {
                    let dtw_start = segment.get_token(0)
                        .map(|t| t.token_data().t_dtw)
                        .unwrap_or(-1);
                    let dtw_end = segment.get_token(n_tokens - 1)
                        .map(|t| t.token_data().t_dtw)
                        .unwrap_or(-1);
                    (
                        if dtw_start > 0 { dtw_start as f32 / 100.0 } else { t0_cs as f32 / 100.0 },
                        if dtw_end   > 0 { dtw_end   as f32 / 100.0 } else { t1_cs as f32 / 100.0 },
                    )
                } else {
                    (t0_cs as f32 / 100.0, t1_cs as f32 / 100.0)
                };

                if end_secs <= start_secs + 0.05 {
                    continue;
                }

                segments.push(Segment { start: start_secs, end: end_secs, text });
            }

            Ok::<Vec<Segment>, AutoSubError>(segments)
        })
        .await
        .map_err(|e| AutoSubError::WhisperDecode(format!("Blocking task panicked: {}", e)))??;

        Ok(result)
    }

    pub fn model_path(&self) -> &str {
        &self.model_path
    }
}
// ── Abort Callback ────────────────────────────────────────────────────────────

unsafe extern "C" fn whisper_abort_callback(user_data: *mut c_void) -> bool {
    if user_data.is_null() {
        return false;
    }
    let abort_flag = &*(user_data as *const AtomicBool);
    abort_flag.load(Ordering::SeqCst)
}

const fn is_apple_silicon() -> bool {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    { true }
    #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
    { false }
}
