#![doc = include_str!("../README.md")]

// Crate entrypoint and stable public surface.
//
// Why: Oxide needs a small, usage-agnostic Rust core that can be consumed from
// multiple environments (native, WASM, and Flutter Rust Bridge) without leaking
// internal module structure into public imports.
//
// How: Keep internal modules organized by responsibility and re-export the
// user-facing types from this file so refactors remain non-breaking.
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

pub use engine::InitContext;
pub use engine::{
    CoreResult, OxideError, Reducer, ReducerEngine, SlicedState, StateChange, StateSnapshot,
};
pub use engine::Context;
pub type ReducerCtx<'a, Input, State, StateSlice = ()> =
    engine::Context<'a, Input, State, StateSlice>;

/// Initializes global runtimes used by optional Oxide features.
///
/// Why: some features (navigation, isolated channels) use explicit global singletons for
/// generated glue code. This helper provides a single, idempotent entry point to initialize
/// all enabled globals consistently.
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

#[cfg(feature = "navigation-binding")]
pub use engine::{
    NavigationCtx, NavigationRuntime, init_navigation, navigation_runtime,
};

#[cfg(feature = "isolated-channels")]
pub use isolated_channels::{
    CallbackRuntime, EventChannelRuntime, IncomingHandler, OxideCallbacking, OxideChannelError,
    OxideChannelResult, OxideEventChannel, OxideEventDuplexChannel, ensure_isolated_channels_initialized,
    init_isolated_channels, isolated_channels_initialized, isolated_channels_runtime,
    OxideIsolatedChannelsRuntime,
};
pub use ffi::watch_receiver_to_stream;
pub use tokio;

#[cfg(any(feature = "state-persistence", feature = "navigation-binding"))]
pub use serde;

#[cfg(test)]
mod tests;
