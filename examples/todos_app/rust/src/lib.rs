//! Rust library for the Oxide “todos” Flutter example.
//!
//! This example demonstrates validation, error handling, and optional state
//! persistence (when enabled by features on the Rust side).
//!
//! # Examples
//! ```
//! use rust_lib_counter_app::api::bridge;
//!
//! bridge::init_app();
//! ```

/// FFI-facing API surface for the Flutter example.
pub mod api;
mod state;
mod frb_generated;

/// Error type exposed across the FFI boundary.
///
/// # Examples
/// ```
/// use rust_lib_counter_app::OxideError;
///
/// let _ = OxideError::Validation {
///     message: "example".to_string(),
/// };
/// ```
pub use oxide_core::OxideError;

#[cfg(feature = "navigation-binding")]
#[oxide_generator_rs::routes]
pub mod routes {
    include!("routes/mod.rs");
}

#[cfg(test)]
mod tests;
