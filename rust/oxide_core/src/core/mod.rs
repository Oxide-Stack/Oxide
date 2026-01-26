//! Core primitives for Oxide state management.
//!
//! This module contains the building blocks used by the public API:
//! - [`Reducer`]: user-defined state transition logic
//! - [`Store`] / [`StoreHandle`]: state storage and async interaction
//! - [`StateSnapshot`]: versioned view of store state
//! - [`Engine`]: registry for store handles keyed by reducer type

mod error;
mod reducer_engine;
mod snapshot;
mod store;
pub(crate) mod task;

pub use error::{CoreError, CoreResult};
pub use reducer_engine::ReducerEngine;
pub use snapshot::StateSnapshot;
pub use store::{Reducer, StateChange};
