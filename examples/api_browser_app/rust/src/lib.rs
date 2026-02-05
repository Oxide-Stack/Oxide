//! Rust library for the Oxide “API browser” Flutter example.
//!
//! The public API is consumed by Flutter via generated bindings (FRB).

/// FFI-facing API surface for the Flutter example.
pub mod api;
mod state;
mod util;
mod frb_generated;

pub use oxide_core::OxideError;
