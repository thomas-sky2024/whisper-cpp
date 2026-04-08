# AutoSub — Dev Log

Ghi nhận các thay đổi quan trọng, bug đã fix, và quyết định kỹ thuật.  
Log whisper inference bình thường không ghi ở đây — chỉ ghi bug/fix/decision có giá trị tham chiếu.

---

## [2026-04-05] Crash khi tắt app (Cmd+Q)

**Lỗi:** `GGML_ASSERT([rsets->data count] == 0)` trong `ggml_metal_rsets_free()`

**Nguyên nhân:** Khi app tắt qua `NSApplication terminate:`, macOS gọi destructor của Metal device. Lúc đó `WhisperContext` (giữ Metal GPU resources) vẫn còn alive bên trong `Arc<Mutex<Option<WhisperEngine>>>` trong `AppState`. Metal residency sets chưa được release → assertion fail.

**Fix:** Hook `on_window_event(Destroyed)` trong `lib.rs` để drop `whisper_engine = None` trước khi process exit. Dùng `window.state::<AppState>()` (Tauri 2.x) + `handle.block_on()` để đảm bảo drop đồng bộ.

**File:** `src-tauri/src/lib.rs`

---

## [2026-04-05] Crash giữa chừng khi dịch video có nhạc

**Lỗi:** `WHISPER_ASSERT: filter_width < a->ne[2]` tại `whisper.cpp:8772`

**Nguyên nhân:**  
1. VAD cắt audio → batch được gom lại  
2. Batch đầu tiên decode xong, whisper phát hiện `single timestamp ending` → skip toàn chunk, `seek` nhảy đến cuối window 30s  
3. Batch tiếp theo bắt đầu từ seek đó → audio còn lại rất ngắn  
4. DTW bật → `whisper_exp_compute_token_level_timestamps_dtw()` gọi với `medfilt_width=7` (hardcoded) nhưng `n_audio_tokens < 7` → assert crash

**Fix:** Pad mọi batch lên đúng 30 giây (`WHISPER_CHUNK_SAMPLES = 480000`) trước khi đưa vào whisper. Với 30s audio, `n_audio_tokens = 1500` >> 7.  
Giữ nguyên DTW — không tắt — vì DTW cần thiết cho timestamp chính xác theo từng từ.

**File:** `src-tauri/src/whisper_native.rs`

---

## [2026-04-05] Lỗi download model: "Không thể di chuyển file"

**Lỗi:** `os error 2 (No such file or directory)` khi rename `.tmp` → dest

**Nguyên nhân:** `fs::rename()` báo lỗi vì thiếu `flush()` trước khi drop file handle, và thư mục đích chưa được tạo đủ trước khi rename trong async context.

**Fix:**
1. Gọi `file.flush()` trước `drop(file)`
2. Gọi `fs::create_dir_all(parent)` trước rename
3. Fallback `copy + remove` nếu rename fail (tránh cross-device error)

**File:** `src-tauri/src/model_manager.rs`

---

## [2026-04-05] Tab Sync → Tab Compare

**Thay đổi:** Thay toàn bộ logic tab Sync (WaveSurfer 2-point sync) bằng tab Compare mới.

**Compare tab làm gì:**
- Input: kịch bản gốc (tiếng Trung) vs output AI (SRT)
- Dùng `diff-match-patch` để so sánh từng ký tự Hán
- Hiển thị chip tương tác: correction (sai), extra (thừa AI), missing (AI bỏ sót)
- Accept/reject từng lỗi → download TXT hoặc SRT (timestamp AI + chữ gốc)
- Nút mở browser → `purpleculture.net` chuyển Giản thể/Phồn thể
- Auto-fill AI output từ `$jobStore.srtContent`

**Lỗi TS khi tích hợp (đã fix):**
- `plugin-opener` export `openUrl` không phải `open`
- `$activeTab === 'sync'` trong template chưa đổi sang `'compare'`
- `$jobStore.status === 'downloading'` — `"downloading"` không có trong `JobStatus` type → đổi sang check `stage?.toLowerCase().includes('download')`

**Files:** `src/lib/CompareTab.svelte` (mới), `src/routes/+page.svelte`, `src/lib/jobStore.ts`

---

## [2026-04-05] Model path migration

**Thay đổi:** Model path đổi từ `~/.autosub/models` sang `~/Library/Application Support/com.autosub.app/models` (Tauri standard AppData dir).

**Migration logic:** Khi khởi động, nếu thư mục cũ tồn tại và thư mục mới chưa có → `fs::rename` tự động migrate (trong `models_directory()` của `ModelManager`).

**File:** `src-tauri/src/model_manager.rs`

---

## [Quyết định kỹ thuật]

**DTW timestamp:** Giữ DTW alignment (`DtwMode::ModelPreset`) vì cải thiện đáng kể độ chính xác timestamp theo từng từ. Fix crash bằng pad audio, không tắt DTW.

**VAD:** Dùng Silero VAD (`silero_vad2.onnx`) native qua ONNX Runtime. VAD cần thiết để batch grouping đúng — nếu thiếu VAD model, fallback chia đều 25s chunk (kém chính xác hơn).

**Whisper singleton:** `WhisperEngine` được giữ trong RAM (`Arc<Mutex<Option<WhisperEngine>>>`) để tránh load 3GB model mỗi job. Chỉ reload khi đổi model.

**no_speech_thold = 0.6:** Filter segment toàn nhạc/noise, tránh decode nhiễu thành text giả. Kết hợp với VAD là 2 lớp bảo vệ.
