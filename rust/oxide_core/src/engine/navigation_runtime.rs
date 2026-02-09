use tokio::sync::{broadcast, watch};

use crate::engine::navigation_ticket_registry::TicketRegistry;
use crate::navigation::{NavCommand, NavRoute, RouteContext};

/// Rust-side navigation runtime.
///
/// Why: navigation is Rust-driven (reducers/effects decide where to go), but Flutter executes
/// actual page transitions. The runtime bridges these worlds by:
/// - emitting a stream of [`NavCommand`] values for Dart to execute
/// - tracking the latest [`RouteContext`] reported by Dart
/// - correlating pushed routes with optional result tickets
pub struct NavigationRuntime {
    command_tx: broadcast::Sender<NavCommand>,
    route_tx: watch::Sender<RouteContext>,
    tickets: TicketRegistry,
}

impl NavigationRuntime {
    /// Creates a new runtime with empty route context.
    pub fn new() -> Self {
        let (command_tx, _command_rx) = broadcast::channel(64);
        let (route_tx, _route_rx) = watch::channel(RouteContext::default());

        Self {
            command_tx,
            route_tx,
            tickets: TicketRegistry::new(),
        }
    }

    /// Subscribes to Rust-emitted navigation commands.
    pub fn subscribe_commands(&self) -> broadcast::Receiver<NavCommand> {
        self.command_tx.subscribe()
    }

    /// Subscribes to route context updates coming from Dart.
    pub fn subscribe_route_context(&self) -> watch::Receiver<RouteContext> {
        self.route_tx.subscribe()
    }

    /// Returns the latest known route context.
    pub fn current_route_context(&self) -> RouteContext {
        self.route_tx.borrow().clone()
    }

    /// Sets the current route context (called by Dart).
    pub fn set_current_route(&self, route: Option<NavRoute>) {
        let _ = self.route_tx.send(RouteContext { current: route });
    }

    /// Emits a push command without expecting a result.
    pub fn push(&self, route: NavRoute) {
        let _ = self.command_tx.send(NavCommand::Push { route, ticket: None });
    }

    /// Emits a push command that expects a result and returns the ticket id.
    pub async fn push_with_ticket(
        &self,
        route: NavRoute,
    ) -> (String, tokio::sync::oneshot::Receiver<serde_json::Value>) {
        let (ticket, rx) = self.tickets.create_ticket().await;
        let _ = self
            .command_tx
            .send(NavCommand::Push { route, ticket: Some(ticket.clone()) });
        (ticket, rx)
    }

    /// Pops the current route without a result value.
    pub fn pop(&self) {
        let _ = self.command_tx.send(NavCommand::Pop { result: None });
    }

    /// Pops the current route with a JSON result value.
    pub fn pop_with_json(&self, result: serde_json::Value) {
        let _ = self.command_tx.send(NavCommand::Pop { result: Some(result) });
    }

    /// Pops routes until a route with the given kind becomes active.
    pub fn pop_until_kind(&self, kind: String) {
        let _ = self.command_tx.send(NavCommand::PopUntil { kind });
    }

    /// Resets the navigation stack to the provided routes.
    pub fn reset(&self, routes: Vec<NavRoute>) {
        let _ = self.command_tx.send(NavCommand::Reset { routes });
    }

    /// Resolves a pending result ticket with the provided JSON result.
    pub async fn emit_result(&self, ticket: &str, result: serde_json::Value) -> bool {
        self.tickets.resolve(ticket, result).await
    }
}
