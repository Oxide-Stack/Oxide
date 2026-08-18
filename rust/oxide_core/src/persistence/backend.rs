use std::path::Path;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use base64::Engine as _;

// Target-specific persistence backends.
//
// Oxide supports native/WASI and browser WASM with a single persistence API.
// Choose an implementation via `cfg` and keep the higher-level worker/codec
// logic backend-agnostic.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub(crate) fn try_read_bytes(path: &Path) -> Option<Vec<u8>> {
    std::fs::read(path).ok()
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub(crate) fn try_write_bytes_atomic(path: &Path, bytes: &[u8]) {
    let _ = write_bytes_atomic(path, bytes);
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn write_bytes_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Use a unique temp name: the bincode snapshot and its debug JSON copy
    // share the same stem, so a fixed `.tmp` extension would let one writer
    // rename away the other writer's temp file.
    let tmp_path = path.with_extension(format!("tmp.{}.{}", std::process::id(), unique_counter()));
    std::fs::write(&tmp_path, bytes)?;
    // `std::fs::rename` cannot replace an existing destination on Windows,
    // so remove the previous snapshot first. This is best-effort on purpose:
    // persistence writes are opportunistic snapshots, not a transactional log.
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    std::fs::rename(tmp_path, path)?;
    Ok(())
}

fn unique_counter() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub(crate) fn try_read_bytes(path: &Path) -> Option<Vec<u8>> {
    let storage = web_sys::window()?.local_storage().ok()??;
    let key = path.to_string_lossy();
    let encoded = storage.get_item(&key).ok()??;
    base64::engine::general_purpose::STANDARD
        .decode(encoded.as_bytes())
        .ok()
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub(crate) fn try_write_bytes_atomic(path: &Path, bytes: &[u8]) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return;
    };

    let key = path.to_string_lossy();
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
    let _ = storage.set_item(&key, &encoded);
}
