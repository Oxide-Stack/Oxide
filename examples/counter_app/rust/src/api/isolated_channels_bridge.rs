//! Flutter Rust Bridge API for the isolated channels demo.
//!
//! This API is additive and feature-gated behind `isolated-channels`.

/// Re-exported so FRB-generated glue code can refer to the type unqualified.
pub use oxide_core::OxideChannelError;

use flutter_rust_bridge::DartFnFuture;

use crate::isolated_channels_demo::{
    CounterDemoDialog, CounterDemoDialogResponse, CounterDemoDuplex, CounterDemoEvent,
    CounterDemoIn, CounterDemoOut, CounterDemoEvents, install_duplex_incoming_handler,
    last_incoming_text,
};

/// A callback request envelope exposed to Dart for the demo dialog service.
///
/// Why this wrapper exists:
/// - The isolated-channels callback macro generates an internal request envelope
///   type whose name begins with underscores.
/// - Dart treats leading-underscore identifiers as library-private, so those
///   generated types cannot be imported and used from other generated files.
/// - Exposing this stable, public wrapper keeps the demo bindings usable.
#[derive(Clone, Debug)]
pub struct CounterDemoDialogPendingRequest {
    /// Request identifier that must be echoed back when responding.
    pub id: u64,
    /// Typed request payload.
    pub request: crate::isolated_channels_demo::CounterDemoDialogRequest,
}

#[flutter_rust_bridge::frb]
/// Initializes the isolated channels demo runtime and installs the duplex handler.
pub fn init_isolated_channels_demo() -> Result<(), oxide_core::OxideError> {
    oxide_core::init_isolated_channels()?;
    install_duplex_incoming_handler();
    Ok(())
}

#[flutter_rust_bridge::frb]
/// Emits a notification from Rust to Dart over the demo event channel.
pub fn emit_counter_demo_notification(message: String) {
    CounterDemoEvents::notify(message);
}

#[flutter_rust_bridge::frb]
/// Streams demo events (Rust → Dart).
pub async fn counter_demo_events_stream(sink: crate::frb_generated::StreamSink<CounterDemoEvent>) {
    crate::isolated_channels_demo::__oxide_isolated_events_counter_demo_events::frb::oxide_events_stream(sink).await
}

#[flutter_rust_bridge::frb]
/// Streams callback requests (Rust → Dart) for the demo dialog service.
pub async fn counter_demo_dialog_requests_stream(
    sink: crate::frb_generated::StreamSink<CounterDemoDialogPendingRequest>,
) {
    loop {
        let Some((id, request)) = crate::isolated_channels_demo::__oxide_isolated_callback_counter_demo_dialog::runtime()
            .recv_request()
            .await
        else {
            break;
        };
        let _ = sink.add(CounterDemoDialogPendingRequest { id, request });
    }
}

#[flutter_rust_bridge::frb]
/// Sends a callback response (Dart → Rust) for the demo dialog service.
pub async fn counter_demo_dialog_respond(
    id: u64,
    response: CounterDemoDialogResponse,
) -> Result<(), oxide_core::OxideChannelError> {
    crate::isolated_channels_demo::__oxide_isolated_callback_counter_demo_dialog::runtime()
        .respond(id, response)
        .await
}

#[flutter_rust_bridge::frb]
/// Requests confirmation from Dart and returns the typed response payload.
pub async fn counter_demo_dialog_confirm(title: String) -> Result<bool, oxide_core::OxideChannelError> {
    CounterDemoDialog::confirm(title).await
}

#[flutter_rust_bridge::frb]
/// Requests confirmation from Dart using flutter_rust_bridge's direct Rust → Dart callback support.
pub async fn counter_demo_dialog_confirm_via_frb_callback(
    title: String,
    dart_confirm: impl Fn(String) -> DartFnFuture<bool>,
) -> bool {
    dart_confirm(title).await
}

#[flutter_rust_bridge::frb]
/// Streams duplex outgoing messages (Rust → Dart).
pub async fn counter_demo_duplex_outgoing_stream(
    sink: crate::frb_generated::StreamSink<CounterDemoOut>,
) {
    crate::isolated_channels_demo::__oxide_isolated_duplex_counter_demo_duplex::frb::oxide_outgoing_stream(sink).await
}

#[flutter_rust_bridge::frb]
/// Sends a duplex outgoing message from Rust to Dart.
pub fn counter_demo_duplex_send(text: String) {
    CounterDemoDuplex::send(CounterDemoOut::Send { text });
}

#[flutter_rust_bridge::frb]
/// Delivers a duplex incoming message from Dart to Rust.
pub fn counter_demo_duplex_incoming(event: CounterDemoIn) -> Result<(), oxide_core::OxideChannelError> {
    crate::isolated_channels_demo::__oxide_isolated_duplex_counter_demo_duplex::frb::oxide_counter_demo_duplex_incoming(event)
}

#[flutter_rust_bridge::frb]
/// Returns the last duplex incoming message text observed by Rust.
pub fn counter_demo_last_incoming_text() -> Option<String> {
    last_incoming_text()
}
