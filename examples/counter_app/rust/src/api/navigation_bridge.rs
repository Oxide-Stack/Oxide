//! Flutter Rust Bridge API surface for Oxide navigation (example app).
//!
//! This module exists so flutter_rust_bridge can discover the navigation APIs via
//! `crate::api`, while the navigation runtime itself lives in `oxide_core`.

use oxide_core::navigation_runtime;
use oxide_core::navigation::NavRoute;
use oxide_core::OxideError;

/// Initializes the Oxide navigation runtime.
#[flutter_rust_bridge::frb]
pub fn init_navigation() -> Result<(), OxideError> {
    oxide_core::init_navigation()?;
    Ok(())
}

/// Streams serialized navigation commands emitted by Rust reducers/effects.
#[flutter_rust_bridge::frb]
pub async fn oxide_nav_commands_stream(
    sink: crate::frb_generated::StreamSink<String>,
) -> Result<(), OxideError> {
    let mut rx = navigation_runtime()?.subscribe_commands();
    loop {
        match rx.recv().await {
            Ok(cmd) => {
                let json = serde_json::to_string(&cmd).unwrap_or_else(|_| "{}".to_string());
                let _ = sink.add(json);
            }
            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
            Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
        }
    }
    Ok(())
}

/// Emits a JSON result for a previously issued ticket id.
#[flutter_rust_bridge::frb]
pub async fn oxide_nav_emit_result(ticket: String, result_json: String) -> Result<(), OxideError> {
    let runtime = navigation_runtime()?;
    let json = serde_json::from_str(&result_json).unwrap_or(serde_json::Value::Null);
    let _ = runtime.emit_result(&ticket, json).await;
    Ok(())
}

/// Updates the current route context in the Rust navigation runtime.
#[flutter_rust_bridge::frb]
pub fn oxide_nav_set_current_route(kind: String, payload_json: String) -> Result<(), OxideError> {
    let runtime = navigation_runtime()?;
    let payload = serde_json::from_str(&payload_json).unwrap_or(serde_json::Value::Null);
    runtime.set_current_route(Some(NavRoute { kind, payload, extras: None }));
    Ok(())
}
