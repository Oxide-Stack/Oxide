use serde::{Deserialize, Serialize};

use crate::navigation::NavRoute;

/// Snapshot of the current navigation context as last reported by the Dart runtime.
///
/// Why: Reducers/effects often need to branch behavior based on where the user currently is
/// (e.g., suppressing background polling while on a detail page).
///
/// How: The Dart navigation handler calls into Rust to keep this context in sync whenever
/// the active route changes.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct RouteContext {
    /// The currently active route, if known.
    pub current: Option<NavRoute>,
}
