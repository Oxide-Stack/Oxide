use super::*;
use tokio_stream::{StreamExt, wrappers::WatchStream};

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

#[test]
fn reducer_default_infer_slices_returns_empty() {
    let reducer = TestReducer::default();
    let before = TestState { value: 1 };
    let after = TestState { value: 2 };
    let slices = reducer.infer_slices(&before, &after);
    assert!(slices.is_empty());
}
