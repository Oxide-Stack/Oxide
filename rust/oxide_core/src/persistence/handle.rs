use crate::core::{Reducer, StateSnapshot, StoreHandle};
use crate::persistence::{decode, default_persistence_path, encode, FilePersistenceWorker, PersistenceConfig};

/// A [`StoreHandle`] wrapper that persists state snapshots to disk.
///
/// When constructed, this handle attempts to restore state from the persistence
/// file if it exists; otherwise it falls back to the provided `initial_state`.
///
/// The persistence codec used by [`encode`] and [`decode`] depends on feature
/// flags. See the [`crate::persistence`] module docs for details.
pub struct PersistentStoreHandle<R: Reducer>
where
    R::State: crate::serde::Serialize + crate::serde::de::DeserializeOwned,
{
    inner: StoreHandle<R>,
    worker: FilePersistenceWorker,
}

impl<R: Reducer> PersistentStoreHandle<R>
where
    R::State: crate::serde::Serialize + crate::serde::de::DeserializeOwned,
{
    /// Creates a persistent store handle.
    ///
    /// If a persisted state exists on disk and can be decoded, that state is used.
    /// Otherwise, `initial_state` is used.
    ///
    /// # Returns
    /// A persistent handle wrapping an in-memory store.
    pub fn new(initial_state: R::State, config: PersistenceConfig) -> Self {
        let path = default_persistence_path(&config.key);
        let restored = std::fs::read(&path).ok().and_then(|bytes| decode(&bytes).ok());

        let state = restored.unwrap_or(initial_state);
        let inner = StoreHandle::<R>::new(state);
        let worker = FilePersistenceWorker::new(path, config.min_interval);

        Self { inner, worker }
    }

    /// Dispatches an action and persists the resulting state snapshot.
    ///
    /// Persistence is best-effort: encoding or disk write failures do not prevent
    /// the snapshot from being returned to the caller.
    ///
    /// # Errors
    /// Returns an error if the underlying store dispatch fails.
    ///
    /// # Returns
    /// The snapshot resulting from dispatching the action.
    pub async fn dispatch(&self, action: R::Action) -> crate::CoreResult<StateSnapshot<R::State>> {
        let snapshot = self.inner.dispatch(action).await?;
        if let Ok(bytes) = encode(&snapshot.state) {
            self.worker.queue(bytes);
        }
        Ok(snapshot)
    }

    /// Returns the current in-memory snapshot.
    ///
    /// # Returns
    /// The current snapshot.
    pub async fn current(&self) -> StateSnapshot<R::State> {
        self.inner.current().await
    }

    /// Subscribes to snapshot updates.
    ///
    /// # Returns
    /// A watch receiver emitting snapshots.
    pub fn subscribe(&self) -> crate::tokio::sync::watch::Receiver<StateSnapshot<R::State>> {
        self.inner.subscribe()
    }
}
