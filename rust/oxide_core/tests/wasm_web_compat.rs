#![cfg(all(target_arch = "wasm32", target_os = "unknown"))]

use std::time::Duration;

use oxide_core::{CoreResult, InitContext, Reducer, ReducerEngine, StateChange};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

thread_local! {
    static THREAD_POOL: flutter_rust_bridge::SimpleThreadPool =
        flutter_rust_bridge::SimpleThreadPool::default();
}

fn thread_pool() -> &'static std::thread::LocalKey<flutter_rust_bridge::SimpleThreadPool> {
    &THREAD_POOL
}

#[cfg_attr(
    feature = "state-persistence",
    derive(oxide_core::serde::Serialize, oxide_core::serde::Deserialize)
)]
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

    fn reduce(&mut self, state: &mut Self::State, action: Self::Action) -> CoreResult<StateChange> {
        match action {
            CounterAction::Inc => state.value = state.value.saturating_add(1),
        }
        Ok(StateChange::Full)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _effect: Self::SideEffect,
    ) -> CoreResult<StateChange> {
        Ok(StateChange::None)
    }
}

#[wasm_bindgen_test(async)]
async fn reducer_engine_dispatch_works_in_web_wasm() {
    let _ = oxide_core::runtime::init(thread_pool);

    let engine =
        ReducerEngine::<CounterReducer>::new(CounterReducer::default(), CounterState { value: 0 })
            .await
            .unwrap();
    let snapshot = engine.dispatch(CounterAction::Inc).await.unwrap();
    assert_eq!(snapshot.state.value, 1);
}

#[cfg(feature = "state-persistence")]
#[wasm_bindgen_test(async)]
async fn persistence_restores_state_in_web_wasm() {
    use oxide_core::persistence::PersistenceConfig;

    let _ = oxide_core::runtime::init(thread_pool);

    let key = format!("oxide_core.web_wasm.test.{}", js_sys::Date::now());
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item(&format!("oxide/{}.bin", key));
            let _ = storage.remove_item(&format!("oxide/{}.json", key));
        }
    }

    let config = PersistenceConfig {
        key: key.clone(),
        min_interval: Duration::from_millis(0),
    };

    let engine = ReducerEngine::<CounterReducer>::new_persistent(
        CounterReducer::default(),
        CounterState { value: 0 },
        config,
    )
    .await
    .unwrap();

    let _ = engine.dispatch(CounterAction::Inc).await.unwrap();
    gloo_timers::future::TimeoutFuture::new(10).await;

    let engine2 = ReducerEngine::<CounterReducer>::new_persistent(
        CounterReducer::default(),
        CounterState { value: 0 },
        PersistenceConfig {
            key,
            min_interval: Duration::from_millis(0),
        },
    )
    .await
    .unwrap();

    let current = engine2.current().await;
    assert_eq!(current.state.value, 1);
}
