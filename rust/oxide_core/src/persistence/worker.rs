use std::path::{Path, PathBuf};
use std::sync::Mutex as StdMutex;
use std::time::Duration;

use tokio::sync::mpsc;

use crate::CoreResult;

// Debounced persistence worker.
//
// Why: dispatch can be frequent; writing every snapshot would be slow and
// unnecessary. A debounced worker keeps the latest state without disk churn.
//
// How: queue encoded payloads over an unbounded channel and write the most
// recent payload once per `min_interval`.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
struct SendTimeoutFuture(gloo_timers::future::TimeoutFuture);

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
unsafe impl Send for SendTimeoutFuture {}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl std::future::Future for SendTimeoutFuture {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::pin::Pin::new(&mut self.0).poll(cx)
    }
}

/// Background worker that writes persistence payloads to disk.
///
/// Writes are debounced: the most recent payload within the configured interval
/// is what gets written.
pub struct FilePersistenceWorker {
    tx: mpsc::UnboundedSender<Vec<u8>>,
    #[cfg(feature = "frb-spawn")]
    _task: StdMutex<Option<flutter_rust_bridge::JoinHandle<()>>>,
}

impl FilePersistenceWorker {
    /// Spawns a new background task writing to `path`.
    ///
    /// # Returns
    /// A worker handle that can be used to enqueue write requests.
    pub fn new(path: PathBuf, min_interval: Duration) -> CoreResult<Self> {
        crate::runtime::ensure_initialized()?;
        let (tx, mut rx) = mpsc::unbounded_channel::<Vec<u8>>();

        #[cfg(feature = "frb-spawn")]
        let task = crate::runtime::spawn(async move {
            worker_loop(&path, min_interval, &mut rx).await;
        });

        Ok(Self {
            tx,
            #[cfg(feature = "frb-spawn")]
            _task: StdMutex::new(Some(task)),
        })
    }

    /// Queues a payload for persistence.
    ///
    /// This method is best-effort: if the receiver is disconnected, the payload
    /// is dropped.
    pub fn queue(&self, bytes: Vec<u8>) {
        let _ = self.tx.send(bytes);
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
async fn worker_loop(
    path: &Path,
    min_interval: Duration,
    rx: &mut mpsc::UnboundedReceiver<Vec<u8>>,
) {
    use std::time::Instant;

    let mut pending: Option<Vec<u8>> = None;
    let mut last_write = Instant::now()
        .checked_sub(min_interval)
        .unwrap_or_else(Instant::now);

    loop {
        let wait_for = match pending {
            None => None,
            Some(_) => {
                let due = last_write + min_interval;
                let now = Instant::now();
                if due <= now {
                    Some(Duration::from_millis(0))
                } else {
                    Some(due.duration_since(now))
                }
            }
        };

        let next = match wait_for {
            None => rx.recv().await,
            Some(duration) => tokio::time::timeout(duration, rx.recv())
                .await
                .unwrap_or(None),
        };

        match next {
            Some(bytes) => pending = Some(bytes),
            None => {
                if let Some(bytes) = pending.take() {
                    let path = path.to_path_buf();
                    let _ = crate::runtime::spawn_blocking(move || {
                        crate::persistence::try_write_bytes_atomic(&path, &bytes);
                    })
                    .await;
                    last_write = Instant::now();
                    continue;
                }
                if rx.is_closed() {
                    break;
                }
            }
        }
    }

    if let Some(bytes) = pending.take() {
        let path = path.to_path_buf();
        let _ = crate::runtime::spawn_blocking(move || {
            crate::persistence::try_write_bytes_atomic(&path, &bytes);
        })
        .await;
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
async fn worker_loop(
    path: &Path,
    min_interval: Duration,
    rx: &mut mpsc::UnboundedReceiver<Vec<u8>>,
) {
    use futures::FutureExt as _;

    let min_interval_ms = min_interval.as_millis().min(u128::from(u32::MAX)) as u32;
    let mut pending: Option<Vec<u8>> = None;
    let mut last_write_ms = now_ms().saturating_sub(u64::from(min_interval_ms));

    loop {
        match pending.take() {
            None => match rx.recv().await {
                Some(bytes) => pending = Some(bytes),
                None => break,
            },
            Some(bytes) => {
                pending = Some(bytes);

                let due_ms = last_write_ms.saturating_add(u64::from(min_interval_ms));
                let now = now_ms();
                let remaining_ms = due_ms.saturating_sub(now).min(u64::from(u32::MAX)) as u32;

                let recv_fut = rx.recv().fuse();
                let sleep_fut =
                    SendTimeoutFuture(gloo_timers::future::TimeoutFuture::new(remaining_ms)).fuse();
                futures::pin_mut!(recv_fut, sleep_fut);

                futures::select_biased! {
                    value = recv_fut => {
                        match value {
                            Some(bytes) => pending = Some(bytes),
                            None => break,
                        }
                    }
                    _ = sleep_fut => {
                        if let Some(bytes) = pending.take() {
                            crate::persistence::try_write_bytes_atomic(path, &bytes);
                            last_write_ms = now_ms();
                        }
                    }
                }
            }
        }
    }

    if let Some(bytes) = pending.take() {
        crate::persistence::try_write_bytes_atomic(path, &bytes);
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn now_ms() -> u64 {
    js_sys::Date::now().max(0.0) as u64
}
