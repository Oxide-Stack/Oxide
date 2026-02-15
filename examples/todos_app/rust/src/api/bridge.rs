use oxide_generator_rs::reducer;

/// Error type exposed across the FFI boundary.
///
/// # Examples
/// ```
/// use rust_lib_counter_app::api::bridge::OxideError;
///
/// let _ = OxideError::Validation {
///     message: "example".to_string(),
/// };
/// ```
pub use oxide_core::OxideError;

use oxide_core::StateChange;
use oxide_core::tokio;
use std::sync::Arc;

use crate::state::{AppAction, AppState, AppStateSlice, TodoItem};

#[flutter_rust_bridge::frb(init)]
/// Initializes Flutter Rust Bridge for this library.
///
/// # Examples
/// ```
/// use rust_lib_counter_app::api::bridge::init_app;
///
/// init_app();
/// ```
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb]
/// Initializes the Oxide runtime for this example.
///
/// Call this once during app startup (after `RustLib.init()` on the Dart side)
/// before creating any engines.
///
/// # Examples
/// ```
/// use rust_lib_counter_app::api::bridge::init_oxide;
///
/// tokio::runtime::Runtime::new()
///     .unwrap()
///     .block_on(async { init_oxide().await.unwrap() });
/// ```
pub async fn init_oxide() -> Result<(), oxide_core::OxideError> {
    fn thread_pool() -> oxide_core::runtime::ThreadPool {
        crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()
    }

    let _ = oxide_core::runtime::init(thread_pool);
    Ok(())
}

#[flutter_rust_bridge::frb]
pub async fn create_shared_engine() -> Result<Arc<AppEngine>, oxide_core::OxideError> {
    static ENGINE: tokio::sync::OnceCell<Arc<AppEngine>> = tokio::sync::OnceCell::const_new();
    let engine = ENGINE.get_or_try_init(|| async { create_engine().await }).await?;
    Ok(Arc::clone(engine))
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
    type SideEffect = ();

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        state: &mut Self::State,
        action: Self::Action,
    ) -> oxide_core::CoreResult<StateChange<AppStateSlice>> {
        match action {
            AppAction::AddTodo { title } => {
                let trimmed = title.trim();
                if trimmed.is_empty() {
                    return Err(OxideError::Validation {
                        message: "todo title must not be empty".to_string(),
                    });
                }

                let id = Self::allocate_id(&mut state.next_id);
                state.todos.push(TodoItem {
                    id,
                    title: trimmed.to_string(),
                    completed: false,
                });
                Ok(StateChange::Infer)
            }
            AppAction::ToggleTodo { id } => {
                Self::toggle_todo(state, &id)?;
                Ok(StateChange::Infer)
            }
            AppAction::DeleteTodo { id } => {
                Self::delete_todo(state, &id)?;
                Ok(StateChange::Infer)
            }
        }
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<StateChange<AppStateSlice>> {
        Ok(StateChange::None)
    }
}

#[flutter_rust_bridge::frb(ignore)]
#[derive(Default)]
pub(crate) struct AppRootReducer {}

impl AppRootReducer {
    fn toggle_todo(state: &mut AppState, id: &str) -> oxide_core::CoreResult<()> {
        let todo = state
            .todos
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| OxideError::NotFound {
                resource: format!("todo:{id}"),
            })?;
        todo.completed = !todo.completed;
        Ok(())
    }

    fn delete_todo(state: &mut AppState, id: &str) -> oxide_core::CoreResult<()> {
        let before_len = state.todos.len();
        state.todos.retain(|t| t.id != id);
        if state.todos.len() == before_len {
            return Err(OxideError::NotFound {
                resource: format!("todo:{id}"),
            });
        }
        Ok(())
    }

    fn allocate_id(next_id: &mut u64) -> String {
        let id = format!("todo-{next_id}");
        *next_id = next_id.saturating_add(1);
        id
    }
}
