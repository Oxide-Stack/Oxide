use serde_json::Value;

pub const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325u64;
pub const FNV_PRIME: u64 = 0x00000100000001b3u64;

pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

pub fn fnv1a_mix_u64(mut hash: u64, v: u64) -> u64 {
    for b in v.to_le_bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

pub fn count_entries(value: &Value) -> u64 {
    value
        .get("entries")
        .and_then(|v| v.as_array())
        .map(|a| a.len() as u64)
        .unwrap_or(0)
}

pub fn canonicalize_json(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let mut entries = std::mem::take(map).into_iter().collect::<Vec<(String, Value)>>();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            let mut next = serde_json::Map::new();
            for (k, mut v) in entries {
                canonicalize_json(&mut v);
                next.insert(k, v);
            }
            *map = next;
        }
        Value::Array(arr) => {
            for v in arr {
                canonicalize_json(v);
            }
        }
        _ => {}
    }
}

