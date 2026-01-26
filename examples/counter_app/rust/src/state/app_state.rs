use oxide_macros::state;

#[flutter_rust_bridge::frb(non_opaque)]
#[state]
pub struct AppState {
    pub counter: u64,
}

impl AppState {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

