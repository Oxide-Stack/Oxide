use oxide_generator_rs::reducer;

#[derive(Default)]
pub struct AsyncReducer;

pub enum AsyncSideEffect {}

#[reducer(engine = AsyncEngine, snapshot = AsyncSnapshot, initial = (), no_frb)]
impl oxide_core::Reducer for AsyncReducer {
    type State = ();
    type Action = ();
    type SideEffect = AsyncSideEffect;

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {}

    async fn reduce(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<'_, Self::Action, Self::State, ()>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<'_, Self::SideEffect, Self::State, ()>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

fn main() {}
