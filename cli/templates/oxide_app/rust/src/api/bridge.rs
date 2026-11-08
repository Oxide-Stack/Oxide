use oxide_generator_rs::reducer;

use crate::state::app_action::AppAction;
use crate::state::app_state::AppState;
use oxide_core::StateChange;

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
        }
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::ReducerCtx<'_, Self::SideEffect, Self::State>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(StateChange::None)
    }
}

#[flutter_rust_bridge::frb(ignore)]
enum AppSideEffect {}

#[flutter_rust_bridge::frb(ignore)]
#[derive(Default)]
#[allow(dead_code)]
struct AppRootReducer {
    sideeffect_tx: Option<oxide_core::tokio::sync::mpsc::UnboundedSender<AppSideEffect>>,
}
