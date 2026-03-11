//! Rust library for the Oxide “API browser” Flutter example.
//!
//! The public API is consumed by Flutter via generated bindings (FRB).

/// FFI-facing API surface for the Flutter example.
pub mod api;
mod frb_generated;
mod state;
mod util;

#[cfg(feature = "isolated-channels")]
mod isolated_channels_demo;

pub use oxide_core::{OxideError, OxideChannelError, CoreResult};
pub use serde_json::Value;

#[oxide_generator_rs::routes]
pub mod routes {
    include!("routes/mod.rs");
}
