//! whisper-rs 0.16 - Tối ưu tốc độ + Flash Attention (Intel AMD + Apple Silicon)

use crate::error::{AutoSubError, Result};
use crate::subtitle::Segment;
use log::info;
use num_cpus;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

fn is_apple_silicon() -> bool {
    cfg!(target_arch = "aarch64") && cfg!(target_os = "macos")
}

fn is_intel_mac_amd() -> bool {
    cfg!(target_arch = "x86_64") && !is_apple_silicon()
}

#[derive(Debug, Clone)]
pub struct WhisperNativeProgress {
    pub percent: f32,
    pub segment_count: usize,
}

pub struct WhisperEngine {
    ctx: Arc<WhisperContext>,
}

impl WhisperEngine {
    pub fn load(model_path: &str) -> Result<Self> {
        let hardware = if is_apple_silicon() {
            "Apple Silicon"
        } else if is_intel_mac_amd() {
            "Intel 2019 + AMD GPU"
        } else {
            "Unknown"
        };

        info!("🔧 WhisperEngine init → Hardware: {}", hardware);

        let mut ctx_params = WhisperContextParameters::default();

        if is_intel_mac_amd() {
            // Bật GPU + Flash Attention (thử tốc độ cao hơn)
            ctx_params.use_gpu = true;
            ctx_params.flash_attn = true;

            std::env::set_var("GGML_METAL_N_CB", "2");
            std::env::set_var("GGML_METAL_MAX_BUFFER_SIZE", "2147483648");
            std::env::set_var("GGML_METAL_MPS", "0");

            info!("🚀 Intel + AMD mode → use_gpu = true + flash_attn = true");
        } else if is_apple_silicon() {
            ctx_params.use_gpu = true;
            ctx_params.flash_attn = true;

            std::env::set_var("GGML_METAL_N_CB", "4");
            std::env::set_var("GGML_METAL_MAX_BUFFER_SIZE", "32212254720");
            info!("🚀 Apple Silicon → use_gpu = true + flash_attn = true (Full GPU)");
        }

        let ctx = WhisperContext::new_with_params(model_path, ctx_params)
            .map_err(|e| AutoSubError::WhisperDecode(format!("Load model failed: {}", e)))?;

        info!("✅ Model loaded successfully on Metal");
        Ok(Self { ctx: Arc::new(ctx) })
    }

    pub async fn transcribe(
        &self,
        samples: Vec<f32>,
        language: &str,
        threads: Option<usize>,
        progress_tx: Option<mpsc::Sender<WhisperNativeProgress>>,
        _abort_flag: Option<Arc<AtomicBool>>,
    ) -> Result<Vec<Segment>> {
        let ctx = self.ctx.clone();
        let lang = if language == "auto" || language.is_empty() {
            String::new()
        } else {
            language.to_string()
        };

        let n_threads = threads.unwrap_or_else(|| {
            if is_apple_silicon() {
                num_cpus::get_physical().max(1)
            } else {
                let cores = num_cpus::get_physical();
                if cores > 4 {
                    cores - 2
                } else {
                    cores
                }
            }
        });

        let start = Instant::now();

        let result = tokio::task::spawn_blocking(move || -> Result<Vec<Segment>> {
            let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 2 });

            if lang.is_empty() {
                params.set_language(None);
            } else {
                params.set_language(Some(&lang));
            }

            params.set_n_threads(n_threads as i32);
            params.set_token_timestamps(true);
            params.set_split_on_word(true);
            params.set_max_len(42);
            params.set_no_speech_thold(0.6);
            params.set_logprob_thold(-1.0);
            params.set_print_special(false);
            params.set_print_progress(false);
            params.set_print_realtime(false);
            params.set_print_timestamps(false);

            let mut state = ctx
                .create_state()
                .map_err(|e| AutoSubError::WhisperDecode(e.to_string()))?;

            state
                .full(params, &samples)
                .map_err(|e| AutoSubError::WhisperDecode(e.to_string()))?;

            let segments: Vec<Segment> = state
                .as_iter()
                .map(|segment| Segment {
                    start: segment.start_timestamp() as f32 / 100.0,
                    end: segment.end_timestamp() as f32 / 100.0,
                    text: segment.to_str().unwrap_or("").to_string(),
                    ..Default::default()
                })
                .collect();

            if let Some(tx) = &progress_tx {
                let _ = tx.try_send(WhisperNativeProgress {
                    percent: 100.0,
                    segment_count: segments.len(),
                });
            }

            info!(
                "✅ Transcription completed in {:.2}s | Segments: {}",
                start.elapsed().as_secs_f32(),
                segments.len()
            );
            Ok(segments)
        })
        .await
        .map_err(|e| AutoSubError::WhisperDecode(format!("Task panicked: {}", e)))??;

        Ok(result)
    }
}
