use std::sync::atomic::{AtomicBool, Ordering};

use tokio::sync::{Mutex, mpsc, watch};

use crate::engine::{CoreResult, OxideError};
use crate::engine::navigation_ticket_registry::TicketRegistry;
use crate::navigation::{NavCommand, NavRoute, OxideRoute, OxideRouteKind, OxideRoutePayload, RouteContext};

/// Rust-side navigation runtime.
///
/// Why: navigation is Rust-driven (reducers/effects decide where to go), but Flutter executes
/// actual page transitions. The runtime bridges these worlds by:
/// - emitting a stream of [`NavCommand`] values for Dart to execute
/// - tracking the latest [`RouteContext`] reported by Dart
/// - correlating pushed routes with optional result tickets
pub struct NavigationRuntime {
    command_tx: mpsc::UnboundedSender<NavCommand>,
    command_rx: Mutex<mpsc::UnboundedReceiver<NavCommand>>,
    command_stream_active: AtomicBool,
    route_tx: watch::Sender<RouteContext>,
    tickets: TicketRegistry,
}

pub struct NavigationCommands<'a> {
    runtime: &'a NavigationRuntime,
}

impl<'a> NavigationCommands<'a> {
    pub async fn recv(&mut self) -> Option<NavCommand> {
        self.runtime.recv_command().await
    }
}

impl Drop for NavigationCommands<'_> {
    fn drop(&mut self) {
        self.runtime
            .command_stream_active
            .store(false, Ordering::Release);
    }
}

impl NavigationRuntime {
    /// Creates a new runtime with empty route context.
    pub fn new() -> Self {
        let (command_tx, command_rx) = mpsc::unbounded_channel::<NavCommand>();
        let (route_tx, _route_rx) = watch::channel(RouteContext::default());

        Self {
            command_tx,
            command_rx: Mutex::new(command_rx),
            command_stream_active: AtomicBool::new(false),
            route_tx,
            tickets: TicketRegistry::new(),
        }
    }

    /// Subscribes to Rust-emitted navigation commands.
    ///
    /// Only a single consumer is supported, because navigation commands are ordered and must not
    /// be dropped. Attempting to subscribe twice returns an error.
    pub fn subscribe_commands(&self) -> CoreResult<NavigationCommands<'_>> {
        if self
            .command_stream_active
            .swap(true, Ordering::AcqRel)
        {
            return Err(OxideError::Validation {
                message: "navigation commands stream already active; only one consumer is supported".into(),
            });
        }

        Ok(NavigationCommands { runtime: self })
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
    pub fn push<R: OxideRoute>(&self, route: R) -> CoreResult<()> {
        self.send_command(NavCommand::Push {
            route: nav_route_from_oxide_route(route)?,
            ticket: None,
        })
    }

    /// Emits a push command that expects a result and returns the ticket id.
    pub async fn push_with_ticket<R: OxideRoute>(
        &self,
        route: R,
    ) -> CoreResult<(String, tokio::sync::oneshot::Receiver<serde_json::Value>)> {
        let (ticket, rx) = self.tickets.create_ticket().await;
        self.send_command(NavCommand::Push {
            route: nav_route_from_oxide_route(route)?,
            ticket: Some(ticket.clone()),
        })?;
        Ok((ticket, rx))
    }

    /// Pops the current route without a result value.
    pub fn pop(&self) -> CoreResult<()> {
        self.send_command(NavCommand::Pop { result: None })
    }

    /// Pops the current route with a JSON result value.
    pub fn pop_with_json(&self, result: serde_json::Value) -> CoreResult<()> {
        self.send_command(NavCommand::Pop { result: Some(result) })
    }

    /// Pops routes until a route with the given kind becomes active.
    pub fn pop_until<K: OxideRouteKind>(&self, kind: K) -> CoreResult<()> {
        self.send_command(NavCommand::PopUntil {
            kind: kind.as_str().to_string(),
        })
    }

    /// Resets the navigation stack to the provided routes.
    pub fn reset<P: OxideRoutePayload>(&self, routes: Vec<P>) -> CoreResult<()> {
        let mut out = Vec::with_capacity(routes.len());
        for route in routes {
            out.push(nav_route_from_oxide_payload(route)?);
        }
        self.send_command(NavCommand::Reset { routes: out })
    }

    /// Resolves a pending result ticket with the provided JSON result.
    pub async fn emit_result(&self, ticket: &str, result: serde_json::Value) -> bool {
        self.tickets.resolve(ticket, result).await
    }

    fn send_command(&self, cmd: NavCommand) -> CoreResult<()> {
        self.command_tx.send(cmd).map_err(|_| OxideError::Internal {
            message: "navigation command stream receiver disconnected".into(),
        })
    }

    async fn recv_command(&self) -> Option<NavCommand> {
        let mut rx = self.command_rx.lock().await;
        rx.recv().await
    }
}

fn nav_route_from_oxide_route<R: OxideRoute>(route: R) -> CoreResult<NavRoute> {
    let kind = route
        .clone()
        .into_payload()
        .kind()
        .as_str()
        .to_string();
    let payload = serde_json::to_value(&route).map_err(|e| OxideError::Internal {
        message: format!("failed to serialize route payload for kind {kind}: {e}"),
    })?;
    let extras = route
        .extras()
        .map(|e| {
            serde_json::to_value(e).map_err(|err| OxideError::Internal {
                message: format!("failed to serialize route extras for kind {kind}: {err}"),
            })
        })
        .transpose()?;

    Ok(NavRoute { kind, payload, extras })
}

fn nav_route_from_oxide_payload<P: OxideRoutePayload>(payload: P) -> CoreResult<NavRoute> {
    let kind = payload.kind().as_str().to_string();
    let payload = match serde_json::to_value(payload).map_err(|e| OxideError::Internal {
        message: format!("failed to serialize route payload for kind {kind}: {e}"),
    })? {
        serde_json::Value::Object(mut obj) => match obj.remove("payload") {
            Some(v) => v,
            None => {
                return Err(OxideError::Validation {
                    message: format!("route payload for kind {kind} is missing a 'payload' field"),
                })
            }
        },
        _ => {
            return Err(OxideError::Validation {
                message: format!("route payload for kind {kind} did not serialize to an object"),
            })
        }
    };
    Ok(NavRoute {
        kind,
        payload,
        extras: None,
    })
}
