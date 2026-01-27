use oxide_generator_rs::reducer;

use crate::reducers::json::JsonReducer;
use crate::state::json_action::JsonAction;
use crate::state::json_state::JsonState;
pub use crate::OxideError;

#[reducer(
    engine = JsonEngine,
    snapshot = JsonStateSnapshot,
    initial = JsonState::new(),
    tokio_handle = crate::runtime::handle()
)]
impl oxide_core::Reducer for JsonRootReducer {
    type State = JsonState;
    type Action = JsonAction;
    type SideEffect = JsonSideEffect;

    fn init(
        &mut self,
        _sideeffect_tx: oxide_core::tokio::sync::mpsc::UnboundedSender<Self::SideEffect>,
    ) {
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        action: Self::Action,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        JsonReducer::reduce(state, action)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

pub enum JsonSideEffect {}

#[derive(Default)]
pub struct JsonRootReducer {}
