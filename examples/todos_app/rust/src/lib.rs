//! Rust library for the Oxide “todos” Flutter example.
//!
//! This example demonstrates validation, error handling, and optional state
//! persistence (when enabled by features on the Rust side).
//!
//! # Examples
//! ```
//! use rust_lib_todos_app::api::bridge;
//!
//! let _ = bridge::create_shared_engine();
//! ```

/// FFI-facing API surface for the Flutter example.
pub mod api;
mod state;
mod frb_generated;

/// Error type exposed across the FFI boundary.
///
/// # Examples
/// ```
/// use rust_lib_todos_app::OxideError;
///
/// let _ = OxideError::Validation {
///     message: "example".to_string(),
/// };
/// ```
pub use oxide_core::OxideError;

#[oxide_generator_rs::routes]
pub mod routes {}

#[cfg(test)]
mod tests;
