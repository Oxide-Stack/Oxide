//! Public API for the counter example’s Rust library.

pub mod bridge;
#[cfg(feature = "navigation-binding")]
pub use crate::navigation::frb as navigation_bridge;
