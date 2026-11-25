// Không mở cửa sổ console phía sau app trên Windows ở bản phát hành.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    ubnd_app_lib::run()
}
