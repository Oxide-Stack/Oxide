use crate::isolated_channels_demo::{
    ShowcaseDemoDialog, ShowcaseDemoDialogResponse, ShowcaseDemoDuplex, ShowcaseDemoEvent,
    ShowcaseDemoEvents, ShowcaseDemoIn, ShowcaseDemoOut, install_duplex_incoming_handler,
    last_incoming_text,
};

#[derive(Clone, Debug)]
pub struct ShowcaseDemoDialogPendingRequest {
    pub id: u64,
    pub request: crate::isolated_channels_demo::ShowcaseDemoDialogRequest,
}

#[flutter_rust_bridge::frb]
pub fn init_isolated_channels_demo() {
    let _ = oxide_core::init_isolated_channels();
    install_duplex_incoming_handler();
}

#[flutter_rust_bridge::frb]
pub fn emit_showcase_demo_notification(message: String) {
    ShowcaseDemoEvents::notify(message);
}

#[flutter_rust_bridge::frb]
pub async fn showcase_demo_events_stream(sink: crate::frb_generated::StreamSink<ShowcaseDemoEvent>) {
    crate::isolated_channels_demo::__oxide_isolated_events_showcase_demo_events::frb::oxide_events_stream(sink).await
}

#[flutter_rust_bridge::frb]
pub async fn showcase_demo_dialog_requests_stream(
    sink: crate::frb_generated::StreamSink<ShowcaseDemoDialogPendingRequest>,
) {
    loop {
        let Some((id, request)) =
            crate::isolated_channels_demo::__oxide_isolated_callback_showcase_demo_dialog::runtime()
                .recv_request()
                .await
        else {
            break;
        };
        let _ = sink.add(ShowcaseDemoDialogPendingRequest { id, request });
    }
}

#[flutter_rust_bridge::frb]
pub async fn showcase_demo_dialog_respond(id: u64, response: ShowcaseDemoDialogResponse) {
    let _ = crate::isolated_channels_demo::__oxide_isolated_callback_showcase_demo_dialog::runtime()
        .respond(id, response)
        .await;
}

#[flutter_rust_bridge::frb]
pub async fn showcase_demo_dialog_confirm(title: String) -> bool {
    ShowcaseDemoDialog::confirm(title).await.unwrap_or(false)
}

#[flutter_rust_bridge::frb]
pub async fn showcase_demo_duplex_outgoing_stream(
    sink: crate::frb_generated::StreamSink<ShowcaseDemoOut>,
) {
    crate::isolated_channels_demo::__oxide_isolated_duplex_showcase_demo_duplex::frb::oxide_outgoing_stream(sink).await
}

#[flutter_rust_bridge::frb]
pub fn showcase_demo_duplex_send(text: String) {
    ShowcaseDemoDuplex::send(ShowcaseDemoOut::Send { text });
}

#[flutter_rust_bridge::frb]
pub fn showcase_demo_duplex_incoming(event: ShowcaseDemoIn) {
    let _ = crate::isolated_channels_demo::__oxide_isolated_duplex_showcase_demo_duplex::frb::oxide_showcase_demo_duplex_incoming(event);
}

#[flutter_rust_bridge::frb]
pub fn showcase_demo_last_incoming_text() -> Option<String> {
    last_incoming_text()
}
