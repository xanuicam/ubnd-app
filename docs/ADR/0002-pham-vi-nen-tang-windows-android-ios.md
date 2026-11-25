# ADR-0002 - Phạm vi nền tảng: CHỈ Windows 10/11, Android, iPhone

- Ngày: 26/11/2026
- Trạng thái: Chấp nhận (chủ dự án chốt 26/11/2026). Thay mục "Windows, macOS, Linux" của ADR-0001.
- Đã sửa bởi ADR-0003 (thêm Windows ARM64; Android thêm armeabi-v7a + x86_64).

## Quyết định

| Nền tảng | Build | Ghi chú |
| --- | --- | --- |
| **Windows 10 (22H2) / 11, x64** | CÓ - pha A1 | Một bộ cài NSIS `.exe` cài theo người dùng (không cần quyền quản trị), ký Authenticode khi có chứng thư; WebView2 cài qua `downloadBootstrapper` (Windows 10 có thể chưa có). Không làm MSI. |
| **Android 7.0+ (minSdk 24), arm64-v8a** | CÓ - pha A2 | AAB lên Google Play (kênh thử nội bộ trước), APK ký cho cài tay. |
| **iPhone (iOS 15+)** | CÓ, khi có Apple Developer (dự kiến 02/2027) - pha A3 | TestFlight rồi App Store. Trước đó KHÔNG có job iOS/macOS nào trong CI. |
| macOS, Linux, Windows 7/8, Windows ARM64, Android x86/x86_64/armeabi-v7a, iPad riêng | **KHÔNG** | Không có người dùng; mỗi đích thêm vào là thêm thời gian build, ký, kiểm thử. Muốn thêm = ADR mới. |

## Hệ quả

- CI: job Windows chạy `windows-2025` (hoặc `windows-latest`), job Android chạy `ubuntu-24.04`; **không** dùng runner
  macOS cho tới pha A3.
- `tauri.conf.json`: `bundle.targets` chỉ `["nsis"]` cho desktop; không khai cấu hình `macOS`/`linux`.
- Updater: chỉ phát `latest.json` cho `windows-x86_64` (Android/iOS cập nhật qua cửa hàng).
- Kiểm thử thật: Windows 10 22H2 + Windows 11 x64; một máy Android thật (Android 10+) + trình giả lập arm64.
