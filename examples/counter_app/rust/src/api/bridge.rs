use oxide_generator_rs::reducer;
use std::sync::OnceLock;
use std::time::Duration;

/// Error type exposed across the FFI boundary.
pub use oxide_core::OxideError;

use crate::state::app_action::AppAction;
use crate::state::app_state::AppState;

static NAV_BOOTSTRAP: OnceLock<()> = OnceLock::new();

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
)]
impl oxide_core::Reducer for AppRootReducer {
    type State = AppState;
    type Action = AppAction;
    type SideEffect = AppSideEffect;

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {
        #[cfg(feature = "navigation-binding")]
        NAV_BOOTSTRAP.get_or_init(|| {
            let _ = oxide_core::init_navigation();
            let runtime = oxide_core::navigation_runtime().ok();
            oxide_core::tokio::spawn(async move {
                if let Some(runtime) = runtime {
                    runtime.set_current_route(Some(oxide_core::navigation::NavRoute {
                        kind: "Splash".into(),
                        payload: serde_json::to_value(crate::routes::SplashRoute {})
                            .unwrap_or(serde_json::Value::Null),
                        extras: None,
                    }));
                }

                oxide_core::tokio::time::sleep(Duration::from_millis(450)).await;

                if let Some(runtime) = runtime {
                    runtime.reset(vec![oxide_core::navigation::NavRoute {
                        kind: "Home".into(),
                        payload: serde_json::to_value(crate::routes::HomeRoute {})
                            .unwrap_or(serde_json::Value::Null),
                        extras: None,
                    }]);
                }
            });
        });
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<'_, Self::Action, Self::State, ()>,
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
        _ctx: oxide_core::Context<'_, Self::SideEffect, Self::State, ()>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

#[flutter_rust_bridge::frb(ignore)]
enum AppSideEffect {}

#[flutter_rust_bridge::frb(ignore)]
#[derive(Default)]
struct AppRootReducer {}
