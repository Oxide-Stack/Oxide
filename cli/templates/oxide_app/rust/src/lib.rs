// Rust library for the {{app_name}} Oxide app.
//
// This crate exposes a small, FFI-friendly API surface used by the Flutter app.

/// FFI-facing API surface.
pub mod api;
// Declared explicitly rather than relying on FRB auto-injecting it, so the
// crate still compiles under `oxide create --no-codegen` and if a future FRB
// stops injecting. The `#[oxide_generator_rs::routes]` expansion below
// hard-depends on `crate::frb_generated`.
mod frb_generated;
mod state;

pub use oxide_core::{CoreResult, OxideError};
pub use serde_json::Value;

#[oxide_generator_rs::routes]
pub mod routes {}
