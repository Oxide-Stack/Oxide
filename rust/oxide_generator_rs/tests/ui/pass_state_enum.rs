use oxide_generator_rs::state;

#[state]
pub enum MyState {
    Empty,
    Loaded { value: u64 },
}

fn main() {}
