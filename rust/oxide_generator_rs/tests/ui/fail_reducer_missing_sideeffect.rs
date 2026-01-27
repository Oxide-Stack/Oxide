use oxide_generator_rs::reducer;

#[derive(Default)]
pub struct MissingSideEffectReducer;

#[reducer(engine = MissingSideEffectEngine, snapshot = MissingSideEffectSnapshot, initial = (), no_frb)]
impl oxide_core::Reducer for MissingSideEffectReducer {
    type State = ();
    type Action = ();

    fn init(
        &mut self,
        _sideeffect_tx: oxide_core::tokio::sync::mpsc::UnboundedSender<Self::SideEffect>,
    ) {
    }

    fn reduce(
        &mut self,
        _state: &mut Self::State,
        _action: Self::Action,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

fn main() {}

