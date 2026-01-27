use oxide_generator_rs::actions;

#[actions]
pub enum MyAction {
    Increment,
    SetLabel { label: String },
}

fn main() {}
