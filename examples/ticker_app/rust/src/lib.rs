//! Rust library for the Oxide “ticker” Flutter example.
//!
//! The Flutter app drives this library via generated bindings.
//!
//! # Examples
//! ```
//! use rust_lib_ticker_app::api::bridge;
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
/// use rust_lib_ticker_app::OxideError;
///
/// let _ = OxideError::Validation {
///     message: "example".to_string(),
/// };
/// ```
pub use oxide_core::OxideError;

#[oxide_generator_rs::routes]
pub mod routes {
    include!("routes/mod.rs");
}

#[cfg(test)]
mod tests;
