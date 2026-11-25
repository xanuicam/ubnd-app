# Luật làm việc trong repo ubnd-app (cho người và agent)

Repo **công khai**. Mọi thứ đẩy lên đây coi như đã công bố vĩnh viễn (bản sao, bộ nhớ đệm, bản fork) - xoá sau không
thu hồi được.

## Bất biến (vi phạm = sự cố bảo mật)

1. **Không bí mật**: không token, mật khẩu, khoá ký, `.env`, chứng chỉ, URL có chữ ký. Bí mật chỉ ở GitHub Secrets.
2. **Không dữ liệu thật**: không dữ liệu công dân/cán bộ, không ảnh chụp màn hình hệ thống thật, không nhật ký máy chủ.
3. **Không mã nghiệp vụ**: giao diện và nghiệp vụ ở repo chính (riêng tư); vỏ chỉ nạp `https://dieuhanh.xanuicam.vn`.
4. **Action ghim SHA đầy đủ** (kèm chú thích tên phiên bản), `permissions:` tối thiểu ở từng workflow/job. Không dùng
   `pull_request_target` với mã của PR. Không nội suy `${{ github.event.* }}` thẳng vào `run:` - đưa qua `env:`.
5. **Vỏ Tauri tối thiểu quyền**: capability chỉ bật đúng plugin cần; CSP chặt; điều hướng ngoài danh sách origin cho
   phép thì mở bằng trình duyệt hệ thống, không mở trong cửa sổ app; không bật `devtools` ở bản phát hành.
6. **Cập nhật tự động phải có chữ ký** (khoá cập nhật Tauri) và chỉ tải qua HTTPS từ GitHub Releases của repo này.

## Quy trình

- Nhánh tính năng → PR → `main`. Không push thẳng `main`, không force-push (luật bảo vệ nhánh chặn sẵn).
- Commit theo Conventional Commits; gạch nối chỉ dùng ASCII `-`.
- Mỗi quyết định kiến trúc/bảo mật có một ADR trong `docs/ADR/`.
- Ngôn ngữ: Rust cho vỏ (an toàn bộ nhớ), TypeScript chỉ khi Tauri bắt buộc. Thêm ngôn ngữ/thư viện mới phải có ADR.
- Sản phẩm đứng tên chủ dự án: không ghi chữ ký công cụ/agent vào README, ghi chú phát hành, commit hay PR.
