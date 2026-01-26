use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;

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
