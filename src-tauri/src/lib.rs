//! Vỏ ứng dụng cài đặt của Hệ thống điều hành UBND xã Núi Cấm (pha A1: Windows 10/11 x64 - ADR-0002).
//!
//! Vỏ chỉ làm bốn việc: mở một cửa sổ nạp `https://dieuhanh.xanuicam.vn`, giữ cửa sổ trong hai origin của hệ thống
//! (`dieu_huong`), đưa liên kết ngoài ra trình duyệt, và tự cập nhật bằng gói có chữ ký (`cap_nhat`). Trang web trong
//! cửa sổ KHÔNG có quyền IPC nào: `app.security.capabilities` để rỗng trong `tauri.conf.json`, nên dù trang bị chèn mã
//! thì cũng không gọi được lệnh nào của vỏ.

mod dieu_huong;

#[cfg(windows)]
mod cap_nhat;

use dieu_huong::{QuyetDinh, URL_KHOI_DONG, quyet_dinh_cua_so_moi, quyet_dinh_dieu_huong};
use tauri::webview::NewWindowResponse;
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;
use url::Url;

const NHAN_CUA_SO: &str = "main";

fn mo_ben_ngoai<R: Runtime>(app: &AppHandle<R>, url: &Url) {
    if let Err(loi) = app.opener().open_url(url.as_str(), None::<&str>) {
        log::warn!("không mở được liên kết ngoài: {loi}");
    }
}

fn dua_cua_so_len_truoc<R: Runtime>(app: &AppHandle<R>) {
    if let Some(cua_so) = app.get_webview_window(NHAN_CUA_SO) {
        let _ = cua_so.unminimize();
        let _ = cua_so.show();
        let _ = cua_so.set_focus();
    }
}

fn tao_cua_so<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let url_dau = Url::parse(URL_KHOI_DONG).expect("URL_KHOI_DONG hợp lệ");
    let app_dh = app.clone();
    let app_cs = app.clone();

    WebviewWindowBuilder::new(app, NHAN_CUA_SO, WebviewUrl::External(url_dau))
        .title("Điều hành công việc - UBND xã Núi Cấm")
        .inner_size(1366.0, 860.0)
        .min_inner_size(960.0, 600.0)
        .center()
        .resizable(true)
        // Kéo-thả tệp vào trang (đính kèm văn bản) phải đi tới trang web, không bị vỏ nuốt.
        .disable_drag_drop_handler()
        .on_navigation(move |url| match quyet_dinh_dieu_huong(url) {
            QuyetDinh::ChoPhep => true,
            QuyetDinh::MoBenNgoai => {
                mo_ben_ngoai(&app_dh, url);
                false
            }
            QuyetDinh::Chan => {
                log::warn!("chặn điều hướng scheme {}", url.scheme());
                false
            }
        })
        .on_new_window(move |url, _dac_tinh| match quyet_dinh_cua_so_moi(&url) {
            // Cửa sổ con cùng phiên đăng nhập (mở công việc ở thẻ mới, xem trước tệp `blob:`).
            QuyetDinh::ChoPhep => NewWindowResponse::Allow,
            QuyetDinh::MoBenNgoai => {
                mo_ben_ngoai(&app_cs, &url);
                NewWindowResponse::Deny
            }
            QuyetDinh::Chan => NewWindowResponse::Deny,
        })
        .build()?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    // Mở app lần hai thì đưa cửa sổ đang chạy lên trước, không mở phiên bản thứ hai (hai tiến trình cùng thư mục dữ
    // liệu WebView2 sẽ tranh khoá cookie). Plugin này phải đăng ký ĐẦU TIÊN.
    #[cfg(desktop)]
    let builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
        dua_cua_so_len_truoc(app);
    }));

    #[cfg(windows)]
    let builder = builder.plugin(tauri_plugin_updater::Builder::new().build());

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            tao_cua_so(app.handle())?;
            #[cfg(windows)]
            cap_nhat::kiem_tra_nen(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("không khởi động được ứng dụng");
}
