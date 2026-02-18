#![cfg(feature = "state-persistence")]

use oxide_core::persistence;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct Model {
    counter: u64,
    label: String,
}

#[test]
fn persistence_round_trip_model() {
    let value = Model {
        counter: 42,
        label: "hello".to_string(),
    };

    let bytes = persistence::encode(&value).expect("encode");
    let decoded: Model = persistence::decode(&bytes).expect("decode");
    assert_eq!(decoded, value);
}

#[test]
fn persistence_decode_rejects_invalid_payload() {
    let bytes = vec![0_u8, 1, 2, 3];
    let decoded: Result<Model, _> = persistence::decode(&bytes);
    assert!(decoded.is_err());
}

#[cfg(feature = "persistence-json")]
#[test]
fn json_codec_is_used_for_encode_decode() {
    let value = Model {
        counter: 7,
        label: "world".to_string(),
    };

    let bytes = persistence::encode(&value).expect("encode");
    assert!(
        bytes.starts_with(b"{"),
        "expected JSON bytes when persistence-json feature is enabled"
    );

    let decoded: Model = persistence::decode(&bytes).expect("decode");
    assert_eq!(decoded, value);
}

#[cfg(feature = "persistence-json")]
#[test]
fn json_string_codec_round_trip() {
    let value = Model {
        counter: 99,
        label: "oxide".to_string(),
    };

    let json = persistence::encode_json(&value).expect("encode");
    let decoded: Model = persistence::decode_json(&json).expect("decode");
    assert_eq!(decoded, value);
}

#[cfg(feature = "persistence-json")]
#[test]
fn json_string_codec_rejects_invalid_payload() {
    let decoded: Result<Model, _> = persistence::decode_json("not-json");
    assert!(decoded.is_err());
}
