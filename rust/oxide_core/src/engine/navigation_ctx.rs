use crate::engine::navigation_runtime::NavigationRuntime;
use crate::navigation::{OxideRoute, OxideRouteKind, RouteContext};

/// Navigation capability surface injected into reducer/effect contexts.
///
/// Why: reducers/effects should not depend on Flutter types, but they often need to issue
/// navigation intents (push/pop) and inspect the current route.
pub struct NavigationCtx<'a> {
    runtime: &'a NavigationRuntime,
    route: &'a RouteContext,
}

impl<'a> NavigationCtx<'a> {
    /// Creates a navigation context snapshot for the current invocation.
    pub fn new(runtime: &'a NavigationRuntime, route: &'a RouteContext) -> Self {
        Self { runtime, route }
    }

    /// Returns the current route context snapshot.
    pub fn route(&self) -> &RouteContext {
        self.route
    }

    /// Pushes a route without expecting a result.
    pub fn push<R: OxideRoute>(&self, route: R) {
        self.runtime.push(route);
    }

    /// Pops the current route without a result.
    pub fn pop(&self) {
        self.runtime.pop();
    }

    /// Pops routes until the given kind becomes active.
    pub fn pop_until<K: OxideRouteKind>(&self, kind: K) {
        self.runtime.pop_until(kind);
    }
}
