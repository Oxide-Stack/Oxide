// Internal (crate-level) behavioral tests.
//
// these tests validate invariants that are easy to accidentally break during
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
    Infer,
    ExplicitSlices,
    MutateThenFail,
}

enum TestSideEffect {
    Increment,
    Noop,
    Infer,
    ExplicitSlices,
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
            TestAction::Infer => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::Infer)
            }
            TestAction::ExplicitSlices => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::Slices(&[]))
            }
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
            TestSideEffect::Infer => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::Infer)
            }
            TestSideEffect::ExplicitSlices => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::Slices(&[]))
            }
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
async fn engine_dispatch_infer_and_explicit_slices_commit_and_emit() {
    init_test_runtime();

    let engine = ReducerEngine::<TestReducer>::new(TestReducer::default(), TestState { value: 0 })
        .await
        .unwrap();

    let inferred = engine.dispatch(TestAction::Infer).await.unwrap();
    assert_eq!(inferred.revision, 1);
    assert_eq!(inferred.state, TestState { value: 1 });

    let explicit = engine.dispatch(TestAction::ExplicitSlices).await.unwrap();
    assert_eq!(explicit.revision, 2);
    assert_eq!(explicit.state, TestState { value: 2 });
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

#[tokio::test]
async fn engine_sideeffect_infer_and_explicit_slices_commit_and_emit() {
    init_test_runtime();

    let engine = ReducerEngine::<TestReducer>::new(TestReducer::default(), TestState { value: 0 })
        .await
        .unwrap();
    let tx = engine.sideeffect_sender();
    let mut stream = WatchStream::new(engine.subscribe());

    let first = stream.next().await.expect("first snapshot");
    assert_eq!(first.revision, 0);

    tx.send(TestSideEffect::Infer).unwrap();
    let second = tokio::time::timeout(std::time::Duration::from_secs(1), stream.next())
        .await
        .expect("infer side-effect update")
        .expect("second snapshot");
    assert_eq!(second.revision, 1);
    assert_eq!(second.state, TestState { value: 1 });

    tx.send(TestSideEffect::ExplicitSlices).unwrap();
    let third = tokio::time::timeout(std::time::Duration::from_secs(1), stream.next())
        .await
        .expect("explicit slices side-effect update")
        .expect("third snapshot");
    assert_eq!(third.revision, 2);
    assert_eq!(third.state, TestState { value: 2 });
}

#[tokio::test]
async fn engine_clone_shares_same_state() {
    init_test_runtime();

    let engine = ReducerEngine::<TestReducer>::new(TestReducer::default(), TestState { value: 0 })
        .await
        .unwrap();
    let cloned = engine.clone();

    let _ = cloned.dispatch(TestAction::Increment).await.unwrap();
    let after = engine.current().await;
    assert_eq!(after.revision, 1);
    assert_eq!(after.state, TestState { value: 1 });
}

#[tokio::test]
async fn engine_sideeffect_none_does_not_emit_snapshot() {
    init_test_runtime();

    let engine = ReducerEngine::<TestReducer>::new(TestReducer::default(), TestState { value: 0 })
        .await
        .unwrap();
    let tx = engine.sideeffect_sender();
    let rx = engine.subscribe();
    let mut stream = WatchStream::new(rx);

    let first = stream.next().await.expect("first snapshot");
    assert_eq!(first.revision, 0);

    tx.send(TestSideEffect::Noop).unwrap();
    let next = tokio::time::timeout(std::time::Duration::from_millis(50), stream.next()).await;
    assert!(next.is_err());
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

#[cfg(feature = "state-persistence")]
#[tokio::test]
async fn sideeffect_loop_reports_persistence_encode_failures() {
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
            Err(S::Error::custom("boom-sideeffect"))
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
            Ok(StateChange::None)
        }

        fn effect(
            &mut self,
            _state: &mut Self::State,
            _ctx: crate::Context<'_, Self::SideEffect, Self::State, ()>,
        ) -> CoreResult<StateChange> {
            Ok(StateChange::Full)
        }
    }

    init_test_runtime();

    let engine = ReducerEngine::<BadReducer>::new_persistent(
        BadReducer::default(),
        BadState,
        PersistenceConfig {
            key: "oxide_core_test_bad_state_sideeffect".to_string(),
            min_interval: std::time::Duration::from_millis(0),
        },
    )
    .await
    .unwrap();

    let mut error_stream = WatchStream::new(engine.subscribe_errors());
    let first = error_stream.next().await.expect("first error value");
    assert!(first.is_none());

    engine.sideeffect_sender().send(()).unwrap();
    let err = tokio::time::timeout(std::time::Duration::from_secs(1), error_stream.next())
        .await
        .expect("persistence side-effect error")
        .expect("error update")
        .expect("some error");
    assert!(matches!(err, OxideError::Persistence { .. }));
    assert!(err.to_string().contains("boom-sideeffect"));
}

#[cfg(feature = "state-persistence")]
#[tokio::test]
async fn sideeffect_loop_persistence_success_writes_latest_state() {
    use crate::persistence::{PersistenceConfig, decode, default_persistence_path};
    use crate::serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct PersistState {
        value: u32,
    }

    #[derive(Default)]
    struct PersistReducer;

    impl Reducer for PersistReducer {
        type State = PersistState;
        type Action = ();
        type SideEffect = ();

        async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

        fn reduce(
            &mut self,
            _state: &mut Self::State,
            _ctx: crate::Context<'_, Self::Action, Self::State, ()>,
        ) -> CoreResult<StateChange> {
            Ok(StateChange::None)
        }

        fn effect(
            &mut self,
            state: &mut Self::State,
            _ctx: crate::Context<'_, Self::SideEffect, Self::State, ()>,
        ) -> CoreResult<StateChange> {
            state.value = state.value.saturating_add(1);
            Ok(StateChange::Full)
        }
    }

    init_test_runtime();

    let key = format!(
        "oxide_core_test_sideeffect_ok_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let path = default_persistence_path(&key);
    let _ = std::fs::remove_file(&path);

    let engine = ReducerEngine::<PersistReducer>::new_persistent(
        PersistReducer,
        PersistState { value: 0 },
        PersistenceConfig {
            key,
            min_interval: std::time::Duration::from_millis(0),
        },
    )
    .await
    .unwrap();

    let mut snapshots = WatchStream::new(engine.subscribe());
    let initial = snapshots.next().await.expect("initial snapshot");
    assert_eq!(initial.state.value, 0);

    engine.sideeffect_sender().send(()).unwrap();
    let next = tokio::time::timeout(std::time::Duration::from_secs(1), snapshots.next())
        .await
        .expect("side-effect snapshot")
        .expect("snapshot value");
    assert_eq!(next.state.value, 1);

    tokio::time::sleep(std::time::Duration::from_millis(75)).await;
    let bytes = std::fs::read(&path).expect("persistence file should exist");
    let restored: PersistState = decode(&bytes).expect("decode persisted state");
    assert_eq!(restored.value, 1);

    let _ = std::fs::remove_file(&path);
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
async fn callback_runtime_respond_reports_unavailable_when_waiter_dropped() {
    crate::init_isolated_channels().unwrap();

    let runtime = std::sync::Arc::new(crate::CallbackRuntime::<u32, u32>::new(8));
    let runtime_invoke = runtime.clone();

    let invoke_task = tokio::spawn(async move { runtime_invoke.invoke(41).await });

    let (id, _req) = runtime.recv_request().await.expect("request");
    invoke_task.abort();
    let _ = invoke_task.await;

    let err = runtime.respond(id, 42).await.unwrap_err();
    assert_eq!(err, crate::OxideChannelError::Unavailable);
}

#[cfg(feature = "isolated-channels")]
#[tokio::test]
async fn callback_runtime_recv_request_returns_none_when_not_initialized() {
    let runtime = crate::CallbackRuntime::<u32, u32>::new(8);
    let received =
        tokio::time::timeout(std::time::Duration::from_millis(30), runtime.recv_request()).await;
    assert!(received.is_err() || received.unwrap().is_none());
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

#[cfg(feature = "isolated-channels")]
#[tokio::test]
async fn incoming_handler_invokes_registered_handler() {
    crate::init_isolated_channels().unwrap();

    let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u32>::new()));
    let seen_clone = seen.clone();

    let handler = crate::IncomingHandler::<u32>::new();
    handler.register(move |value| {
        seen_clone.lock().unwrap().push(value);
    });

    handler.handle(7).unwrap();
    let values = seen.lock().unwrap().clone();
    assert_eq!(values, vec![7]);
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

#[cfg(feature = "frb-spawn")]
#[tokio::test]
async fn runtime_spawn_executes_and_returns_result() {
    // Provide a thread_pool compatible with FRB APIs.
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }

    // Initialize runtime (idempotent)
    let _ = crate::runtime::init(thread_pool);

    // Spawn an async task via FRB and await its JoinHandle for the result.
    let handle = crate::runtime::spawn(async move { 123u32 });
    let result = handle.await.unwrap();
    assert_eq!(result, 123u32);
}

#[cfg(feature = "frb-spawn")]
#[tokio::test]
async fn runtime_spawn_blocking_executes_and_returns_result() {
    // Provide a thread_pool compatible with FRB APIs.
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }

    // Initialize runtime (idempotent)
    let _ = crate::runtime::init(thread_pool);

    // Spawn a blocking function via FRB and await its JoinHandle for the result.
    let handle = crate::runtime::spawn_blocking(|| 7u32);
    let result = handle.await.unwrap();
    assert_eq!(result, 7u32);
}

#[tokio::test]
async fn runtime_init_and_spawn_paths_exercised() {
    // Provide a thread_pool compatible with FRB APIs.
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }

    // Initialize FRB runtime via runtime::init and then engine globals.
    let _ = crate::runtime::init(thread_pool);
    let _ = crate::init_engine_globals();
    // init_from_frb should also succeed in the normal path.
    let _ = crate::init_from_frb(thread_pool);

    // Exercise thread_pool accessor
    let _tp = crate::runtime::thread_pool().expect("thread pool present");

    // Exercise safe_spawn path: spawn a simple future and ensure it runs.
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    let ran = Arc::new(AtomicBool::new(false));
    let ran_clone = ran.clone();
    crate::runtime::safe_spawn(async move {
        ran_clone.store(true, Ordering::SeqCst);
    });
    // Allow the spawned future a short time to execute.
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    assert!(ran.load(Ordering::SeqCst));
}
