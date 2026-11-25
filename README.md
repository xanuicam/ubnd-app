# ubnd-app - Vỏ ứng dụng cài đặt của Hệ thống điều hành UBND xã Núi Cấm

Repo này CHỈ chứa **lớp vỏ ứng dụng** (Tauri 2) để cài Hệ thống điều hành công việc của UBND xã Núi Cấm như một phần
mềm trên Windows 10/11 (chip Intel/AMD và chip ARM), Android và (sau này) iPhone - xem
[ADR-0002](docs/ADR/0002-pham-vi-nen-tang-windows-android-ios.md), [ADR-0003](docs/ADR/0003-windows-arm64-va-phu-moi-dien-thoai.md). Toàn bộ nghiệp vụ, dữ liệu và xác thực vẫn nằm ở hệ thống web
`https://dieuhanh.xanuicam.vn` và cửa đăng nhập tập trung `https://id.xanuicam.vn`.

## Repo này có gì và KHÔNG có gì

| Có | Không bao giờ có |
| --- | --- |
| Cấu hình Tauri, mã Rust của vỏ (cửa sổ, cập nhật, deep link, quét QR) | Mã nghiệp vụ của hệ thống điều hành |
| Workflow build, ký, phát hành | Bí mật, khoá ký, token (chỉ ở GitHub Secrets) |
| Biểu tượng, tài liệu, ADR | Dữ liệu công dân, dữ liệu cán bộ, ảnh chụp màn hình có dữ liệu thật |

Repo để **công khai** có chủ đích: mã vỏ không chứa gì cần giấu, và repo công khai được dùng GitHub Actions miễn phí để
build cho mọi nền tảng (xem [ADR-0001](docs/ADR/0001-vo-tauri-repo-cong-khai.md)).

## Trạng thái

| Pha | Nội dung | Trạng thái |
| --- | --- | --- |
| A0 | Khung repo + nền bảo mật (quét bí mật, Dependabot, luật bảo vệ nhánh, workflow ghim SHA) | Xong |
| A1 | Windows 10/11 x64 + ARM64: bộ cài NSIS cài theo người dùng, cập nhật tự động có ký ([ADR-0004](docs/ADR/0004-cua-so-nap-tu-xa-dieu-huong-cap-nhat.md)) | Mã xong, chờ bản phát hành đầu |
| A2 | Android 7.0+ (arm64-v8a, armeabi-v7a, x86_64) lên Google Play + quét QR đăng nhập | Chưa |
| A3 | iPhone (khi có tài khoản Apple Developer, dự kiến 02/2027) | Chưa |

Kế hoạch đầy đủ nằm ở repo chính (riêng tư): `docs/KE_HOACH_APP_DA_NEN_TANG.md`.

## Nguyên tắc bảo mật

Đọc [SECURITY.md](SECURITY.md) để báo lỗ hổng. Luật cho người đóng góp nằm ở [AGENTS.md](AGENTS.md).

## Phát triển

```bash
npm ci                       # Tauri CLI ghim phiên bản
cd src-tauri && cargo test   # test chính sách điều hướng (chạy được cả trên Linux)
npx tauri dev                # mở cửa sổ trỏ dieuhanh.xanuicam.vn (cần WebView2 trên Windows / WebKitGTK trên Linux)
```

Phát hành: [docs/PHAT_HANH.md](docs/PHAT_HANH.md).
