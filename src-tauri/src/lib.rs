//! Vỏ ứng dụng cài đặt của Hệ thống điều hành UBND xã Núi Cấm (pha A1: Windows 10/11 x64 - ADR-0002).
//!
//! Vỏ chỉ làm bốn việc: mở một cửa sổ nạp `https://dieuhanh.xanuicam.vn` (cửa sổ ứng dụng khác của hệ thống mở RIÊNG
//! theo ADR-0005), giữ mỗi cửa sổ trong đúng origin của nó (`dieu_huong`), đưa liên kết ngoài ra trình duyệt, và tự
//! cập nhật bằng gói có chữ ký (`cap_nhat`). Trang web trong
//! cửa sổ KHÔNG có quyền IPC nào: `app.security.capabilities` để rỗng trong `tauri.conf.json`, nên dù trang bị chèn mã
//! thì cũng không gọi được lệnh nào của vỏ.

mod dieu_huong;

#[cfg(windows)]
mod cap_nhat;

use dieu_huong::{
    CuaSo, QuyetDinh, URL_KHOI_DONG, nhan_va_tieu_de, quyet_dinh_cua_so_moi_cho,
    quyet_dinh_dieu_huong_cho,
};
use tauri::webview::NewWindowResponse;
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;
use url::Url;

const NHAN_CUA_SO: &str = "main";
const TIEU_DE_CHINH: &str = "Điều hành công việc - UBND xã Núi Cấm";

fn mo_ben_ngoai<R: Runtime>(app: &AppHandle<R>, url: &Url) {
    if let Err(loi) = app.opener().open_url(url.as_str(), None::<&str>) {
        log::warn!("không mở được liên kết ngoài: {loi}");
    }
}

fn dua_len_truoc<R: Runtime>(cua_so: &tauri::WebviewWindow<R>) {
    let _ = cua_so.unminimize();
    let _ = cua_so.show();
    let _ = cua_so.set_focus();
}

fn dua_cua_so_len_truoc<R: Runtime>(app: &AppHandle<R>) {
    if let Some(cua_so) = app.get_webview_window(NHAN_CUA_SO) {
        dua_len_truoc(&cua_so);
    }
}

/// Nạp `url` vào cửa sổ chính rồi đưa nó lên trước (liên kết từ một cửa sổ ứng dụng tới `dieuhanh.`).
fn ve_cua_so_chinh<R: Runtime>(app: &AppHandle<R>, url: Url) {
    if let Some(cua_so) = app.get_webview_window(NHAN_CUA_SO) {
        if let Err(loi) = cua_so.navigate(url) {
            log::warn!("không nạp được cửa sổ chính: {loi}");
        }
        dua_len_truoc(&cua_so);
    }
}

/// Mở cửa sổ riêng của một ứng dụng (ADR-0005), hoặc đưa cửa sổ đang mở lên trước. Liên kết sâu (đường dẫn khác `/`)
/// thì nạp vào cửa sổ đang mở; bấm lại ô ở trang gốc thì KHÔNG nạp lại (người dùng có thể đang soạn dở).
fn mo_cua_so_ung_dung<R: Runtime>(app: &AppHandle<R>, host: &'static str, url: Url) {
    let Some((nhan, tieu_de)) = nhan_va_tieu_de(host) else {
        return;
    };
    if let Some(cua_so) = app.get_webview_window(nhan) {
        if url.path() != "/" || url.query().is_some() {
            let _ = cua_so.navigate(url);
        }
        dua_len_truoc(&cua_so);
        return;
    }
    if let Err(loi) = tao_cua_so(app, CuaSo::UngDung(host), nhan, tieu_de, url) {
        log::warn!("không mở được cửa sổ {nhan}: {loi}");
    }
}

/// Thực hiện quyết định không phải `ChoPhep`. Tạo/nạp cửa sổ chạy ở VÒNG LẶP CHÍNH sau khi bộ xử lý điều hướng trả về
/// (dựng cửa sổ ngay trong callback của WebView2 có thể treo vòng lặp sự kiện).
fn thuc_hien<R: Runtime>(app: &AppHandle<R>, quyet_dinh: QuyetDinh, url: &Url) {
    match quyet_dinh {
        QuyetDinh::ChoPhep => {}
        QuyetDinh::MoBenNgoai => mo_ben_ngoai(app, url),
        QuyetDinh::Chan => log::warn!("chặn điều hướng scheme {}", url.scheme()),
        QuyetDinh::MoCuaSoUngDung(host) => {
            let (a, u) = (app.clone(), url.clone());
            let _ = app.run_on_main_thread(move || mo_cua_so_ung_dung(&a, host, u));
        }
        QuyetDinh::VeCuaSoChinh => {
            let (a, u) = (app.clone(), url.clone());
            let _ = app.run_on_main_thread(move || ve_cua_so_chinh(&a, u));
        }
    }
}

fn tao_cua_so<R: Runtime>(
    app: &AppHandle<R>,
    cua_so: CuaSo,
    nhan: &str,
    tieu_de: &str,
    url_dau: Url,
) -> tauri::Result<()> {
    let app_dh = app.clone();
    let app_cs = app.clone();

    WebviewWindowBuilder::new(app, nhan, WebviewUrl::External(url_dau))
        .title(tieu_de)
        .inner_size(1366.0, 860.0)
        .min_inner_size(960.0, 600.0)
        .center()
        .resizable(true)
        // Kéo-thả tệp vào trang (đính kèm văn bản) phải đi tới trang web, không bị vỏ nuốt.
        .disable_drag_drop_handler()
        .on_navigation(move |url| {
            let quyet_dinh = quyet_dinh_dieu_huong_cho(cua_so, url);
            let cho = quyet_dinh == QuyetDinh::ChoPhep;
            thuc_hien(&app_dh, quyet_dinh, url);
            cho
        })
        .on_new_window(move |url, _dac_tinh| {
            let quyet_dinh = quyet_dinh_cua_so_moi_cho(cua_so, &url);
            if quyet_dinh == QuyetDinh::ChoPhep {
                // Cửa sổ con cùng phiên đăng nhập (mở công việc ở thẻ mới, xem trước tệp `blob:`).
                return NewWindowResponse::Allow;
            }
            thuc_hien(&app_cs, quyet_dinh, &url);
            NewWindowResponse::Deny
        })
        .build()?;
    Ok(())
}

fn tao_cua_so_chinh<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let url_dau = Url::parse(URL_KHOI_DONG).expect("URL_KHOI_DONG hợp lệ");
    tao_cua_so(app, CuaSo::Chinh, NHAN_CUA_SO, TIEU_DE_CHINH, url_dau)
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
            tao_cua_so_chinh(app.handle())?;
            #[cfg(windows)]
            cap_nhat::kiem_tra_nen(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("không khởi động được ứng dụng");
}
