use oxide_macros::actions;

#[actions]
pub enum MyAction {
    Increment,
    SetLabel { label: String },
}

fn main() {}

