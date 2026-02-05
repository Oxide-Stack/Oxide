use tokio::sync::mpsc;

use crate::engine::CoreResult;

// Reducer and state-change contracts.
//
// Why: The engine needs a small, deterministic interface for applying updates
// that remains easy to reason about from both Rust and generated FFI surfaces.
//
// How: `Reducer` keeps mutation synchronous (for serialized updates) while
// allowing async work by pushing results back as side-effects.
/// Initialization context passed to [`Reducer::init`].
///
/// `InitContext` exists to give reducers a single place to receive all
/// runtime-provided resources they need to bootstrap background work:
///
/// - [`InitContext::sideeffect_tx`] lets reducers enqueue side-effects for
///   out-of-band processing by the engine.
/// - On Flutter targets using Flutter Rust Bridge (FRB), the `frb-spawn` feature
///   also provides a thread pool reference suitable for blocking work.
/// flutter_rust_bridge:ignore
pub struct InitContext<SideEffect> {
    /// Sender used to enqueue side-effects for background processing.
    pub sideeffect_tx: mpsc::UnboundedSender<SideEffect>,
    /// Thread pool used for blocking work on web/WASM (provided by FRB).
    #[cfg(feature = "frb-spawn")]
    pub thread_pool: crate::runtime::ThreadPool,
}

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
/// Reducers are intentionally split into three phases:
///
/// - [`Reducer::init`]: async initialization where the reducer can spawn tasks
///   and retain cloned handles to `sideeffect_tx`.
/// - [`Reducer::reduce`]: synchronous state transitions driven by user actions.
/// - [`Reducer::effect`]: synchronous state transitions driven by background work.
///
/// Keeping `reduce`/`effect` synchronous makes update ordering easy to reason
/// about (the engine serializes all mutations behind a single mutex), while
/// still enabling async I/O by performing it in spawned tasks and sending the
/// result back as a side-effect.
///
/// # Threading
/// Reducers are used by [`ReducerEngine`](crate::ReducerEngine). Actions are
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
    ///
    /// Implementations typically clone `ctx.sideeffect_tx` and move it into any
    /// background tasks the reducer spawns.
    async fn init(&mut self, ctx: InitContext<Self::SideEffect>);

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
