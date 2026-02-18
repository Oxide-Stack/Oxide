//! Oxide isolated channels demo for the API browser example.
//!
//! This module is feature-gated behind `isolated-channels` and demonstrates
//! the transport-only channel primitives in a way that does not alter the app’s
//! existing behavior unless the demo APIs are invoked from Dart.

pub mod channels;
pub mod state;

pub use channels::*;
pub use state::*;
