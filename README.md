# ubnd-app - Vỏ ứng dụng cài đặt của Hệ thống điều hành UBND xã Núi Cấm

Repo này CHỈ chứa **lớp vỏ ứng dụng** (Tauri 2) để cài Hệ thống điều hành công việc của UBND xã Núi Cấm như một phần
mềm trên Windows, macOS, Linux, Android và (sau này) iOS. Toàn bộ nghiệp vụ, dữ liệu và xác thực vẫn nằm ở hệ thống web
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
| A1 | Vỏ Tauri 2 desktop (Windows/macOS/Linux), cập nhật tự động có ký | Chưa |
| A2 | Android (Google Play) + iOS (khi có tài khoản Apple Developer) + quét QR đăng nhập | Chưa |

Kế hoạch đầy đủ nằm ở repo chính (riêng tư): `docs/KE_HOACH_APP_DA_NEN_TANG.md`.

## Nguyên tắc bảo mật

Đọc [SECURITY.md](SECURITY.md) để báo lỗ hổng. Luật cho người đóng góp nằm ở [AGENTS.md](AGENTS.md).
