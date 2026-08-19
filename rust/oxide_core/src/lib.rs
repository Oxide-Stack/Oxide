#![doc = include_str!("../README.md")]

// Stable public surface for the Rust core.
mod engine;

#[cfg(feature = "navigation-binding")]
/// Typed navigation primitives (Rust-driven, Flutter-native).
pub mod navigation;

#[cfg(feature = "isolated-channels")]
/// Transport-only typed Rust ↔ Dart channels.
pub mod isolated_channels;

/// FFI-oriented utilities.
pub mod ffi;

/// Async runtime utilities backed by Flutter Rust Bridge.
pub mod runtime;

#[cfg(feature = "state-persistence")]
/// Optional state persistence utilities.
pub mod persistence;

pub use engine::Context;
pub use engine::InitContext;
pub use engine::{
    CoreResult, OxideError, Reducer, ReducerEngine, SlicedState, StateChange, StateSnapshot,
};
pub type ReducerCtx<'a, Input, State, StateSlice = ()> =
    engine::Context<'a, Input, State, StateSlice>;

/// Initializes global runtimes used by optional Oxide features.
pub fn init_engine_globals() -> CoreResult<()> {
    #[cfg(feature = "navigation-binding")]
    {
        engine::init_navigation()?;
    }

    #[cfg(feature = "isolated-channels")]
    {
        isolated_channels::init_isolated_channels()?;
    }

    Ok(())
}

#[cfg(feature = "frb-spawn")]
pub fn init_from_frb(thread_pool_provider: fn() -> runtime::ThreadPool) -> CoreResult<()> {
    let _ = runtime::init(thread_pool_provider);
    init_engine_globals()?;
    Ok(())
}

#[cfg(not(feature = "frb-spawn"))]
pub fn init_from_frb(_thread_pool_provider: fn() -> ()) -> CoreResult<()> {
    runtime::ensure_initialized()?;
    init_engine_globals()?;
    Ok(())
}

#[cfg(feature = "navigation-binding")]
pub use engine::{NavigationCtx, NavigationRuntime, init_navigation, navigation_runtime};

pub use ffi::watch_receiver_to_stream;
#[cfg(feature = "isolated-channels")]
pub use isolated_channels::{
    CallbackRuntime, EventChannelRuntime, IncomingHandler, OxideCallbacking, OxideChannelError,
    OxideChannelResult, OxideEventChannel, OxideEventDuplexChannel, OxideIsolatedChannelsRuntime,
    ensure_isolated_channels_initialized, init_isolated_channels, isolated_channels_initialized,
    isolated_channels_runtime,
};
pub use tokio;

#[cfg(any(feature = "state-persistence", feature = "navigation-binding"))]
pub use serde;

#[cfg(test)]
mod tests;
