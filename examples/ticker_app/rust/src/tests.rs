use crate::api::bridge::AppEngine;
use crate::state::AppAction;

#[tokio::test]
async fn manual_tick_increments_ticks() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let _ = engine.dispatch(AppAction::ManualTick).await.expect("dispatch");
    let snapshot = engine.current().await;
    assert!(snapshot.state.ticks > 0);
}

#[tokio::test]
async fn reset_sets_ticks_to_zero() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let _ = engine.dispatch(AppAction::ManualTick).await.expect("dispatch");
    let snapshot = engine.dispatch(AppAction::Reset).await.expect("dispatch");
    assert_eq!(snapshot.state.ticks, 0);
}

#[tokio::test]
async fn auto_tick_requires_running() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let before = engine.current().await;
    let _ = engine.dispatch(AppAction::AutoTick).await.expect("dispatch");
    let after = engine.current().await;
    assert_eq!(before.state.ticks, after.state.ticks);
}

#[tokio::test]
async fn start_ticker_spawns_background_auto_ticks() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let mut rx = engine.subscribe();
    let before = rx.borrow().clone();
    let _ = engine
        .dispatch(AppAction::StartTicker { interval_ms: 10 })
        .await
        .expect("dispatch");

    let started = tokio::time::timeout(std::time::Duration::from_secs(1), rx.changed()).await;
    if started.is_err() {
        panic!("start ticker did not apply within expected timeout");
    }

    let changed = tokio::time::timeout(std::time::Duration::from_secs(1), rx.changed()).await;
    if changed.is_err() {
        panic!("auto tick did not apply within expected timeout");
    }

    let snap = rx.borrow().clone();
    assert!(snap.state.ticks > before.state.ticks);
    assert_eq!(snap.state.last_tick_source, "auto");
}

#[tokio::test]
async fn side_effect_tick_updates_state() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let mut rx = engine.subscribe();
    let before = rx.borrow().clone();
    let _ = engine
        .dispatch(AppAction::EmitSideEffectTick)
        .await
        .expect("dispatch");

    let changed = tokio::time::timeout(std::time::Duration::from_secs(2), rx.changed()).await;
    if changed.is_err() {
        panic!("side effect did not apply within expected timeout");
    }
    let snap = rx.borrow().clone();
    assert!(snap.state.ticks > before.state.ticks);
    assert_eq!(snap.state.last_tick_source, "side_effect");
}
