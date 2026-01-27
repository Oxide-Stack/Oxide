use crate::reducers::util::fnv1a_mix_u64;
use crate::state::counter_action::CounterAction;
use crate::state::counter_state::CounterState;

pub struct CounterReducer;

impl CounterReducer {
    pub fn reduce(
        state: &mut CounterState,
        action: CounterAction,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        let CounterAction::Run { iterations } = action;
        if iterations == 0 {
            return Ok(oxide_core::StateChange::None);
        }

        for _ in 0..iterations {
            state.counter = state.counter.saturating_add(1);
            state.checksum = fnv1a_mix_u64(state.checksum, state.counter);
        }

        Ok(oxide_core::StateChange::FullUpdate)
    }
}
