//! Shared demo state for the API browser app’s isolated-channels example.
//!
//! This is deliberately minimal and synchronous to keep the demo focused on the
//! channel primitives rather than on state management.

use std::sync::{Mutex, OnceLock};

/// Lazily-initialized storage for the last duplex incoming message.
///
/// Why `OnceLock<Mutex<...>>`:
/// - The demo is additive and may never be invoked, so initialization should be lazy.
/// - The value can be accessed from multiple threads (FFI calls + runtime tasks).
static LAST_INCOMING: OnceLock<Mutex<Option<String>>> = OnceLock::new();

/// Records the last incoming duplex message text.
pub fn set_last_incoming_text(text: String) {
    let lock = LAST_INCOMING.get_or_init(|| Mutex::new(None));
    // Poisoning is not actionable for this demo; recovering keeps it resilient.
    *lock.lock().unwrap_or_else(|e| e.into_inner()) = Some(text);
}

/// Returns the last incoming duplex message text, if any.
pub fn last_incoming_text() -> Option<String> {
    LAST_INCOMING
        .get()
        .and_then(|m| m.lock().ok().and_then(|v| v.clone()))
}
