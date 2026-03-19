#![cfg(feature = "navigation-binding")]

use oxide_core::{CoreResult, InitContext, OxideError, Reducer, ReducerEngine, StateChange};
use tokio_stream::StreamExt;

#[derive(Clone)]
struct TestState;

#[derive(Clone)]
enum TestAction {
    Noop,
}

#[derive(Clone)]
enum TestSideEffect {
    Tick,
}

#[derive(Default)]
struct TestReducer;

impl Reducer for TestReducer {
    type State = TestState;
    type Action = TestAction;
    type SideEffect = TestSideEffect;

    async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<'_, Self::Action, Self::State, ()>,
    ) -> CoreResult<StateChange> {
        Ok(StateChange::None)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<'_, Self::SideEffect, Self::State, ()>,
    ) -> CoreResult<StateChange> {
        Ok(StateChange::Full)
    }
}

fn init_runtime_without_navigation() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = oxide_core::runtime::init(thread_pool);
}

#[tokio::test]
async fn sideeffect_loop_reports_missing_navigation_runtime() {
    init_runtime_without_navigation();

    let engine = ReducerEngine::<TestReducer>::new(TestReducer, TestState)
        .await
        .unwrap();
    let mut errors = tokio_stream::wrappers::WatchStream::new(engine.subscribe_errors());
    let first = errors.next().await.expect("initial error value");
    assert!(first.is_none());

    engine.sideeffect_sender().send(TestSideEffect::Tick).unwrap();

    let err = tokio::time::timeout(std::time::Duration::from_secs(1), errors.next())
        .await
        .expect("error update timeout")
        .expect("error update")
        .expect("expected error value");

    assert!(matches!(err, OxideError::Validation { .. }));
    assert!(err
        .to_string()
        .contains("navigation runtime not initialized"));
}

