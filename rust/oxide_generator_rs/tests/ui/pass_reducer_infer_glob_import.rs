use oxide_generator_rs::{actions, reducer, state};

#[state(sliced = true)]
pub struct MyState {
    pub a: u64,
    pub b: u64,
}

#[actions]
pub enum MyAction {
    IncA,
}

pub enum MySideEffect {}

#[derive(Default)]
pub struct MyReducer;

#[reducer(engine = MyEngine, snapshot = MySnapshot, initial = MyState { a: 0, b: 0 }, no_frb)]
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
        // Why: Many users prefer importing enum variants for brevity.
        // How: The macro should still detect `Infer` usage even when `StateChange`
        // isn't spelled out at the callsite.
        use oxide_core::StateChange::*;
        match action {
            MyAction::IncA => {
                state.a = state.a.saturating_add(1);
                Ok(Infer)
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

fn main() {
    let _ = MyStateSlice::A;
}

