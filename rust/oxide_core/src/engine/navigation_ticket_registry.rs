use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::Value as JsonValue;
use tokio::sync::{Mutex, oneshot};

/// In-memory ticket registry used to correlate Dart route results back to Rust.
///
/// Why: Rust-driven navigation can optionally await a pushed route. The runtime assigns a
/// ticket id when a result is requested and stores a resolver in this registry.
///
/// How: Dart receives the ticket id on push, then calls back into Rust with `emit_result`
/// when the route completes.
pub struct TicketRegistry {
    next_id: AtomicU64,
    pending: Mutex<HashMap<String, oneshot::Sender<JsonValue>>>,
}

impl TicketRegistry {
    /// Creates a new empty ticket registry.
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            pending: Mutex::new(HashMap::new()),
        }
    }

    /// Allocates a new ticket and returns its id and a receiver for the eventual result.
    pub async fn create_ticket(&self) -> (String, oneshot::Receiver<JsonValue>) {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let ticket = format!("nav-{id}");
        let (tx, rx) = oneshot::channel();

        let mut pending = self.pending.lock().await;
        pending.insert(ticket.clone(), tx);

        (ticket, rx)
    }

    /// Resolves a pending ticket with the provided JSON result value.
    pub async fn resolve(&self, ticket: &str, result: JsonValue) -> bool {
        let tx = {
            let mut pending = self.pending.lock().await;
            pending.remove(ticket)
        };

        if let Some(tx) = tx {
            let _ = tx.send(result);
            true
        } else {
            false
        }
    }
}
