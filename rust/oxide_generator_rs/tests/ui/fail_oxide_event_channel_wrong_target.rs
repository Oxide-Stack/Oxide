#![cfg(feature = "isolated-channels")]

use oxide_generator_rs::oxide_event_channel;

#[oxide_event_channel]
pub struct NotAnImpl;

fn main() {}
