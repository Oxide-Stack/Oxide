use std::sync::{OnceLock, RwLock};

const DEFAULT_API_BASE_URL: &str = "https://jsonplaceholder.typicode.com";
static API_BASE_URL: OnceLock<RwLock<String>> = OnceLock::new();

pub use oxide_core::OxideError;

fn base_url_lock() -> &'static RwLock<String> {
    API_BASE_URL.get_or_init(|| RwLock::new(DEFAULT_API_BASE_URL.to_string()))
}

pub fn api_base_url() -> String {
    base_url_lock()
        .read()
        .expect("api base url lock")
        .clone()
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb]
pub async fn init_oxide() -> Result<(), oxide_core::OxideError> {
    fn thread_pool() -> oxide_core::runtime::ThreadPool {
        crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()
    }

    let _ = oxide_core::runtime::init(thread_pool);
    crate::navigation::runtime::init()?;
    Ok(())
}

#[flutter_rust_bridge::frb]
pub fn set_api_base_url(url: String) {
    *base_url_lock().write().expect("api base url lock") = url;
}

#[flutter_rust_bridge::frb]
pub fn get_api_base_url() -> String {
    api_base_url()
}

#[flutter_rust_bridge::frb]
pub fn reset_api_base_url() {
    *base_url_lock().write().expect("api base url lock") = DEFAULT_API_BASE_URL.to_string();
}
