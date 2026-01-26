use oxide_macros::actions;

#[actions]
pub enum AppAction {
    Increment,
    Decrement,
    Reset,
}
