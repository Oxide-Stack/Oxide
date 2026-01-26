use oxide_macros::state;

/// Single todo item stored in [`AppState`].
#[state]
pub struct TodoItem {
    /// Stable identifier for this item.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Whether the item is completed.
    pub completed: bool,
}

/// State for the todos example.
#[state]
pub struct AppState {
    /// Current list of todos.
    pub todos: Vec<TodoItem>,
    /// Monotonically increasing ID source.
    pub next_id: u64,
}

impl AppState {
    /// Creates an empty todos state.
    ///
    /// # Returns
    /// A state with no todos and a starting ID counter.
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            next_id: 1,
        }
    }
}

