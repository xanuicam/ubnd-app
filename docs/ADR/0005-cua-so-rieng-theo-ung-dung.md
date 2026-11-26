# ADR-0005 - Mỗi ứng dụng của hệ thống một cửa sổ riêng, danh sách origin theo từng cửa sổ (pha A1b)

- Ngày: 26/11/2026
- Trạng thái: Chấp nhận
- Sửa đổi: ADR-0004 mục 2 (danh sách origin chung `ORIGIN_TRONG_APP`) - hệ quả cuối của ADR-0004 đã báo trước việc này.

## Bối cảnh

Chủ dự án chốt 26/11/2026: một app, màn "Ứng dụng của tôi" (trang `/ung-dung` của `dieuhanh.`, danh sách ô do máy chủ
tính theo quyền thật - `GET /api/auth/me/ung-dung` ở repo chính). Các ứng dụng `quantri.`, `dulieu.`, `khodulieu.`,
`trungtamdieuhanh.`, `tailieu.` phải mở được TRONG app, nhưng KHÔNG gộp origin: SSO của hệ thống dựa vào cookie
host-only theo từng origin, gộp các app vào một cửa sổ được phép hiển thị mọi origin là cho một lỗ XSS ở trang này chạy
trong khung mà người dùng tưởng là trang kia.

## Quyết định

1. **Danh sách origin theo cửa sổ** (`dieu_huong::CuaSo`):
   - Cửa sổ chính `main`: `dieuhanh.` + `id.` (như ADR-0004).
   - Cửa sổ ứng dụng `ud-<tên>`: CHỈ origin của ứng dụng đó + `id.` (đi SSO). Bảng `UNG_DUNG_CUA_SO_RIENG` là nguồn duy
     nhất (host, nhãn cửa sổ, tiêu đề).
2. **Chuyển cửa sổ thay vì nới danh sách**: điều hướng hoặc `target=_blank` tới một ứng dụng trong bảng ⇒ chặn ở cửa sổ
   hiện tại, mở (hoặc đưa lên trước) cửa sổ riêng của ứng dụng đó và nạp URL. Cửa sổ ứng dụng điều hướng tới `dieuhanh.` ⇒
   nạp vào cửa sổ chính. Bấm lại ô ở trang gốc không nạp lại cửa sổ đang mở (người dùng có thể đang soạn dở); liên kết
   sâu thì nạp.
3. **Không có cửa sổ riêng**: `thongtin.` (trang công khai - mở bằng trình duyệt), `kiosk.` (tách thành dự án riêng, ADR
   008 ở repo chính), mọi host khác. So khớp vẫn TUYỆT ĐỐI (https, đúng host, cổng mặc định, không thông tin đăng nhập).
4. **Tạo cửa sổ ở vòng lặp chính** (`run_on_main_thread`) sau khi bộ xử lý điều hướng trả về - không dựng cửa sổ ngay
   trong callback của WebView2.
5. Vẫn **không IPC**: cửa sổ ứng dụng dùng chung `capabilities = []`; quyền vào từng ứng dụng do máy chủ quyết (lớp 1),
   danh sách ô do máy chủ trả (lớp 2), vỏ không tự suy quyền.

## Hệ quả

- Thêm một ứng dụng web đứng sau đăng nhập vào hệ thống ⇒ thêm một dòng vào `UNG_DUNG_CUA_SO_RIENG` + ô ở máy chủ cùng đợt.
- Mọi cửa sổ dùng chung thư mục dữ liệu WebView2 (cookie theo origin như trình duyệt) - đăng nhập `id.` một lần dùng được
  cho mọi cửa sổ, đúng hành vi SSO trên web.
- Cửa sổ con (`NewWindowResponse::Allow`) của cửa sổ ứng dụng chỉ mở được cho origin của chính nó - như ADR-0004.
- `quantri.` trên điện thoại (A2) dùng cùng bảng; khoá màn hình ngắn hơn cho cửa sổ quản trị để pha A2 quyết.
