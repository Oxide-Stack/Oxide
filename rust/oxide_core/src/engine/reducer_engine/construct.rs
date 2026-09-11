use std::sync::Arc;
#[cfg(feature = "state-persistence")]
use std::sync::Mutex as StdMutex;

use tokio::sync::{mpsc, watch};

use crate::engine::{CoreResult, InitContext, OxideError, Reducer, StateSnapshot};

#[cfg(feature = "state-persistence")]
use crate::persistence::{
    DecodedSnapshot, FilePersistenceWorker, PersistenceConfig, decode_tracked,
    default_persistence_debug_json_path, default_persistence_path, encode,
    encode_debug_json_and_validate, try_read_bytes,
};

#[cfg(feature = "state-persistence")]
use super::types::DebugPersistenceHooks;
use super::{EngineState, ReducerEngine, Shared, sideeffect_loop};
#[cfg(feature = "state-persistence")]
use super::PersistenceHooks;

impl<R, StateSlice> ReducerEngine<R, StateSlice>
where
    R: Reducer<StateSlice>,
    StateSlice: Copy + PartialEq + Eq + Send + Sync + 'static,
{
    /// Creates a new engine with the given reducer and initial state.
    pub async fn new(reducer: R, initial_state: R::State) -> CoreResult<Self> {
        Self::new_inner(reducer, initial_state, None, None).await
    }

    #[cfg(feature = "state-persistence")]
    /// Creates an engine that persists state snapshots to disk.
    ///
    /// When persistence is enabled, every emitted snapshot is encoded and queued for writing
    /// by a background persistence worker (using `config.min_interval` for write throttling).
    ///
    /// Restores are best-effort: if persisted bytes exist but cannot be decoded (corrupt or
    /// incompatible snapshot), the engine starts from `initial_state` and surfaces a
    /// [`OxideError::Persistence`] on [`subscribe_errors`](Self::subscribe_errors).
    pub async fn new_persistent(
        reducer: R,
        initial_state: R::State,
        config: PersistenceConfig,
    ) -> CoreResult<Self>
    where
        R::State: crate::serde::Serialize + crate::serde::de::DeserializeOwned,
    {
        crate::runtime::ensure_initialized()?;

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
        let decoded = restored_bytes.map(|bytes| decode_tracked::<R::State>(&bytes));
        let (state, initial_error) = match decoded {
            Some(Ok(DecodedSnapshot::Current(state))) => (state, None),
            Some(Ok(DecodedSnapshot::Legacy(state))) => (state, None),
            Some(Err(err)) => (
                initial_state,
                Some(OxideError::Persistence {
                    message: format!(
                        "persisted state could not be restored (corrupt or incompatible snapshot): {err}"
                    ),
                }),
            ),
            None => (initial_state, None),
        };
        let worker = FilePersistenceWorker::new(path, config.min_interval)?;
        let debug_path = default_persistence_debug_json_path(&config.key);
        let debug = Some(DebugPersistenceHooks {
            worker: FilePersistenceWorker::new(debug_path, config.min_interval)?,
            encode: Box::new(|state, bytes| encode_debug_json_and_validate(state, bytes)),
        });
        let persistence = PersistenceHooks {
            worker,
            encode: Box::new(|state| encode(state)),
            debug: StdMutex::new(debug),
        };
        Self::new_inner(reducer, state, Some(persistence), initial_error).await
    }

    async fn new_inner(
        mut reducer: R,
        initial_state: R::State,
        #[cfg(feature = "state-persistence")] persistence: Option<PersistenceHooks<R::State>>,
        #[cfg(not(feature = "state-persistence"))] _persistence: Option<()>,
        initial_error: Option<OxideError>,
    ) -> CoreResult<Self> {
        crate::runtime::ensure_initialized()?;

        let initial_snapshot = StateSnapshot {
            revision: 0,
            state: initial_state.clone(),
            slices: Vec::new(),
        };
        let (tx, _rx) = watch::channel(initial_snapshot);
        let (error_tx, _error_rx) = watch::channel::<Option<crate::engine::OxideError>>(None);
        if let Some(err) = initial_error {
            let _ = error_tx.send(Some(err));
        }

        let (sideeffect_tx, sideeffect_rx) = mpsc::unbounded_channel::<R::SideEffect>();
        let init_ctx = InitContext {
            sideeffect_tx: sideeffect_tx.clone(),
            #[cfg(feature = "frb-spawn")]
            thread_pool: crate::runtime::thread_pool()?,
        };
        reducer.init(init_ctx).await;

        let shared = Arc::new(Shared {
            engine_state: tokio::sync::Mutex::new(EngineState {
                state: initial_state,
                revision: 0,
                reducer,
            }),
            tx,
            error_tx,
            #[cfg(feature = "frb-spawn")]
            sideeffect_task: std::sync::Mutex::new(None),
            #[cfg(feature = "state-persistence")]
            persistence,
        });

        #[cfg(feature = "frb-spawn")]
        {
            let handle = crate::runtime::spawn(sideeffect_loop(Arc::clone(&shared), sideeffect_rx));
            let mut slot = shared
                .sideeffect_task
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            *slot = Some(handle);
        }

        Ok(Self {
            shared,
            sideeffect_tx,
        })
    }

    /// Returns a sender that can be used to enqueue side-effects for background processing.
    ///
    /// The engine's background loop stays alive while any sender clone exists.
    pub fn sideeffect_sender(&self) -> mpsc::UnboundedSender<R::SideEffect> {
        self.sideeffect_tx.clone()
    }

    /// Aborts the background side-effect loop idempotently.
    ///
    /// Pending/queued side-effects are dropped and later sends via
    /// [`sideeffect_sender`](Self::sideeffect_sender) fail harmlessly. Dispatching actions and
    /// subscribing to snapshots/errors keeps working. Dropping every engine handle also stops
    /// the loop naturally once all sender clones are gone.
    pub fn shutdown(&self) {
        #[cfg(all(feature = "frb-spawn", not(target_arch = "wasm32")))]
        {
            let slot = self
                .shared
                .sideeffect_task
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if let Some(handle) = slot.as_ref() {
                handle.abort();
            }
        }
        #[cfg(any(not(feature = "frb-spawn"), target_arch = "wasm32"))]
        {
            // No abortable task handle on this target; the loop exits once every
            // engine handle and sender clone is dropped.
        }
    }
}
