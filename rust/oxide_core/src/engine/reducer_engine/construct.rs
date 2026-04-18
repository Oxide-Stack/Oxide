use std::sync::Arc;

use tokio::sync::{mpsc, watch};

use crate::engine::{CoreResult, InitContext, Reducer, StateSnapshot};

#[cfg(feature = "state-persistence")]
use crate::persistence::{
    FilePersistenceWorker, PersistenceConfig, decode, default_persistence_path, encode,
    try_read_bytes,
};

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
        let (error_tx, _error_rx) = watch::channel::<Option<crate::engine::OxideError>>(None);

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
            sideeffect_tx: sideeffect_tx.clone(),
            #[cfg(feature = "frb-spawn")]
            _sideeffect_task: std::sync::Mutex::new(None),
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
}
