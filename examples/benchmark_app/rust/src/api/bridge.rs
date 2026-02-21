pub use oxide_core::ReducerEngine;

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
pub fn open_charts() -> Result<(), oxide_core::OxideError> {
    oxide_core::runtime::ensure_initialized()?;
    let _ = oxide_core::navigation_runtime()?;
    oxide_core::runtime::safe_spawn(async move {
        if let Ok(engine) = crate::api::nav_bridge::nav_engine().await {
            let _ = engine
                .dispatch(crate::api::nav_bridge::BenchNavAction::OpenCharts)
                .await;
        }
    });
    Ok(())
}
