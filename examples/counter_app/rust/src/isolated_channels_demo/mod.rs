//! Oxide isolated channels demo for the counter example.
//!
//! This module is feature-gated behind `isolated-channels` and is intentionally
//! transport-only. It demonstrates:
//!
//! - Rust → Dart event streaming (OxideEventChannel)
//! - Rust → Dart → Rust callbacks (OxideCallbacking)
//! - Lightweight duplex semantics (OxideEventDuplexChannel)

pub mod channels;
pub mod state;

pub use channels::*;
pub use state::*;
