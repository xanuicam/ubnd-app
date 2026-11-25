# Phát hành bản Windows

## Khoá ký cập nhật

| Nửa | Nơi giữ |
| --- | --- |
| Công khai | `src-tauri/tauri.conf.json` → `plugins.updater.pubkey` |
| Riêng + mật khẩu | GitHub Secrets `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` (chỉ ghi, không đọc lại được) và bản gốc ngoài repo trên máy chủ dự án + bản sao lưu |

**Mất khoá riêng = mọi bản đã cài không bao giờ nhận cập nhật nữa**; người dùng phải tự tải bộ cài mới. Đổi khoá thì
phải đổi cả pubkey trong `tauri.conf.json` và phát một bản ký bằng KHOÁ CŨ mang pubkey mới trước.

Khoá ký cập nhật KHÁC chứng thư Authenticode (cái sau làm SmartScreen hết cảnh báo, tốn phí, chưa có).

## Các bước

1. Tăng phiên bản ở cả ba nơi: `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`
   (`node scripts/kiem-phien-ban.mjs` để kiểm) và chạy `cargo update -p ubnd-app` trong `src-tauri/` cho `Cargo.lock`.
2. Thêm mục `## X.Y.Z` vào `CHANGELOG.md` (người dùng thấy đoạn này trong hộp thoại cập nhật).
3. PR → merge vào `main`.
4. Gắn thẻ trên `main`: `git tag vX.Y.Z && git push origin vX.Y.Z`.
5. Workflow `phat-hanh` build x64 + ARM64, tạo bản phát hành **nháp**. Tải bộ cài về thử trên máy thật.
6. Bấm **Publish** trên GitHub. Từ lúc đó mọi máy đã cài thấy bản mới ở lần mở app kế tiếp.

Chạy tay workflow `phat-hanh` (Actions → Run workflow) chỉ build và để lại artifact, không tạo bản phát hành - dùng để
thử đóng gói.
