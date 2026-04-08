//! Native Voice Activity Detection (VAD) implementation using sherpa-onnx (Silero VAD).
//!
//! PHASE 1: Native VAD replacing FFmpeg's silenceremove filter and out-of-process checks.
//!
//! Why? 
//! - Native VAD is much faster and runs in RAM.
//! - Remaps timestamps correctly so we can transcribe only speech parts
//!   while keeping original movie timeline.

use crate::error::{AutoSubError, Result};
use sherpa_onnx::{SileroVadModelConfig, VadModelConfig, VoiceActivityDetector};

/// Config for VAD processing.
#[derive(Debug, Clone)]
pub struct VadConfig {
    pub model_path: String,
    pub threshold: f32,
    pub min_silence_ms: u32,
}

impl Default for VadConfig {
    fn default() -> Self {
        Self {
            model_path: "".to_string(),
            threshold: 0.5,
            min_silence_ms: 300,
        }
    }
}

/// A speech segment emitted by VAD.
#[derive(Debug, Clone)]
pub struct SpeechSegment {
    pub start_ms: u32,
    pub end_ms: u32,
    pub samples: Vec<f32>,
}

/// Output from VAD pass.
#[derive(Debug, Clone)]
pub struct VadOutput {
    /// Speech segments found by VAD
    pub segments: Vec<SpeechSegment>,
}

/// Processes raw PCM samples through Silero VAD.
/// 
/// `samples`: Input PCM data (Normalized f32, 16kHz, mono).
/// `config`: VAD parameters.
pub fn process(samples: &[f32], config: &VadConfig, sample_rate: u32) -> Result<VadOutput> {
    if samples.is_empty() {
        return Ok(VadOutput {
            segments: vec![]
        });
    }

    // ── Khởi tạo Silero VAD ──────────────────────────────────────────────────
    let silero_config = SileroVadModelConfig {
        model: Some(config.model_path.clone()),
        threshold: config.threshold,
        min_silence_duration: config.min_silence_ms as f32 / 1000.0,
        min_speech_duration: 0.1, // Bỏ qua speech < 100ms
        window_size: 512,
        max_speech_duration: 30.0,
    };

    let vad_config = VadModelConfig {
        silero_vad: silero_config,
        sample_rate: sample_rate as i32,
        num_threads: 1,
        provider: Some("cpu".to_string()),
        ..Default::default()
    };

    let vad = VoiceActivityDetector::create(&vad_config, 30.0)
        .ok_or_else(|| AutoSubError::Vad("Failed to create VAD: Detector instance is null".to_string()))?;

    // ── Placeholder cho speech segments ─────────────────────────────────────
    let mut speech_segments: Vec<SpeechSegment> = Vec::new();
    let samples_per_ms = sample_rate as f32 / 1000.0;

    // ── Xử lý theo từng window (512 samples) ─────────────────────────────────
    let window_size = 512usize;
    for chunk in samples.chunks(window_size) {
        if chunk.len() < window_size {
            break; // Thường samples.len() là bội số của 512 nếu từ ffmpeg tốt
        }

        vad.accept_waveform(chunk);

        // Fetch segment từ VAD buffer nếu có
        while let Some(speech_event) = vad.front() {
            let start = speech_event.start() as u32;
            let samples = speech_event.samples();
            let end = start + samples.len() as u32;

            speech_segments.push(SpeechSegment {
                start_ms: (start as f32 / samples_per_ms) as u32,
                end_ms: (end as f32 / samples_per_ms) as u32,
                samples: samples.to_vec(),
            });

            vad.pop();
        }
    }

    // Flush segment cuối nếu còn
    vad.flush();
    while let Some(speech_event) = vad.front() {
        let start = speech_event.start() as u32;
        let samples = speech_event.samples();
        let end = start + samples.len() as u32;

        speech_segments.push(SpeechSegment {
            start_ms: (start as f32 / samples_per_ms) as u32,
            end_ms: (end as f32 / samples_per_ms) as u32,
            samples: samples.to_vec(),
        });
        vad.pop();
    }

    Ok(VadOutput {
        segments: speech_segments,
    })
}
