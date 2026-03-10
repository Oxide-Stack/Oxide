//! Public API for the todos example’s Rust library.
//!
//! # Examples
//! ```
//! use rust_lib_todos_app::api::bridge;
//!
//! let _ = bridge::create_shared_engine();
//! ```

pub mod bridge;

#[cfg(feature = "navigation-binding")]
pub mod oxide_navigation {
    pub use crate::routes::oxide_navigation::*;
}
