use oxide_core::navigation::NavRoute;
use oxide_core::navigation_runtime;
use oxide_core::OxideError;

#[flutter_rust_bridge::frb]
pub fn init_navigation() -> Result<(), OxideError> {
    oxide_core::init_navigation()?;
    Ok(())
}

#[flutter_rust_bridge::frb]
pub async fn oxide_nav_commands_stream(
    sink: crate::frb_generated::StreamSink<String>,
) -> Result<(), OxideError> {
    let mut rx = navigation_runtime()?.subscribe_commands()?;
    while let Some(cmd) = rx.recv().await {
        let json = serde_json::to_string(&cmd).map_err(|e| OxideError::Internal {
            message: format!("failed to serialize navigation command: {e}"),
        })?;
        let _ = sink.add(json);
    }
    Ok(())
}

#[flutter_rust_bridge::frb]
pub async fn oxide_nav_emit_result(ticket: String, result_json: String) -> Result<(), OxideError> {
    let runtime = navigation_runtime()?;
    let json = serde_json::from_str(&result_json).map_err(|e| OxideError::Validation {
        message: format!("invalid navigation result JSON: {e}"),
    })?;
    let _ = runtime.emit_result(&ticket, json).await;
    Ok(())
}

#[flutter_rust_bridge::frb]
pub fn oxide_nav_set_current_route(kind: String, payload_json: String) -> Result<(), OxideError> {
    let runtime = navigation_runtime()?;
    let payload = serde_json::from_str(&payload_json).map_err(|e| OxideError::Validation {
        message: format!("invalid current-route JSON payload: {e}"),
    })?;
    runtime.set_current_route(Some(NavRoute { kind, payload, extras: None }));
    Ok(())
}
