use oxide_generator_rs::state;

#[state]
pub struct JsonState {
    pub counter: u64,
    pub checksum: u64,
}

impl JsonState {
    pub fn new() -> Self {
        Self {
            counter: 0,
            checksum: 0xcbf29ce484222325u64,
        }
    }
}

