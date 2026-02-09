use oxide_generator_rs::{actions, reducer, state};

#[state]
pub struct MyState {
    pub a: u64,
}

#[actions]
pub enum MyAction {
    IncA,
}

pub enum MySideEffect {}

#[derive(Default)]
pub struct MyReducer;

#[reducer(engine = MyEngine, snapshot = MySnapshot, initial = MyState { a: 0 }, no_frb)]
impl oxide_core::Reducer for MyReducer {
    type State = MyState;
    type Action = MyAction;
    type SideEffect = MySideEffect;

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<
            '_,
            Self::Action,
            Self::State,
            <Self::State as oxide_core::SlicedState>::StateSlice,
        >,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match ctx.input {
            MyAction::IncA => {
                state.a = state.a.saturating_add(1);
                Ok(oxide_core::StateChange::Infer)
            }
        }
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<
            '_,
            Self::SideEffect,
            Self::State,
            <Self::State as oxide_core::SlicedState>::StateSlice,
        >,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

fn main() {}
