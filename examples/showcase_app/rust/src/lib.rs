pub mod api;
pub mod config;
mod frb_generated;
#[cfg(feature = "isolated-channels")]
mod isolated_channels_demo;

pub use oxide_core::{CoreResult, OxideChannelError, OxideError};
pub use serde_json::Value;

#[oxide_generator_rs::routes]
pub mod routes {}
