use oxide_macros::actions;

/// Actions that can be dispatched in the todos example.
#[actions]
pub enum AppAction {
    /// Add a new todo with the given title.
    AddTodo { title: String },
    /// Toggle completion for the todo with the given ID.
    ToggleTodo { id: String },
    /// Delete the todo with the given ID.
    DeleteTodo { id: String },
}
