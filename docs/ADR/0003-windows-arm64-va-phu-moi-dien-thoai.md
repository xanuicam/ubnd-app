# ADR-0003 - Thêm Windows ARM64 và phủ tối đa điện thoại Android/iPhone

- Ngày: 26/11/2026
- Trạng thái: Chấp nhận (chủ dự án chốt 26/11/2026). SỬA ADR-0002 ở các dòng dưới; phần còn lại của ADR-0002 giữ nguyên.

## Bối cảnh

Yêu cầu: MỌI máy Windows 10/11 (cả chip x64 lẫn chip ARM như Snapdragon X) và MỌI điện thoại Android/iPhone đang dùng
được đều cài và chạy được app. "Mọi" có giới hạn kỹ thuật cứng ở mức hệ điều hành - ghi rõ để không hứa quá.

## Quyết định

| Nền tảng | Đích build | Phủ được | Không phủ được (giới hạn cứng) |
| --- | --- | --- | --- |
| Windows 10 (1809+) / 11 **x64** | `x86_64-pc-windows-msvc` | mọi máy Intel/AMD 64-bit | Windows 32-bit, Windows 7/8 (WebView2 + Tauri 2 không hỗ trợ) |
| Windows 11 / 10 **ARM64** | `aarch64-pc-windows-msvc` (bản gốc ARM, không chạy giả lập) | Snapdragon X, Surface Pro X/ARM | - |
| **Android 7.0+** (API 24) | `arm64-v8a` + `armeabi-v7a` + `x86_64` trong MỘT AAB (Play tự cắt theo máy) | gần như mọi máy đang lưu hành (máy 32-bit giá rẻ, máy 64-bit, Chromebook/máy x86_64) | Android 6.0 trở xuống (Tauri 2 đòi API 24); máy chưa cập nhật Android System WebView phải cập nhật qua Play |
| **iPhone** iOS (mức tối thiểu Tauri 2 hỗ trợ, đo lại khi vào pha A3) | `aarch64-apple-ios` | mọi iPhone còn nhận bản iOS đó | iPhone quá cũ không lên được iOS tối thiểu |

Đóng gói Windows: **hai bộ cài NSIS** riêng `..._x64-setup.exe` và `..._arm64-setup.exe` (cài theo người dùng, WebView2
`downloadBootstrapper`) + trang tải tự chọn đúng bản theo `navigator.userAgentData.getHighEntropyValues(['architecture'])`
(mặc định x64 - máy ARM vẫn chạy được bản x64 qua giả lập nếu chọn nhầm). Updater phát `latest.json` có CẢ
`windows-x86_64` và `windows-aarch64`, mỗi máy tự lấy đúng bản.

## Hệ quả

- CI Windows: ma trận 2 đích trên runner Windows (biên dịch chéo ARM64 bằng MSVC + `rustup target add
  aarch64-pc-windows-msvc`); không cần máy ARM để build. Kiểm thử ARM64 thật: một máy Snapdragon/Surface hoặc máy ảo
  Windows 11 ARM - nếu không có thì ghi rõ "chưa kiểm trên máy thật" ở ghi chú phát hành.
- CI Android: một job build AAB đa ABI + APK cài tay (APK đa ABI, lớn hơn - chấp nhận).
- Không thêm macOS/Linux (ADR-0002 giữ nguyên).
