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
//! - With `persistence-json` enabled, [`encode`] and [`decode`] use JSON.
//! - Otherwise, [`encode`] and [`decode`] use bincode.

mod backend;
mod codec;
mod config;
mod path;
mod worker;

pub(crate) use backend::{try_read_bytes, try_write_bytes_atomic};
pub use codec::{decode, encode};
pub use config::PersistenceConfig;
pub use path::default_persistence_path;
pub use worker::FilePersistenceWorker;

#[cfg(feature = "persistence-json")]
pub use codec::{decode_json, encode_json};
