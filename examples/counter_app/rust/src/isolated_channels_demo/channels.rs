//! Isolated-channels demo channel definitions for the counter example.
//!
//! This module is intentionally small and transport-only:
//! - It declares the channel payload types.
//! - It binds those types to Oxide channel runtimes via macros.
//!
//! The Flutter example consumes these channels through feature-gated FRB bridge
//! functions in `crate::api::isolated_channels_bridge`.

use oxide_generator_rs::{oxide_callback, oxide_event_channel};

use crate::isolated_channels_demo::set_last_incoming_text;

/// Demo event channel emitting user-facing notifications.
pub struct CounterDemoEvents {}

#[oxide_event_channel]
impl oxide_core::OxideEventChannel for CounterDemoEvents {
    /// Event payload enum streamed from Rust to Dart.
    type Events = CounterDemoEvent;
}

/// Event payloads emitted by [`CounterDemoEvents`].
#[derive(Clone, Debug)]
pub enum CounterDemoEvent {
    /// A human-readable notification message (shown by Flutter in the demo).
    Notify { message: String },
}

/// Demo callback service used to request user confirmation from Dart.
pub struct CounterDemoDialog {}

#[oxide_callback(no_frb)]
impl oxide_core::OxideCallbacking for CounterDemoDialog {
    /// Request envelope sent from Rust to Dart.
    type Request = CounterDemoDialogRequest;
    /// Response envelope returned from Dart back into Rust.
    type Response = CounterDemoDialogResponse;
}

/// Request enum for [`CounterDemoDialog`].
#[derive(Clone, Debug)]
pub enum CounterDemoDialogRequest {
    /// Ask Flutter to confirm an action and return the user’s choice.
    Confirm { title: String },
}

/// Response enum for [`CounterDemoDialog`].
#[derive(Clone, Debug)]
pub enum CounterDemoDialogResponse {
    /// Result for [`CounterDemoDialogRequest::Confirm`].
    Confirm(bool),
}

/// Demo duplex channel.
pub struct CounterDemoDuplex {}

#[oxide_event_channel]
impl oxide_core::OxideEventDuplexChannel for CounterDemoDuplex {
    /// Outgoing events emitted by Rust and observed by Dart.
    type Outgoing = CounterDemoOut;
    /// Incoming events emitted by Dart and delivered into Rust.
    type Incoming = CounterDemoIn;
}

/// Outgoing messages emitted by [`CounterDemoDuplex`].
#[derive(Clone, Debug)]
pub enum CounterDemoOut {
    /// Send a text message from Rust to Flutter.
    Send { text: String },
}

/// Incoming messages delivered from Dart into Rust.
#[derive(Clone, Debug)]
pub enum CounterDemoIn {
    /// Deliver a text message from Flutter into Rust.
    Receive { text: String },
}

/// Installs the duplex incoming handler for the demo.
pub fn install_duplex_incoming_handler() {
    // The handler updates shared demo state, which the Flutter example can query.
    CounterDemoDuplex::register_incoming(|event| match event {
        CounterDemoIn::Receive { text } => set_last_incoming_text(text),
    });
}
