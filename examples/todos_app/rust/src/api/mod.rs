//! Public API for the todos example’s Rust library.
//!
//! # Examples
//! ```
//! use rust_lib_counter_app::api::bridge;
//!
//! bridge::init_app();
//! ```

pub mod bridge;

#[cfg(feature = "navigation-binding")]
pub mod oxide_navigation {
    pub use crate::routes::oxide_navigation::*;
}
