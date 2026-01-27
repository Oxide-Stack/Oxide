#![doc = include_str!("../README.md")]

mod core;

/// FFI-oriented utilities.
pub mod ffi;

#[cfg(feature = "state-persistence")]
/// Optional state persistence utilities.
pub mod persistence;

pub use core::{CoreError, CoreResult, Reducer, ReducerEngine, StateChange, StateSnapshot};
pub use ffi::{OxideError, watch_receiver_to_stream};
pub use tokio;

#[cfg(feature = "state-persistence")]
pub use serde;

#[cfg(test)]
mod tests;
