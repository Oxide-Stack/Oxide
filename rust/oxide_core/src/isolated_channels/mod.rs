//! Transport-only, isolated channels for Rust ↔ Dart communication.
//!
//! This module implements the locked `OxideIsolatedChannels` specification:
//!
//! - [`OxideEventChannel`]: Rust → Dart fire-and-forget events
//! - [`OxideCallbacking`]: Rust → Dart → Rust request/response callbacks
//! - [`OxideEventDuplexChannel`]: lightweight paired streams (Outgoing + Incoming)
//!
//! The types here are intentionally runtime-light and isolated from reducers/effects.
//! Generated glue lives in downstream crates (via `oxide_generator_rs`) and uses
//! these runtime primitives without relying on implicit routing or registries.

mod callbacking;
mod error;
mod event_channel;
mod event_duplex_channel;
mod runtime;

pub use callbacking::CallbackRuntime;
pub use error::{OxideChannelError, OxideChannelResult};
pub use event_channel::{EventChannelRuntime, OxideEventChannel};
pub use event_duplex_channel::{IncomingHandler, OxideEventDuplexChannel};
pub use runtime::{
    OxideIsolatedChannelsRuntime, ensure_isolated_channels_initialized, init_isolated_channels,
    isolated_channels_initialized, isolated_channels_runtime,
};

pub use callbacking::OxideCallbacking;
