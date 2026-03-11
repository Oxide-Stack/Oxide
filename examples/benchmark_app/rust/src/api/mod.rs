//! Public API for the benchmark example’s Rust library.

pub mod bridge;
pub mod counter_bridge;
pub mod json_bridge;
pub mod nav_bridge;
pub mod sieve_bridge;

#[cfg(feature = "navigation-binding")]
pub mod oxide_navigation {
    pub use crate::routes::oxide_navigation::*;
}
