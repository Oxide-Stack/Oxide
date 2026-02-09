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
    #[cfg(feature = "navigation-binding")]
    crate::navigation::runtime::init()?;
    Ok(())
}

#[cfg(feature = "navigation-binding")]
#[flutter_rust_bridge::frb]
pub fn open_charts() -> Result<(), oxide_core::OxideError> {
    let runtime = oxide_core::navigation_runtime()?;
    runtime.push(oxide_core::navigation::NavRoute {
        kind: "Charts".into(),
        payload: serde_json::to_value(crate::routes::ChartsRoute {}).unwrap_or(serde_json::Value::Null),
        extras: None,
    });
    Ok(())
}
