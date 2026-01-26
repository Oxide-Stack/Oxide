use oxide_macros::reducer;

#[derive(Default)]
pub struct MissingArgsReducer;

pub enum MissingArgsSideEffect {}

#[reducer(snapshot = MissingSnapshot, initial = (), no_frb)]
impl oxide_core::Reducer for MissingArgsReducer {
    type State = ();
    type Action = ();
    type SideEffect = MissingArgsSideEffect;

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
