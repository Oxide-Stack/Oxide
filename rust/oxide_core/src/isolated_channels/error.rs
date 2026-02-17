use thiserror::Error;

/// Error type shared across the Oxide isolated channels feature.
///
/// This error model is intentionally FFI-friendly and avoids panics in all
/// generated and runtime-facing APIs.
#[derive(Debug, Clone, Error, PartialEq)]
pub enum OxideChannelError {
    /// The channel runtime or endpoint is not available (not initialized or disconnected).
    #[error("channel unavailable")]
    Unavailable,

    /// A response was received but did not match the expected request/response binding.
    #[error("unexpected response")]
    UnexpectedResponse,

    /// Serialization failed while encoding/decoding payloads across the FFI boundary.
    #[error("serialization error")]
    SerializationError,

    /// A platform-specific error message coming from the embedding layer.
    #[error("platform error: {0}")]
    PlatformError(String),
}

/// Convenience result alias for channel operations.
pub type OxideChannelResult<T> = Result<T, OxideChannelError>;

