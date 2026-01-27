use oxide_generator_rs::state;

#[flutter_rust_bridge::frb(non_opaque)]
#[state]
pub struct CounterState {
    pub counter: u64,
    pub checksum: u64,
}

impl CounterState {
    pub fn new() -> Self {
        Self {
            counter: 0,
            checksum: 0xcbf29ce484222325u64,
        }
    }
}
