use oxide_macros::reducer;

use crate::reducers::sieve::SieveReducer;
use crate::state::sieve_action::SieveAction;
use crate::state::sieve_state::SieveState;
pub use crate::OxideError;

#[reducer(
    engine = SieveEngine,
    snapshot = SieveStateSnapshot,
    initial = SieveState::new(),
    tokio_handle = crate::runtime::handle()
)]
impl oxide_core::Reducer for SieveRootReducer {
    type State = SieveState;
    type Action = SieveAction;
    type SideEffect = SieveSideEffect;

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
        SieveReducer::reduce(state, action)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

pub enum SieveSideEffect {}

#[derive(Default)]
pub struct SieveRootReducer {}
