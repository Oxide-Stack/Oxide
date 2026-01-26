use thiserror::Error;

/// Convenience alias used throughout the crate.
pub type CoreResult<T> = Result<T, CoreError>;

/// Error type for core operations.
///
/// This error is intentionally small and string-based to allow the crate to be
/// embedded in different contexts (including FFI boundaries) without exposing
/// many dependency-specific error types.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CoreError {
    /// An unexpected error that indicates a bug or invariant violation.
    #[error("internal error: {message}")]
    Internal { message: String },

    /// An error caused by invalid input or an action that cannot be applied.
    #[error("validation error: {message}")]
    Validation { message: String },

    /// A requested resource does not exist.
    #[error("not found: {resource}")]
    NotFound { resource: String },

    /// An error related to serializing or persisting state.
    #[error("persistence error: {message}")]
    Persistence { message: String },
}
