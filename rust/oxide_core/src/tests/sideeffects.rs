use super::*;
use tokio_stream::{StreamExt, wrappers::WatchStream};

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

#[test]
fn test_side_effect_noop_variant_is_constructible() {
    let _ = TestSideEffect::Noop;
}
