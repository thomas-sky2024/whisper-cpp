use std::path::PathBuf;
use std::fs;
use std::sync::Mutex;
use std::collections::HashSet;
use log::{debug, warn, info};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use futures_util::StreamExt;
use lazy_static::lazy_static;

lazy_static! {
    /// Theo dõi các model đang được tải để tránh tải trùng lặp
    static ref ACTIVE_DOWNLOADS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub size_str: String,
    pub filename: String,
    pub is_downloaded: bool,
}

/// Danh sách model với URL tải trực tiếp từ Hugging Face
struct ModelSpec {
    id: &'static str,
    filename: &'static str,
    url: &'static str,
}

fn model_specs() -> Vec<ModelSpec> {
    vec![
        ModelSpec {
            id: "large_v3_turbo_q8",
            filename: "ggml-large-v3-turbo-q8_0.bin",
            url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3-turbo-q8_0.bin",
        },
        ModelSpec {
            id: "large_v3_q5",
            filename: "ggml-large-v3-q5_0.bin",
            url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3-q5_0.bin",
        },
        ModelSpec {
            id: "large_v2_q8",
            filename: "ggml-large-v2-q8_0.bin",
            url: "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v2-q8_0.bin",
        },
        ModelSpec {
            id: "silero_vad",
            filename: "silero_vad.onnx",
            url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/silero_vad.onnx",
        },
    ]
}

pub struct ModelManager;

impl ModelManager {
    /// Returns the canonical absolute path to the models directory.
    /// Uses standard OS AppData directory to comply with system conventions.
    fn models_directory() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let autosub_dir = home.join(".autosub");
        let models_dir = autosub_dir.join("models");

        // [MIGRATION LOGIC] Automatically migrate models from old Application Support location to ~/.autosub/models
        if let Some(data_dir) = dirs::data_local_dir() {
            let old_data_dir = data_dir.join("com.autosub.app");
            let old_models_dir = old_data_dir.join("models");

            if old_models_dir.exists() && !models_dir.exists() {
                log::info!("Migrating models from {:?} to {:?}", old_models_dir, models_dir);
                let _ = fs::create_dir_all(&autosub_dir);
                if let Err(e) = fs::rename(&old_models_dir, &models_dir) {
                    warn!("Failed to migrate models: {}", e);
                }
            }
        }

        if !models_dir.exists() {
            let _ = fs::create_dir_all(&models_dir);
        }

        debug!("model_manager: models_directory -> {:?}", models_dir);
        models_dir
    }


    /// Returns a centralized list of supported model specifications for this hardware (Mac Intel i9 + AMD 555X).
    pub fn get_available_models() -> Vec<ModelItem> {
        vec![
            ModelItem {
                id: "large_v3_turbo_q8".into(),
                name: "Large V3 Turbo (Q8_0)".into(),
                description: "Cao cấp nhất: Độ chính xác cực cao, tối ưu cho Mac i9 32GB RAM. Cần RAM lớn.".into(),
                size_str: "893 MB".into(),
                filename: "ggml-large-v3-turbo-q8_0.bin".into(),
                is_downloaded: false,
            },
            ModelItem {
                id: "large_v3_q5".into(),
                name: "Large V3 (Q5_0)".into(),
                description: "Cân bằng: Model Large V3 bản gốc, độ nén Q5. Rất ổn định cho tiếng Việt phức tạp.".into(),
                size_str: "1.08 GB".into(),
                filename: "ggml-large-v3-q5_0.bin".into(),
                is_downloaded: false,
            },
            ModelItem {
                id: "large_v2_q8".into(),
                name: "Large V2 (Q8_0)".into(),
                description: "Chuyên sâu: Model V2 nổi tiếng về khả năng bám đuôi timestamps, bản Q8 cực nặng.".into(),
                size_str: "1.58 GB".into(),
                filename: "ggml-large-v2-q8_0.bin".into(),
                is_downloaded: false,
            },
        ]
    }

    /// Returns the absolute path to a model file.
    pub fn get_model_path(model_name: &str) -> PathBuf {
        let models_dir = Self::models_directory();
        let trimmed = model_name.trim();

        // 1. Check if it's already a full existing path in models_dir
        let direct_path = models_dir.join(trimmed);
        if direct_path.exists() {
            return direct_path;
        }

        // 2. Normalize by removing common prefixes/suffixes if present
        let base = trimmed
            .trim_start_matches("ggml-")
            .trim_end_matches(".bin");

        // 3. Try common variants
        let variants = vec![
            format!("ggml-{}.bin", base),
            format!("{}.bin", base),
            trimmed.to_string(),
        ];

        for variant in variants {
            let p = models_dir.join(&variant);
            if p.exists() {
                return p;
            }
        }

        // Fallback to the most standard whisper.cpp format
        models_dir.join(format!("ggml-{}.bin", base))
    }

    /// Verifies if a model exists and has a minimum size.
    pub fn verify_model(model_name: &str) -> bool {
        let path = Self::get_model_path(model_name);
        if !path.exists() {
            return false;
        }
        if let Ok(metadata) = fs::metadata(&path) {
            return metadata.len() > 10 * 1024 * 1024;
        }
        false
    }

    /// Returns a list of all verified models in the models directory.
    pub fn list_models() -> Vec<String> {
        let models_dir = Self::models_directory();
        let mut models = Vec::new();

        if let Ok(entries) = fs::read_dir(&models_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("ggml-") && name.ends_with(".bin") {
                    let model_name = name
                        .trim_start_matches("ggml-")
                        .trim_end_matches(".bin")
                        .to_string();
                    if Self::verify_model(&model_name) {
                        models.push(model_name);
                    }
                }
            }
        }

        models
    }

    pub fn get_vad_model_path() -> PathBuf {
        let models_dir = Self::models_directory();
        models_dir.join("silero_vad.onnx")
    }

    pub fn vad_model_ready() -> bool {
        let path = Self::get_vad_model_path();
        if let Ok(metadata) = fs::metadata(&path) {
            // Silero VAD v4/v5 ranges from 1MB to 2.5MB, 
            // but some quantized versions might be smaller.
            return metadata.len() > 500 * 1024;
        }
        false
    }

    pub fn get_models_dir() -> String {
        Self::models_directory().to_string_lossy().to_string()
    }

    pub fn get_demucs_model_path() -> PathBuf {
        let models_dir = Self::models_directory();
        let variants = vec![
            "ggml-model-htdemucs-4s-f16.bin",
            "ggml-model-htdemucs-4s.bin",
            "ggml-htdemucs-4s.bin",
            "ggml-demucs.bin",
            "demucs.bin",
        ];
        for variant in variants {
            let path = models_dir.join(variant);
            if path.exists() {
                return path;
            }
        }
        models_dir.join("ggml-model-htdemucs-4s-f16.bin")
    }

    #[allow(dead_code)]
    pub fn demucs_model_ready() -> bool {
        let path = Self::get_demucs_model_path();
        if let Ok(metadata) = fs::metadata(&path) {
            return metadata.len() > 50 * 1024 * 1024;
        }
        false
    }
}

// ── Tauri Commands ────────────────────────────────────────────────────────────

#[tauri::command]
pub fn open_models_dir(app: AppHandle) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    let path = ModelManager::get_models_dir();
    app.opener().open_path(path, None::<&str>).map_err(|e| e.to_string())
}

/// Lấy trạng thái đã tải của 3 model tinh hoa
#[tauri::command]
pub fn get_models_status(_app: AppHandle) -> Result<Vec<ModelItem>, String> {
    let models_dir = ModelManager::models_directory();
    let mut available_models = ModelManager::get_available_models();

    for model in available_models.iter_mut() {
        let file_path = models_dir.join(&model.filename);
        model.is_downloaded = file_path.exists() && {
            fs::metadata(&file_path)
                .map(|m| m.len() > 10 * 1024 * 1024)
                .unwrap_or(false)
        };
    }

    Ok(available_models)
}

/// Event payload gửi về frontend khi đang tải model
#[derive(Clone, Serialize)]
pub struct ModelDownloadProgress {
    pub model_id: String,
    pub downloaded_mb: f64,
    pub total_mb: f64,
    pub percent: f64,
    pub speed_mbps: f64,
    pub done: bool,
    pub error: Option<String>,
}

/// Tải model từ Hugging Face, phát event tiến độ về frontend
#[tauri::command]
pub async fn download_model_by_id(model_id: String, app: AppHandle) -> Result<(), String> {
    // Tìm spec của model từ danh sách tập trung
    let specs = model_specs();
    let spec = specs.iter().find(|s| s.id == model_id)
        .or_else(|| {
              // Fallback check against ModelManager's available models if not in hardcoded specs
              // This is a safety measure to ensure consistency
              None
        })
        .ok_or_else(|| format!("Không tìm thấy model spec: {}", model_id))?;

    // KIỂM TRA ĐANG TẢI (TRÁNH TRANH CHẤP)
    {
        let mut active = ACTIVE_DOWNLOADS.lock().map_err(|_| "Lỗi lock ACTIVE_DOWNLOADS".to_string())?;
        if active.contains(&model_id) {
            warn!("model_manager: {} đang được tải rồi, bỏ qua yêu cầu mới", model_id);
            return Err(format!("Model {} đang được tải, vui lòng đợi.", model_id));
        }
        active.insert(model_id.clone());
    }

    let models_dir = ModelManager::models_directory();
    fs::create_dir_all(&models_dir)
        .map_err(|e| format!("Không thể tạo thư mục models: {}", e))?;

    let dest_path = models_dir.join(spec.filename);
    let tmp_path = models_dir.join(format!("{}.tmp", spec.filename));

    // Nếu đã tồn tại và hợp lệ thì bỏ qua
    if dest_path.exists() {
        if let Ok(meta) = fs::metadata(&dest_path) {
            if meta.len() > 10 * 1024 * 1024 {
                info!("model_manager: {} đã tồn tại, bỏ qua tải", spec.filename);
                let _ = app.emit("model-download-progress", ModelDownloadProgress {
                    model_id: model_id.clone(),
                    downloaded_mb: 0.0,
                    total_mb: 0.0,
                    percent: 100.0,
                    speed_mbps: 0.0,
                    done: true,
                    error: None,
                });
                return Ok(());
            }
        }
    }

    info!("model_manager: bắt đầu tải {} từ {}", spec.filename, spec.url);

    let url = spec.url.to_string();
    let model_id_clone = model_id.clone();
    let app_clone = app.clone();

    // Chạy download trong task bất đồng bộ
    tokio::spawn(async move {
        let result = download_file_with_progress(
            &url,
            &tmp_path,
            &dest_path,
            &model_id_clone,
            app_clone.clone(),
        ).await;

        // GIẢI PHÓNG TRẠNG THÁI ĐANG TẢI
        {
            if let Ok(mut active) = ACTIVE_DOWNLOADS.lock() {
                active.remove(&model_id_clone);
            }
        }

        match result {
            Ok(_) => {
                info!("model_manager: tải xong {}", model_id_clone);
                let _ = app_clone.emit("model-download-progress", ModelDownloadProgress {
                    model_id: model_id_clone,
                    downloaded_mb: 0.0,
                    total_mb: 0.0,
                    percent: 100.0,
                    speed_mbps: 0.0,
                    done: true,
                    error: None,
                });
            }
            Err(e) => {
                warn!("model_manager: lỗi tải {}: {}", model_id_clone, e);
                // Xóa file tmp nếu lỗi
                let _ = fs::remove_file(&tmp_path);
                let _ = app_clone.emit("model-download-progress", ModelDownloadProgress {
                    model_id: model_id_clone,
                    downloaded_mb: 0.0,
                    total_mb: 0.0,
                    percent: 0.0,
                    speed_mbps: 0.0,
                    done: true,
                    error: Some(e),
                });
            }
        }
    });

    Ok(())
}

async fn download_file_with_progress(
    url: &str,
    tmp_path: &PathBuf,
    dest_path: &PathBuf,
    model_id: &str,
    app: AppHandle,
) -> Result<(), String> {
    use std::io::Write;
    use std::time::Instant;

    let client = reqwest::Client::builder()
        .user_agent("AutoSub/0.1")
        .build()
        .map_err(|e| e.to_string())?;

    let response = client.get(url)
        .send()
        .await
        .map_err(|e| format!("Lỗi kết nối: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}: {}", response.status(), url));
    }

    let total_bytes = response.content_length().unwrap_or(0);
    let total_mb = total_bytes as f64 / 1024.0 / 1024.0;

    // Mở file tmp để ghi
    let mut file = fs::File::create(tmp_path)
        .map_err(|e| format!("Không thể tạo file tạm: {}", e))?;

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    let start_time = Instant::now();
    let mut last_emit = Instant::now();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Lỗi đọc dữ liệu: {}", e))?;
        file.write_all(&chunk)
            .map_err(|e| format!("Lỗi ghi file: {}", e))?;
        downloaded += chunk.len() as u64;

        // Gửi progress mỗi 300ms để không spam event
        if last_emit.elapsed().as_millis() >= 300 {
            let elapsed = start_time.elapsed().as_secs_f64();
            let downloaded_mb = downloaded as f64 / 1024.0 / 1024.0;
            let speed_mbps = if elapsed > 0.0 { downloaded_mb / elapsed } else { 0.0 };
            let percent = if total_bytes > 0 {
                (downloaded as f64 / total_bytes as f64) * 100.0
            } else {
                0.0
            };

            let _ = app.emit("model-download-progress", ModelDownloadProgress {
                model_id: model_id.to_string(),
                downloaded_mb,
                total_mb,
                percent,
                speed_mbps,
                done: false,
                error: None,
            });

            last_emit = Instant::now();
        }
    }

    // Flush và đóng file trước khi rename
    file.flush().map_err(|e| format!("Lỗi flush file: {}", e))?;
    drop(file);

    // Đảm bảo thư mục đích tồn tại
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Không thể tạo thư mục đích: {}", e))?;
    }

    // Thử rename trước (nhanh hơn), fallback sang copy+delete nếu cross-device
    if fs::rename(tmp_path, dest_path).is_err() {
        fs::copy(tmp_path, dest_path)
            .map_err(|e| format!("Không thể copy file: {}", e))?;
        let _ = fs::remove_file(tmp_path);
    }

    Ok(())
}

/// Tải VAD model (silero_vad2.onnx) — dùng chung logic
#[tauri::command]
pub async fn download_vad_model(app: AppHandle) -> Result<(), String> {
    download_model_by_id("silero_vad".to_string(), app).await
}
