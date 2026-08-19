//! Rust library for the Oxide “benchmark” Flutter example.
//!
//! The public API is consumed by Flutter via generated bindings.

/// FFI-facing API surface for the Flutter example.
pub mod api;
mod frb_generated;
mod state;
mod util;

pub use oxide_core::{CoreResult, OxideError};
pub use serde_json::Value;

#[oxide_generator_rs::routes]
pub mod routes {}

#[cfg(test)]
mod tests;
