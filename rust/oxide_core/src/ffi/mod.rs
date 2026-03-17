//! FFI-facing helpers.
//!
//! This module contains small utility types intended for consumption by other
//! runtimes and bindings layers.

// Maintenance: keep these helpers generic and dependency-light so they can be
// safely used by multiple binding strategies without pulling in app concepts.
mod stream;
#[cfg(feature = "frb-spawn")]
pub mod logger;

pub use stream::watch_receiver_to_stream;
