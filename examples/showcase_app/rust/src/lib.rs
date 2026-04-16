pub mod api;
pub mod config;
mod frb_generated;

pub use oxide_core::{CoreResult, OxideError};
pub use serde_json::Value;

#[oxide_generator_rs::routes]
pub mod routes {}
