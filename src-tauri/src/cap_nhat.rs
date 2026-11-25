//! Tự cập nhật (AGENTS.md luật 6). Chỉ Windows (ADR-0002: Android/iOS cập nhật qua cửa hàng).
//!
//! Gói cập nhật chỉ tải từ `latest.json` của GitHub Releases repo này qua HTTPS, và plugin từ chối mọi gói không ký
//! bằng khoá có nửa công khai ghi trong `tauri.conf.json` (`plugins.updater.pubkey`). Người dùng được hỏi trước khi cài:
//! cán bộ có thể đang soạn dở văn bản, tự khởi động lại là mất nội dung chưa lưu.

use std::time::Duration;

use tauri::{AppHandle, Runtime};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_updater::UpdaterExt;

/// Chờ cửa sổ nạp xong trang trước khi gọi mạng lần nữa, để lần mở app đầu tiên không chậm vì kiểm cập nhật.
const TRE_KHOI_DONG: Duration = Duration::from_secs(8);

pub fn kiem_tra_nen<R: Runtime>(app: AppHandle<R>) {
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(TRE_KHOI_DONG).await;
        if let Err(loi) = kiem_tra(app).await {
            // Mất mạng, GitHub lỗi, chữ ký sai: ghi lại rồi bỏ qua; app vẫn dùng bình thường ở bản hiện tại.
            log::warn!("kiểm tra cập nhật thất bại: {loi}");
        }
    });
}

async fn kiem_tra<R: Runtime>(app: AppHandle<R>) -> Result<(), tauri_plugin_updater::Error> {
    let Some(ban_moi) = app.updater()?.check().await? else {
        return Ok(());
    };

    let loi_nhan = format!(
        "Đã có phiên bản {} (đang dùng {}).\n\nCài đặt sẽ đóng ứng dụng rồi mở lại. Hãy lưu nội dung đang soạn trước khi \
         đồng ý.",
        ban_moi.version, ban_moi.current_version
    );
    let (gui, nhan) = std::sync::mpsc::channel::<bool>();
    app.dialog()
        .message(loi_nhan)
        .title("Cập nhật ứng dụng")
        .kind(MessageDialogKind::Info)
        .buttons(MessageDialogButtons::OkCancelCustom(
            "Cập nhật ngay".into(),
            "Để sau".into(),
        ))
        .show(move |dong_y| {
            let _ = gui.send(dong_y);
        });
    let dong_y = tauri::async_runtime::spawn_blocking(move || nhan.recv().unwrap_or(false))
        .await
        .unwrap_or(false);
    if !dong_y {
        return Ok(());
    }

    // Trình cài NSIS chạy chế độ `passive` (tauri.conf.json) và tự đóng app; `restart` chỉ chạy tới nếu cài xong
    // mà tiến trình vẫn còn.
    ban_moi.download_and_install(|_, _| {}, || {}).await?;
    app.restart();
}
