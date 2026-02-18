//! Engine primitives for Oxide state management.
//!
//! This module contains the building blocks used by the public API:
//!
//! - [`Reducer`]: user-defined state transition logic (actions + side-effects)
//! - [`InitContext`]: initialization context passed to [`Reducer::init`]
//! - [`ReducerEngine`]: runtime that owns state, emits snapshots, and runs side-effects
//! - [`StateSnapshot`]: revisioned view of the current state
//! - [`OxideError`]/[`CoreResult`]: error model suitable for FFI boundaries

// Maintenance: keep this module as the internal boundary for "engine semantics"
// (state, reducer traits, snapshot model, and dispatch loop). Anything target-
// specific should live in `runtime`, `ffi`, or `persistence`.
mod error;
mod context;
mod reducer_engine;
mod snapshot;
mod store;

#[cfg(feature = "navigation-binding")]
mod navigation_ctx;
#[cfg(feature = "navigation-binding")]
mod navigation_runtime;
#[cfg(feature = "navigation-binding")]
mod navigation_singleton;
#[cfg(feature = "navigation-binding")]
mod navigation_ticket_registry;

pub use error::{CoreResult, OxideError};
pub use context::Context;
pub use reducer_engine::ReducerEngine;
pub use snapshot::StateSnapshot;
pub use store::{InitContext, Reducer, SlicedState, StateChange};
#[cfg(feature = "navigation-binding")]
pub use navigation_ctx::NavigationCtx;
#[cfg(feature = "navigation-binding")]
pub use navigation_runtime::NavigationRuntime;
#[cfg(feature = "navigation-binding")]
pub use navigation_singleton::{init_navigation, navigation_runtime};
