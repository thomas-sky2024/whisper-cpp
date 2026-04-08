//! Pipeline v2 — batch grouping + DTW timestamp + multilingual fix.
//!
//! Giải quyết 3 vấn đề đã quan sát:
//!   1. Timestamp drift (large-v3-turbo) → DTW + batch grouping giữ khoảng lặng
//!   2. Multilingual bias (Japanese → Chinese) → language=None mỗi batch
//!   3. Smart Demucs → chỉ chạy khi RMS noise ratio > 0.4

use crate::{
    cache, demucs,
    error::{AutoSubError, Result},
    ffmpeg, job_manager::JobManager, model_manager::ModelManager,
    post_process, subtitle, validator,
    vad::{self, VadConfig},
    whisper_native::{WhisperEngine, WhisperNativeProgress},
};
use hound; // BẮT BUỘC: dùng trong read_wav_f32()
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tauri_plugin_shell::ShellExt;
use tokio::sync::{mpsc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressPayload {
    pub stage: String,
    pub percent: f32,
    pub segment_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineOptions {
    pub video_path: String,
    pub language: String,
    pub model: String,
    // performance_mode removed — thread count handled by ModelVariant::recommended_threads()
    // Frontend có thể vẫn gửi field này, serde sẽ ignore vì không có trong struct
    #[serde(default)]
    pub isolate_vocals: bool,
    #[serde(default = "default_vad_threshold")]
    pub vad_threshold: f32,
}

fn default_vad_threshold() -> f32 { 0.5 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    pub segments: Vec<subtitle::Segment>,
    pub srt_content: String,
    pub txt_content: String,
    pub duration_secs: f32,
    pub from_cache: bool,
}

pub struct AppStateV2 {
    pub job_manager: Arc<JobManager>,
    pub whisper_engine: Arc<Mutex<Option<WhisperEngine>>>,
    pub loaded_model: Arc<Mutex<String>>,
    pub exit_flag: Arc<AtomicBool>,
}

impl AppStateV2 {
    pub fn new() -> Self {
        Self {
            job_manager: Arc::new(JobManager::new()),
            whisper_engine: Arc::new(Mutex::new(None)),
            loaded_model: Arc::new(Mutex::new(String::new())),
            exit_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Load model vào RAM nếu chưa load hoặc đổi model.
    /// Singleton — tránh load 3GB I/O mỗi job.
    pub async fn get_or_load_engine(&self, model_path: &str) -> Result<()> {
        let mut engine = self.whisper_engine.lock().await;
        let mut loaded = self.loaded_model.lock().await;
        if engine.is_none() || *loaded != model_path {
            info!("pipeline_v2: loading model {}", model_path);
            *engine = Some(WhisperEngine::load(model_path)?);
            *loaded = model_path.to_string();
        }
        Ok(())
    }
}

// ── Batch grouping ────────────────────────────────────────────────────────────

struct AudioBatch {
    samples: Vec<f32>,
    original_start_secs: f32,
}

/// Gom VAD segments thành batches ~25s, GIỮ khoảng lặng ngắn (<2s) bên trong.
///
/// Lý do giữ khoảng lặng:
///   Concat thuần speech → Whisper thấy tốc độ nói đều bất thường → drift timestamp.
///   Giữ khoảng nghỉ tự nhiên → Whisper bám sóng âm đúng qua DTW.
fn group_into_batches(
    pcm: &[f32],
    vad_output: &vad::VadOutput,
    sample_rate: u32,
    target_secs: f32,
) -> Vec<AudioBatch> {
    let sr = sample_rate as f32;
    let max_silence_in_batch = 2.0f32;
    let mut batches = Vec::new();

    let regions: Vec<(f32, f32)> = vad_output.segments.iter()
        .map(|seg| (seg.start_ms as f32 / 1000.0, seg.end_ms as f32 / 1000.0))
        .collect();

    if regions.is_empty() {
        for (i, chunk) in pcm.chunks((target_secs * sr) as usize).enumerate() {
            batches.push(AudioBatch {
                samples: chunk.to_vec(),
                original_start_secs: i as f32 * target_secs,
            });
        }
        return batches;
    }

    let mut batch_start: Option<f32> = None;
    let mut batch_end = 0.0f32;

    for (i, &(seg_start, seg_end)) in regions.iter().enumerate() {
        let is_last = i == regions.len() - 1;

        match batch_start {
            None => {
                batch_start = Some(seg_start);
                batch_end = seg_end;
            }
            Some(bstart) => {
                let silence = seg_start - batch_end;
                let duration = seg_end - bstart;
                let should_flush = silence > max_silence_in_batch || duration > target_secs;

                if should_flush {
                    push_batch(pcm, bstart, batch_end, sr, &mut batches);
                    batch_start = Some(seg_start);
                    batch_end = seg_end;
                } else {
                    batch_end = seg_end;
                }
            }
        }

        if is_last {
            if let Some(bstart) = batch_start {
                push_batch(pcm, bstart, batch_end, sr, &mut batches);
            }
        }
    }

    info!("group_into_batches: {} regions → {} batches", regions.len(), batches.len());
    batches
}

fn push_batch(pcm: &[f32], start_secs: f32, end_secs: f32, sr: f32, out: &mut Vec<AudioBatch>) {
    let s = (start_secs * sr) as usize;
    let e = (end_secs * sr) as usize;
    if s < pcm.len() {
        out.push(AudioBatch {
            samples: pcm[s..e.min(pcm.len())].to_vec(),
            original_start_secs: start_secs,
        });
    }
}

// ── Smart Demucs (RMS detection) ─────────────────────────────────────────────

fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() { return 0.0; }
    let sq: f32 = samples.iter().map(|&s| s * s).sum();
    (sq / samples.len() as f32).sqrt()
}

/// Detect nhạc nền nặng bằng cách so sánh RMS noise/speech.
/// noise_rms / speech_rms > 0.4 → cần Demucs.
fn should_run_demucs(pcm: &[f32], vad_output: &vad::VadOutput, sample_rate: u32) -> bool {
    let sr = sample_rate as f32;
    let mut speech_buf: Vec<f32> = Vec::new();
    let mut noise_buf: Vec<f32> = Vec::new();
    let mut prev_end = 0.0f32;

    for seg in &vad_output.segments {
        let seg_start = seg.start_ms as f32 / 1000.0;
        let seg_end = seg.end_ms as f32 / 1000.0;

        // Đoạn trước segment = noise/nhạc nền
        if seg_start > prev_end + 0.05 {
            let ns = (prev_end * sr) as usize;
            let ne = (seg_start * sr) as usize;
            if ne <= pcm.len() {
                noise_buf.extend_from_slice(&pcm[ns..ne]);
            }
        }

        speech_buf.extend_from_slice(&seg.samples);
        prev_end = seg_end;
    }

    let speech_rms = rms(&speech_buf);
    let noise_rms = rms(&noise_buf);
    let ratio = if speech_rms > 0.001 { noise_rms / speech_rms } else { 0.0 };

    info!(
        "RMS: speech={:.4} noise={:.4} ratio={:.2} → Demucs={}",
        speech_rms, noise_rms, ratio, ratio > 0.4
    );
    ratio > 0.4
}

// ── Emit helper ───────────────────────────────────────────────────────────────

fn emit(app: &AppHandle, stage: &str, pct: f32, segs: usize) {
    let _ = app.emit("pipeline-progress", ProgressPayload {
        stage: stage.to_string(),
        percent: pct,
        segment_count: segs,
    });
}

// ── Main pipeline ─────────────────────────────────────────────────────────────

pub async fn run(
    app: AppHandle,
    opts: PipelineOptions,
    job_mgr: Arc<JobManager>,
    state: Arc<AppStateV2>,
) -> Result<PipelineResult> {
    let video_path = &opts.video_path;
    let model_name = &opts.model;
    let lang = &opts.language;

    // Stage 0: Cache check
    emit(&app, "Checking cache", 2.0, 0);
    if let Ok(Some(cached)) = cache::check_cache(video_path, model_name, lang) {
        let srt = tokio::fs::read_to_string(&cached).await
            .map_err(|e| AutoSubError::Cache(format!("Read cache: {}", e)))?;
        let segs = parse_srt(&srt);
        let txt = subtitle::to_txt(&segs);
        emit(&app, "Done (from cache)", 100.0, segs.len());
        return Ok(PipelineResult {
            segments: segs, srt_content: srt, txt_content: txt,
            duration_secs: 0.0, from_cache: true,
        });
    }

    if !ModelManager::verify_model(model_name) {
        return Err(AutoSubError::WhisperDecode(format!("Model '{}' không tìm thấy.", model_name)));
    }
    let model_path = ModelManager::get_model_path(model_name).to_string_lossy().to_string();

    let dur = {
        let ffp = app.shell().sidecar("ffprobe").ok()
            .or_else(|| app.shell().sidecar("ffmpeg").ok());
        if let Some(s) = ffp { ffmpeg::get_video_duration(s, video_path).await.unwrap_or(0.0) }
        else { 0.0 }
    };

    // Stage 1: FFmpeg → audio.wav (16kHz mono PCM)
    emit(&app, "Extracting audio", 5.0, 0);
    job_mgr.update_progress("Extracting audio", 5.0);
    cache::update_state(video_path, model_name, lang, dur, cache::PipelineState::Extracting)?;

    let cache_dir = cache::cache_dir(video_path)?;
    tokio::fs::create_dir_all(&cache_dir).await
        .map_err(|e| AutoSubError::Cache(format!("mkdir: {}", e)))?;
    let audio_path = cache_dir.join("audio.wav").to_string_lossy().to_string();

    crate::utils::retry(|| async {
        let (tx, mut rx) = mpsc::channel::<ffmpeg::FfmpegProgress>(32);
        let a2 = app.clone(); let j2 = job_mgr.clone();
        tokio::spawn(async move {
            let mut last_emit = std::time::Instant::now();
            let mut last_s = -1.0;
            while let Some(p) = rx.recv().await {
                let s = 5.0 + p.percent * 0.15; // 5% → 20%
                if s - last_s >= 1.0 || last_emit.elapsed().as_millis() > 300 {
                    emit(&a2, "Extracting audio", s, 0);
                    j2.update_progress("Extracting audio", s);
                    last_s = s;
                    last_emit = std::time::Instant::now();
                }
            }
        });
        let sc = app.shell().sidecar("ffmpeg")
            .map_err(|e| AutoSubError::SidecarNotFound(e.to_string()))?;
        ffmpeg::extract_audio(sc, video_path, &audio_path, dur, Some(tx)).await
    }, 2).await?;

    // Stage 2: Đọc WAV vào RAM
    emit(&app, "Analyzing audio", 22.0, 0);
    let pcm = read_wav_f32(&audio_path).await?;
    info!("pipeline_v2: {:.1}s audio ({} samples)", pcm.len() as f32 / 16000.0, pcm.len());

    // Stage 3: VAD — detect speech segments
    emit(&app, "Voice detection", 25.0, 0);
    let vad_out = if ModelManager::vad_model_ready() {
        let vm = ModelManager::get_vad_model_path().to_string_lossy().to_string();
        let pcm2 = pcm.clone();
        let cfg = VadConfig { model_path: vm, threshold: opts.vad_threshold, min_silence_ms: 300 };
        match tokio::task::spawn_blocking(move || vad::process(&pcm2, &cfg, 16000)).await {
            Ok(Ok(v)) => {
                info!("pipeline_v2: VAD found {} speech segments", v.segments.len());
                Some(v)
            }
            Ok(Err(e)) => { warn!("pipeline_v2: VAD error: {}", e); None }
            Err(e)     => { warn!("pipeline_v2: VAD panic: {}", e); None }
        }
    } else {
        warn!("pipeline_v2: VAD model không có — batch chia đều 25s (kém hiệu quả)");
        None
    };

    // Stage 3.5: Smart Demucs — chỉ khi phát hiện nhạc nền nặng hoặc user bật
    let mut final_audio = audio_path.clone();
    let need_demucs = opts.isolate_vocals
        || vad_out.as_ref().map_or(false, |v| should_run_demucs(&pcm, v, 16000));

    if need_demucs {
        emit(&app, "Separating vocals", 28.0, 0);
        info!("pipeline_v2: Demucs activated (manual={}, RMS-based={})",
            opts.isolate_vocals, !opts.isolate_vocals);
        if let Ok(sc) = app.shell().sidecar("demucs-main") {
            let dm = ModelManager::get_demucs_model_path().to_string_lossy().to_string();
            match demucs::separate_vocals(sc, &audio_path, &cache_dir.to_string_lossy(), &dm).await {
                Ok(p)  => { info!("pipeline_v2: vocals → {}", p); final_audio = p; }
                Err(e) => warn!("pipeline_v2: Demucs failed ({}), dùng audio gốc", e),
            }
        } else {
            warn!("pipeline_v2: demucs-main sidecar không có");
        }
    }

    // Đọc lại PCM nếu Demucs tạo file mới
    let pcm_final = if final_audio != audio_path {
        read_wav_f32(&final_audio).await?
    } else {
        pcm.clone()
    };

    // Stage 4: Whisper với batch grouping
    emit(&app, "Transcribing", 35.0, 0);
    job_mgr.update_progress("Transcribing", 35.0);
    cache::update_state(video_path, model_name, lang, dur, cache::PipelineState::Transcribing)?;

    state.get_or_load_engine(&model_path).await?;
    let engine_guard = state.whisper_engine.lock().await;
    let engine = engine_guard.as_ref().unwrap(); // safe: vừa load ở trên

    let batches = if let Some(ref v) = vad_out {
        group_into_batches(&pcm_final, v, 16000, 25.0)
    } else {
        pcm_final.chunks(25 * 16000)
            .enumerate()
            .map(|(i, c)| AudioBatch {
                samples: c.to_vec(),
                original_start_secs: i as f32 * 25.0,
            })
            .collect()
    };

    let total = batches.len();
    info!("pipeline_v2: {} batches to transcribe", total);
    let mut all_segs: Vec<subtitle::Segment> = Vec::new();

    for (idx, batch) in batches.into_iter().enumerate() {
        // Kiểm tra dừng hoặc thoát app
        if state.exit_flag.load(Ordering::SeqCst) || job_mgr.state() == crate::job_manager::JobState::Cancelled {
            info!("pipeline_v2: cancellation or exit detected, stopping batch loop");
            break;
        }

        let pstart = 35.0 + (idx as f32 / total as f32) * 45.0;
        let pend   = 35.0 + ((idx + 1) as f32 / total as f32) * 45.0;

        let (tx, mut rx) = mpsc::channel::<WhisperNativeProgress>(32);
        let a2 = app.clone(); let j2 = job_mgr.clone();
        tokio::spawn(async move {
            let mut last_emit = std::time::Instant::now();
            let mut last_s = -1.0;
            while let Some(p) = rx.recv().await {
                let s = pstart + (p.percent / 100.0) * (pend - pstart);
                if s - last_s >= 1.0 || last_emit.elapsed().as_millis() > 300 {
                    emit(&a2, "Transcribing", s, 0);
                    j2.update_progress("Transcribing", s);
                    last_s = s;
                    last_emit = std::time::Instant::now();
                }
            }
        });

        // language=None mỗi batch → auto-detect → fix multilingual drift
        let batch_lang = if lang == "auto" || lang.is_empty() { "auto" } else { lang.as_str() };

        let mut segs = engine.transcribe(batch.samples, batch_lang, None, Some(tx), Some(state.exit_flag.clone())).await?;

        // Cộng offset batch về timeline gốc video
        // Đây là bước quan trọng nhất để kết quả các batch khớp timeline
        for seg in &mut segs {
            seg.start += batch.original_start_secs;
            seg.end   += batch.original_start_secs;
        }

        info!(
            "pipeline_v2: batch {}/{}: {} segs, offset={:.1}s",
            idx + 1, total, segs.len(), batch.original_start_secs
        );
        all_segs.extend(segs);
    }

    drop(engine_guard);
    cache::update_state(video_path, model_name, lang, dur, cache::PipelineState::Transcribed)?;

    // Stage 5: Validate
    emit(&app, "Validating", 80.0, all_segs.len());
    job_mgr.update_progress("Validating", 80.0);
    let validated = validator::validate(all_segs);

    // Stage 6: Post-process
    emit(&app, "Post-processing", 85.0, validated.len());
    cache::update_state(video_path, model_name, lang, dur, cache::PipelineState::Processing)?;
    let processed = post_process::process(validated);

    // Stage 7: Export
    emit(&app, "Exporting", 95.0, processed.len());
    let srt = subtitle::to_srt(&processed);
    let txt = subtitle::to_txt(&processed);
    cache::save_final(video_path, &srt, model_name, lang, dur)?;

    emit(&app, "Done", 100.0, processed.len());
    job_mgr.complete();

    Ok(PipelineResult {
        segments: processed,
        srt_content: srt,
        txt_content: txt,
        duration_secs: dur,
        from_cache: false,
    })
}

// ── Audio helpers ─────────────────────────────────────────────────────────────

/// Đọc WAV file → Vec<f32> normalized (-1.0..1.0), 16kHz mono.
/// `use hound;` BẮT BUỘC ở đầu file — đừng bỏ.
async fn read_wav_f32(path: &str) -> Result<Vec<f32>> {
    let p = path.to_string();
    tokio::task::spawn_blocking(move || {
        let mut r = hound::WavReader::open(&p)
            .map_err(|e| AutoSubError::AudioExtract(format!("WAV open '{}': {}", p, e)))?;

        let spec = r.spec();
        info!(
            "read_wav: {}ch {}Hz {}bit {:?}",
            spec.channels, spec.sample_rate, spec.bits_per_sample, spec.sample_format
        );

        if spec.sample_rate != 16000 {
            return Err(AutoSubError::AudioExtract(format!(
                "Expected 16kHz, got {}Hz. FFmpeg conversion failed?",
                spec.sample_rate
            )));
        }

        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Int => {
                let max = (1i64 << (spec.bits_per_sample - 1)) as f32;
                r.samples::<i32>().filter_map(|s| s.ok()).map(|s| s as f32 / max).collect()
            }
            hound::SampleFormat::Float => {
                r.samples::<f32>().filter_map(|s| s.ok()).collect()
            }
        };

        info!("read_wav: {} samples ({:.1}s)", samples.len(), samples.len() as f32 / 16000.0);
        Ok(samples)
    })
    .await
    .map_err(|e| AutoSubError::AudioExtract(format!("WAV read panicked: {}", e)))?
}

// ── SRT parser ────────────────────────────────────────────────────────────────

fn parse_srt(srt: &str) -> Vec<subtitle::Segment> {
    let mut out = Vec::new();
    for block in srt.trim().split("\n\n") {
        let lines: Vec<&str> = block.lines().collect();
        if lines.len() < 3 { continue; }
        let parts: Vec<&str> = lines[1].split(" --> ").collect();
        if parts.len() != 2 { continue; }
        out.push(subtitle::Segment {
            start: parse_ts(parts[0].trim()),
            end:   parse_ts(parts[1].trim()),
            text:  lines[2..].join("\n"),
        });
    }
    out
}

fn parse_ts(ts: &str) -> f32 {
    let (hms, ms) = ts.split_once(',').unwrap_or((ts, "0"));
    let p: Vec<f32> = hms.split(':').filter_map(|x| x.parse().ok()).collect();
    let ms: f32 = ms.parse().unwrap_or(0.0) / 1000.0;
    if p.len() == 3 { p[0] * 3600.0 + p[1] * 60.0 + p[2] + ms } else { 0.0 }
}
