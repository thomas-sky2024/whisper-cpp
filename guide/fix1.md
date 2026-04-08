Dưới đây là bản tổng hợp toàn bộ các vấn đề và giải pháp để bạn tiện theo dõi và áp dụng vào mã nguồn của AutoSub.

1. App Crash khi thoát (Lỗi GGML/Metal SIGABRT)
Nguyên nhân: Race condition (điều kiện đua). Tiến trình chính gọi std::process::exit() để giải phóng GPU Metal, nhưng luồng nền (background thread) vẫn đang cố gắng khởi tạo tài nguyên đó.

Cách khắc phục (Code Backend):

Graceful Shutdown: Tuyệt đối không dùng process::exit() đột ngột.

Cờ tín hiệu (Flag): Dùng Arc<AtomicBool> để làm cờ exit_flag. Khi người dùng tắt app, bật cờ này lên để báo cho luồng Whisper/GGML biết.

Đồng bộ luồng: Sử dụng .join() hoặc .await để đợi luồng AI dọn dẹp và kết thúc hoàn toàn rồi mới cho tiến trình chính thoát.

Cập nhật dependencies: Đảm bảo whisper-rs và lõi ggml đang ở phiên bản mới nhất vì các bản cũ xử lý dọn dẹp Metal trên chip Apple Silicon khá kém.

2. Bấm "Dừng" nhưng tiến trình chạy lại (Logic Loop)
Nguyên nhân: Xung đột trạng thái (State) giữa UI và Backend, hoặc luồng (task) cũ chưa bị tiêu diệt hẳn nên khi quay về trạng thái ban đầu, nó tự động kích hoạt lại chu trình.

Cách khắc phục:

Hủy Task triệt để: Trong Rust (Tokio), khi nhận lệnh "Dừng", phải gọi trực tiếp handle.abort() để giết chết task đang chạy dở, tránh việc nó chạy xong một "chunk" âm thanh rồi tự động lặp lại.

Kiểm tra UI Side-Effects (Tauri/React): Đảm bảo hàm xử lý nút "Dừng" chỉ làm nhiệm vụ ngắt kết nối/đổi biến is_running = false, chứ không vô tình kích hoạt các hàm useEffect hoặc watcher gọi lại hàm start().

Reset State cẩn thận: Chỉ đưa các biến như index, progress về 0 sau khi đã xác nhận task AI dưới nền thực sự bị hủy.

3. Tải video bằng yt-dlp báo lỗi "Too Many Requests" (HTTP 429)
Nguyên nhân: IP local bị YouTube đánh dấu là bot do gửi yêu cầu tải xuống hàng loạt hoặc không có danh tính người dùng hợp lệ.

Cách khắc phục:

Sử dụng Cookies (Hiệu quả nhất): Bổ sung tham số --cookies-from-browser chrome (hoặc edge, firefox, safari) vào câu lệnh gọi yt-dlp.

Dùng file dự phòng: Nếu trình duyệt bị khóa hoặc lỗi quyền, cấp cho người dùng tùy chọn dẫn link đến file cookies.txt (tạo ra từ extension trình duyệt).

Cập nhật công cụ: Luôn gọi lệnh yt-dlp -U (hoặc nâng cấp qua pip) để có bản vá thuật toán chống bot mới nhất của YouTube.

Rate Limiting: Thêm tham số --sleep-interval 5 để tạo khoảng nghỉ ngẫu nhiên giữa các lần tải.

4. Lưu ý về Quyền hạn và UX cho máy khách (macOS)
Vấn đề: Khi mang máy qua cho khách dùng (hoặc khách tự cài), các tính năng như đọc Cookies từ trình duyệt sẽ bị macOS chặn.

Cách khắc phục:

Full Disk Access: Hướng dẫn khách hàng vào System Settings > Privacy & Security > Full Disk Access để cấp quyền cho app AutoSub.

Giao diện tùy chọn: Làm một menu dropdown trên UI để khách hàng tự chọn trình duyệt họ đang dùng (để pass đúng tham số browser cho lệnh yt-dlp), thay vì hardcode một trình duyệt duy nhất trong code.