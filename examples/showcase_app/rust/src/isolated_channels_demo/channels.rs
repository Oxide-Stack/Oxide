use oxide_generator_rs::{oxide_callback, oxide_event_channel};

use crate::isolated_channels_demo::set_last_incoming_text;

pub struct ShowcaseDemoEvents {}

#[oxide_event_channel]
impl oxide_core::OxideEventChannel for ShowcaseDemoEvents {
    type Events = ShowcaseDemoEvent;
}

#[derive(Clone, Debug)]
pub enum ShowcaseDemoEvent {
    Notify { message: String },
}

pub struct ShowcaseDemoDialog {}

#[oxide_callback(no_frb)]
impl oxide_core::OxideCallbacking for ShowcaseDemoDialog {
    type Request = ShowcaseDemoDialogRequest;
    type Response = ShowcaseDemoDialogResponse;
}

#[derive(Clone, Debug)]
pub enum ShowcaseDemoDialogRequest {
    Confirm { title: String },
}

#[derive(Clone, Debug)]
pub enum ShowcaseDemoDialogResponse {
    Confirm(bool),
}

pub struct ShowcaseDemoDuplex {}

#[oxide_event_channel]
impl oxide_core::OxideEventDuplexChannel for ShowcaseDemoDuplex {
    type Outgoing = ShowcaseDemoOut;
    type Incoming = ShowcaseDemoIn;
}

#[derive(Clone, Debug)]
pub enum ShowcaseDemoOut {
    Send { text: String },
}

#[derive(Clone, Debug)]
pub enum ShowcaseDemoIn {
    Receive { text: String },
}

pub fn install_duplex_incoming_handler() {
    ShowcaseDemoDuplex::register_incoming(|event| match event {
        ShowcaseDemoIn::Receive { text } => set_last_incoming_text(text),
    });
}
