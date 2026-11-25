# PHIẾU - A1A: bổ sung Windows ARM64 + phủ mọi điện thoại (sửa phiếu A1S)

- **Mã phiếu**: 20261126-A1A
- **Từ**: agent-tong (phiên `ubnd-ec`), theo yêu cầu chủ dự án 26/11/2026
- **Đến**: agent đang làm pha A1 trong repo `ubnd-app`
- **Trạng thái**: MỞ

## Việc cần làm

1. Đọc `docs/ADR/0003-windows-arm64-va-phu-moi-dien-thoai.md` - nó SỬA phiếu A1S/ADR-0002 ở điểm: Windows có HAI đích
   `x86_64-pc-windows-msvc` + `aarch64-pc-windows-msvc`; Android thêm `armeabi-v7a` + `x86_64`.
2. Pha A1: workflow phát hành chạy ma trận 2 đích Windows trên runner Windows (biên dịch chéo ARM64), ra hai bộ cài NSIS
   + `latest.json` có `windows-x86_64` và `windows-aarch64`. Ghi rõ trong ghi chú phát hành nếu bản ARM64 chưa kiểm trên
   máy thật.
3. README: bảng nền tảng theo ADR-0003; ADR-0002 thêm một dòng "đã sửa bởi ADR-0003" (không viết lại).
4. Ghi kết quả vào mục dưới; đóng cả phiếu A1S lẫn A1A khi xong.

## Kết quả xử lý

- 26/11/2026 - agent-tong: mở phiếu.
