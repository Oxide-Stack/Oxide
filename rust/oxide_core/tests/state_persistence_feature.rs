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
