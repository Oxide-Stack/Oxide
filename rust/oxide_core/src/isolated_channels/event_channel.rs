use std::marker::PhantomData;

use tokio::sync::broadcast;

/// Marker trait describing a single-direction Rust → Dart event channel.
///
/// Implementations are consumed by the `#[oxide_event_channel]` macro to
/// generate strongly typed send helpers and FRB stream glue.
pub trait OxideEventChannel {
    /// Event payload enum emitted over this channel.
    type Events: Clone + Send + 'static;
}

/// Runtime backing an [`OxideEventChannel`].
///
/// This type is intended to be used by macro-generated code. Applications
/// should not need to construct it directly.
pub struct EventChannelRuntime<Events> {
    tx: broadcast::Sender<Events>,
    _marker: PhantomData<Events>,
}

impl<Events> EventChannelRuntime<Events>
where
    Events: Clone + Send + 'static,
{
    /// Creates a new event runtime with the provided broadcast buffer capacity.
    pub fn new(buffer: usize) -> Self {
        let (tx, _) = broadcast::channel(buffer);
        Self {
            tx,
            _marker: PhantomData,
        }
    }

    /// Emits an event to all active subscribers.
    pub fn emit(&self, event: Events) {
        let _ = self.tx.send(event);
    }

    /// Subscribes to events from this channel.
    pub fn subscribe(&self) -> broadcast::Receiver<Events> {
        self.tx.subscribe()
    }
}

