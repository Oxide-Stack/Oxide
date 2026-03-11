pub use oxide_core::ReducerEngine;

#[flutter_rust_bridge::frb]
pub fn open_charts() -> Result<(), oxide_core::OxideError> {
    // previously this dispatched a navigation action; navigation is now
    // handled directly on the Dart side, so the bridge method is retained
    // only for demonstration/logging and performs no work.
    oxide_core::runtime::ensure_initialized()?;
    // we still touch the navigation runtime to ensure it exists
    let _ = oxide_core::navigation_runtime()?;
    Ok(())
}
