#![cfg(feature = "state-persistence")]

// Safety-net test for persistence restore behavior.
//
// Why: refactors in codecs/backends must never cause corrupt persisted payloads
// to crash initialization. The engine should fall back to the caller-provided
// initial state.
use oxide_core::persistence::PersistenceConfig;
use oxide_core::{CoreResult, InitContext, Reducer, ReducerEngine, StateChange};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct State {
    counter: u64,
}

#[derive(Debug, Clone)]
enum Action {}

enum SideEffect {}

#[derive(Default)]
struct ReducerImpl;

impl Reducer for ReducerImpl {
    type State = State;
    type Action = Action;
    type SideEffect = SideEffect;

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

#[tokio::test]
async fn persistence_ignores_corrupt_data_and_falls_back_to_initial_state() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = oxide_core::runtime::init(thread_pool);

    let key = "oxide_core.test.persistence_corrupt_data.v1".to_string();
    let path = oxide_core::persistence::default_persistence_path(&key);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create persistence directory");
    }
    let _ = std::fs::remove_file(&path);
    std::fs::write(&path, [0_u8, 1, 2, 3, 4, 5]).expect("write corrupt persistence bytes");

    let engine = ReducerEngine::<ReducerImpl>::new_persistent(
        ReducerImpl::default(),
        State { counter: 42 },
        PersistenceConfig {
            key,
            min_interval: std::time::Duration::from_millis(0),
        },
    )
    .await
    .unwrap();

    let snap = engine.current().await;
    assert_eq!(snap.revision, 0);
    assert_eq!(snap.state.counter, 42);
}
