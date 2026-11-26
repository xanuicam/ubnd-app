//! Chính sách điều hướng của cửa sổ app (AGENTS.md luật 5, ADR-0004, ADR-0005).
//!
//! MỖI CỬA SỔ MỘT DANH SÁCH ORIGIN (ADR-0005, pha A1b): cửa sổ chính hiển thị app cán bộ + cửa đăng nhập; mỗi ứng dụng
//! khác của hệ thống (`quantri.`, `dulieu.`...) mở trong CỬA SỔ RIÊNG chỉ hiển thị origin của chính nó + cửa đăng nhập.
//! Không cửa sổ nào hiển thị origin của ứng dụng khác - giữ đúng cách ly phiên host-only của SSO như trên trình duyệt.
//! Mọi liên kết ra ngoài mở bằng trình duyệt hệ thống, để một trang lạ không bao giờ chạy trong khung app (người dùng
//! không thấy thanh địa chỉ, nên không phân biệt được trang thật với trang giả).
//!
//! Hàm ở đây là hàm thuần (không đụng Tauri) để kiểm thử được trên mọi máy, kể cả máy không có WebView.
//!
//! ⚠️ Bộ xử lý điều hướng của wry KHÔNG giống nhau giữa các nền tảng: Windows (WebView2 `NavigationStarting`) chỉ báo
//! khung chính, còn macOS (WKNavigationDelegate) và Linux (WebKitGTK `decide-policy`) báo CẢ iframe. Vì vậy các nguồn
//! nhúng mà CSP `frame-src` của hai trang đã cho phép phải được cho qua ở đây - nếu không, mỗi video YouTube hay khung
//! Cloudflare Turnstile trên macOS/Linux sẽ bị chặn và bật ra một thẻ trình duyệt. CSP của trang vẫn là lớp quyết định
//! iframe nào được nạp; danh sách dưới chỉ không chặn thêm.

use url::Url;

/// Origin được hiển thị trong cửa sổ CHÍNH. So khớp TUYỆT ĐỐI scheme + host + cổng, không so tiền tố chuỗi
/// (`https://dieuhanh.xanuicam.vn.ke-gian.vn` không được lọt).
pub const ORIGIN_TRONG_APP: [&str; 2] = ["dieuhanh.xanuicam.vn", "id.xanuicam.vn"];

/// Cửa đăng nhập tập trung - mọi cửa sổ ứng dụng đều phải tới được để đi SSO.
pub const ORIGIN_DANG_NHAP: &str = "id.xanuicam.vn";

/// Ứng dụng của hệ thống được mở trong cửa sổ RIÊNG (màn "Ứng dụng của tôi" - repo chính `GET /api/auth/me/ung-dung`).
/// (host, nhãn cửa sổ, tiêu đề). `thongtin.` (trang công khai) và `kiosk.` (dự án riêng) cố ý KHÔNG có ở đây.
pub const UNG_DUNG_CUA_SO_RIENG: [(&str, &str, &str); 5] = [
    (
        "quantri.xanuicam.vn",
        "ud-quantri",
        "Quản trị hệ thống - UBND xã Núi Cấm",
    ),
    (
        "dulieu.xanuicam.vn",
        "ud-dulieu",
        "Bảng dữ liệu - UBND xã Núi Cấm",
    ),
    (
        "khodulieu.xanuicam.vn",
        "ud-khodulieu",
        "Kho dữ liệu - UBND xã Núi Cấm",
    ),
    (
        "trungtamdieuhanh.xanuicam.vn",
        "ud-trungtamdieuhanh",
        "Trung tâm điều hành - UBND xã Núi Cấm",
    ),
    (
        "tailieu.xanuicam.vn",
        "ud-tailieu",
        "Tài liệu cuộc họp - UBND xã Núi Cấm",
    ),
];

/// Cửa sổ đang ra quyết định.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CuaSo {
    /// Cửa sổ chính (`dieuhanh.`).
    Chinh,
    /// Cửa sổ riêng của một ứng dụng - mang host của ứng dụng đó (phần tử của `UNG_DUNG_CUA_SO_RIENG`).
    UngDung(&'static str),
}

impl CuaSo {
    /// Tra cửa sổ theo nhãn Tauri; nhãn lạ ⇒ `None` (không đoán quyền cho cửa sổ không biết).
    #[cfg(test)]
    pub fn tu_nhan(nhan: &str) -> Option<CuaSo> {
        if nhan == "main" {
            return Some(CuaSo::Chinh);
        }
        UNG_DUNG_CUA_SO_RIENG
            .iter()
            .find(|(_, n, _)| *n == nhan)
            .map(|(host, _, _)| CuaSo::UngDung(host))
    }

    /// Origin cửa sổ này được hiển thị.
    fn origin(self) -> Vec<&'static str> {
        match self {
            CuaSo::Chinh => ORIGIN_TRONG_APP.to_vec(),
            CuaSo::UngDung(host) => vec![host, ORIGIN_DANG_NHAP],
        }
    }
}

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
    /// Chặn ở cửa sổ này, mở (hoặc đưa lên trước) cửa sổ riêng của ứng dụng có host đó, nạp đúng URL này.
    MoCuaSoUngDung(&'static str),
    /// Chặn ở cửa sổ ứng dụng, nạp URL này vào cửa sổ CHÍNH (liên kết từ `quantri.` tới một công việc ở `dieuhanh.`).
    VeCuaSoChinh,
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

/// Host của ứng dụng có cửa sổ riêng mà URL thuộc về (https, đúng host, cổng mặc định, không thông tin đăng nhập).
fn ung_dung_cua(url: &Url) -> Option<&'static str> {
    UNG_DUNG_CUA_SO_RIENG
        .iter()
        .map(|(host, _, _)| *host)
        .find(|host| la_https_cua(url, &[host]))
}

/// Nhãn + tiêu đề cửa sổ của một ứng dụng.
pub fn nhan_va_tieu_de(host: &str) -> Option<(&'static str, &'static str)> {
    UNG_DUNG_CUA_SO_RIENG
        .iter()
        .find(|(h, _, _)| *h == host)
        .map(|(_, nhan, tieu_de)| (*nhan, *tieu_de))
}

/// URL thuộc origin hiển thị trong cửa sổ chính.
#[cfg(test)]
pub fn la_origin_trong_app(url: &Url) -> bool {
    la_origin_cua(CuaSo::Chinh, url)
}

fn la_origin_cua(cua_so: CuaSo, url: &Url) -> bool {
    la_https_cua(url, &cua_so.origin())
}

/// `blob:https://dieuhanh.xanuicam.vn/<uuid>` - tệp do chính trang trong cửa sổ tạo ra (xem trước PDF, ảnh).
fn la_blob_cua(cua_so: CuaSo, url: &Url) -> bool {
    url.scheme() == "blob"
        && Url::parse(url.path()).is_ok_and(|ben_trong| la_origin_cua(cua_so, &ben_trong))
}

/// Liên kết mở được bằng ứng dụng của hệ điều hành. Chỉ bốn scheme quen thuộc; mọi scheme khác bị chặn.
fn mo_ben_ngoai_duoc(url: &Url) -> bool {
    match url.scheme() {
        "http" | "https" => url.host_str().is_some(),
        "mailto" | "tel" => true,
        _ => false,
    }
}

/// Phần chung: origin của cửa sổ thì cho; ứng dụng khác có cửa sổ riêng thì chuyển sang đó; còn lại ra ngoài/chặn.
fn ngoai_origin(url: &Url) -> QuyetDinh {
    if la_origin_cua(CuaSo::Chinh, url) {
        // Chỉ tới được đây từ cửa sổ ứng dụng (cửa sổ chính đã cho qua ở bước trước).
        QuyetDinh::VeCuaSoChinh
    } else if let Some(host) = ung_dung_cua(url) {
        QuyetDinh::MoCuaSoUngDung(host)
    } else if mo_ben_ngoai_duoc(url) {
        QuyetDinh::MoBenNgoai
    } else {
        QuyetDinh::Chan
    }
}

/// Quyết định cho một lượt điều hướng trong một cửa sổ (khung chính, và iframe trên macOS/Linux).
pub fn quyet_dinh_dieu_huong_cho(cua_so: CuaSo, url: &Url) -> QuyetDinh {
    if la_origin_cua(cua_so, url) || la_https_cua(url, &NGUON_NHUNG) || la_blob_cua(cua_so, url) {
        return QuyetDinh::ChoPhep;
    }
    // Khung rỗng của iframe (`about:blank`, `about:srcdoc`) không nạp gì từ mạng.
    if url.scheme() == "about" && matches!(url.path(), "blank" | "srcdoc") {
        return QuyetDinh::ChoPhep;
    }
    ngoai_origin(url)
}

/// Quyết định cho `window.open` / liên kết `target="_blank"` trong một cửa sổ. Khác điều hướng thường: nguồn nhúng mở
/// cửa sổ mới thì đó là người dùng bấm "xem trên YouTube" - đưa ra trình duyệt chứ không mở cửa sổ con không có chính
/// sách điều hướng. Ô của màn "Ứng dụng của tôi" (`target="_blank"` tới `quantri.`...) đi nhánh `MoCuaSoUngDung`.
pub fn quyet_dinh_cua_so_moi_cho(cua_so: CuaSo, url: &Url) -> QuyetDinh {
    if la_origin_cua(cua_so, url) || la_blob_cua(cua_so, url) {
        return QuyetDinh::ChoPhep;
    }
    ngoai_origin(url)
}

/// Cửa sổ chính - giữ tên cũ cho test của pha A1.
#[cfg(test)]
pub fn quyet_dinh_dieu_huong(url: &Url) -> QuyetDinh {
    quyet_dinh_dieu_huong_cho(CuaSo::Chinh, url)
}

/// Cửa sổ chính - giữ tên cũ cho test của pha A1.
#[cfg(test)]
pub fn quyet_dinh_cua_so_moi(url: &Url) -> QuyetDinh {
    quyet_dinh_cua_so_moi_cho(CuaSo::Chinh, url)
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
    fn trang_cong_khai_va_ngoai_he_thong_mo_bang_trinh_duyet() {
        for s in [
            "https://thongtin.xanuicam.vn/thu-tuc",
            "https://kiosk.xanuicam.vn/",
            "https://xanuicam.vn/",
            "https://zalo.me/123",
        ] {
            assert_eq!(dh(s), QuyetDinh::MoBenNgoai, "{s}");
            assert_eq!(cs(s), QuyetDinh::MoBenNgoai, "{s}");
        }
    }

    #[test]
    fn ung_dung_khac_cua_he_thong_mo_cua_so_rieng() {
        assert_eq!(
            dh("https://quantri.xanuicam.vn/"),
            QuyetDinh::MoCuaSoUngDung("quantri.xanuicam.vn")
        );
        assert_eq!(
            cs("https://dulieu.xanuicam.vn/bang/1"),
            QuyetDinh::MoCuaSoUngDung("dulieu.xanuicam.vn")
        );
        assert_eq!(
            cs("https://tailieu.xanuicam.vn/tai-lieu/abc"),
            QuyetDinh::MoCuaSoUngDung("tailieu.xanuicam.vn")
        );
        // Biến thể giả mạo/sai cổng/không https KHÔNG được coi là ứng dụng của hệ thống.
        for s in [
            "https://quantri.xanuicam.vn.ke-gian.vn/",
            "http://quantri.xanuicam.vn/",
            "https://quantri.xanuicam.vn:8443/",
            "https://u:p@quantri.xanuicam.vn/",
        ] {
            assert_eq!(cs(s), QuyetDinh::MoBenNgoai, "{s}");
        }
    }

    fn dh_ud(host: &'static str, s: &str) -> QuyetDinh {
        quyet_dinh_dieu_huong_cho(CuaSo::UngDung(host), &Url::parse(s).unwrap())
    }

    fn cs_ud(host: &'static str, s: &str) -> QuyetDinh {
        quyet_dinh_cua_so_moi_cho(CuaSo::UngDung(host), &Url::parse(s).unwrap())
    }

    #[test]
    fn cua_so_ung_dung_chi_hien_origin_cua_no_va_cua_dang_nhap() {
        let q = "quantri.xanuicam.vn";
        assert_eq!(
            dh_ud(q, "https://quantri.xanuicam.vn/account"),
            QuyetDinh::ChoPhep
        );
        assert_eq!(
            dh_ud(q, "https://id.xanuicam.vn/?next=x"),
            QuyetDinh::ChoPhep
        );
        assert_eq!(
            dh_ud(q, "blob:https://quantri.xanuicam.vn/u"),
            QuyetDinh::ChoPhep
        );
        // App cán bộ KHÔNG hiển thị trong cửa sổ quản trị - chuyển về cửa sổ chính.
        assert_eq!(
            dh_ud(q, "https://dieuhanh.xanuicam.vn/tasks"),
            QuyetDinh::VeCuaSoChinh
        );
        assert_eq!(
            cs_ud(q, "https://dieuhanh.xanuicam.vn/tasks/1"),
            QuyetDinh::VeCuaSoChinh
        );
        assert_eq!(
            dh_ud(q, "https://dulieu.xanuicam.vn/"),
            QuyetDinh::MoCuaSoUngDung("dulieu.xanuicam.vn")
        );
        assert_eq!(
            dh_ud(q, "blob:https://dieuhanh.xanuicam.vn/u"),
            QuyetDinh::Chan
        );
        assert_eq!(
            cs_ud(q, "https://quantri.xanuicam.vn/x"),
            QuyetDinh::ChoPhep
        );
        assert_eq!(
            cs_ud(q, "https://thongtin.xanuicam.vn/"),
            QuyetDinh::MoBenNgoai
        );
        assert_eq!(cs_ud(q, "javascript:alert(1)"), QuyetDinh::Chan);
    }

    #[test]
    fn nhan_cua_so() {
        assert_eq!(CuaSo::tu_nhan("main"), Some(CuaSo::Chinh));
        assert_eq!(
            CuaSo::tu_nhan("ud-quantri"),
            Some(CuaSo::UngDung("quantri.xanuicam.vn"))
        );
        assert_eq!(CuaSo::tu_nhan("la"), None);
        for (host, nhan, _) in UNG_DUNG_CUA_SO_RIENG {
            assert_eq!(CuaSo::tu_nhan(nhan), Some(CuaSo::UngDung(host)));
            assert_eq!(nhan_va_tieu_de(host).map(|(n, _)| n), Some(nhan));
            // Nhãn Tauri chỉ nhận chữ, số, `-`, `/`, `:`, `_`.
            assert!(nhan.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'));
        }
        // Cửa sổ ứng dụng không bao giờ được trỏ vào kiosk hay trang công khai.
        for host in [
            "kiosk.xanuicam.vn",
            "thongtin.xanuicam.vn",
            "id.xanuicam.vn",
            "dieuhanh.xanuicam.vn",
        ] {
            assert_eq!(nhan_va_tieu_de(host), None, "{host}");
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
