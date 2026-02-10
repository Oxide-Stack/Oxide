//! Rust library for the Oxide “counter” Flutter example.
//!
//! This crate exposes a small, FFI-friendly API surface used by the Flutter app.
//! The public types are primarily generated via `oxide_generator_rs` and
//! `flutter_rust_bridge`.

/// FFI-facing API surface for the Flutter example.
pub mod api;
mod state;
mod frb_generated;

/// Error type exposed across the FFI boundary.
pub use oxide_core::OxideError;

#[oxide_generator_rs::routes]
pub mod routes {
    include!("routes/mod.rs");
}

#[cfg(test)]
mod tests;
