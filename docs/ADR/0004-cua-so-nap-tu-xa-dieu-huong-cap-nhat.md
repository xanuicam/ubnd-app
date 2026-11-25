# ADR-0004 - Cửa sổ nạp từ xa, chính sách điều hướng và cập nhật có ký (pha A1)

- Ngày: 26/11/2026
- Trạng thái: Chấp nhận

## Bối cảnh

ADR-0001 chốt vỏ nạp thẳng `https://dieuhanh.xanuicam.vn`; ADR-0002/0003 chốt chỉ Windows 10/11 x64 + ARM64 cho
desktop. Cần chốt: trang web trong cửa sổ được làm gì, liên kết đi đâu, và bản mới tới máy người dùng ra sao.

## Quyết định

1. **Không IPC cho trang web.** `app.security.capabilities = []`, `withGlobalTauri = false`. Mọi việc của vỏ (cửa sổ,
   cập nhật, mở liên kết) chạy bằng Rust; trang `dieuhanh.` có bị chèn mã cũng không gọi được lệnh nào của vỏ.
   Cửa sổ tạo bằng mã (`windows: []`) để gắn được bộ xử lý điều hướng.
2. **Chính sách điều hướng** (`src-tauri/src/dieu_huong.rs`, có test):
   - Nạp trong cửa sổ: CHỈ `https://dieuhanh.xanuicam.vn` và `https://id.xanuicam.vn` (so khớp tuyệt đối host + cổng
     mặc định, không thông tin đăng nhập trong URL), `blob:` của hai origin đó, `about:blank|srcdoc`.
   - Nguồn nhúng theo CSP `frame-src` của web (YouTube nocookie, Office Viewer, Cloudflare Turnstile) được cho qua, vì
     wry trên macOS/Linux/di động báo cả iframe vào cùng bộ xử lý; Windows chỉ báo khung chính.
   - `http(s)` khác, `mailto:`, `tel:` - chặn trong cửa sổ, mở bằng ứng dụng của hệ điều hành. Mọi scheme khác (`file:`,
     `ms-msdt:`, `data:`, `javascript:`...) bị chặn hẳn.
   - `window.open`/`target=_blank` tới hai origin của hệ thống: cửa sổ con cùng phiên (giữ đăng nhập); còn lại ra
     trình duyệt.
3. **Cập nhật**: `tauri-plugin-updater`, chỉ Windows. Endpoint duy nhất
   `https://github.com/xanuicam/ubnd-app/releases/latest/download/latest.json`; gói phải ký bằng khoá có nửa công khai
   trong `tauri.conf.json`. Kiểm 8 giây sau khi mở, HỎI trước khi cài (cán bộ có thể đang soạn dở). Trình cài NSIS
   `passive`, cài theo người dùng (không cần quyền quản trị).
4. **Phát hành**: thẻ `vX.Y.Z` ⇒ build hai đích trên runner Windows (ARM64 biên dịch chéo) bằng `tauri build` trực
   tiếp, một job cuối ghép `latest.json` cho cả `windows-x86_64` lẫn `windows-aarch64` rồi tạo bản phát hành **nháp**.
   Không dùng `tauri-action` ở ma trận: hai job song song cùng sửa `latest.json` của một bản phát hành thì job sau đè
   job trước. Bản nháp không hiện ở `releases/latest` ⇒ chủ dự án bấm Publish là điểm duyệt cuối.
5. **Tên định danh**: `vn.xanuicam.dieuhanh` (thư mục dữ liệu WebView2 + cookie). Đổi = mất phiên đăng nhập và cài
   song song hai app - coi như bất biến.

## Hệ quả

- Thêm nguồn nhúng mới ở web (`frame-src`) phải thêm vào `NGUON_NHUNG` cùng đợt, nếu không bản di động (A2/A3) chặn nó.
- App cần mạng; mất mạng thấy trang lỗi của WebView2 (trang ngoại tuyến riêng để pha sau).
- Cửa sổ con (`NewWindowResponse::Allow`) không mang chính sách điều hướng của cửa sổ chính - chấp nhận vì chỉ mở được
  cho hai origin của hệ thống.
- Bộ cài chưa ký Authenticode tới khi có chứng thư: SmartScreen cảnh báo lần đầu.
