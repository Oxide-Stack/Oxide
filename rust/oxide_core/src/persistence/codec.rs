use crate::engine::{CoreResult, OxideError};

// Persistence encoding/decoding boundary.
//
// Snapshots use a versioned frame: fixed magic bytes (`OXSNAP`), a
// little-endian u32 format version, then the bincode payload. Payloads written
// by older Oxide versions were raw bincode with no frame; [`decode`] and
// [`decode_tracked`] still accept those via a legacy fallback so previously
// persisted files keep restoring (reported as [`DecodedSnapshot::Legacy`]).
// An optional debug JSON copy can be validated against the bincode bytes.

const SNAPSHOT_MAGIC: &[u8; 6] = b"OXSNAP";
const SNAPSHOT_FORMAT_VERSION: u32 = 1;

/// A decoded persistence payload, including which format produced it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodedSnapshot<T> {
    /// Decoded from the current versioned frame written by [`encode`].
    Current(T),
    /// Decoded via the legacy raw-bincode fallback (pre-versioned snapshots).
    Legacy(T),
}

impl<T> DecodedSnapshot<T> {
    /// Consumes the outcome and returns the decoded value regardless of format.
    pub fn into_value(self) -> T {
        match self {
            DecodedSnapshot::Current(value) | DecodedSnapshot::Legacy(value) => value,
        }
    }
}

/// Serializes a value to bytes for persistence.
///
/// This uses bincode wrapped in a versioned snapshot frame (magic + format
/// version), so future format changes can be detected instead of silently
/// mis-decoding.
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
    let payload = bincode::serialize(value).map_err(|e| OxideError::Persistence {
        message: e.to_string(),
    })?;
    let mut frame =
        Vec::with_capacity(SNAPSHOT_MAGIC.len() + std::mem::size_of::<u32>() + payload.len());
    frame.extend_from_slice(SNAPSHOT_MAGIC);
    frame.extend_from_slice(&SNAPSHOT_FORMAT_VERSION.to_le_bytes());
    frame.extend_from_slice(&payload);
    Ok(frame)
}

/// Deserializes a value from a persistence payload and reports which format was used.
///
/// Current payloads ([`encode`] frames) yield [`DecodedSnapshot::Current`]. Anything that does
/// not match the magic/version header is retried as legacy raw-bincode bytes, yielding
/// [`DecodedSnapshot::Legacy`] on success.
///
/// # Errors
/// Returns [`OxideError::Persistence`] if neither format can be decoded.
pub fn decode_tracked<T>(bytes: &[u8]) -> CoreResult<DecodedSnapshot<T>>
where
    T: serde::de::DeserializeOwned,
{
    let header_len = SNAPSHOT_MAGIC.len() + std::mem::size_of::<u32>();
    if bytes.len() >= header_len && &bytes[..SNAPSHOT_MAGIC.len()] == SNAPSHOT_MAGIC {
        let version_bytes = &bytes[SNAPSHOT_MAGIC.len()..header_len];
        // Guarded by `len >= header_len`, so this is exactly 4 bytes.
        let version = u32::from_le_bytes(match version_bytes.try_into() {
            Ok(v) => v,
            Err(_) => {
                return Err(OxideError::Persistence {
                    message: "snapshot header is truncated".to_string(),
                });
            }
        });
        if version == SNAPSHOT_FORMAT_VERSION {
            return bincode::deserialize(&bytes[header_len..])
                .map(DecodedSnapshot::Current)
                .map_err(|e| OxideError::Persistence {
                    message: e.to_string(),
                });
        }
        // A known magic with an unsupported version is a future-versioned
        // frame, not legacy raw bincode. Reject it rather than mis-reading the
        // header bytes as payload data.
        // Note: a legacy payload that coincidentally starts with `OXSNAP`
        // is treated as versioned and rejected; the tradeoff favors never
        // mis-decoding future frames as legacy data.
        return Err(OxideError::Persistence {
            message: format!("unsupported snapshot format version {version}"),
        });
    }

    // No magic header: legacy raw-bincode payload written before versioning existed.
    bincode::deserialize(bytes)
        .map(DecodedSnapshot::Legacy)
        .map_err(|e| OxideError::Persistence {
            message: e.to_string(),
        })
}

/// Deserializes a value from a persistence payload.
///
/// This expects bytes produced by [`encode`]; legacy raw-bincode payloads are also accepted
/// (see [`decode_tracked`] to distinguish the formats).
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
    Ok(decode_tracked(bytes)?.into_value())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    struct Sample {
        counter: u64,
    }

    #[test]
    fn encode_writes_versioned_frame_and_decode_round_trips() {
        let value = Sample { counter: 7 };
        let bytes = encode(&value).unwrap();

        assert!(bytes.starts_with(b"OXSNAP"));
        assert_eq!(&bytes[6..10], &1_u32.to_le_bytes());

        let tracked: DecodedSnapshot<Sample> = decode_tracked(&bytes).unwrap();
        assert_eq!(tracked, DecodedSnapshot::Current(Sample { counter: 7 }));
        assert_eq!(tracked.into_value(), Sample { counter: 7 });

        let plain: Sample = decode(&bytes).unwrap();
        assert_eq!(plain, value);
    }

    #[test]
    fn decode_accepts_legacy_raw_bincode_payloads() {
        let legacy = bincode::serialize(&Sample { counter: 9 }).unwrap();

        let tracked: DecodedSnapshot<Sample> = decode_tracked(&legacy).unwrap();
        assert_eq!(tracked, DecodedSnapshot::Legacy(Sample { counter: 9 }));
        assert_eq!(tracked.into_value(), Sample { counter: 9 });

        let plain: Sample = decode(&legacy).unwrap();
        assert_eq!(plain.counter, 9);
    }

    #[test]
    fn decode_rejects_garbage_and_unknown_versions() {
        let err = decode_tracked::<Sample>(&[0_u8, 1, 2, 3, 4, 5]);
        assert!(err.is_err());
        assert!(matches!(err.unwrap_err(), OxideError::Persistence { .. }));

        let future_version = b"OXSNAP".iter().copied().chain(2_u32.to_le_bytes()).chain(bincode::serialize(&Sample { counter: 1 }).unwrap()).collect::<Vec<u8>>();
        let err = decode_tracked::<Sample>(&future_version);
        assert!(matches!(err.unwrap_err(), OxideError::Persistence { .. }));
    }
}
