use serde::{Deserialize, Serialize};

use crate::navigation::NavRoute;
use serde_json::Value as JsonValue;

/// Operation associated with a route update event.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RouteOperation {
    Push,
    Pop,
    PopUntil,
    Reset,
    Sync,
}

impl Default for RouteOperation {
    fn default() -> Self {
        Self::Sync
    }
}

/// Rich route update context reported by the Dart runtime.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct RouteUpdate {
    pub operation: RouteOperation,
    pub route: Option<NavRoute>,
    pub result: Option<JsonValue>,
    pub arguments: Option<JsonValue>,
    pub first_push: bool,
}

/// Snapshot of the current navigation context as last reported by the Dart runtime.
///
/// Reducers/effects often need to branch behavior based on where the user currently is
/// (e.g., suppressing background polling while on a detail page).
///
/// The Dart navigation handler calls into Rust to keep this context in sync whenever
/// the active route changes.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct RouteContext {
    /// The currently active route, if known.
    pub current: Option<NavRoute>,
    /// Most recent route transition details, if reported by Dart.
    pub last_update: Option<RouteUpdate>,
}
