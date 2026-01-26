use oxide_macros::reducer;

use crate::reducers::counter::CounterReducer;
use crate::state::counter_action::CounterAction;
use crate::state::counter_state::CounterState;
pub use crate::OxideError;

#[reducer(
    engine = CounterEngine,
    snapshot = CounterStateSnapshot,
    initial = CounterState::new(),
    tokio_handle = crate::runtime::handle()
)]
impl oxide_core::Reducer for CounterRootReducer {
    type State = CounterState;
    type Action = CounterAction;
    type SideEffect = CounterSideEffect;

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
        CounterReducer::reduce(state, action)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

pub enum CounterSideEffect {}

#[derive(Default)]
pub struct CounterRootReducer {}
