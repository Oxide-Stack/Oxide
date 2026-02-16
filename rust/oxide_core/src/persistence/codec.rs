use crate::engine::{CoreResult, OxideError};

// Persistence encoding/decoding boundary.
//
// Why: persistence needs a stable, ergonomic API that can switch formats via
// feature flags without changing call sites.
//
// How: `encode`/`decode` are gated on `persistence-json` to choose JSON vs bincode
// while mapping errors into `OxideError::Persistence`.
#[cfg(not(feature = "persistence-json"))]
/// Serializes a value to bytes for persistence.
///
/// When `persistence-json` is disabled, this uses bincode.
///
/// # Errors
/// Returns [`OxideError::Persistence`] if serialization fails.
///
/// # Returns
/// A byte payload that can be written to disk and later passed to [`decode`].
pub fn encode<T>(value: &T) -> CoreResult<Vec<u8>>
where
    T: serde::Serialize,
{
    bincode::serialize(value).map_err(|e| OxideError::Persistence {
        message: e.to_string(),
    })
}

#[cfg(feature = "persistence-json")]
/// Serializes a value to bytes for persistence.
///
/// When `persistence-json` is enabled, this uses JSON.
///
/// # Errors
/// Returns [`OxideError::Persistence`] if serialization fails.
///
/// # Returns
/// A UTF-8 JSON byte payload that can be written to disk and later passed to [`decode`].
pub fn encode<T>(value: &T) -> CoreResult<Vec<u8>>
where
    T: serde::Serialize,
{
    serde_json::to_vec(value).map_err(|e| OxideError::Persistence {
        message: e.to_string(),
    })
}

#[cfg(not(feature = "persistence-json"))]
/// Deserializes a value from a persistence payload.
///
/// When `persistence-json` is disabled, this expects bincode bytes produced by [`encode`].
///
/// # Errors
/// Returns [`OxideError::Persistence`] if deserialization fails.
///
/// # Returns
/// The decoded value.
pub fn decode<T>(bytes: &[u8]) -> CoreResult<T>
where
    T: serde::de::DeserializeOwned,
{
    bincode::deserialize(bytes).map_err(|e| OxideError::Persistence {
        message: e.to_string(),
    })
}

#[cfg(feature = "persistence-json")]
/// Deserializes a value from a persistence payload.
///
/// When `persistence-json` is enabled, this expects UTF-8 JSON bytes produced by [`encode`].
///
/// # Errors
/// Returns [`OxideError::Persistence`] if deserialization fails.
///
/// # Returns
/// The decoded value.
pub fn decode<T>(bytes: &[u8]) -> CoreResult<T>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_slice(bytes).map_err(|e| OxideError::Persistence {
        message: e.to_string(),
    })
}

#[cfg(feature = "persistence-json")]
/// Serializes a value to a JSON string.
///
/// This is a convenience API and is only available when `persistence-json` is enabled.
///
/// # Errors
/// Returns [`OxideError::Persistence`] if serialization fails.
///
/// # Returns
/// A UTF-8 JSON string.
pub fn encode_json<T>(value: &T) -> CoreResult<String>
where
    T: serde::Serialize,
{
    serde_json::to_string(value).map_err(|e| OxideError::Persistence {
        message: e.to_string(),
    })
}

#[cfg(feature = "persistence-json")]
/// Deserializes a value from a JSON string.
///
/// # Errors
/// Returns [`OxideError::Persistence`] if parsing fails.
///
/// # Returns
/// The decoded value.
pub fn decode_json<T>(value: &str) -> CoreResult<T>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(value).map_err(|e| OxideError::Persistence {
        message: e.to_string(),
    })
}
