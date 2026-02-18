use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::sync::{Mutex, mpsc, oneshot};

use super::{OxideChannelError, OxideChannelResult, ensure_isolated_channels_initialized};

/// Marker trait describing a Rust → Dart → Rust callback interface.
///
/// Implementations are consumed by the `#[oxide_callback]` macro to generate
/// strongly typed async methods and FRB request/response glue.
pub trait OxideCallbacking {
    /// Request enum sent from Rust to Dart.
    type Request: Send + 'static;

    /// Response enum sent from Dart back to Rust.
    type Response: Send + 'static;
}

/// Runtime backing an [`OxideCallbacking`] implementation.
///
/// This type is intended to be used by macro-generated code. It provides:
///
/// - a request queue (Rust → Dart)
/// - a pending response table keyed by request id (Dart → Rust)
pub struct CallbackRuntime<Request, Response> {
    next_id: AtomicU64,
    requests_tx: mpsc::Sender<(u64, Request)>,
    requests_rx: Mutex<mpsc::Receiver<(u64, Request)>>,
    pending: Mutex<HashMap<u64, oneshot::Sender<Response>>>,
    _marker: PhantomData<(Request, Response)>,
}

impl<Request, Response> CallbackRuntime<Request, Response>
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Creates a new callback runtime with the provided request buffer capacity.
    pub fn new(buffer: usize) -> Self {
        let (requests_tx, requests_rx) = mpsc::channel(buffer);
        Self {
            next_id: AtomicU64::new(1),
            requests_tx,
            requests_rx: Mutex::new(requests_rx),
            pending: Mutex::new(HashMap::new()),
            _marker: PhantomData,
        }
    }

    /// Enqueues a request and waits for the corresponding response.
    pub async fn invoke(&self, request: Request) -> OxideChannelResult<Response> {
        ensure_isolated_channels_initialized()?;

        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().await.insert(id, tx);

        if self.requests_tx.send((id, request)).await.is_err() {
            self.pending.lock().await.remove(&id);
            return Err(OxideChannelError::Unavailable);
        }

        rx.await.map_err(|_| OxideChannelError::Unavailable)
    }

    /// Receives the next pending request for streaming to Dart.
    pub async fn recv_request(&self) -> Option<(u64, Request)> {
        ensure_isolated_channels_initialized().ok()?;
        self.requests_rx.lock().await.recv().await
    }

    /// Delivers a response for a previously issued request id.
    pub async fn respond(&self, id: u64, response: Response) -> OxideChannelResult<()> {
        ensure_isolated_channels_initialized()?;

        let tx = self.pending.lock().await.remove(&id);
        let Some(tx) = tx else {
            return Err(OxideChannelError::UnexpectedResponse);
        };

        tx.send(response).map_err(|_| OxideChannelError::Unavailable)
    }
}

