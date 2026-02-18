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
        ctx: oxide_core::Context<'_, Self::Action, Self::State, ()>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match ctx.input {
            MyAction::Inc => {
                state.count = state.count.saturating_add(1);
                Ok(oxide_core::StateChange::Full)
            }
        }
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<'_, Self::SideEffect, Self::State, ()>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

#[cfg(feature = "state-persistence")]
async fn smoke_persistence_api() -> oxide_core::CoreResult<()> {
    let engine = MyEngine::new().await?;
    let _ = engine.encode_current_state().await?;

    let bytes = MyEngine::encode_state_value(&MyState { count: 1 })?;
    let decoded = MyEngine::decode_state_value(&bytes)?;
    let _ = decoded.count;

    Ok(())
}

fn main() {}
