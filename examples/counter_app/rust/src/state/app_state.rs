use oxide_generator_rs::state;

#[flutter_rust_bridge::frb(non_opaque)]
#[state]
pub struct AppState {
    pub counter: u64,
    pub last_confirmed: Option<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            counter: 0,
            last_confirmed: None,
        }
    }
}
