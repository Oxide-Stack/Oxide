use oxide_generator_rs::actions;

#[actions]
pub enum AppAction {
    Increment,
    Decrement,
    Reset,
    OpenDetail,
    OpenConfirm { title: String },
    Pop,
    PopUntilHome,
    ResetStack,
}
