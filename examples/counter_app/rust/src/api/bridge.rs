use oxide_generator_rs::reducer;

/// Error type exposed across the FFI boundary.
pub use oxide_core::OxideError;

use std::sync::OnceLock;

use crate::state::app_action::AppAction;
use crate::state::app_state::AppState;

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
            AppAction::Increment => {
                let next = state.counter.saturating_add(1);
                if next == state.counter {
                    return Ok(oxide_core::StateChange::None);
                }
                state.counter = next;
                Ok(oxide_core::StateChange::FullUpdate)
            }
            AppAction::Decrement => {
                let next = state.counter.saturating_sub(1);
                if next == state.counter {
                    return Ok(oxide_core::StateChange::None);
                }
                state.counter = next;
                Ok(oxide_core::StateChange::FullUpdate)
            }
            AppAction::Reset => {
                if state.counter == 0 {
                    return Ok(oxide_core::StateChange::None);
                }
                state.counter = 0;
                Ok(oxide_core::StateChange::FullUpdate)
            }
        }
    }

    fn effect(
        &mut self,
        state: &mut Self::State,
        effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match effect {
            AppSideEffect::Increment => self.reduce(state, AppAction::Increment),
            AppSideEffect::Decrement => self.reduce(state, AppAction::Decrement),
            AppSideEffect::Reset => self.reduce(state, AppAction::Reset),
        }
    }
}

pub enum AppSideEffect {
    Increment,
    Decrement,
    Reset,
}

#[derive(Default)]
pub struct AppReducer;

static SIDEFFECT_TX: OnceLock<oxide_core::tokio::sync::mpsc::UnboundedSender<AppSideEffect>> =
    OnceLock::new();

