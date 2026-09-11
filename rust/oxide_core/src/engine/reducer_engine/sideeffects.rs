use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use tokio::sync::mpsc;

use crate::engine::{Context, Reducer, StateChange, StateSnapshot};

#[cfg(feature = "navigation-binding")]
use crate::engine::{NavigationCtx, navigation_runtime};

use super::Shared;

static ACTIVE_SIDEEFFECT_LOOPS: AtomicUsize = AtomicUsize::new(0);

/// Number of engine side-effect loops currently running.
///
/// Test/diagnostic-only instrumentation: the count is incremented when a
/// `sideeffect_loop` task starts and decremented when it exits (including when
/// aborted via [`ReducerEngine::shutdown`](super::ReducerEngine::shutdown)).
#[doc(hidden)]
pub fn debug_active_sideeffect_loops() -> usize {
    ACTIVE_SIDEEFFECT_LOOPS.load(Ordering::SeqCst)
}

struct SideeffectLoopGuard;

impl SideeffectLoopGuard {
    fn new() -> Self {
        ACTIVE_SIDEEFFECT_LOOPS.fetch_add(1, Ordering::SeqCst);
        Self
    }
}

impl Drop for SideeffectLoopGuard {
    fn drop(&mut self) {
        ACTIVE_SIDEEFFECT_LOOPS.fetch_sub(1, Ordering::SeqCst);
    }
}

pub(super) async fn sideeffect_loop<R, StateSlice>(
    shared: Arc<Shared<R, StateSlice>>,
    mut rx: mpsc::UnboundedReceiver<R::SideEffect>,
) where
    R: Reducer<StateSlice>,
    StateSlice: Copy + PartialEq + Eq + Send + Sync + 'static,
{
    let _loop_guard = SideeffectLoopGuard::new();
    while let Some(effect) = rx.recv().await {
        let mut engine_state = shared.engine_state.lock().await;
        let before_snapshot = StateSnapshot {
            revision: engine_state.revision,
            state: engine_state.state.clone(),
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
        let mut next_state = engine_state.state.clone();
        let ctx = Context {
            input: &effect,
            state_snapshot: &before_snapshot,
            #[cfg(feature = "navigation-binding")]
            nav: NavigationCtx::new(runtime, &route_ctx),
        };
        let change = match engine_state.reducer.effect(&mut next_state, ctx) {
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
            StateChange::Infer => engine_state
                .reducer
                .infer_slices(&engine_state.state, &next_state),
            StateChange::Slices(slices) => slices.to_vec(),
        };

        engine_state.state = next_state;
        engine_state.revision = engine_state.revision.saturating_add(1);

        let snapshot = StateSnapshot {
            revision: engine_state.revision,
            state: engine_state.state.clone(),
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
