use std::sync::Arc;

use tokio::sync::{Mutex, mpsc, watch};

use crate::core::task::{spawn_detached, spawn_detached_with_handle};
use crate::core::{CoreResult, Reducer, StateChange, StateSnapshot};

#[cfg(feature = "state-persistence")]
use crate::persistence::{
    FilePersistenceWorker, PersistenceConfig, decode, default_persistence_path, encode,
};

struct EngineState<R: Reducer> {
    state: R::State,
    revision: u64,
    reducer: R,
}

#[cfg(feature = "state-persistence")]
struct PersistenceHooks<S> {
    worker: FilePersistenceWorker,
    encode: Box<dyn Fn(&S) -> CoreResult<Vec<u8>> + Send + Sync>,
}

struct Shared<R: Reducer> {
    state: Mutex<EngineState<R>>,
    tx: watch::Sender<StateSnapshot<R::State>>,
    sideeffect_tx: mpsc::UnboundedSender<R::SideEffect>,
    #[cfg(feature = "state-persistence")]
    persistence: Option<PersistenceHooks<R::State>>,
}

/// A reducer-driven engine that owns state, broadcasts snapshots, and runs side-effects.
///
/// `ReducerEngine` is the core runtime primitive used by Oxide's generated FRB surface:
///
/// - [`dispatch`](Self::dispatch) applies an action via [`Reducer::reduce`], updates state, and
///   broadcasts a [`StateSnapshot`] to subscribers.
/// - Side-effects can be enqueued by the reducer (via [`Reducer::init`]) or manually via
///   [`sideeffect_sender`](Self::sideeffect_sender); they are processed in a background task.
///
/// # Concurrency
/// Actions are applied serially (behind an internal mutex). Snapshot updates are broadcast via
/// a Tokio [`tokio::sync::watch`] channel, so new subscribers immediately receive the latest snapshot.
///
/// # Runtime requirements
/// Creating an engine spawns a detached background task. Spawning follows Oxide's internal
/// priority order (explicit handle → current runtime → global handle → internal runtime fallback),
/// and may panic if no suitable runtime is available (depending on feature flags).
pub struct ReducerEngine<R: Reducer> {
    shared: Arc<Shared<R>>,
}

impl<R: Reducer> Clone for ReducerEngine<R> {
    fn clone(&self) -> Self {
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<R: Reducer> ReducerEngine<R> {
    /// Creates a new engine with the given reducer and initial state.
    pub fn new(reducer: R, initial_state: R::State) -> Self {
        Self::new_inner(reducer, initial_state, None, None)
    }

    /// Creates a reducer engine that uses the provided Tokio runtime handle for background tasks.
    ///
    /// The handle is used to spawn the internal side-effect loop.
    ///
    /// Note that the runtime backing `handle` must remain alive for as long as the engine's
    /// background processing is needed.
    pub fn new_with_handle(
        reducer: R,
        initial_state: R::State,
        handle: tokio::runtime::Handle,
    ) -> Self {
        Self::new_inner(reducer, initial_state, None, Some(handle))
    }

    /// Creates a reducer engine that uses the provided Tokio runtime for background tasks.
    ///
    /// This is a convenience wrapper around [`new_with_handle`](Self::new_with_handle).
    pub fn new_with_runtime(
        reducer: R,
        initial_state: R::State,
        runtime: &tokio::runtime::Runtime,
    ) -> Self {
        Self::new_with_handle(reducer, initial_state, runtime.handle().clone())
    }

    #[cfg(feature = "state-persistence")]
    /// Creates an engine that persists state snapshots to disk.
    ///
    /// When persistence is enabled, every emitted snapshot is encoded and queued for writing
    /// by a background persistence worker (using `config.min_interval` for write throttling).
    pub fn new_persistent(reducer: R, initial_state: R::State, config: PersistenceConfig) -> Self
    where
        R::State: crate::serde::Serialize + crate::serde::de::DeserializeOwned,
    {
        let path = default_persistence_path(&config.key);
        let restored = std::fs::read(&path)
            .ok()
            .and_then(|bytes| decode(&bytes).ok());
        let state = restored.unwrap_or(initial_state);
        let worker = FilePersistenceWorker::new(path, config.min_interval);
        let persistence = PersistenceHooks {
            worker,
            encode: Box::new(|state| encode(state)),
        };
        Self::new_inner(reducer, state, Some(persistence), None)
    }

    #[cfg(feature = "state-persistence")]
    /// Creates a persistent reducer engine using the provided Tokio runtime handle for background tasks.
    ///
    /// This behaves like [`new_persistent`](Self::new_persistent), but uses `handle` to spawn the
    /// engine's internal background task.
    pub fn new_persistent_with_handle(
        reducer: R,
        initial_state: R::State,
        config: PersistenceConfig,
        handle: tokio::runtime::Handle,
    ) -> Self
    where
        R::State: crate::serde::Serialize + crate::serde::de::DeserializeOwned,
    {
        let path = default_persistence_path(&config.key);
        let restored = std::fs::read(&path)
            .ok()
            .and_then(|bytes| decode(&bytes).ok());
        let state = restored.unwrap_or(initial_state);
        let worker = FilePersistenceWorker::new(path, config.min_interval);
        let persistence = PersistenceHooks {
            worker,
            encode: Box::new(|state| encode(state)),
        };
        Self::new_inner(reducer, state, Some(persistence), Some(handle))
    }

    #[cfg(feature = "state-persistence")]
    /// Creates a persistent reducer engine using the provided Tokio runtime for background tasks.
    ///
    /// This is a convenience wrapper around
    /// [`new_persistent_with_handle`](Self::new_persistent_with_handle).
    pub fn new_persistent_with_runtime(
        reducer: R,
        initial_state: R::State,
        config: PersistenceConfig,
        runtime: &tokio::runtime::Runtime,
    ) -> Self
    where
        R::State: crate::serde::Serialize + crate::serde::de::DeserializeOwned,
    {
        Self::new_persistent_with_handle(reducer, initial_state, config, runtime.handle().clone())
    }

    fn new_inner(
        mut reducer: R,
        initial_state: R::State,
        #[cfg(feature = "state-persistence")] persistence: Option<PersistenceHooks<R::State>>,
        #[cfg(not(feature = "state-persistence"))] _persistence: Option<()>,
        handle: Option<tokio::runtime::Handle>,
    ) -> Self {
        let initial_snapshot = StateSnapshot {
            revision: 0,
            state: initial_state.clone(),
        };
        let (tx, _rx) = watch::channel(initial_snapshot);

        let (sideeffect_tx, sideeffect_rx) = mpsc::unbounded_channel::<R::SideEffect>();
        reducer.init(sideeffect_tx.clone());

        let shared = Arc::new(Shared {
            state: Mutex::new(EngineState {
                state: initial_state,
                revision: 0,
                reducer,
            }),
            tx,
            sideeffect_tx: sideeffect_tx.clone(),
            #[cfg(feature = "state-persistence")]
            persistence,
        });

        if let Some(handle) = handle.as_ref() {
            spawn_detached_with_handle(
                Some(handle),
                sideeffect_loop(Arc::clone(&shared), sideeffect_rx),
            );
        } else {
            spawn_detached(sideeffect_loop(Arc::clone(&shared), sideeffect_rx));
        }

        Self { shared }
    }

    /// Returns a sender that can be used to enqueue side-effects for background processing.
    pub fn sideeffect_sender(&self) -> mpsc::UnboundedSender<R::SideEffect> {
        self.shared.sideeffect_tx.clone()
    }

    /// Dispatches an action and returns the resulting snapshot.
    ///
    /// If the reducer reports [`StateChange::None`], the current snapshot is returned and no
    /// new snapshot is broadcast.
    ///
    /// # Errors
    /// Returns any error produced by the reducer.
    pub async fn dispatch(&self, action: R::Action) -> CoreResult<StateSnapshot<R::State>> {
        let mut state = self.shared.state.lock().await;
        let mut next_state = state.state.clone();
        let change = state.reducer.reduce(&mut next_state, action)?;

        match change {
            StateChange::None => Ok(StateSnapshot {
                revision: state.revision,
                state: state.state.clone(),
            }),
            StateChange::FullUpdate => {
                state.state = next_state;
                state.revision = state.revision.saturating_add(1);

                let snapshot = StateSnapshot {
                    revision: state.revision,
                    state: state.state.clone(),
                };
                let _ = self.shared.tx.send(snapshot.clone());
                self.persist_if_enabled(&snapshot);
                Ok(snapshot)
            }
        }
    }

    /// Returns the current snapshot without dispatching an action.
    pub async fn current(&self) -> StateSnapshot<R::State> {
        let state = self.shared.state.lock().await;
        StateSnapshot {
            revision: state.revision,
            state: state.state.clone(),
        }
    }

    /// Subscribes to snapshot updates.
    ///
    /// The returned receiver immediately contains the latest snapshot and will be notified on
    /// every subsequent [`StateChange::FullUpdate`].
    pub fn subscribe(&self) -> watch::Receiver<StateSnapshot<R::State>> {
        self.shared.tx.subscribe()
    }

    fn persist_if_enabled(&self, _snapshot: &StateSnapshot<R::State>) {
        #[cfg(feature = "state-persistence")]
        {
            if let Some(persistence) = &self.shared.persistence {
                if let Ok(bytes) = (persistence.encode)(&_snapshot.state) {
                    persistence.worker.queue(bytes);
                }
            }
        }
    }
}

async fn sideeffect_loop<R: Reducer>(
    shared: Arc<Shared<R>>,
    mut rx: mpsc::UnboundedReceiver<R::SideEffect>,
) {
    while let Some(effect) = rx.recv().await {
        let mut state = shared.state.lock().await;
        let mut next_state = state.state.clone();
        let change = match state.reducer.effect(&mut next_state, effect) {
            Ok(change) => change,
            Err(_) => {
                continue;
            }
        };

        if change == StateChange::FullUpdate {
            state.state = next_state;
            state.revision = state.revision.saturating_add(1);

            let snapshot = StateSnapshot {
                revision: state.revision,
                state: state.state.clone(),
            };
            let _ = shared.tx.send(snapshot.clone());

            #[cfg(feature = "state-persistence")]
            {
                if let Some(persistence) = &shared.persistence {
                    if let Ok(bytes) = (persistence.encode)(&snapshot.state) {
                        persistence.worker.queue(bytes);
                    }
                }
            }
        }
    }
}
