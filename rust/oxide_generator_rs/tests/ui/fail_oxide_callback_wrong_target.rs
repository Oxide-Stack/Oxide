#![cfg(feature = "isolated-channels")]

use oxide_generator_rs::oxide_callback;

#[oxide_callback]
pub struct NotAnImpl;

fn main() {}
