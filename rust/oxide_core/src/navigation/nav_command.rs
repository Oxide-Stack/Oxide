use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::navigation::NavRoute;

/// A navigation command emitted by Rust and consumed by the Dart navigation runtime.
///
/// The Dart side listens for a stream of commands and executes them using the configured
/// handler (Navigator 1.0 / GoRouter / custom).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum NavCommand {
    /// Pushes a new route.
    Push {
        /// Route to push.
        route: NavRoute,
        /// Optional ticket id used to correlate an eventual result back to Rust.
        ticket: Option<String>,
    },
    /// Pops the current route.
    Pop {
        /// Optional result value returned to the previous route.
        result: Option<JsonValue>,
    },
    /// Pops routes until the first route with the given kind becomes active.
    PopUntil { kind: String },
    /// Resets the stack to the given list of routes.
    Reset { routes: Vec<NavRoute> },
}
