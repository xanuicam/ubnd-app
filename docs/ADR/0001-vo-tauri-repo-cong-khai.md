# ADR-0001 - Vỏ Tauri 2 trong một repo công khai riêng

- Ngày: 26/11/2026
- Trạng thái: Chấp nhận (chủ dự án chốt 25/11/2026, đồng ý tạo repo công khai 26/11/2026)

## Bối cảnh

Hệ thống điều hành UBND xã Núi Cấm là ứng dụng web (React + Vite, backend Express) đã chạy thật. Cần bản cài đặt cho
Windows, macOS, Linux, Android, iOS. Repo chính là riêng tư và đã hết phút GitHub Actions miễn phí (không có kinh phí
trả thêm), nên không build được bản cài đa nền tảng ở đó - nhất là macOS/iOS cần máy chủ macOS.

## Quyết định

1. **Tauri 2** bọc hệ thống web, không viết lại giao diện (Flutter/React Native buộc viết lại hàng trăm trang).
   Vỏ viết bằng **Rust** (an toàn bộ nhớ, nhị phân nhỏ, dùng WebView của hệ điều hành).
2. **Repo công khai riêng** `xanuicam/ubnd-app` chỉ chứa vỏ + workflow: được Actions miễn phí cho mọi hệ điều hành.
3. Pha đầu vỏ **nạp thẳng** `https://dieuhanh.xanuicam.vn` (đăng nhập qua `id.xanuicam.vn` như trên web). Đóng gói
   giao diện vào app + OIDC/PKCE để lại cho pha sau, có ADR riêng.
4. Khoá ký chỉ ở GitHub Secrets; bản phát hành và gói cập nhật đều ký.

## Hệ quả

- Tốt: một nguồn giao diện cho web + mọi app; sửa web là app có ngay; CI miễn phí.
- Phải giữ: repo công khai nên luật ở `AGENTS.md` (không bí mật, không dữ liệu thật, không mã nghiệp vụ) là bắt buộc;
  luật bảo vệ nhánh + quét bí mật + push protection bật từ ngày đầu.
- Rủi ro: app phụ thuộc mạng (không chạy offline) cho tới pha đóng gói giao diện.
