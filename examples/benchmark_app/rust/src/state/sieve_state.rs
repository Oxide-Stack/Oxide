use oxide_generator_rs::state;

#[state]
pub struct SieveState {
    pub counter: u64,
    pub checksum: u64,
}

impl SieveState {
    pub fn new() -> Self {
        Self {
            counter: 0,
            checksum: 0xcbf29ce484222325u64,
        }
    }
}

