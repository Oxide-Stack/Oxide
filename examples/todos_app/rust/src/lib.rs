//! Rust library for the Oxide “todos” Flutter example.
//!
//! This example demonstrates validation, error handling, and optional state
//! persistence (when enabled by features on the Rust side).

/// FFI-facing API surface for the Flutter example.
pub mod api;
mod state;
mod frb_generated;

/// Error type exposed across the FFI boundary.
pub use oxide_core::OxideError;

#[cfg(test)]
mod tests;
