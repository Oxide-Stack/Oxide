use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// A runtime route instance used at the Rust ↔ Dart boundary.
///
/// Why: Rust needs a stable, schema-flexible transport shape that can represent any
/// application-defined route. The strongly-typed route structs live in the application
/// crate, but the navigation runtime needs a single type to send/receive.
///
/// How: We serialize concrete route structs to JSON and tag them with a `kind` string
/// (matching the generated Dart `RouteKind`).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NavRoute {
    /// Route kind discriminator (e.g. `"Splash"` or `"Player"`).
    pub kind: String,
    /// JSON payload for the concrete route struct.
    pub payload: JsonValue,
    /// Optional JSON extras payload.
    pub extras: Option<JsonValue>,
}
