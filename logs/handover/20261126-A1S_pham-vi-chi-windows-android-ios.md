# PHIẾU - A1S: thu hẹp phạm vi nền tảng (Windows 10/11 + Android + iPhone)

- **Mã phiếu**: 20261126-A1S
- **Từ**: agent-tong (phiên `ubnd-ec`), theo yêu cầu chủ dự án 26/11/2026
- **Đến**: agent đang làm pha A1 trong repo `ubnd-app`
- **Trạng thái**: MỞ

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
