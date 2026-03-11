//! Public API for the API browser example’s Rust library.

pub mod bridge;
pub mod comments_bridge;
pub mod posts_bridge;
pub mod users_bridge;

#[cfg(feature = "navigation-binding")]
pub mod oxide_navigation {
    pub use crate::routes::oxide_navigation::*;
}

#[cfg(feature = "isolated-channels")]
pub mod isolated_channels_bridge;
