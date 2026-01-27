use tokio::sync::mpsc;

use crate::core::CoreResult;

/// Indicates whether applying an action/effect produced a new state snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateChange {
    /// No externally-visible change; no new snapshot should be emitted.
    None,
    /// State changed and a new snapshot should be emitted.
    FullUpdate,
}

/// A reducer defines how actions mutate state and how side-effects are applied.
///
/// Implementations are expected to be deterministic with respect to their inputs:
/// given the same `state` + `action`/`effect`, they should produce the same
/// resulting state and [`StateChange`].
///
/// # Threading
/// Reducers are used by [`ReducerEngine`](crate::core::ReducerEngine). Actions are
/// dispatched serially; you do not need to make your reducer internally
/// thread-safe beyond the trait bounds.
pub trait Reducer: Send + Sync + 'static {
    /// The store state type.
    type State: Clone + Send + Sync + 'static;
    /// The action type consumed by [`Reducer::reduce`].
    type Action: Send + 'static;
    /// A reducer-defined side-effect type that can be sent to the engine.
    type SideEffect: Send + 'static;

    /// Called once when the reducer is installed in an engine.
    ///
    /// The provided sender can be used by the reducer to enqueue side-effects
    /// (for example, from within [`Reducer::reduce`]).
    fn init(&mut self, sideeffect_tx: mpsc::UnboundedSender<Self::SideEffect>);

    /// Applies an action to the provided `state`.
    ///
    /// Returning [`StateChange::FullUpdate`] indicates that a new snapshot should
    /// be emitted to subscribers.
    fn reduce(&mut self, state: &mut Self::State, action: Self::Action) -> CoreResult<StateChange>;

    /// Applies a previously-enqueued side-effect to the provided `state`.
    ///
    /// Side-effects are processed by the engine's background loop and are
    /// intended for work that must be applied out-of-band from normal action
    /// dispatch.
    fn effect(
        &mut self,
        state: &mut Self::State,
        effect: Self::SideEffect,
    ) -> CoreResult<StateChange>;
}
