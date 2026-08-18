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

use std::sync::atomic::{AtomicU8, Ordering};

static DEBUG_JSON_POLICY: AtomicU8 = AtomicU8::new(DEBUG_JSON_POLICY_AUTO);

const DEBUG_JSON_POLICY_AUTO: u8 = 0;
const DEBUG_JSON_POLICY_ON: u8 = 1;
const DEBUG_JSON_POLICY_OFF: u8 = 2;

fn policy_from_env() -> u8 {
    match option_env!("OXIDE_DEBUG_JSON") {
        Some(value)
            if value.eq_ignore_ascii_case("on")
                || value.eq_ignore_ascii_case("true")
                || value == "1" =>
        {
            DEBUG_JSON_POLICY_ON
        }
        Some(value)
            if value.eq_ignore_ascii_case("off")
                || value.eq_ignore_ascii_case("false")
                || value == "0" =>
        {
            DEBUG_JSON_POLICY_OFF
        }
        _ => DEBUG_JSON_POLICY_AUTO,
    }
}

fn policy_enabled(policy: u8) -> bool {
    match policy {
        DEBUG_JSON_POLICY_ON => true,
        DEBUG_JSON_POLICY_OFF => false,
        _ => cfg!(debug_assertions),
    }
}

/// Enables or disables debug JSON persistence copies.
pub fn set_debug_json_enabled(enabled: bool) {
    DEBUG_JSON_POLICY.store(
        if enabled {
            DEBUG_JSON_POLICY_ON
        } else {
            DEBUG_JSON_POLICY_OFF
        },
        Ordering::Relaxed,
    );
}

/// Returns `true` if debug JSON persistence copies are enabled.
pub fn debug_json_enabled() -> bool {
    let policy = DEBUG_JSON_POLICY.load(Ordering::Relaxed);
    let policy = if policy == DEBUG_JSON_POLICY_AUTO {
        policy_from_env()
    } else {
        policy
    };
    policy_enabled(policy)
}
