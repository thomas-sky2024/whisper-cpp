use crate::error::{AutoSubError, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{command, AppHandle, Emitter, Manager};
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Deserialize)]
pub struct DownloadOptions {
    pub url: String,
    pub format: String, // "mp4", "mp3", or "wav"
    pub save_local: bool,
    pub output_dir: Option<PathBuf>,
    pub browser: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct DownloadResult {
    pub file_path: PathBuf,
    pub title: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum DownloadEvent {
    Progress {
        percentage: f32,
        speed: String,
        eta: String,
    },
}

#[command]
pub async fn download_media(app: AppHandle, opts: DownloadOptions) -> Result<DownloadResult> {
    let sidecar = app
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| AutoSubError::Download(format!("Failed to find yt-dlp sidecar: {}", e)))?;

    // Use app cache dir if no output dir provided
    let output_dir = opts.output_dir.unwrap_or_else(|| {
        app.path()
            .app_cache_dir()
            .expect("Failed to get cache dir")
            .join("downloads")
    });
    std::fs::create_dir_all(&output_dir)?;

    let mut args = vec![
        "--no-playlist",
        "--newline",
        "--progress",
        "--output",
        "%(title)s.%(ext)s",
        "--sleep-interval", "5",
    ];

    if let Some(ref browser) = opts.browser {
        if !browser.is_empty() {
             args.extend(["--cookies-from-browser", browser]);
        }
    }

    match opts.format.as_str() {
        "mp3" => {
            args.extend(["-x", "--audio-format", "mp3"]);
        }
        "mp4" => {
            args.extend([
                "-S",
                "ext:mp4:m4a",
                "--merge-output-format",
                "mp4",
            ]);
        }
        _ => {
            // Default to wav for transcription pipeline
            args.extend(["-x", "--audio-format", "wav", "--audio-quality", "0"]);
        }
    }

    args.push(&opts.url);

    let (mut rx, _child) = sidecar
        .args(args)
        .current_dir(&output_dir)
        .spawn()
        .map_err(|e| AutoSubError::Download(format!("Failed to spawn yt-dlp: {}", e)))?;

    let mut final_path = None;
    let mut title = String::from("Unknown");

    // Improved Regexes
    let re_progress = Regex::new(
        r"\[download\]\s+([\d.]+)\%\s+of\s+[\d.]+\w+\s+at\s+([\d.]+\w+/s)\s+ETA\s+([\d:]+)",
    )
    .unwrap();


    while let Some(event) = rx.recv().await {
        if let tauri_plugin_shell::process::CommandEvent::Stdout(line) = event {
            let line_str = String::from_utf8_lossy(&line);

            if let Some(caps) = re_progress.captures(&line_str) {
                let percentage = caps
                    .get(1)
                    .and_then(|m| m.as_str().parse::<f32>().ok())
                    .unwrap_or(0.0);
                let speed = caps
                    .get(2)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
                let eta = caps
                    .get(3)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();

                app.emit(
                    "download-progress",
                    DownloadEvent::Progress {
                        percentage,
                        speed,
                        eta,
                    },
                )
                .ok();
            } else if line_str.contains("Destination:") {
                let parts: Vec<&str> = line_str.splitn(2, "Destination:").collect();
                if parts.len() == 2 {
                    let path_str = parts[1].trim().trim_matches('"');
                    let path = PathBuf::from(path_str);
                    final_path = Some(output_dir.join(&path));
                    title = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| path_str.to_string());
                }
            } else if line_str.contains("has already been downloaded") {
                if let Some(start) = line_str.find(']') {
                    if let Some(end) = line_str.find("has already been downloaded") {
                        let path_str = line_str[start + 1..end].trim().trim_matches('"');
                        let path = PathBuf::from(path_str);
                        final_path = Some(output_dir.join(&path));
                        title = path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| path_str.to_string());
                    }
                }
            } else if line_str.contains("Merging formats into") {
                if let Some(start) = line_str.find("into \"") {
                    if let Some(end) = line_str[start + 6..].find('"') {
                        let path_str = &line_str[start + 6..start + 6 + end];
                        let path = PathBuf::from(path_str);
                        final_path = Some(output_dir.join(&path));
                        title = path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| path_str.to_string());
                    }
                }
            }
        } else if let tauri_plugin_shell::process::CommandEvent::Stderr(line) = event {
            let line_str = String::from_utf8_lossy(&line);
            log::warn!("yt-dlp stderr: {}", line_str.trim());
        } else if let tauri_plugin_shell::process::CommandEvent::Terminated(payload) = event {
            if payload.code != Some(0) {
                return Err(AutoSubError::Download(format!(
                    "yt-dlp exited with error code {:?}",
                    payload.code
                )));
            }
        }
    }

    if let Some(path) = final_path {
        // If we want to keep it in a specific location for the user (save_local)
        if opts.save_local {
            let download_dir = app.path().download_dir()?.join("AutoSub");
            std::fs::create_dir_all(&download_dir)?;
            let dest_name = path
                .file_name()
                .ok_or_else(|| AutoSubError::Download("Invalid filename".into()))?;
            let dest_path = download_dir.join(dest_name);
            std::fs::copy(&path, &dest_path)?;
            log::info!("Media saved to: {}", dest_path.display());
        }

        Ok(DownloadResult {
            file_path: path,
            title,
        })
    } else {
        Err(AutoSubError::Download(
            "Could not determine downloaded file path".into(),
        ))
    }
}
