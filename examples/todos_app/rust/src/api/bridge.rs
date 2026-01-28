
use oxide_generator_rs::reducer;

/// Error type exposed across the FFI boundary.
pub use oxide_core::OxideError;

use std::sync::OnceLock;

use crate::state::{AppAction, TodoItem,AppState};

#[flutter_rust_bridge::frb(init)]
/// Initializes Flutter Rust Bridge for this library.
pub fn init_app() {
    let _ = crate::runtime::runtime();
    flutter_rust_bridge::setup_default_user_utils();
}

#[reducer(
    engine = AppEngine,
    snapshot = AppStateSnapshot,
    initial = AppState::new(),
    persist = "oxide.todos.state.v1",
    persist_min_interval_ms = 200,
    tokio_handle = crate::runtime::handle()
)]
impl oxide_core::Reducer for AppReducer {
    type State = AppState;
    type Action = AppAction;
    type SideEffect = AppSideEffect;

    fn init(
        &mut self,
        sideeffect_tx: oxide_core::tokio::sync::mpsc::UnboundedSender<Self::SideEffect>,
    ) {
        let _ = SIDEFFECT_TX.set(sideeffect_tx);
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        action: Self::Action,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match action {
            AppAction::AddTodo { title } => {
                let trimmed = title.trim();
                if trimmed.is_empty() {
                    return Err(OxideError::Validation {
                        message: "todo title must not be empty".to_string(),
                    });
                }
                let id = format!("todo-{}", state.next_id);
                state.next_id = state.next_id.saturating_add(1);
                state.todos.push(TodoItem {
                    id,
                    title: trimmed.to_string(),
                    completed: false,
                });
            }
            AppAction::ToggleTodo { id } => {
                let todo = state
                    .todos
                    .iter_mut()
                    .find(|t| t.id == id)
                    .ok_or_else(|| OxideError::NotFound {
                        resource: format!("todo:{id}"),
                    })?;
                todo.completed = !todo.completed;
            }
            AppAction::DeleteTodo { id } => {
                let before_len = state.todos.len();
                state.todos.retain(|t| t.id != id);
                if state.todos.len() == before_len {
                    return Err(OxideError::NotFound {
                        resource: format!("todo:{id}"),
                    });
                }
            }
        }

        Ok(oxide_core::StateChange::FullUpdate)
    }

    fn effect(
        &mut self,
        state: &mut Self::State,
        effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match effect {
            AppSideEffect::SyncAddTodo { title } => self.reduce(state, AppAction::AddTodo { title }),
        }
    }
}

pub enum AppSideEffect {
    SyncAddTodo { title: String },
}

#[derive(Default)]
pub struct AppReducer {}

static SIDEFFECT_TX: OnceLock<oxide_core::tokio::sync::mpsc::UnboundedSender<AppSideEffect>> =
    OnceLock::new();
