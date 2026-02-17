//! Proc-macro implementation for the Oxide isolated channels feature.
//!
//! These macros generate predictable, minimal glue code that follows the locked
//! `OxideIsolatedChannels` specification.

mod callback;
mod event;
mod naming;
mod scan;
mod validate;

pub use callback::{expand_oxide_callback, OxideCallbackArgs};
pub use event::{expand_oxide_event_channel, OxideEventChannelArgs};

#[cfg(test)]
mod tests;
