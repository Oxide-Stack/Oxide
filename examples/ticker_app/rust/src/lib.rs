//! Rust library for the Oxide “ticker” Flutter example.
//!
//! The Flutter app drives this library via generated bindings.

/// FFI-facing API surface for the Flutter example.
pub mod api;
mod state;
mod frb_generated;

/// Error type exposed across the FFI boundary.
pub use oxide_core::OxideError;

#[cfg(test)]
mod tests;
