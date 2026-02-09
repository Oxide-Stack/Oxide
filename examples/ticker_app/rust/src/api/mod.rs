//! Public API for the ticker example’s Rust library.
//!
//! # Examples
//! ```
//! use rust_lib_ticker_app::api::bridge;
//!
//! bridge::init_app();
//! ```

pub mod bridge;
#[cfg(feature = "navigation-binding")]
pub use crate::navigation::frb as navigation_bridge;
