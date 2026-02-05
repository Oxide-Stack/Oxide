use oxide_generator_rs::{actions, reducer, state};

#[state]
pub struct MyState {
    pub count: u64,
}

#[actions]
pub enum MyAction {
    Inc,
}

pub enum MySideEffect {}

#[derive(Default)]
pub struct MyReducer;

#[reducer(engine = MyEngine, snapshot = MySnapshot, initial = MyState { count: 0 }, no_frb)]
impl oxide_core::Reducer for MyReducer {
    type State = MyState;
    type Action = MyAction;
    type SideEffect = MySideEffect;

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        state: &mut Self::State,
        action: Self::Action,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match action {
            MyAction::Inc => {
                state.count = state.count.saturating_add(1);
                Ok(oxide_core::StateChange::FullUpdate)
            }
        }
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
