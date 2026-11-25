//! Chính sách điều hướng của cửa sổ app (AGENTS.md luật 5).
//!
//! Cửa sổ chỉ hiển thị hai origin của hệ thống: app cán bộ và cửa đăng nhập tập trung. Mọi liên kết ra ngoài mở bằng
//! trình duyệt hệ thống, để một trang lạ không bao giờ chạy trong khung app (người dùng không thấy thanh địa chỉ, nên
//! không phân biệt được trang thật với trang giả).
//!
//! Hàm ở đây là hàm thuần (không đụng Tauri) để kiểm thử được trên mọi máy, kể cả máy không có WebView.
//!
//! ⚠️ Bộ xử lý điều hướng của wry KHÔNG giống nhau giữa các nền tảng: Windows (WebView2 `NavigationStarting`) chỉ báo
//! khung chính, còn macOS (WKNavigationDelegate) và Linux (WebKitGTK `decide-policy`) báo CẢ iframe. Vì vậy các nguồn
//! nhúng mà CSP `frame-src` của hai trang đã cho phép phải được cho qua ở đây - nếu không, mỗi video YouTube hay khung
//! Cloudflare Turnstile trên macOS/Linux sẽ bị chặn và bật ra một thẻ trình duyệt. CSP của trang vẫn là lớp quyết định
//! iframe nào được nạp; danh sách dưới chỉ không chặn thêm.

use url::Url;

/// Origin được hiển thị trong cửa sổ app. So khớp TUYỆT ĐỐI scheme + host + cổng, không so tiền tố chuỗi
/// (`https://dieuhanh.xanuicam.vn.ke-gian.vn` không được lọt).
pub const ORIGIN_TRONG_APP: [&str; 2] = ["dieuhanh.xanuicam.vn", "id.xanuicam.vn"];

/// Nguồn iframe mà CSP `frame-src` của `dieuhanh.` và `id.` cho phép (đối chiếu `frontend/public/_headers`,
/// `frontend-id/public/_headers` ở repo chính). Thêm nguồn nhúng mới ở web thì thêm vào đây cùng đợt.
pub const NGUON_NHUNG: [&str; 3] = [
    "www.youtube-nocookie.com",
    "view.officeapps.live.com",
    "challenges.cloudflare.com",
];

/// URL gốc mà cửa sổ app mở khi khởi động.
pub const URL_KHOI_DONG: &str = "https://dieuhanh.xanuicam.vn/";

#[derive(Debug, PartialEq, Eq)]
pub enum QuyetDinh {
    /// Cho nạp trong cửa sổ app.
    ChoPhep,
    /// Chặn và mở bằng trình duyệt (hoặc ứng dụng thư/điện thoại) của hệ điều hành.
    MoBenNgoai,
    /// Chặn hẳn: scheme lạ có thể gọi trình xử lý giao thức bất kỳ trên máy (vd `ms-msdt:`, `file:`).
    Chan,
}

fn la_https_cua(url: &Url, danh_sach: &[&str]) -> bool {
    url.scheme() == "https"
        && url.port().is_none()
        && url.username().is_empty()
        && url.password().is_none()
        && url.host_str().is_some_and(|host| danh_sach.contains(&host))
}

/// URL thuộc origin hiển thị trong app (https, đúng host, cổng mặc định, không kèm thông tin đăng nhập).
pub fn la_origin_trong_app(url: &Url) -> bool {
    la_https_cua(url, &ORIGIN_TRONG_APP)
}

/// `blob:https://dieuhanh.xanuicam.vn/<uuid>` - tệp do chính trang trong app tạo ra (xem trước PDF, ảnh).
fn la_blob_trong_app(url: &Url) -> bool {
    url.scheme() == "blob"
        && Url::parse(url.path()).is_ok_and(|ben_trong| la_origin_trong_app(&ben_trong))
}

/// Liên kết mở được bằng ứng dụng của hệ điều hành. Chỉ bốn scheme quen thuộc; mọi scheme khác bị chặn.
fn mo_ben_ngoai_duoc(url: &Url) -> bool {
    match url.scheme() {
        "http" | "https" => url.host_str().is_some(),
        "mailto" | "tel" => true,
        _ => false,
    }
}

/// Quyết định cho một lượt điều hướng trong cửa sổ (khung chính, và iframe trên macOS/Linux).
pub fn quyet_dinh_dieu_huong(url: &Url) -> QuyetDinh {
    if la_origin_trong_app(url) || la_https_cua(url, &NGUON_NHUNG) || la_blob_trong_app(url) {
        return QuyetDinh::ChoPhep;
    }
    // Khung rỗng của iframe (`about:blank`, `about:srcdoc`) không nạp gì từ mạng.
    if url.scheme() == "about" && matches!(url.path(), "blank" | "srcdoc") {
        return QuyetDinh::ChoPhep;
    }
    if mo_ben_ngoai_duoc(url) {
        QuyetDinh::MoBenNgoai
    } else {
        QuyetDinh::Chan
    }
}

/// Quyết định cho `window.open` / liên kết `target="_blank"`. Khác điều hướng thường: nguồn nhúng mở cửa sổ mới thì
/// đó là người dùng bấm "xem trên YouTube" - đưa ra trình duyệt chứ không mở cửa sổ con không có chính sách điều hướng.
pub fn quyet_dinh_cua_so_moi(url: &Url) -> QuyetDinh {
    if la_origin_trong_app(url) || la_blob_trong_app(url) {
        QuyetDinh::ChoPhep
    } else if mo_ben_ngoai_duoc(url) {
        QuyetDinh::MoBenNgoai
    } else {
        QuyetDinh::Chan
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dh(s: &str) -> QuyetDinh {
        quyet_dinh_dieu_huong(&Url::parse(s).unwrap())
    }

    fn cs(s: &str) -> QuyetDinh {
        quyet_dinh_cua_so_moi(&Url::parse(s).unwrap())
    }

    #[test]
    fn hai_origin_he_thong_nap_trong_app() {
        assert_eq!(dh("https://dieuhanh.xanuicam.vn/"), QuyetDinh::ChoPhep);
        assert_eq!(
            dh("https://dieuhanh.xanuicam.vn/tasks/12?a=1#b"),
            QuyetDinh::ChoPhep
        );
        assert_eq!(
            dh("https://id.xanuicam.vn/login?next=x"),
            QuyetDinh::ChoPhep
        );
        assert_eq!(dh("https://dieuhanh.xanuicam.vn:443/"), QuyetDinh::ChoPhep);
    }

    #[test]
    fn host_gia_mao_va_bien_the_khong_lot() {
        for s in [
            "https://dieuhanh.xanuicam.vn.ke-gian.vn/",
            "https://xanuicam.vn.ke-gian.vn/",
            "https://ke-gian.vn/dieuhanh.xanuicam.vn",
            "https://dieuhanh.xanuicam.vn@ke-gian.vn/",
            "https://ke-gian.vn/?u=https://id.xanuicam.vn",
        ] {
            assert_eq!(dh(s), QuyetDinh::MoBenNgoai, "{s}");
        }
        // Không phải https, sai cổng, hoặc có thông tin đăng nhập trong URL: không nạp trong app.
        assert_eq!(dh("http://dieuhanh.xanuicam.vn/"), QuyetDinh::MoBenNgoai);
        assert_eq!(
            dh("https://dieuhanh.xanuicam.vn:8443/"),
            QuyetDinh::MoBenNgoai
        );
        assert_eq!(
            dh("https://u:p@dieuhanh.xanuicam.vn/"),
            QuyetDinh::MoBenNgoai
        );
    }

    #[test]
    fn app_khac_cua_he_thong_mo_bang_trinh_duyet() {
        for s in [
            "https://thongtin.xanuicam.vn/thu-tuc",
            "https://quantri.xanuicam.vn/",
            "https://tailieu.xanuicam.vn/",
            "https://xanuicam.vn/",
            "https://zalo.me/123",
        ] {
            assert_eq!(dh(s), QuyetDinh::MoBenNgoai, "{s}");
        }
    }

    #[test]
    fn nguon_nhung_theo_csp_cho_qua_khi_dieu_huong() {
        assert_eq!(
            dh("https://www.youtube-nocookie.com/embed/abc"),
            QuyetDinh::ChoPhep
        );
        assert_eq!(
            dh("https://view.officeapps.live.com/op/embed.aspx?src=x"),
            QuyetDinh::ChoPhep
        );
        assert_eq!(
            dh("https://challenges.cloudflare.com/cdn-cgi/x"),
            QuyetDinh::ChoPhep
        );
        assert_eq!(dh("about:blank"), QuyetDinh::ChoPhep);
        assert_eq!(dh("about:srcdoc"), QuyetDinh::ChoPhep);
        assert_eq!(
            dh("https://www.youtube.com/watch?v=abc"),
            QuyetDinh::MoBenNgoai
        );
    }

    #[test]
    fn blob_chi_cua_origin_trong_app() {
        assert_eq!(
            dh("blob:https://dieuhanh.xanuicam.vn/0b1c-uuid"),
            QuyetDinh::ChoPhep
        );
        assert_eq!(dh("blob:https://ke-gian.vn/0b1c-uuid"), QuyetDinh::Chan);
        assert_eq!(
            cs("blob:https://dieuhanh.xanuicam.vn/0b1c-uuid"),
            QuyetDinh::ChoPhep
        );
    }

    #[test]
    fn scheme_nguy_hiem_bi_chan() {
        for s in [
            "file:///C:/Windows/System32/calc.exe",
            "ms-msdt:/id PCWDiagnostic",
            "javascript:alert(1)",
            "data:text/html,<h1>x</h1>",
            "ftp://ke-gian.vn/a",
            "zalo://chat",
            "about:config",
        ] {
            assert_eq!(dh(s), QuyetDinh::Chan, "{s}");
            assert_eq!(cs(s), QuyetDinh::Chan, "{s}");
        }
    }

    #[test]
    fn thu_va_dien_thoai_mo_bang_ung_dung_he_thong() {
        assert_eq!(dh("mailto:vanphong@xanuicam.vn"), QuyetDinh::MoBenNgoai);
        assert_eq!(dh("tel:+842963000000"), QuyetDinh::MoBenNgoai);
    }

    #[test]
    fn cua_so_moi_nguon_nhung_ra_trinh_duyet() {
        assert_eq!(
            cs("https://dieuhanh.xanuicam.vn/tasks/1"),
            QuyetDinh::ChoPhep
        );
        assert_eq!(
            cs("https://www.youtube-nocookie.com/embed/abc"),
            QuyetDinh::MoBenNgoai
        );
        assert_eq!(cs("https://thongtin.xanuicam.vn/"), QuyetDinh::MoBenNgoai);
    }
}
