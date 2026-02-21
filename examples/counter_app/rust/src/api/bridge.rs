use oxide_generator_rs::reducer;

/// Error type exposed across the FFI boundary.
pub use oxide_core::OxideError;

use crate::state::app_action::AppAction;
use crate::state::app_state::AppState;
use oxide_core::StateChange;

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

    async fn init(&mut self, ctx: oxide_core::InitContext<Self::SideEffect>) {
        self.sideeffect_tx = Some(ctx.sideeffect_tx);
        if let Ok(runtime) = oxide_core::navigation_runtime() {
            let _ = runtime.push(crate::routes::HomeRoute {});
        }
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::ReducerCtx<'_, Self::Action, Self::State>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match ctx.input {
            AppAction::Increment => {
                let next = state.counter.saturating_add(1);
                if next == state.counter {
                    return Ok(oxide_core::StateChange::None);
                }
                state.counter = next;
                Ok(StateChange::Full)
            }
            AppAction::Decrement => {
                let next = state.counter.saturating_sub(1);
                if next == state.counter {
                    return Ok(oxide_core::StateChange::None);
                }
                state.counter = next;
                Ok(StateChange::Full)
            }
            AppAction::Reset => {
                if state.counter == 0 {
                    return Ok(oxide_core::StateChange::None);
                }
                state.counter = 0;
                Ok(StateChange::Full)
            }
            AppAction::OpenDetail => {
                let _ = ctx.nav.push(crate::routes::CounterDetailRoute { start: state.counter });
                Ok(StateChange::None)
            }
            AppAction::OpenConfirm { title } => {
                state.last_confirmed = None;
                let Some(tx) = self.sideeffect_tx.as_ref() else {
                    return Ok(StateChange::Full);
                };
                let tx = tx.clone();
                let title = title.clone();
                oxide_core::runtime::spawn(async move {
                    let Ok(runtime) = oxide_core::navigation_runtime() else {
                        return;
                    };
                    let Ok((_ticket, rx)) = runtime.push_with_ticket(crate::routes::ConfirmRoute { title }).await else {
                        return;
                    };
                    let Ok(value) = rx.await else {
                        return;
                    };
                    let ok = serde_json::from_value::<bool>(value).unwrap_or(false);
                    let _ = tx.send(AppSideEffect::ConfirmResolved { ok });
                });
                Ok(StateChange::Full)
            }
            AppAction::Pop => {
                let _ = ctx.nav.pop();
                Ok(StateChange::None)
            }
            AppAction::PopUntilHome => {
                let _ = ctx.nav.pop_until(crate::routes::RouteKind::Home);
                Ok(StateChange::None)
            }
            AppAction::ResetStack => {
                if let Ok(runtime) = oxide_core::navigation_runtime() {
                    let _ = runtime.reset(vec![crate::routes::RoutePayload::Home(crate::routes::HomeRoute {})]);
                }
                Ok(StateChange::None)
            }
        }
    }

    fn effect(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::ReducerCtx<'_, Self::SideEffect, Self::State>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match ctx.input {
            AppSideEffect::ConfirmResolved { ok } => {
                state.last_confirmed = Some(*ok);
                Ok(StateChange::Full)
            }
        }
    }
}

#[flutter_rust_bridge::frb(ignore)]
enum AppSideEffect {
    ConfirmResolved { ok: bool },
}

#[flutter_rust_bridge::frb(ignore)]
#[derive(Default)]
struct AppRootReducer {
    sideeffect_tx: Option<oxide_core::tokio::sync::mpsc::UnboundedSender<AppSideEffect>>,
}
