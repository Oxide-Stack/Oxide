//! Rust library for the Oxide “counter” Flutter example.
//!
//! This crate exposes a small, FFI-friendly API surface used by the Flutter app.
//! The public types are primarily generated via `oxide_generator_rs` and
//! `flutter_rust_bridge`.

/// FFI-facing API surface for the Flutter example.
pub mod api;
mod frb_generated;
mod state; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */

#[cfg(feature = "isolated-channels")]
mod isolated_channels_demo;

pub use oxide_core::{CoreResult, OxideChannelError, OxideError};
pub use serde_json::Value;

#[oxide_generator_rs::routes]
pub mod routes {}

#[cfg(test)]
mod tests;
