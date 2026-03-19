#![cfg(feature = "isolated-channels")]

use oxide_generator_rs::{oxide_callback, oxide_event_channel};

pub enum UiEvent {
    Tick(u64),
    Reset,
}

pub enum OutgoingEvent {
    Broadcast(String),
}

pub enum IncomingEvent {
    Start,
}

pub enum DialogRequest {
    Confirm { title: String },
    Ping,
}

pub enum DialogResponse {
    Confirm(bool),
    Ping,
}

pub struct AnalyticsChannel;

#[oxide_event_channel(no_frb)]
impl oxide_core::OxideEventChannel for AnalyticsChannel {
    type Events = UiEvent;
}

pub struct DuplexChannel;

#[oxide_event_channel(orphaned = true, no_frb)]
impl oxide_core::OxideEventDuplexChannel for DuplexChannel {
    type Outgoing = OutgoingEvent;
    type Incoming = IncomingEvent;
}

pub struct DialogService;

#[oxide_callback(no_frb)]
impl oxide_core::OxideCallbacking for DialogService {
    type Request = DialogRequest;
    type Response = DialogResponse;
}

fn main() {}
