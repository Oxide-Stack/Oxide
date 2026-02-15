use thiserror::Error;

// Cross-boundary error model.
//
// Why: A stable, string-based error type keeps FFI surfaces predictable and
// avoids leaking dependency-specific error enums across language boundaries.
/// Convenience alias used throughout the crate.
pub type CoreResult<T> = Result<T, OxideError>;

/// Error type for core operations.
///
/// This error is intentionally small and string-based to allow the crate to be
/// embedded in different contexts (including FFI boundaries) without exposing
/// many dependency-specific error types.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum OxideError {
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
