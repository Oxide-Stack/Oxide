use std::sync::{Arc, Mutex as StdMutex};

use tokio::sync::{Mutex, mpsc, watch};

use crate::engine::{Context, CoreResult, InitContext, OxideError, Reducer, StateChange, StateSnapshot};

#[cfg(feature = "navigation-binding")]
use crate::engine::{NavigationCtx, navigation_runtime};

#[cfg(feature = "state-persistence")]
use crate::persistence::{
    FilePersistenceWorker, PersistenceConfig, decode, default_persistence_path, encode,
    try_read_bytes,
};

// Core reducer runtime.
//
// Why: Oxide needs a single place that defines the transactional update semantics
// (commit vs no-op vs error) so every binding layer sees identical behavior.
//
// How: Guard state behind a mutex, broadcast snapshots through a watch channel,
// and run side-effects in a background loop that uses the same commit rule.
struct EngineState<R, StateSlice>
where
    R: Reducer<StateSlice>,
    StateSlice: Copy + PartialEq + Eq + Send + Sync + 'static,
{
    state: R::State,
    revision: u64,
    reducer: R,
}

#[cfg(feature = "state-persistence")]
struct PersistenceHooks<S> {
    worker: FilePersistenceWorker,
    encode: Box<dyn Fn(&S) -> CoreResult<Vec<u8>> + Send + Sync>,
}

struct Shared<R, StateSlice>
where
    R: Reducer<StateSlice>,
    StateSlice: Copy + PartialEq + Eq + Send + Sync + 'static,
{
    state: Mutex<EngineState<R, StateSlice>>,
    tx: watch::Sender<StateSnapshot<R::State, StateSlice>>,
    error_tx: watch::Sender<Option<OxideError>>,
    sideeffect_tx: mpsc::UnboundedSender<R::SideEffect>,
    #[cfg(feature = "frb-spawn")]
    _sideeffect_task: StdMutex<Option<flutter_rust_bridge::JoinHandle<()>>>,
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
/// ## Update rule (clone-first semantics)
///
/// For both actions and side-effects, the engine applies updates transactionally:
///
/// 1. Clone the current state.
/// 2. Run reducer logic against the clone.
/// 3. If the reducer returns an error, discard the clone (state is unchanged).
/// 4. If the reducer returns [`StateChange::None`], discard the clone (no snapshot emitted).
/// 5. If the reducer returns [`StateChange::Full`], commit the clone and emit a snapshot.
///
/// # Concurrency
/// Actions are applied serially (behind an internal mutex). Snapshot updates are broadcast via
/// a Tokio [`tokio::sync::watch`] channel, so new subscribers immediately receive the latest snapshot.
///
/// # Runtime requirements
/// Creating an engine spawns a background task via Flutter Rust Bridge.
/// Call `initOxide()` from Dart (after `RustLib.init()`) before creating engines.
pub struct ReducerEngine<R, StateSlice = ()>
where
    R: Reducer<StateSlice>,
    StateSlice: Copy + PartialEq + Eq + Send + Sync + 'static,
{
    shared: Arc<Shared<R, StateSlice>>,
}

impl<R, StateSlice> Clone for ReducerEngine<R, StateSlice>
where
    R: Reducer<StateSlice>,
    StateSlice: Copy + PartialEq + Eq + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<R, StateSlice> ReducerEngine<R, StateSlice>
where
    R: Reducer<StateSlice>,
    StateSlice: Copy + PartialEq + Eq + Send + Sync + 'static,
{
    /// Creates a new engine with the given reducer and initial state.
    pub async fn new(reducer: R, initial_state: R::State) -> CoreResult<Self> {
        Self::new_inner(reducer, initial_state, None).await
    }

    #[cfg(feature = "state-persistence")]
    /// Creates an engine that persists state snapshots to disk.
    ///
    /// When persistence is enabled, every emitted snapshot is encoded and queued for writing
    /// by a background persistence worker (using `config.min_interval` for write throttling).
    pub async fn new_persistent(
        reducer: R,
        initial_state: R::State,
        config: PersistenceConfig,
    ) -> CoreResult<Self>
    where
        R::State: crate::serde::Serialize + crate::serde::de::DeserializeOwned,
    {
        let path = default_persistence_path(&config.key);
        let restored_bytes = {
            #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
            {
                let read_path = path.clone();
                crate::runtime::spawn_blocking(move || try_read_bytes(&read_path))
                    .await
                    .ok()
                    .flatten()
            }

            #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
            {
                try_read_bytes(&path)
            }
        };
        let restored = restored_bytes.and_then(|bytes| decode(&bytes).ok());
        let state = restored.unwrap_or(initial_state);
        let worker = FilePersistenceWorker::new(path, config.min_interval)?;
        let persistence = PersistenceHooks {
            worker,
            encode: Box::new(|state| encode(state)),
        };
        Self::new_inner(reducer, state, Some(persistence)).await
    }

    async fn new_inner(
        mut reducer: R,
        initial_state: R::State,
        #[cfg(feature = "state-persistence")] persistence: Option<PersistenceHooks<R::State>>,
        #[cfg(not(feature = "state-persistence"))] _persistence: Option<()>,
    ) -> CoreResult<Self> {
        crate::runtime::ensure_initialized()?;

        let initial_snapshot = StateSnapshot {
            revision: 0,
            state: initial_state.clone(),
            slices: Vec::new(),
        };
        let (tx, _rx) = watch::channel(initial_snapshot);
        let (error_tx, _error_rx) = watch::channel::<Option<OxideError>>(None);

        let (sideeffect_tx, sideeffect_rx) = mpsc::unbounded_channel::<R::SideEffect>();
        let init_ctx = InitContext {
            sideeffect_tx: sideeffect_tx.clone(),
            #[cfg(feature = "frb-spawn")]
            thread_pool: crate::runtime::thread_pool()?,
        };
        reducer.init(init_ctx).await;

        let shared = Arc::new(Shared {
            state: Mutex::new(EngineState {
                state: initial_state,
                revision: 0,
                reducer,
            }),
            tx,
            error_tx,
            sideeffect_tx: sideeffect_tx.clone(),
            #[cfg(feature = "frb-spawn")]
            _sideeffect_task: StdMutex::new(None),
            #[cfg(feature = "state-persistence")]
            persistence,
        });

        #[cfg(feature = "frb-spawn")]
        {
            let handle = crate::runtime::spawn(sideeffect_loop(Arc::clone(&shared), sideeffect_rx));
            let mut slot = shared
                ._sideeffect_task
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            *slot = Some(handle);
        }

        Ok(Self { shared })
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
    pub async fn dispatch(
        &self,
        action: R::Action,
    ) -> CoreResult<StateSnapshot<R::State, StateSlice>> {
        let mut state = self.shared.state.lock().await;
        let before_snapshot = StateSnapshot {
            revision: state.revision,
            state: state.state.clone(),
            slices: Vec::new(),
        };

        #[cfg(feature = "navigation-binding")]
        let (runtime, route_ctx) = {
            let runtime = navigation_runtime()?;
            let route_ctx = runtime.current_route_context();
            (runtime, route_ctx)
        };

        // Apply reducer logic against a cloned state so failures never partially
        // mutate the committed state.
        let mut next_state = state.state.clone();
        let ctx = Context {
            input: &action,
            state_snapshot: &before_snapshot,
            #[cfg(feature = "navigation-binding")]
            nav: NavigationCtx::new(runtime, &route_ctx),
        };
        let change = state.reducer.reduce(&mut next_state, ctx)?;

        match change {
            // Why: "no externally-visible change" should not spam watchers.
            StateChange::None => Ok(before_snapshot),
            StateChange::Full => {
                state.state = next_state;
                state.revision = state.revision.saturating_add(1);

                let snapshot = StateSnapshot {
                    revision: state.revision,
                    state: state.state.clone(),
                    slices: Vec::new(),
                };
                let _ = self.shared.tx.send(snapshot.clone());
                self.persist_if_enabled(&snapshot);
                Ok(snapshot)
            }
            StateChange::Infer => {
                let slices = state.reducer.infer_slices(&state.state, &next_state);

                state.state = next_state;
                state.revision = state.revision.saturating_add(1);

                let snapshot = StateSnapshot {
                    revision: state.revision,
                    state: state.state.clone(),
                    slices,
                };
                let _ = self.shared.tx.send(snapshot.clone());
                self.persist_if_enabled(&snapshot);
                Ok(snapshot)
            }
            StateChange::Slices(slices) => {
                state.state = next_state;
                state.revision = state.revision.saturating_add(1);

                let snapshot = StateSnapshot {
                    revision: state.revision,
                    state: state.state.clone(),
                    slices: slices.to_vec(),
                };
                let _ = self.shared.tx.send(snapshot.clone());
                self.persist_if_enabled(&snapshot);
                Ok(snapshot)
            }
        }
    }

    /// Returns the current snapshot without dispatching an action.
    pub async fn current(&self) -> StateSnapshot<R::State, StateSlice> {
        let state = self.shared.state.lock().await;
        StateSnapshot {
            revision: state.revision,
            state: state.state.clone(),
            slices: Vec::new(),
        }
    }

    /// Subscribes to snapshot updates.
    ///
    /// The returned receiver immediately contains the latest snapshot and will be notified on
    /// every subsequent committed update.
    pub fn subscribe(&self) -> watch::Receiver<StateSnapshot<R::State, StateSlice>> {
        self.shared.tx.subscribe()
    }

    /// Subscribes to engine errors that cannot be returned from the originating call site.
    ///
    /// This includes:
    /// - background side-effect failures (`Reducer::effect`)
    /// - persistence encoding failures (when `state-persistence` is enabled)
    /// - missing runtime dependencies (e.g., navigation runtime not initialized)
    pub fn subscribe_errors(&self) -> watch::Receiver<Option<OxideError>> {
        self.shared.error_tx.subscribe()
    }

    fn persist_if_enabled(&self, _snapshot: &StateSnapshot<R::State, StateSlice>) {
        #[cfg(feature = "state-persistence")]
        {
            if let Some(persistence) = &self.shared.persistence {
                match (persistence.encode)(&_snapshot.state) {
                    Ok(bytes) => persistence.worker.queue(bytes),
                    Err(err) => {
                        let _ = self.shared.error_tx.send(Some(err));
                    }
                }
            }
        }
    }
}

async fn sideeffect_loop<R, StateSlice>(
    shared: Arc<Shared<R, StateSlice>>,
    mut rx: mpsc::UnboundedReceiver<R::SideEffect>,
) where
    R: Reducer<StateSlice>,
    StateSlice: Copy + PartialEq + Eq + Send + Sync + 'static,
{
    while let Some(effect) = rx.recv().await {
        let mut state = shared.state.lock().await;
        let before_snapshot = StateSnapshot {
            revision: state.revision,
            state: state.state.clone(),
            slices: Vec::new(),
        };

        #[cfg(feature = "navigation-binding")]
        let (runtime, route_ctx) = match navigation_runtime() {
            Ok(runtime) => {
                let route_ctx = runtime.current_route_context();
                (runtime, route_ctx)
            }
            Err(err) => {
                let _ = shared.error_tx.send(Some(err));
                continue;
            }
        };
        let mut next_state = state.state.clone();
        let ctx = Context {
            input: &effect,
            state_snapshot: &before_snapshot,
            #[cfg(feature = "navigation-binding")]
            nav: NavigationCtx::new(runtime, &route_ctx),
        };
        let change = match state.reducer.effect(&mut next_state, ctx) {
            Ok(change) => change,
            Err(err) => {
                let _ = shared.error_tx.send(Some(err));
                continue;
            }
        };

        let slices: Vec<StateSlice> = match change {
            StateChange::None => {
                continue;
            }
            StateChange::Full => Vec::new(),
            StateChange::Infer => state.reducer.infer_slices(&state.state, &next_state),
            StateChange::Slices(slices) => slices.to_vec(),
        };

        state.state = next_state;
        state.revision = state.revision.saturating_add(1);

        let snapshot = StateSnapshot {
            revision: state.revision,
            state: state.state.clone(),
            slices,
        };
        let _ = shared.tx.send(snapshot.clone());

        #[cfg(feature = "state-persistence")]
        {
            if let Some(persistence) = &shared.persistence {
                match (persistence.encode)(&snapshot.state) {
                    Ok(bytes) => persistence.worker.queue(bytes),
                    Err(err) => {
                        let _ = shared.error_tx.send(Some(err));
                    }
                }
            }
        }
    }
}
