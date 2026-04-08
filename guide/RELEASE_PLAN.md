# Kế hoạch xuất bản AutoSub

## Trạng thái hiện tại

| Hạng mục | Trạng thái |
|---|---|
| Core pipeline (VAD → Whisper → SRT) | ✅ Hoạt động |
| Model download + auto-migrate | ✅ Hoạt động |
| Tab Transcribe | ✅ Hoạt động |
| Tab Review | ✅ Hoạt động |
| Tab Compare | ✅ Hoạt động |
| Crash khi tắt app (Metal) | ✅ Đã fix |
| Crash video có nhạc (DTW) | ✅ Đã fix |
| Build release (pnpm tauri build) | ⬜ Chưa thử |
| App icon | ⬜ Chưa có |
| Code signing / notarization | ⬜ Chưa |
| Auto-updater | ⬜ Chưa |

---

## Giai đoạn 1 — Ổn định (làm trước khi build)

### 1.1 Kiểm tra các edge case còn lại

- [ ] Video không có tiếng nói (100% nhạc) → VAD trả về 0 segment → pipeline xử lý thế nào?
- [ ] File video lớn (>2 giờ) → memory và thời gian xử lý
- [ ] Mạng bị ngắt giữa chừng khi download model → file .tmp còn lại → lần sau có resume không?
- [ ] User mở 2 job cùng lúc → job_manager có handle đúng không?
- [ ] Model bị corrupt (size đúng nhưng nội dung sai) → whisper load fail → error message rõ ràng chưa?

### 1.2 UX còn thiếu

- [ ] **Thông báo khi xong:** Hiện tại chỉ cập nhật tab Review, không có notification rõ ràng
- [ ] **Nút mở thư mục output** sau khi export SRT/TXT thành công
- [ ] **Nhớ lại model đã chọn** qua các lần mở app (lưu vào `localStorage` hoặc Tauri store)
- [ ] **Hiển thị thời gian xử lý** (bắt đầu lúc nào, mất bao lâu)
- [ ] **Xử lý lỗi ở frontend rõ hơn:** Error hiện tại chỉ log console, không hiện UI

### 1.3 Cleanup code

- [ ] Xóa `SyncTab.svelte` (không còn dùng)
- [ ] Xóa `subtitle_sync.rs` nếu không còn backend command nào dùng
- [ ] Kiểm tra warnings còn lại từ `pnpm run check`
- [ ] Xóa log `whisper_full_with_state` verbose trong release build (set `WHISPER_DEBUG=0` hoặc compile flag)

---

## Giai đoạn 2 — Build release

### 2.1 Chuẩn bị assets

**App icon** (bắt buộc cho macOS):
```
src-tauri/icons/
  icon.png        (1024x1024, PNG)
  icon.icns       (macOS)
  icon.ico        (Windows)
```
Dùng `pnpm tauri icon icon.png` để tự generate tất cả kích thước.

**Bundle identifier** (kiểm tra `tauri.conf.json`):
```json
"identifier": "com.autosub.app"
```

### 2.2 Tắt debug output trong release

Trong `src-tauri/Cargo.toml`, kiểm tra profile release:
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

Trong `src-tauri/src/lib.rs`, `env_logger::init()` sẽ chỉ log `WARN+` trong release nếu không set `RUST_LOG`. Whisper log verbose (`whisper_full_with_state`) đến từ C++ — có thể suppress bằng cách thêm vào `build.rs`:
```rust
// Hoặc set trong tauri.conf.json beforeDevCommand / beforeBuildCommand
```

### 2.3 Build thử

```bash
# Kiểm tra build frontend trước
pnpm run build

# Build toàn bộ Tauri app
pnpm tauri build

# Output: src-tauri/target/release/bundle/
#   macos/    → .app + .dmg
#   dmg/      → installer
```

**Lưu ý macOS:** Build sẽ tạo `.app` và `.dmg`. Để distribute ngoài App Store cần:
- Apple Developer account ($99/năm)
- Code signing: `codesign --deep --sign "Developer ID Application: ..."` 
- Notarization: `xcrun notarytool submit app.dmg --wait`

Nếu chỉ distribute cho người dùng cụ thể (không qua App Store), có thể bỏ qua bước này — user cần tắt Gatekeeper hoặc right-click → Open lần đầu.

---

## Giai đoạn 3 — Distribution

### Option A: GitHub Releases (đơn giản nhất)

1. Tạo repo GitHub (private hoặc public)
2. Tag release: `git tag v1.0.0`
3. Upload `.dmg` lên GitHub Releases
4. User download và cài tay

**Không cần:** Code signing, notarization (user phải bypass Gatekeeper lần đầu)

### Option B: Tauri Updater (cho tương lai)

Tauri có built-in updater plugin. Khi có phiên bản mới:
- App tự check endpoint update
- Download và install tự động

Cần thêm `tauri-plugin-updater` và endpoint JSON chứa thông tin version.

### Option C: Mac App Store

Phức tạp nhất — cần:
- Apple Developer account
- Sandbox entitlements (hạn chế filesystem access)
- Review process (~1-7 ngày)

Không khuyến nghị cho lần đầu.

---

## Checklist trước khi release v1.0

### Must-have
- [ ] Không crash khi tắt app ✅
- [ ] Không crash khi dịch video có nhạc ✅
- [ ] Download model hoạt động ổn ✅
- [ ] Export SRT/TXT hoạt động
- [ ] App icon có mặt
- [ ] `pnpm tauri build` thành công không lỗi
- [ ] Test trên clean machine (chưa cài model, lần đầu chạy)

### Nice-to-have
- [ ] Notification khi xong
- [ ] Nhớ model đã chọn
- [ ] Thời gian xử lý hiển thị
- [ ] Suppress whisper verbose log trong release

### Skip cho v1.0
- Auto-updater
- App Store distribution
- Windows support
- Demucs vocal separation (tính năng beta)
