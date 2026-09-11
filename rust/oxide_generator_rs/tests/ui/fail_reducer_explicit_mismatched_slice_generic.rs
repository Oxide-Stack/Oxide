use oxide_generator_rs::{actions, reducer, state};

#[state(sliced = true)]
pub struct MyState {
    pub a: u64,
}

// A second sliced state whose slice type intentionally mismatches the reducer's
// explicit trait generic below.
#[state(sliced = true)]
pub struct OtherState {
    pub b: u64,
}

#[actions]
pub enum MyAction {
    IncA,
}

pub enum MySideEffect {}

#[derive(Default)]
pub struct MyReducer;

#[reducer(engine = MyEngine, snapshot = MySnapshot, initial = MyState { a: 0 }, no_frb)]
impl oxide_core::Reducer<OtherStateSlice> for MyReducer {
    type State = MyState;
    type Action = MyAction;
    type SideEffect = MySideEffect;

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<'_, Self::Action, Self::State, MyStateSlice>,
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
        _ctx: oxide_core::Context<'_, Self::SideEffect, Self::State, MyStateSlice>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

fn main() {}
