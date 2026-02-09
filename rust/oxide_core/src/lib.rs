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

#[cfg(feature = "navigation-binding")]
pub use engine::{
    NavigationCtx, NavigationRuntime, init_navigation, navigation_runtime,
};
pub use ffi::watch_receiver_to_stream;
pub use tokio;

#[cfg(any(feature = "state-persistence", feature = "navigation-binding"))]
pub use serde;

#[cfg(test)]
mod tests;
