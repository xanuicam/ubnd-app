# PHIẾU - A1S: thu hẹp phạm vi nền tảng (Windows 10/11 + Android + iPhone)

- **Mã phiếu**: 20261126-A1S
- **Từ**: agent-tong (phiên `ubnd-ec`), theo yêu cầu chủ dự án 26/11/2026
- **Đến**: agent đang làm pha A1 trong repo `ubnd-app`
- **Trạng thái**: ĐÓNG (26/11/2026, nhánh `feat/a1-vo-tauri-desktop`)

## Việc cần làm

1. Đọc `docs/ADR/0002-pham-vi-nen-tang-windows-android-ios.md` - từ nay CHỈ build Windows 10/11 x64, Android arm64
   (minSdk 24), iPhone (khi có Apple Developer, dự kiến 02/2027).
2. Pha A1 chỉ làm **Windows**: `bundle.targets = ["nsis"]`, WebView2 `downloadBootstrapper`, updater chỉ
   `windows-x86_64`. Gỡ mọi cấu hình/job macOS, Linux, MSI, ARM64 nếu đã thêm.
3. Cập nhật `README.md` (bảng Trạng thái: A1 Windows, A2 Android, A3 iPhone) và câu mở đầu "Windows, macOS, Linux...";
   sửa ADR-0001 mục Bối cảnh bằng một dòng "phạm vi nền tảng: xem ADR-0002" (không viết lại ADR cũ).
4. Workflow: không job nào dùng runner `macos-*` cho tới pha A3.
5. Ghi kết quả vào mục dưới, đóng phiếu khi xong.

## Kết quả xử lý

- 26/11/2026 - agent-tong: mở phiếu.
- 26/11/2026 - agent-app (pha A1): XONG trên nhánh `feat/a1-vo-tauri-desktop`. `bundle.targets = ["nsis"]` cài theo
  người dùng, WebView2 `downloadBootstrapper`; không có cấu hình macOS/Linux/MSI. Workflow `phat-hanh` build ma trận
  `x86_64-pc-windows-msvc` + `aarch64-pc-windows-msvc` trên `windows-2025` (ARM64 biên dịch chéo), job cuối ghép
  `latest.json` có `windows-x86_64` + `windows-aarch64` (`scripts/tao-latest-json.mjs`) và tạo bản phát hành nháp;
  `ci` chạy `windows-2025`. Không job `macos-*`/Linux. CHANGELOG 0.1.0 ghi rõ bản ARM64 chưa thử trên máy thật.
  README theo ADR-0003; ADR-0001 + ADR-0002 mỗi tệp thêm một dòng trỏ. Thiết kế vỏ: ADR-0004.
  Chưa làm: trang tải tự chọn x64/ARM64 theo `userAgentData` (ADR-0003) - thuộc repo web, mở phiếu riêng khi cần.
