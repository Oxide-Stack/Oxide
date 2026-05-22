//! State persistence utilities.
//!
//! This module is only available when the `state-persistence` feature is enabled.
//!
//! ## Backends
//!
//! Oxide selects a persistence backend based on the compilation target:
//!
//! - Native / WASI: atomic filesystem writes (write temp file then rename).
//! - Web (`wasm32-unknown-unknown`): `window.localStorage` using a key derived
//!   from [`default_persistence_path`], storing the payload as base64.
//!
//! ## Serialization format
//! - Persisted snapshots always use bincode.
//! - Optional debug JSON copies can be generated when enabled.

mod backend;
mod codec;
mod config;
mod path;
mod worker;

pub(crate) use backend::{try_read_bytes, try_write_bytes_atomic};
pub(crate) use codec::encode_debug_json_and_validate;
pub use codec::{decode, encode};
pub use config::PersistenceConfig;
pub use path::{default_persistence_debug_json_path, default_persistence_path};
pub use worker::FilePersistenceWorker;

use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_JSON_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enables or disables debug JSON persistence copies.
pub fn set_debug_json_enabled(enabled: bool) {
    DEBUG_JSON_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Returns `true` if debug JSON persistence copies are enabled.
pub fn debug_json_enabled() -> bool {
    DEBUG_JSON_ENABLED.load(Ordering::Relaxed)
}
