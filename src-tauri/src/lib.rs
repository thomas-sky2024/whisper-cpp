//! lib.rs v2 — tích hợp VAD native + Whisper native.
//!
//! Thay đổi so với v1:
//! - AppState → AppStateV2 (thêm whisper_engine: Arc<Mutex<Option<WhisperEngine>>>)
//! - pipeline::run → pipeline_v2::run
//! - Thêm module vad và whisper_native
//! - Frontend commands giữ nguyên 100% — không cần đổi TypeScript

mod cache;
mod demucs;
mod downloader;
mod error;
mod ffmpeg;
mod job_manager;
mod model_manager;
mod pipeline_v2;      // THAY pipeline → pipeline_v2
mod post_process;
mod subtitle;
mod subtitle_sync;
mod thermal;
mod utils;
mod validator;
mod vad;              // MỚI: native VAD module
mod whisper_native;   // MỚI: native Whisper module

// Unused JobManager import removed
use pipeline_v2::{AppStateV2, PipelineOptions, PipelineResult};
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_shell::ShellExt;

/// Shared app state v2 — thread-safe, giữ Whisper model trong RAM.
pub struct AppState {
    pub inner: Arc<AppStateV2>,
}

// ── Tauri Commands (giữ nguyên signature — frontend tương thích) ──────────────

#[tauri::command]
async fn start_pipeline(
    app: AppHandle,
    state: State<'_, AppState>,
    opts: PipelineOptions,
) -> Result<PipelineResult, error::AutoSubError> {
    let jm = state.inner.job_manager.clone();
    jm.reset();
    
    let app_handle = app.clone();
    let state_inner = state.inner.clone();
    let jm_task = jm.clone();
    
    let task = tokio::spawn(async move {
        pipeline_v2::run(app_handle, opts, jm_task, state_inner).await
    });
    
    {
        let mut active = jm.active_task.lock().unwrap();
        *active = Some(task.abort_handle());
    }
    
    let result = task.await;
    
    // Clear the task handle
    {
        let mut active = jm.active_task.lock().unwrap();
        *active = None;
    }
    
    match result {
        Ok(res) => res,
        Err(e) if e.is_cancelled() => Err(error::AutoSubError::Cancelled),
        Err(e) => Err(error::AutoSubError::WhisperDecode(format!("Task error: {}", e))),
    }
}

#[tauri::command]
async fn cancel_job(state: State<'_, AppState>) -> Result<(), error::AutoSubError> {
    state.inner.job_manager.cancel();
    Ok(())
}

#[tauri::command]
async fn get_job_state(
    state: State<'_, AppState>,
) -> Result<job_manager::JobState, error::AutoSubError> {
    Ok(state.inner.job_manager.state())
}

#[tauri::command]
async fn check_model(model_name: String) -> Result<bool, error::AutoSubError> {
    Ok(model_manager::ModelManager::verify_model(&model_name))
}

#[tauri::command]
async fn list_models() -> Result<Vec<String>, error::AutoSubError> {
    Ok(model_manager::ModelManager::list_models())
}

#[derive(serde::Serialize)]
pub struct EnvironmentAudit {
    pub ffmpeg: bool,
    pub whisper: bool,   // true nếu whisper-rs model loaded, hoặc sidecar exists
    pub ytdlp: bool,
    pub models_dir: String,
    pub vad_ready: bool, // MỚI: trạng thái VAD model
}

#[tauri::command]
async fn audit_environment(
    app: AppHandle,
    _state: State<'_, AppState>,
) -> Result<EnvironmentAudit, error::AutoSubError> {
    let ffmpeg_ok = app.shell().sidecar("ffmpeg").is_ok();
    let ytdlp_ok = app.shell().sidecar("yt-dlp").is_ok();
    let vad_ready = model_manager::ModelManager::vad_model_ready();

    // Whisper: kiểm tra có model nào không (native engine)
    let whisper_ok = !model_manager::ModelManager::list_models().is_empty();

    Ok(EnvironmentAudit {
        ffmpeg: ffmpeg_ok,
        whisper: whisper_ok,
        ytdlp: ytdlp_ok,
        models_dir: model_manager::ModelManager::get_models_dir(),
        vad_ready,
    })
}

/// Unload whisper model khỏi RAM (dùng khi user muốn giải phóng memory).
#[tauri::command]
async fn unload_model(state: State<'_, AppState>) -> Result<(), error::AutoSubError> {
    let mut engine = state.inner.whisper_engine.lock().await;
    *engine = None;
    let mut loaded = state.inner.loaded_model.lock().await;
    loaded.clear();
    log::info!("lib: whisper model unloaded from RAM");
    Ok(())
}

#[tauri::command]
async fn apply_subtitle_sync(
    segments: Vec<subtitle::Segment>,
    start_idx: usize,
    shift_start_sec: f32,
    end_idx: usize,
    shift_end_sec: f32,
) -> Result<Vec<subtitle::Segment>, error::AutoSubError> {
    subtitle_sync::apply_point_sync(segments, start_idx, shift_start_sec, end_idx, shift_end_sec)
}

#[tauri::command]
async fn export_file(path: String, content: String) -> Result<(), error::AutoSubError> {
    tokio::fs::write(&path, content).await.map_err(|e| {
        error::AutoSubError::Export(format!("Failed to write file: {}", e))
    })
}

#[tauri::command]
async fn clear_cache() -> Result<(), error::AutoSubError> {
    cache::clear_all_cache()
}

// ── App Entry Point ───────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            inner: Arc::new(AppStateV2::new()),
        })
        .invoke_handler(tauri::generate_handler![
            start_pipeline,
            cancel_job,
            get_job_state,
            check_model,
            list_models,
            audit_environment,
            unload_model,
            export_file,
            apply_subtitle_sync,
            clear_cache,
            downloader::download_media,
            model_manager::get_models_status,
            model_manager::download_model_by_id,
            model_manager::download_vad_model,
            model_manager::open_models_dir,
        ]);

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building AutoSub v2");

    app.run(|app_handle, event| match event {
        tauri::RunEvent::ExitRequested { .. } => {
            log::info!("lib: ExitRequested, performing final cleanup");
            let state = app_handle.state::<AppState>();
            
            // 1. Tín hiệu dừng cho engine và pipeline
            state.inner.exit_flag.store(true, std::sync::atomic::Ordering::SeqCst);
            state.inner.job_manager.cancel();

            // 2. Giải phóng Whisper Engine (Metal resources)
            // Dùng blocking_lock vì chúng ta đang trong sync thread của tauri run
            log::info!("lib: dropping whisper engine");
            {
                let mut engine = state.inner.whisper_engine.blocking_lock();
                *engine = None;
            }
            {
                let mut loaded = state.inner.loaded_model.blocking_lock();
                loaded.clear();
            }

            // 3. Đợi một chút để spawn_blocking threads kịp thoát và drop Arc clones
            // Điều này cực kỳ quan trọng trên macOS để tránh ggml_metal crash khi process exit
            std::thread::sleep(std::time::Duration::from_millis(200));
            log::info!("lib: cleanup complete, exiting");
        }
        _ => {}
    });
}
