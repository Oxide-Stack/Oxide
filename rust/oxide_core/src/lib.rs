//! Core Rust runtime for Oxide.
//!
//! This crate provides:
//! - A lightweight async reducer engine ([`ReducerEngine`]) built around a user-defined [`Reducer`].
//! - Optional persistence helpers (feature-gated) for writing state snapshots to disk.
//! - Small FFI-oriented utilities for integrating with other runtimes.
//!
//! ## Feature flags
//! - `state-persistence`: enables disk persistence support.
//! - `persistence-json`: when enabled (and `state-persistence` is enabled), persistence uses JSON.
//!   When disabled, persistence uses bincode.

mod core;

/// FFI-oriented utilities.
pub mod ffi;

#[cfg(feature = "state-persistence")]
/// Optional state persistence utilities.
pub mod persistence;

pub use core::{CoreError, CoreResult, Reducer, ReducerEngine, StateChange, StateSnapshot};
pub use ffi::{OxideError, watch_receiver_to_stream};
pub use tokio as tokio;

#[cfg(feature = "state-persistence")]
pub use serde as serde;

#[cfg(test)]
mod tests;
