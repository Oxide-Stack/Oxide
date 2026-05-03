pub mod channels;

pub use channels::*;

use std::sync::{Mutex, OnceLock};

static LAST_INCOMING_TEXT: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn last_incoming_cell() -> &'static Mutex<Option<String>> {
    LAST_INCOMING_TEXT.get_or_init(|| Mutex::new(None))
}

pub fn set_last_incoming_text(text: String) {
    if let Ok(mut guard) = last_incoming_cell().lock() {
        *guard = Some(text);
    }
}

pub fn last_incoming_text() -> Option<String> {
    last_incoming_cell().lock().ok().and_then(|guard| guard.clone())
}
