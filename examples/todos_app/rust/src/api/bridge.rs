
use oxide_generator_rs::reducer;

/// Error type exposed across the FFI boundary.
pub use oxide_core::OxideError;

use crate::state::{AppAction, AppState, TodoItem};

#[flutter_rust_bridge::frb(init)]
/// Initializes Flutter Rust Bridge for this library.
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb]
pub async fn init_oxide() -> Result<(), oxide_core::OxideError> {
    fn thread_pool() -> oxide_core::runtime::ThreadPool {
        crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()
    }

    let _ = oxide_core::runtime::init(thread_pool);
    Ok(())
}

#[reducer(
    engine = AppEngine,
    snapshot = AppStateSnapshot,
    initial = AppState::new(),
    persist = "oxide.todos.state.v1",
    persist_min_interval_ms = 200,
)]
impl oxide_core::Reducer for AppRootReducer {
    type State = AppState;
    type Action = AppAction;
    type SideEffect = AppSideEffect;

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {}

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
        _state: &mut Self::State,
        _effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

#[flutter_rust_bridge::frb(ignore)]
pub(crate) enum AppSideEffect {}

#[flutter_rust_bridge::frb(ignore)]
#[derive(Default)]
pub(crate) struct AppRootReducer {}
