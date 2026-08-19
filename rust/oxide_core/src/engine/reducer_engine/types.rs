use std::sync::Mutex as StdMutex;

use tokio::sync::{Mutex, mpsc, watch};

#[cfg(feature = "state-persistence")]
use crate::engine::CoreResult;
use crate::engine::{OxideError, Reducer, StateSnapshot};

#[cfg(feature = "state-persistence")]
use crate::persistence::FilePersistenceWorker;

// Core reducer runtime.
//
// Oxide needs a single place that defines the transactional update semantics
// (commit vs no-op vs error) so every binding layer sees identical behavior.
//
// Guard state behind a mutex, broadcast snapshots through a watch channel,
// and run side-effects in a background loop that uses the same commit rule.
pub(super) struct EngineState<R, StateSlice>
where
    R: Reducer<StateSlice>,
    StateSlice: Copy + PartialEq + Eq + Send + Sync + 'static,
{
    pub(super) state: R::State,
    pub(super) revision: u64,
    pub(super) reducer: R,
}

#[cfg(feature = "state-persistence")]
pub(super) struct PersistenceHooks<S> {
    pub(super) worker: FilePersistenceWorker,
    pub(super) encode: Box<dyn Fn(&S) -> CoreResult<Vec<u8>> + Send + Sync>,
    pub(super) debug: StdMutex<Option<DebugPersistenceHooks<S>>>,
}

#[cfg(feature = "state-persistence")]
pub(super) struct DebugPersistenceHooks<S> {
    pub(super) worker: FilePersistenceWorker,
    pub(super) encode: Box<dyn Fn(&S, &[u8]) -> CoreResult<Vec<u8>> + Send + Sync>,
}

pub(super) struct Shared<R, StateSlice>
where
    R: Reducer<StateSlice>,
    StateSlice: Copy + PartialEq + Eq + Send + Sync + 'static,
{
    pub(super) engine_state: Mutex<EngineState<R, StateSlice>>,
    pub(super) tx: watch::Sender<StateSnapshot<R::State, StateSlice>>,
    pub(super) error_tx: watch::Sender<Option<OxideError>>,
    pub(super) sideeffect_tx: mpsc::UnboundedSender<R::SideEffect>,
    #[cfg(feature = "frb-spawn")]
    pub(super) _sideeffect_task: StdMutex<Option<flutter_rust_bridge::JoinHandle<()>>>,
    #[cfg(feature = "state-persistence")]
    pub(super) persistence: Option<PersistenceHooks<R::State>>,
}
