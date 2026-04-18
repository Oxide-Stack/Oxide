mod construct;
mod dispatch;
mod sideeffects;
mod types;

use std::sync::Arc;

use crate::engine::Reducer;

use sideeffects::sideeffect_loop;
use types::{EngineState, Shared};
#[cfg(feature = "state-persistence")]
use types::PersistenceHooks;

/// A reducer-driven engine that owns state, broadcasts snapshots, and runs side-effects.
///
/// `ReducerEngine` is the core runtime primitive used by Oxide's generated FRB surface:
///
/// - [`dispatch`](Self::dispatch) applies an action via [`Reducer::reduce`], updates state, and
///   broadcasts a [`StateSnapshot`](crate::engine::StateSnapshot) to subscribers.
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
/// 4. If the reducer returns [`StateChange::None`](crate::engine::StateChange::None), discard the clone (no snapshot emitted).
/// 5. If the reducer returns [`StateChange::Full`](crate::engine::StateChange::Full), commit the clone and emit a snapshot.
///
/// # Concurrency
/// Actions are applied serially (behind an internal mutex). Snapshot updates are broadcast via
/// a Tokio [`tokio::sync::watch`] channel, so new subscribers immediately receive the latest snapshot.
///
/// # Runtime requirements
/// Creating an engine spawns a background task via Flutter Rust Bridge.
/// Call `OxideStack.init()` from Dart (which calls `RustLib.init()` and the generated init hook) before creating engines.
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
