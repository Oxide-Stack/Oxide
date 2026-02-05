// Internal (crate-level) behavioral tests.
//
// Why: these tests validate invariants that are easy to accidentally break during
// refactors (transactionality, snapshot emission rules, and error handling).
use tokio_stream::{StreamExt, wrappers::WatchStream};

use crate::{CoreResult, InitContext, OxideError, Reducer, ReducerEngine, StateChange};

#[derive(Debug, Clone, PartialEq, Eq)]
struct TestState {
    value: u64,
}

#[derive(Clone)]
enum TestAction {
    Increment,
    Noop,
    MutateThenFail,
}

enum TestSideEffect {
    Increment,
    Noop,
}

#[derive(Default)]
struct TestReducer;

impl Reducer for TestReducer {
    type State = TestState;
    type Action = TestAction;
    type SideEffect = TestSideEffect;

    async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

    fn reduce(&mut self, state: &mut Self::State, action: Self::Action) -> CoreResult<StateChange> {
        match action {
            TestAction::Increment => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::FullUpdate)
            }
            TestAction::Noop => Ok(StateChange::None),
            TestAction::MutateThenFail => {
                state.value = state.value.saturating_add(1);
                Err(OxideError::Internal {
                    message: "boom".to_string(),
                })
            }
        }
    }

    fn effect(
        &mut self,
        state: &mut Self::State,
        effect: Self::SideEffect,
    ) -> CoreResult<StateChange> {
        match effect {
            TestSideEffect::Increment => self.reduce(state, TestAction::Increment),
            TestSideEffect::Noop => Ok(StateChange::None),
        }
    }
}

#[tokio::test]
async fn engine_emits_after_full_update_dispatch() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = crate::runtime::init(thread_pool);

    let engine = ReducerEngine::<TestReducer>::new(TestReducer::default(), TestState { value: 0 })
        .await
        .unwrap();
    let rx = engine.subscribe();
    let mut stream = WatchStream::new(rx);

    let first = stream.next().await.expect("first snapshot");
    assert_eq!(first.revision, 0);
    assert_eq!(first.state, TestState { value: 0 });

    let snap = engine.dispatch(TestAction::Increment).await.unwrap();
    assert_eq!(snap.revision, 1);
    assert_eq!(snap.state, TestState { value: 1 });

    let second = stream.next().await.expect("second snapshot");
    assert_eq!(second.revision, 1);
    assert_eq!(second.state, TestState { value: 1 });
}

#[tokio::test]
async fn engine_does_not_emit_or_bump_revision_on_none() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = crate::runtime::init(thread_pool);

    let engine = ReducerEngine::<TestReducer>::new(TestReducer::default(), TestState { value: 0 })
        .await
        .unwrap();
    let rx = engine.subscribe();
    let mut stream = WatchStream::new(rx);

    let first = stream.next().await.expect("first snapshot");
    assert_eq!(first.revision, 0);

    let snap = engine.dispatch(TestAction::Noop).await.unwrap();
    assert_eq!(snap.revision, 0);
    assert_eq!(snap.state, TestState { value: 0 });

    let next = tokio::time::timeout(std::time::Duration::from_millis(50), stream.next()).await;
    assert!(next.is_err());
}

#[tokio::test]
async fn engine_does_not_commit_state_on_error() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = crate::runtime::init(thread_pool);

    let engine = ReducerEngine::<TestReducer>::new(TestReducer::default(), TestState { value: 0 })
        .await
        .unwrap();

    let before = engine.current().await;
    assert_eq!(before.revision, 0);
    assert_eq!(before.state, TestState { value: 0 });

    let err = engine
        .dispatch(TestAction::MutateThenFail)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("boom"));

    let after = engine.current().await;
    assert_eq!(after.revision, 0);
    assert_eq!(after.state, TestState { value: 0 });
}

#[tokio::test]
async fn engine_processes_sideeffects_and_emits_snapshots() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = crate::runtime::init(thread_pool);

    let engine = ReducerEngine::<TestReducer>::new(TestReducer::default(), TestState { value: 0 })
        .await
        .unwrap();
    let tx = engine.sideeffect_sender();
    let rx = engine.subscribe();
    let mut stream = WatchStream::new(rx);

    let first = stream.next().await.expect("first snapshot");
    assert_eq!(first.revision, 0);

    tx.send(TestSideEffect::Increment).unwrap();
    let second = tokio::time::timeout(std::time::Duration::from_secs(1), stream.next())
        .await
        .expect("side-effect update")
        .expect("second snapshot");
    assert_eq!(second.revision, 1);
    assert_eq!(second.state, TestState { value: 1 });
}
