use std::sync::Arc;

use tokio::sync::{watch, Mutex};

use crate::core::{CoreResult, Reducer, StateSnapshot, Store};

/// Async handle for interacting with a [`Store`].
///
/// This type is cheap to clone and can be shared across tasks.
pub struct StoreHandle<R: Reducer> {
    inner: Arc<Mutex<Store<R>>>,
    tx: watch::Sender<StateSnapshot<R::State>>,
}

impl<R: Reducer> Clone for StoreHandle<R> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            tx: self.tx.clone(),
        }
    }
}

impl<R: Reducer> StoreHandle<R> {
    /// Creates a new store handle with `initial_state`.
    ///
    /// # Returns
    /// A handle that can dispatch actions, query the current snapshot, and subscribe to updates.
    pub fn new(initial_state: R::State) -> Self {
        let store = Store::<R>::new(initial_state);
        let tx = store.sender();
        Self {
            inner: Arc::new(Mutex::new(store)),
            tx,
        }
    }

    /// Dispatches `action` to the underlying store.
    ///
    /// # Errors
    /// Returns an error if the reducer fails to apply the action.
    ///
    /// # Returns
    /// The newly committed snapshot.
    pub async fn dispatch(&self, action: R::Action) -> CoreResult<StateSnapshot<R::State>> {
        let mut store = self.inner.lock().await;
        store.dispatch(action)
    }

    /// Returns the current snapshot.
    ///
    /// # Returns
    /// The latest snapshot at the time of the call.
    pub async fn current(&self) -> StateSnapshot<R::State> {
        let store = self.inner.lock().await;
        store.current_snapshot()
    }

    /// Subscribes to snapshot updates.
    ///
    /// # Returns
    /// A receiver that is notified whenever a new snapshot is published.
    pub fn subscribe(&self) -> watch::Receiver<StateSnapshot<R::State>> {
        self.tx.subscribe()
    }
}
