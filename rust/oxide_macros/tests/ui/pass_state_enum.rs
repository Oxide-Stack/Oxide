use oxide_macros::state;

#[state]
pub enum MyState {
    Empty,
    Loaded { value: u64 },
}

fn main() {}

