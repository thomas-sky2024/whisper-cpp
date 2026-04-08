# 🎬 AutoSub — AI Subtitle Generator for macOS

[Tiếng Việt bên dưới](#tiếng-việt-🇻🇳)

AutoSub is a high-performance, native macOS application designed for professional subtitle generation. Optimized for **Mac Intel (2019-2020)** with **AMD GPUs**, it leverages **whisper.cpp**, **sherpa-onnx (VAD)**, and **Rust** to provide a fast, offline, and completely private transcription experience.

![AutoSub Hero](https://via.placeholder.com/1200x600.png?text=AutoSub+Premium+Native+UI)

## ✨ Key Features

- **Offline AI Transcription**: Powered by whisper.cpp with Metal acceleration. No API keys, no internet required.
- **Smart VAD Processing**: Zero-latency Silero VAD for accurate silence detection and chunk-based processing.
- **Compare & Sync Tab**: A unique tool to align AI-generated SRTs with original scripts using character-mapping algorithms.
- **Rich Format Export**: Export to SRT/TXT with high-accuracy timestamps and CJK-aware formatting.
- **Native Efficiency**: Built with Tauri v2 for minimal memory footprint and maximum performance on Mac Intel hardware.

---

## 🛠️ Hardware & Model Recommendations

For the best experience on **Mac Intel (AMD Radeon Pro 500/600 Series)**:

- **GPU Memory**: 4GB VRAM is tight but enough. 
- **Recommended Model**: Use **Whisper Large V3 Turbo (Quantized Q5_0 or Q4_0)**. It offers the best balance of speed and accuracy while staying under the 4GB VRAM limit.
- **Performance Mode**: Toggle between **Balanced** (8-core) and **MaxSpeed** (Full thread utilization) in the settings.

---

## 🚀 Installation (How to Bypass Gatekeeper)

Since AutoSub is an indie project, macOS might block it during initial launch. Follow these steps:

1. **Download** the `.dmg` from the [Latest Release](https://github.com/your-username/auto-sub/releases).
2. **Install**: Drag AutoSub to your **Applications** folder.
3. **Bypass Security**:
   - Open **Terminal** (CMD + Space, type `Terminal`).
   - Copy and paste the following command:
     ```bash
     xattr -cr /Applications/AutoSub.app
     ```
   - Press Enter.
4. **Launch**: You can now open AutoSub normally from Launchpad.

---

# Tiếng Việt 🇻🇳

AutoSub là ứng dụng native dành cho macOS, giúp tạo phụ đề tự động bằng AI với hiệu suất cao. Được tối ưu đặc biệt cho các dòng **Mac Intel (2019-2020)** sử dụng **GPU AMD**, ứng dụng kết hợp sức mạnh của **whisper.cpp**, **VAD (Voice Activity Detection)** và **Rust** để mang lại trải nghiệm biên dịch offline, nhanh chóng và bảo mật tuyệt đối.

## ✨ Tính năng nổi bật

- **Biên dịch AI Offline**: Sử dụng whisper.cpp với tăng tốc phần cứng Metal. Không cần mạng, không tốn phí API.
- **Lọc giọng nói thông minh (VAD)**: Tích hợp Silero VAD để chia nhỏ file âm thanh chính xác, tối ưu cho các video có nhiều khoảng lặng hoặc nhạc nền.
- **Tab So sánh & Đồng bộ**: Công cụ độc đáo giúp ghép chữ từ kịch bản gốc vào mốc thời gian (timestamp) của AI một cách chính xác tới từng ký tự.
- **Xuất file đa dạng**: Hỗ trợ SRT, TXT với định dạng chuẩn SEO và xử lý tốt tiếng Trung/Nhật/Hàn.

---

## 🚀 Hướng dẫn cài đặt & Vượt rào bảo mật Apple

Do ứng dụng chưa có chữ ký số trả phí của Apple, macOS sẽ báo lỗi "App is damaged" hoặc chặn không cho mở. Bạn chỉ cần thực hiện 1 thao tác nhỏ sau:

1. **Tải về** file `.dmg` từ mục [Releases](https://github.com/your-username/auto-sub/releases).
2. **Kéo thả** AutoSub vào thư mục **Applications**.
3. **Mở Quyền**:
   - Mở ứng dụng **Terminal** trên Mac.
   - Dán dòng lệnh sau và nhấn Enter:
     ```bash
     xattr -cr /Applications/AutoSub.app
     ```
4. **Bắt đầu**: Bây giờ bạn đã có thể mở AutoSub bình thường từ Launchpad.

---

## 📦 Build from Source (Dành cho Developer)

```bash
pnpm install
pnpm tauri dev
```

---

Built with ❤️ by [Thomas] for the professional subtitle editing community.
