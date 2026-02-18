//! Isolated-channels demo channel definitions for the API browser example.
//!
//! This module declares transport-only channel payload types and binds them to
//! Oxide isolated-channel runtimes via macros. The Flutter app consumes these
//! channels via feature-gated FRB bridge functions in `crate::api`.

use oxide_generator_rs::{oxide_callback, oxide_event_channel};

use crate::isolated_channels_demo::set_last_incoming_text;

/// Demo event channel emitting user-facing notifications.
pub struct ApiBrowserDemoEvents {}

#[oxide_event_channel]
impl oxide_core::OxideEventChannel for ApiBrowserDemoEvents {
    /// Event payload enum streamed from Rust to Dart.
    type Events = ApiBrowserDemoEvent;
}

/// Event payloads emitted by [`ApiBrowserDemoEvents`].
#[derive(Clone, Debug)]
pub enum ApiBrowserDemoEvent {
    /// A human-readable notification message (shown by Flutter in the demo).
    Notify { message: String },
}

/// Demo callback service used to request user confirmation from Dart.
pub struct ApiBrowserDemoDialog {}

#[oxide_callback(no_frb)]
impl oxide_core::OxideCallbacking for ApiBrowserDemoDialog {
    /// Request envelope sent from Rust to Dart.
    type Request = ApiBrowserDemoDialogRequest;
    /// Response envelope returned from Dart back into Rust.
    type Response = ApiBrowserDemoDialogResponse;
}

/// Request enum for [`ApiBrowserDemoDialog`].
#[derive(Clone, Debug)]
pub enum ApiBrowserDemoDialogRequest {
    /// Ask Flutter to confirm an action and return the user’s choice.
    Confirm { title: String },
}

/// Response enum for [`ApiBrowserDemoDialog`].
#[derive(Clone, Debug)]
pub enum ApiBrowserDemoDialogResponse {
    /// Result for [`ApiBrowserDemoDialogRequest::Confirm`].
    Confirm(bool),
}

/// Demo duplex channel.
pub struct ApiBrowserDemoDuplex {}

#[oxide_event_channel]
impl oxide_core::OxideEventDuplexChannel for ApiBrowserDemoDuplex {
    /// Outgoing events emitted by Rust and observed by Dart.
    type Outgoing = ApiBrowserDemoOut;
    /// Incoming events emitted by Dart and delivered into Rust.
    type Incoming = ApiBrowserDemoIn;
}

/// Outgoing messages emitted by [`ApiBrowserDemoDuplex`].
#[derive(Clone, Debug)]
pub enum ApiBrowserDemoOut {
    /// Send a text message from Rust to Flutter.
    Send { text: String },
}

/// Incoming messages delivered from Dart into Rust.
#[derive(Clone, Debug)]
pub enum ApiBrowserDemoIn {
    /// Deliver a text message from Flutter into Rust.
    Receive { text: String },
}

/// Installs the duplex incoming handler for the demo.
pub fn install_duplex_incoming_handler() {
    // The handler updates shared demo state, which the Flutter example can query.
    ApiBrowserDemoDuplex::register_incoming(|event| match event {
        ApiBrowserDemoIn::Receive { text } => set_last_incoming_text(text),
    });
}
