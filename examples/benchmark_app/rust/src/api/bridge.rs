#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    let _ = crate::runtime::runtime();
    flutter_rust_bridge::setup_default_user_utils();
}

