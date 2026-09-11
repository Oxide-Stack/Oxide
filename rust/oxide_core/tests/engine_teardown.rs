//! Regression tests for engine teardown.
//!
//! `Shared` must not own the side-effect sender: it used to keep the background
//! loop alive forever, leaking the whole engine state even after every
//! `ReducerEngine` handle was dropped. These tests observe loop lifetime via
//! `debug_active_sideeffect_loops`.

use std::sync::{Mutex, MutexGuard};

use oxide_core::{
    CoreResult, InitContext, Reducer, ReducerEngine, StateChange, debug_active_sideeffect_loops,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestState;

#[derive(Debug, Clone)]
enum TestAction {}

#[derive(Debug, Clone)]
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
        Ok(StateChange::None)
    }
}

fn init_test_runtime() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = oxide_core::runtime::init(thread_pool);
}

// The instrumentation is process-global; serialize the lifecycle tests so they
// do not observe each other's engines.
fn test_lock() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

async fn wait_for_active_loops(target: usize) {
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        if debug_active_sideeffect_loops() == target {
            return;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "side-effect loop count never reached {target} (currently {})",
            debug_active_sideeffect_loops()
        );
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
}

#[tokio::test]
async fn dropping_last_engine_handle_terminates_sideeffect_loop() {
    let _guard = test_lock();
    init_test_runtime();

    assert_eq!(debug_active_sideeffect_loops(), 0);

    let engine = ReducerEngine::<TestReducer>::new(TestReducer, TestState)
        .await
        .unwrap();
    wait_for_active_loops(1).await;

    // Cloned handles keep the loop alive until every sender is gone.
    let cloned = engine.clone();
    let sender = engine.sideeffect_sender();
    drop(engine);
    wait_for_active_loops(1).await;

    drop(sender);
    drop(cloned);
    wait_for_active_loops(0).await;
}

#[tokio::test]
async fn shutdown_aborts_loop_and_subsequent_effect_sends_are_noops() {
    let _guard = test_lock();
    init_test_runtime();

    let engine = ReducerEngine::<TestReducer>::new(TestReducer, TestState)
        .await
        .unwrap();
    wait_for_active_loops(1).await;

    engine.shutdown();
    engine.shutdown(); // idempotent
    wait_for_active_loops(0).await;

    // The receiver was dropped with the aborted task; sends fail without panicking.
    let result = engine.sideeffect_sender().send(TestSideEffect::Tick);
    assert!(result.is_err());
}

#[tokio::test]
async fn sequential_engines_never_exceed_one_concurrent_loop() {
    let _guard = test_lock();
    init_test_runtime();

    let mut peak = 0;
    for _ in 0..3 {
        let engine = ReducerEngine::<TestReducer>::new(TestReducer, TestState)
            .await
            .unwrap();
        wait_for_active_loops(1).await;
        peak = peak.max(debug_active_sideeffect_loops());
        drop(engine);
        wait_for_active_loops(0).await;
    }
    assert_eq!(peak, 1);
}
