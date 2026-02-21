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
    Fail,
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
        state: &mut Self::State,
        ctx: crate::Context<'_, Self::Action, Self::State, ()>,
    ) -> CoreResult<StateChange> {
        match ctx.input {
            TestAction::Increment => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::Full)
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
        ctx: crate::Context<'_, Self::SideEffect, Self::State, ()>,
    ) -> CoreResult<StateChange> {
        match ctx.input {
            TestSideEffect::Increment => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::Full)
            }
            TestSideEffect::Noop => Ok(StateChange::None),
            TestSideEffect::Fail => {
                state.value = state.value.saturating_add(1);
                Err(OxideError::Internal {
                    message: "effect boom".to_string(),
                })
            }
        }
    }
}

fn init_test_runtime() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = crate::runtime::init(thread_pool);
    #[cfg(feature = "navigation-binding")]
    {
        let _ = crate::init_navigation();
    }
}

#[tokio::test]
async fn engine_emits_after_full_update_dispatch() {
    init_test_runtime();

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
    init_test_runtime();

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
    init_test_runtime();

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
    init_test_runtime();

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

#[tokio::test]
async fn engine_reports_sideeffect_errors_and_does_not_commit() {
    init_test_runtime();

    let engine = ReducerEngine::<TestReducer>::new(TestReducer::default(), TestState { value: 0 })
        .await
        .unwrap();
    let tx = engine.sideeffect_sender();
    let mut error_stream = WatchStream::new(engine.subscribe_errors());

    let first = error_stream.next().await.expect("first error value");
    assert!(first.is_none());

    tx.send(TestSideEffect::Fail).unwrap();
    let err = tokio::time::timeout(std::time::Duration::from_secs(1), error_stream.next())
        .await
        .expect("side-effect error")
        .expect("error update")
        .expect("some error");
    assert!(err.to_string().contains("effect boom"));

    let after = engine.current().await;
    assert_eq!(after.revision, 0);
    assert_eq!(after.state, TestState { value: 0 });
}

#[cfg(feature = "state-persistence")]
#[tokio::test]
async fn engine_reports_persistence_encode_failures() {
    use crate::persistence::PersistenceConfig;
    use crate::serde::{Deserialize, Serialize};

    #[derive(Clone)]
    struct BadState;

    impl Serialize for BadState {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: crate::serde::Serializer,
        {
            use crate::serde::ser::Error as _;
            Err(S::Error::custom("boom"))
        }
    }

    impl<'de> Deserialize<'de> for BadState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: crate::serde::Deserializer<'de>,
        {
            let _ = crate::serde::de::IgnoredAny::deserialize(deserializer)?;
            Ok(BadState)
        }
    }

    #[derive(Default)]
    struct BadReducer;

    impl Reducer for BadReducer {
        type State = BadState;
        type Action = ();
        type SideEffect = ();

        async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

        fn reduce(
            &mut self,
            _state: &mut Self::State,
            _ctx: crate::Context<'_, Self::Action, Self::State, ()>,
        ) -> CoreResult<StateChange> {
            Ok(StateChange::Full)
        }

        fn effect(
            &mut self,
            _state: &mut Self::State,
            _ctx: crate::Context<'_, Self::SideEffect, Self::State, ()>,
        ) -> CoreResult<StateChange> {
            Ok(StateChange::None)
        }
    }

    init_test_runtime();

    let engine = ReducerEngine::<BadReducer>::new_persistent(
        BadReducer::default(),
        BadState,
        PersistenceConfig {
            key: "oxide_core_test_bad_state".to_string(),
            min_interval: std::time::Duration::from_millis(0),
        },
    )
    .await
    .unwrap();

    let mut error_stream = WatchStream::new(engine.subscribe_errors());
    let first = error_stream.next().await.expect("first error value");
    assert!(first.is_none());

    let _ = engine.dispatch(()).await.unwrap();
    let err = tokio::time::timeout(std::time::Duration::from_secs(1), error_stream.next())
        .await
        .expect("persistence error")
        .expect("error update")
        .expect("some error");
    assert!(matches!(err, OxideError::Persistence { .. }));
    assert!(err.to_string().contains("boom"));
}

#[cfg(feature = "isolated-channels")]
#[tokio::test]
async fn callback_runtime_resolves_pending_requests() {
    crate::init_isolated_channels().unwrap();

    let runtime = std::sync::Arc::new(crate::CallbackRuntime::<u32, u32>::new(8));
    let runtime_responder = runtime.clone();

    let responder = tokio::spawn(async move {
        let (id, req) = runtime_responder.recv_request().await.expect("request");
        runtime_responder
            .respond(id, req + 1)
            .await
            .expect("respond");
    });

    let out = runtime.invoke(41).await.expect("invoke");
    assert_eq!(out, 42);
    responder.await.unwrap();
}

#[cfg(feature = "isolated-channels")]
#[tokio::test]
async fn callback_runtime_rejects_unknown_response_ids() {
    crate::init_isolated_channels().unwrap();

    let runtime = crate::CallbackRuntime::<u32, u32>::new(8);
    let err = runtime.respond(999, 1).await.unwrap_err();
    assert_eq!(err, crate::OxideChannelError::UnexpectedResponse);
}

#[cfg(feature = "isolated-channels")]
#[tokio::test]
async fn event_channel_delivers_events_to_subscribers() {
    crate::init_isolated_channels().unwrap();

    let runtime = crate::EventChannelRuntime::<u32>::new(8);
    let mut rx = runtime.subscribe();
    runtime.emit(7);

    let next = tokio::time::timeout(std::time::Duration::from_secs(1), rx.recv())
        .await
        .expect("recv")
        .expect("event");
    assert_eq!(next, 7);
}

#[cfg(feature = "isolated-channels")]
#[tokio::test]
async fn incoming_handler_reports_unavailable_when_missing() {
    crate::init_isolated_channels().unwrap();

    let handler = crate::IncomingHandler::<u32>::new();
    let err = handler.handle(1).unwrap_err();
    assert_eq!(err, crate::OxideChannelError::Unavailable);
}

#[cfg(feature = "isolated-channels")]
#[tokio::test]
async fn incoming_handler_converts_panics_to_platform_error() {
    crate::init_isolated_channels().unwrap();

    let handler = crate::IncomingHandler::<u32>::new();
    handler.register(|_| panic!("boom"));

    let err = handler.handle(1).unwrap_err();
    assert!(matches!(err, crate::OxideChannelError::PlatformError(_)));
}

#[test]
fn reducer_default_infer_slices_returns_empty() {
    let reducer = TestReducer::default();
    let before = TestState { value: 1 };
    let after = TestState { value: 2 };
    let slices = reducer.infer_slices(&before, &after);
    assert!(slices.is_empty());
}

#[tokio::test]
async fn watch_receiver_to_stream_emits_updates() {
    let (tx, rx) = tokio::sync::watch::channel(1_u32);
    let mut stream = crate::watch_receiver_to_stream(rx);

    let first = stream.next().await.unwrap();
    assert_eq!(first, 1);

    tx.send(2).unwrap();
    let second = stream.next().await.unwrap();
    assert_eq!(second, 2);
}

#[test]
fn test_side_effect_noop_variant_is_constructible() {
    let _ = TestSideEffect::Noop;
}
