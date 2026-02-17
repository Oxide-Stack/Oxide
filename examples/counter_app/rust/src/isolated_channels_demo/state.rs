//! Shared demo state for the counter app’s isolated-channels example.
//!
//! This is deliberately tiny and synchronous:
//! - The demo only needs to store a “last seen” string from the duplex incoming
//!   handler.
//! - Access is performed from multiple threads (FFI calls + runtime tasks), so
//!   it must be synchronized.

use std::sync::{Mutex, OnceLock};

/// Lazily-initialized storage for the last duplex incoming message.
///
/// Why `OnceLock<Mutex<...>>`:
/// - The example can run without initializing the isolated channel runtime, so
///   this avoids any eager allocation.
/// - A `Mutex` is sufficient since this is a low-frequency, demo-only value.
static LAST_INCOMING: OnceLock<Mutex<Option<String>>> = OnceLock::new();

/// Records the last incoming duplex message text.
pub fn set_last_incoming_text(text: String) {
    let lock = LAST_INCOMING.get_or_init(|| Mutex::new(None));
    // Poisoning is not actionable for this demo; recovering the inner value keeps
    // the example resilient if a panic occurs in another thread.
    *lock.lock().unwrap_or_else(|e| e.into_inner()) = Some(text);
}

/// Returns the last incoming duplex message text, if any.
pub fn last_incoming_text() -> Option<String> {
    LAST_INCOMING
        .get()
        .and_then(|m| m.lock().ok().and_then(|v| v.clone()))
}
