# Chính sách bảo mật

## Báo lỗ hổng

**Không** mở issue công khai cho lỗ hổng bảo mật. Hãy dùng **Security > Report a vulnerability** (báo cáo riêng tư của
GitHub) ngay trên repo này. Chúng tôi phản hồi trong 5 ngày làm việc và sẽ cùng bạn thống nhất thời điểm công bố.

Phạm vi: mã vỏ ứng dụng trong repo này (Tauri, Rust, workflow build/ký/phát hành, cơ chế cập nhật). Lỗ hổng của hệ thống
web (`*.xanuicam.vn`) cũng có thể báo qua kênh này; vui lòng không thử tấn công trên hệ thống thật đang phục vụ người dân.

## Cam kết của repo

- Không bí mật nào nằm trong mã hay lịch sử git: quét bằng gitleaks ở mỗi lượt đẩy + GitHub secret scanning và push
  protection.
- Mọi GitHub Action ghim theo **SHA commit đầy đủ**, không theo tag; quyền workflow mặc định chỉ đọc, job nào cần ghi phải
  tự khai `permissions:`.
- Bản phát hành được ký (Authenticode / khoá Android / khoá cập nhật Tauri); khoá ký chỉ ở GitHub Secrets.
- Ứng dụng chỉ nạp nội dung từ danh sách origin cho phép (`https://*.xanuicam.vn`), không mở API hệ thống cho trang web.
- Phụ thuộc được Dependabot theo dõi hằng tuần.
