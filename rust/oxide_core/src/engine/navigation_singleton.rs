use std::sync::OnceLock;

use crate::engine::{CoreResult, OxideError};
use crate::engine::navigation_runtime::NavigationRuntime;

static NAVIGATION: OnceLock<NavigationRuntime> = OnceLock::new();

/// Initializes the global navigation runtime.
///
/// Why: navigation relies on a runtime singleton that can be reached from reducers/effects
/// and from FRB endpoints. Initialization is explicit so applications control startup order.
pub fn init_navigation() -> CoreResult<()> {
    let _ = NAVIGATION.get_or_init(NavigationRuntime::new);
    Ok(())
}

/// Returns the global navigation runtime if initialized.
pub fn navigation_runtime() -> CoreResult<&'static NavigationRuntime> {
    NAVIGATION.get().ok_or_else(|| OxideError::Internal {
        message: "navigation runtime not initialized; call init_navigation() first".into(),
    })
}
