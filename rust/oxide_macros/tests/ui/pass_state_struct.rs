use oxide_macros::state;

#[state]
pub struct MyState {
    pub count: u64,
    pub label: String,
}

fn main() {}

