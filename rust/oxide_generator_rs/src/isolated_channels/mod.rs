//! Proc-macro implementation for the Oxide isolated channels feature.
//!
//! These macros generate predictable, minimal glue code that follows the locked
//! `OxideIsolatedChannels` specification.

mod callback;
mod common;
mod event;
mod naming;
mod scan;
mod validate;

pub use callback::{OxideCallbackArgs, expand_oxide_callback};
pub use event::{OxideEventChannelArgs, expand_oxide_event_channel};

#[cfg(test)]
mod tests;
