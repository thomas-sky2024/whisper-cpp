#!/bin/bash
set -e

# This script downloads sidecar binaries for AutoSub
# Usage: ./src-tauri/setup-binaries.sh

BIN_DIR="src-tauri/binaries"
mkdir -p "$BIN_DIR"

echo "--- Setting up sidecar binaries ---"

# yt-dlp setup (macOS Universal)
ARCH=$(uname -m)
if [ "$ARCH" == "arm64" ]; then
    TRIPLE="aarch64-apple-darwin"
else
    TRIPLE="x86_64-apple-darwin"
fi

echo "   Setting up binaries for $TRIPLE..."

# 1. yt-dlp
YTDLP_PATH=$(which yt-dlp || echo "")
if [ -n "$YTDLP_PATH" ]; then
    echo "   Found yt-dlp at $YTDLP_PATH. Copying..."
    cp "$YTDLP_PATH" "$BIN_DIR/yt-dlp-$TRIPLE"
else
    echo "   Downloading yt-dlp..."
    curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos -o "$BIN_DIR/yt-dlp-$TRIPLE"
fi
chmod +x "$BIN_DIR/yt-dlp-$TRIPLE"

# 2. ffmpeg
FFMPEG_PATH=$(which ffmpeg || echo "")
if [ -n "$FFMPEG_PATH" ]; then
    echo "   Found ffmpeg at $FFMPEG_PATH. Copying..."
    cp "$FFMPEG_PATH" "$BIN_DIR/ffmpeg-$TRIPLE"
else
    echo "   WARNING: ffmpeg not found in system path! Please install it."
    # We might need a downloader here, but README says install it.
fi
[ -f "$BIN_DIR/ffmpeg-$TRIPLE" ] && chmod +x "$BIN_DIR/ffmpeg-$TRIPLE"

# 3. demucs-main (Stub for now)
if [ ! -f "$BIN_DIR/demucs-main-$TRIPLE" ]; then
    echo "   Creating stub for demucs-main..."
    echo "#!/bin/bash" > "$BIN_DIR/demucs-main-$TRIPLE"
    echo "echo 'Demucs stub called with: \$*'" >> "$BIN_DIR/demucs-main-$TRIPLE"
    echo "exit 0" >> "$BIN_DIR/demucs-main-$TRIPLE"
    chmod +x "$BIN_DIR/demucs-main-$TRIPLE"
fi

# 4. whisper-main (check)
if [ ! -f "$BIN_DIR/whisper-main-$TRIPLE" ]; then
    echo "   WARNING: whisper-main binary missing. Please run build-scripts/build-whisper.sh"
else
    echo "   whisper-main binary found at $BIN_DIR/whisper-main-$TRIPLE"
fi

echo "--- Sidecar binaries setup complete ---"
