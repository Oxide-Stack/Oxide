use oxide_generator_rs::reducer;

/// Error type exposed across the FFI boundary.
pub use oxide_core::OxideError;

use crate::state::app_action::AppAction;
use crate::state::app_state::AppState;

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
    crate::navigation::runtime::init()?;
    Ok(())
}

#[reducer(
    engine = AppEngine,
    snapshot = AppStateSnapshot,
    initial = AppState::new(),
)]
impl oxide_core::Reducer for AppRootReducer {
    type State = AppState;
    type Action = AppAction;
    type SideEffect = AppSideEffect;

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {
        if let Ok(runtime) = oxide_core::navigation_runtime() {
            runtime.push(crate::routes::HomeRoute {});
        }
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<'_, Self::Action, Self::State>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match ctx.input {
            AppAction::Increment => {
                let next = state.counter.saturating_add(1);
                if next == state.counter {
                    return Ok(oxide_core::StateChange::None);
                }
                state.counter = next;
                Ok(oxide_core::StateChange::Full)
            }
            AppAction::Decrement => {
                let next = state.counter.saturating_sub(1);
                if next == state.counter {
                    return Ok(oxide_core::StateChange::None);
                }
                state.counter = next;
                Ok(oxide_core::StateChange::Full)
            }
            AppAction::Reset => {
                if state.counter == 0 {
                    return Ok(oxide_core::StateChange::None);
                }
                state.counter = 0;
                Ok(oxide_core::StateChange::Full)
            }
        }
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<'_, Self::SideEffect, Self::State>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

#[flutter_rust_bridge::frb(ignore)]
enum AppSideEffect {}

#[flutter_rust_bridge::frb(ignore)]
#[derive(Default)]
struct AppRootReducer {}
