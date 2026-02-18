use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, RwLock};

use super::{OxideChannelError, OxideChannelResult};

/// Marker trait describing a lightweight duplex channel.
///
/// Duplex is modeled as two independent directions:
///
/// - Outgoing: Rust → Dart (stream of [`Self::Outgoing`])
/// - Incoming: Dart → Rust (function calls carrying [`Self::Incoming`])
pub trait OxideEventDuplexChannel {
    /// Rust → Dart event enum.
    type Outgoing: Clone + Send + 'static;

    /// Dart → Rust event enum.
    type Incoming: Send + 'static;
}

/// Runtime storage for an incoming Dart → Rust handler.
///
/// The handler is stored explicitly (no implicit routing) and is protected from
/// unwinding across the FFI boundary.
pub struct IncomingHandler<Incoming> {
    handler: RwLock<Option<Arc<dyn Fn(Incoming) + Send + Sync + 'static>>>,
}

impl<Incoming> IncomingHandler<Incoming>
where
    Incoming: Send + 'static,
{
    /// Creates an empty handler registry.
    pub fn new() -> Self {
        Self {
            handler: RwLock::new(None),
        }
    }

    /// Registers (or replaces) the active incoming handler.
    pub fn register(&self, handler: impl Fn(Incoming) + Send + Sync + 'static) {
        let mut guard = self.handler.write().unwrap_or_else(|e| e.into_inner());
        *guard = Some(Arc::new(handler));
    }

    /// Invokes the incoming handler if present.
    pub fn handle(&self, event: Incoming) -> OxideChannelResult<()> {
        let guard = self.handler.read().unwrap_or_else(|e| e.into_inner());
        let handler = guard.clone();
        let Some(handler) = handler else {
            return Err(OxideChannelError::Unavailable);
        };

        match catch_unwind(AssertUnwindSafe(|| (handler)(event))) {
            Ok(()) => Ok(()),
            Err(_) => Err(OxideChannelError::PlatformError(
                "incoming handler panicked".to_string(),
            )),
        }
    }
}
