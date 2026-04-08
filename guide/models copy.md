# Build Intel Mac DMG Implementation Plan

This plan outlines the steps to build a `.dmg` installer for Intel-based Macs (`x86_64-apple-darwin`) using the binaries provided.

## User Review Required

> [!IMPORTANT]
> **Missing ffprobe binary**: The code in `pipeline_v2.rs` attempts to use `ffprobe` as a sidecar, but it is currently missing from `src-tauri/binaries` and `tauri.conf.json`. I will add `ffmpeg` as a fallback or you may need to provide an `ffprobe` binary.
> [!WARNING]
> **Rust Target**: This process requires the `x86_64-apple-darwin` Rust target. I will attempt to add it, but it may require `sudo` if your environment is restricted.

## Proposed Changes

### 1. Sidecar Binary Preparation
Rename and organize the downloaded binaries in `src-tauri/binaries` to follow Tauri's naming convention (`<binary>-x86_64-apple-darwin`).

- [MODIFY] Rename `src-tauri/binaries/ffmpeg` to `src-tauri/binaries/ffmpeg-x86_64-apple-darwin`
- [MODIFY] Copy `src-tauri/binaries/yt-dlp_macos` to `src-tauri/binaries/yt-dlp-x86_64-apple-darwin`
- [NEW] Create a stub for `demucs-main-x86_64-apple-darwin` (if a real one isn't provided) -> không cần cài đặt nữa , bỏ ra khỏi config của tauri.

### 2. Configuration Updates
Update `tauri.conf.json` to ensure all sidecars are correctly registered.

#### [MODIFY] [tauri.conf.json](file:///Users/tt/Documents/vibe-coding/whisper/auto-sub/src-tauri/tauri.conf.json)
- Add `binaries/ffprobe` to `externalBin` (with fallback to ffmpeg if ffprobe is missing).

### 3. Build Command
Execute the build for the Intel target.

- Run `rustup target add x86_64-apple-darwin`
- Run `pnpm tauri build --target x86_64-apple-darwin`

## Open Questions
1. Do you have an `ffprobe` binary for Intel? If not, the app will try to use `ffmpeg` for duration detection.
2. Are there any other specific Intel binaries you downloaded (e.g., `whisper-main`) that need to be included as sidecars?

## Verification Plan

### Automated Tests
- Check if the build command completes successfully.
- Verify the generated `.dmg` exists in `src-tauri/target/x86_64-apple-darwin/release/bundle/dmg/`.

### Manual Verification
- The user will need to test the `.dmg` on an Intel Mac to ensure sidecars (ffmpeg, yt-dlp) are bundled and executable.
