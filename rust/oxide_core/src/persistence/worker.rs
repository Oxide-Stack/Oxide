use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Background worker that writes persistence payloads to disk.
///
/// Writes are debounced: the most recent payload within the configured interval
/// is what gets written.
pub struct FilePersistenceWorker {
    tx: mpsc::Sender<Vec<u8>>,
}

impl FilePersistenceWorker {
    /// Spawns a new worker thread writing to `path`.
    ///
    /// # Returns
    /// A worker handle that can be used to enqueue write requests.
    pub fn new(path: PathBuf, min_interval: Duration) -> Self {
        let (tx, rx) = mpsc::channel::<Vec<u8>>();

        thread::spawn(move || {
            let mut pending: Option<Vec<u8>> = None;
            let mut last_write = Instant::now()
                .checked_sub(min_interval)
                .unwrap_or_else(Instant::now);

            loop {
                // When there is no pending payload, park for a long time.
                // When there is a pending payload, wait until it is due.
                let timeout = match pending {
                    None => Duration::from_secs(3600),
                    Some(_) => {
                        let due = last_write + min_interval;
                        let now = Instant::now();
                        if due <= now {
                            Duration::from_millis(0)
                        } else {
                            due.duration_since(now)
                        }
                    }
                };

                match rx.recv_timeout(timeout) {
                    Ok(bytes) => {
                        pending = Some(bytes);
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        if let Some(bytes) = pending.take() {
                            let _ = write_bytes_atomic(&path, &bytes);
                            last_write = Instant::now();
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }

            if let Some(bytes) = pending.take() {
                let _ = write_bytes_atomic(&path, &bytes);
            }
        });

        Self { tx }
    }

    /// Queues a payload for persistence.
    ///
    /// This method is best-effort: if the receiver is disconnected, the payload
    /// is dropped.
    pub fn queue(&self, bytes: Vec<u8>) {
        let _ = self.tx.send(bytes);
    }
}

fn write_bytes_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let tmp_path = path.with_extension("tmp");
    std::fs::write(&tmp_path, bytes)?;
    std::fs::rename(tmp_path, path)?;
    Ok(())
}
