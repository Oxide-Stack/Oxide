use crate::engine::{CoreResult, OxideError};

// Persistence encoding/decoding boundary.
//
// Persistence uses a stable bincode payload, with an optional debug JSON copy
// that can be validated against the bincode bytes.

/// Serializes a value to bytes for persistence.
///
/// This uses bincode.
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

/// Deserializes a value from a persistence payload.
///
/// This expects bincode bytes produced by [`encode`].
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

pub(crate) fn encode_debug_json_and_validate<T>(
    value: &T,
    expected_bincode: &[u8],
) -> CoreResult<Vec<u8>>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let json = serde_json::to_vec_pretty(value).map_err(|e| OxideError::Persistence {
        message: format!("failed to encode debug JSON: {e}"),
    })?;
    let decoded: T = serde_json::from_slice(&json).map_err(|e| OxideError::Persistence {
        message: format!("failed to decode debug JSON: {e}"),
    })?;
    let decoded_bincode = encode(&decoded)?;
    if decoded_bincode != expected_bincode {
        return Err(OxideError::Persistence {
            message: "debug JSON copy diverged from bincode payload".to_string(),
        });
    }
    Ok(json)
}
