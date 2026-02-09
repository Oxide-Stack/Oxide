#![cfg(all(target_arch = "wasm32", target_os = "wasi"))]

use oxide_core::{CoreResult, InitContext, Reducer, ReducerEngine, StateChange};

thread_local! {
    static THREAD_POOL: flutter_rust_bridge::SimpleThreadPool =
        flutter_rust_bridge::SimpleThreadPool::default();
}

fn thread_pool() -> &'static std::thread::LocalKey<flutter_rust_bridge::SimpleThreadPool> {
    &THREAD_POOL
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CounterState {
    value: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CounterAction {
    Inc,
}

enum CounterSideEffect {}

#[derive(Default)]
struct CounterReducer;

impl Reducer for CounterReducer {
    type State = CounterState;
    type Action = CounterAction;
    type SideEffect = CounterSideEffect;

    async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<'_, Self::Action, Self::State, ()>,
    ) -> CoreResult<StateChange> {
        match ctx.input {
            CounterAction::Inc => state.value = state.value.saturating_add(1),
        }
        Ok(StateChange::Full)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<'_, Self::SideEffect, Self::State, ()>,
    ) -> CoreResult<StateChange> {
        Ok(StateChange::None)
    }
}

#[test]
fn reducer_engine_compiles_for_wasi() {
    let _ = async {
        let _ = oxide_core::runtime::init(thread_pool);

        let _engine = ReducerEngine::<CounterReducer>::new(
            CounterReducer::default(),
            CounterState { value: 0 },
        )
        .await
        .unwrap();
    };
}
