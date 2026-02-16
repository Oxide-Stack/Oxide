use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;

// Stream adapter helpers for FFI.
//
// Why: many binding layers want a `Stream`-like API rather than a `watch::Receiver`.
// How: `tokio_stream` provides a small wrapper that keeps semantics consistent.
/// Converts a [`watch::Receiver`] into a [`WatchStream`].
///
/// This can be useful when exposing store updates to stream-based consumers.
///
/// # Returns
/// A stream that yields the current value immediately and then yields on changes.
pub fn watch_receiver_to_stream<T>(rx: watch::Receiver<T>) -> WatchStream<T>
where
    T: Clone + Send + Sync + 'static,
{
    WatchStream::new(rx)
}
