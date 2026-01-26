use thiserror::Error;

use crate::CoreError;

/// Error type intended for FFI boundaries.
///
/// This mirrors [`CoreError`] but is kept in a separate type to allow different
/// naming and mapping strategies in generated bindings.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
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

impl From<CoreError> for OxideError {
    /// Converts a [`CoreError`] into an [`OxideError`].
    fn from(value: CoreError) -> Self {
        match value {
            CoreError::Internal { message } => Self::Internal { message },
            CoreError::Validation { message } => Self::Validation { message },
            CoreError::NotFound { resource } => Self::NotFound { resource },
            CoreError::Persistence { message } => Self::Persistence { message },
        }
    }
}
