use flutter_rust_bridge::frb;
use oxide_generator_rs::reducer;

use crate::util::fnv1a_mix_u64;
use crate::state::counter_action::CounterAction;
use crate::state::counter_state::CounterState;
pub use crate::OxideError;

#[reducer(
    engine = CounterEngine,
    snapshot = CounterStateSnapshot,
    initial = CounterState::new(),
)]
impl oxide_core::Reducer for CounterRootReducer {
    type State = CounterState;
    type Action = CounterAction;
    type SideEffect = CounterSideEffect;

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {
        if let Ok(runtime) = oxide_core::navigation_runtime() {
            let _ = runtime.push(crate::routes::HomeRoute {});
        }
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::ReducerCtx<'_, Self::Action, Self::State>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        let CounterAction::Run { iterations } = ctx.input;
        if *iterations == 0 {
            return Ok(oxide_core::StateChange::None);
        }

        for _ in 0..*iterations {
            state.counter = state.counter.saturating_add(1);
            state.checksum = fnv1a_mix_u64(state.checksum, state.counter);
        }

        Ok(oxide_core::StateChange::Full)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::ReducerCtx<'_, Self::SideEffect, Self::State>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}
#[frb(ignore)]
enum CounterSideEffect {}
#[frb(ignore)]
#[derive(Default)]
struct CounterRootReducer {}
