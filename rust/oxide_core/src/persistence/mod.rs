//! State persistence utilities.
//!
//! This module is only available when the `state-persistence` feature is enabled.
//!
//! ## Serialization format
//! - With `persistence-json` enabled, [`encode`] and [`decode`] use JSON.
//! - Otherwise, [`encode`] and [`decode`] use bincode.

mod codec;
mod config;
mod path;
mod worker;

pub use codec::{decode, encode};
pub use config::PersistenceConfig;
pub use path::default_persistence_path;
pub use worker::FilePersistenceWorker;

#[cfg(feature = "persistence-json")]
pub use codec::{decode_json, encode_json};
