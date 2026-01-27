use oxide_generator_rs::reducer;

#[derive(Default)]
pub struct BadReducer;

pub enum BadSideEffect {}

#[reducer(engine = BadEngine, snapshot = BadSnapshot, initial = ())]
impl oxide_core::Reducer for BadReducer {
    type State = ();
    type Action = ();
    type SideEffect = BadSideEffect;

    fn init(
        &mut self,
        _sideeffect_tx: oxide_core::tokio::sync::mpsc::UnboundedSender<Self::SideEffect>,
    ) {
    }

    fn reduce(
        &mut self,
        _state: &mut Self::State,
        _action: Self::Action,
    ) -> oxide_core::CoreResult<()> {
        Ok(())
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
