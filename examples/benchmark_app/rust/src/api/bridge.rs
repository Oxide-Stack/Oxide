pub use oxide_core::ReducerEngine;

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
