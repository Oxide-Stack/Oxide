use crate::api::bridge::AppEngine;
use crate::state::AppAction;

#[tokio::test]
async fn manual_tick_increments_ticks() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let _ = engine.dispatch(AppAction::ManualTick).await.expect("dispatch");
    let snapshot = engine.current().await;
    assert!(snapshot.state.tick.ticks > 0);
}

#[tokio::test]
async fn reset_sets_ticks_to_zero() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let _ = engine.dispatch(AppAction::ManualTick).await.expect("dispatch");
    let snapshot = engine.dispatch(AppAction::Reset).await.expect("dispatch");
    assert_eq!(snapshot.state.tick.ticks, 0);
}

#[tokio::test]
async fn auto_tick_requires_running() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let before = engine.current().await;
    let _ = engine.dispatch(AppAction::AutoTick).await.expect("dispatch");
    let after = engine.current().await;
    assert_eq!(before.state.tick.ticks, after.state.tick.ticks);
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

    let start_snap = rx.borrow().clone();
    assert_eq!(start_snap.slices, vec![crate::state::app_state::AppStateSlice::Control]);

    let changed = tokio::time::timeout(std::time::Duration::from_secs(1), rx.changed()).await;
    if changed.is_err() {
        panic!("auto tick did not apply within expected timeout");
    }

    let snap = rx.borrow().clone();
    assert_eq!(snap.slices, vec![crate::state::app_state::AppStateSlice::Tick]);
    assert!(snap.state.tick.ticks > before.state.tick.ticks);
    assert_eq!(snap.state.tick.last_tick_source, "auto");
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
    assert_eq!(snap.slices, vec![crate::state::app_state::AppStateSlice::Tick]);
    assert!(snap.state.tick.ticks > before.state.tick.ticks);
    assert_eq!(snap.state.tick.last_tick_source, "side_effect");
}

#[tokio::test]
async fn start_ticker_is_idempotent_for_same_interval() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let _ = engine
        .dispatch(AppAction::StartTicker { interval_ms: 50 })
        .await
        .expect("dispatch");
    let after_first = engine.current().await;

    let _ = engine
        .dispatch(AppAction::StartTicker { interval_ms: 50 })
        .await
        .expect("dispatch");
    let after_second = engine.current().await;

    assert_eq!(after_second.revision, after_first.revision);
    assert_eq!(after_second.state, after_first.state);
}

#[tokio::test]
async fn start_ticker_updates_interval_when_running() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let _ = engine
        .dispatch(AppAction::StartTicker { interval_ms: 10 })
        .await
        .expect("dispatch");
    let first = engine.current().await;
    assert_eq!(first.state.control.interval_ms, 10);

    let snap = engine
        .dispatch(AppAction::StartTicker { interval_ms: 20 })
        .await
        .expect("dispatch");
    assert_eq!(snap.slices, vec![crate::state::app_state::AppStateSlice::Control]);
    assert_eq!(snap.state.control.interval_ms, 20);

    let mut rx = engine.subscribe();
    let _ = tokio::time::timeout(std::time::Duration::from_secs(1), rx.changed())
        .await
        .expect("expected auto tick after interval update");
}

#[tokio::test]
async fn stop_ticker_is_noop_when_stopped() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let before = engine.current().await;
    let _ = engine.dispatch(AppAction::StopTicker).await.expect("dispatch");
    let after = engine.current().await;
    assert_eq!(after.revision, before.revision);
    assert_eq!(after.state, before.state);
}

#[tokio::test]
async fn reset_is_noop_from_initial_state() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let before = engine.current().await;
    let _ = engine.dispatch(AppAction::Reset).await.expect("dispatch");
    let after = engine.current().await;
    assert_eq!(after.revision, before.revision);
    assert_eq!(after.state, before.state);
}

#[test]
fn frb_init_app_is_callable() {
    crate::api::bridge::init_app();
}

#[tokio::test]
async fn stop_ticker_stops_background_task() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let mut rx = engine.subscribe();

    let _ = engine
        .dispatch(AppAction::StartTicker { interval_ms: 10 })
        .await
        .expect("dispatch");
    let _ = tokio::time::timeout(std::time::Duration::from_secs(1), rx.changed())
        .await
        .expect("expected start snapshot");
    let _ = tokio::time::timeout(std::time::Duration::from_secs(1), rx.changed())
        .await
        .expect("expected at least one auto tick");

    let stopped = engine
        .dispatch(AppAction::StopTicker)
        .await
        .expect("dispatch");
    assert_eq!(stopped.slices, vec![crate::state::app_state::AppStateSlice::Control]);
    assert!(!stopped.state.control.is_running);

    let _ = tokio::time::timeout(std::time::Duration::from_secs(1), rx.changed())
        .await
        .expect("expected stop snapshot");

    let next = tokio::time::timeout(std::time::Duration::from_millis(200), rx.changed()).await;
    assert!(next.is_err());
}

#[tokio::test]
async fn auto_tick_action_updates_tick_slice_when_running() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = AppEngine::new().await.unwrap();
    let before = engine.current().await;

    let _ = engine
        .dispatch(AppAction::StartTicker { interval_ms: 100 })
        .await
        .expect("dispatch");

    let snap = engine.dispatch(AppAction::AutoTick).await.expect("dispatch");
    assert_eq!(snap.slices, vec![crate::state::app_state::AppStateSlice::Tick]);
    assert!(snap.state.tick.ticks > before.state.tick.ticks);
    assert_eq!(snap.state.tick.last_tick_source, "auto");
}

#[tokio::test]
async fn frb_engine_helpers_are_callable() {
    crate::api::bridge::init_oxide().await.unwrap();
    let engine = crate::api::bridge::create_engine().await.unwrap();
    let _ = crate::api::bridge::current(&engine).await;
    let _ = crate::api::bridge::dispatch(&engine, AppAction::ManualTick)
        .await
        .unwrap();
    crate::api::bridge::dispose_engine(&engine);
}
