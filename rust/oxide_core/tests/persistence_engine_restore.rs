#![cfg(feature = "state-persistence")]

use oxide_core::persistence::PersistenceConfig;
use oxide_core::{CoreResult, InitContext, OxideError, Reducer, ReducerEngine, StateChange};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct State {
    counter: u64,
}

#[derive(Debug, Clone)]
enum Action {
    Inc,
    Fail,
}

enum SideEffect {}

#[derive(Default)]
struct ReducerImpl;

impl Reducer for ReducerImpl {
    type State = State;
    type Action = Action;
    type SideEffect = SideEffect;

    async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

    fn reduce(&mut self, state: &mut Self::State, action: Self::Action) -> CoreResult<StateChange> {
        match action {
            Action::Inc => {
                state.counter = state.counter.saturating_add(1);
                Ok(StateChange::FullUpdate)
            }
            Action::Fail => Err(OxideError::Internal {
                message: "expected failure".to_string(),
            }),
        }
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _effect: Self::SideEffect,
    ) -> CoreResult<StateChange> {
        Ok(StateChange::None)
    }
}

#[tokio::test]
async fn persistence_restores_state_across_engines() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = oxide_core::runtime::init(thread_pool);

    let key = "oxide_core.test.persistence_engine_restore.v1".to_string();
    let path = oxide_core::persistence::default_persistence_path(&key);
    let _ = std::fs::remove_file(&path);

    {
        let engine = ReducerEngine::<ReducerImpl>::new_persistent(
            ReducerImpl::default(),
            State { counter: 0 },
            PersistenceConfig {
                key: key.clone(),
                min_interval: std::time::Duration::from_millis(0),
            },
        )
        .await
        .unwrap();
        let _ = engine.dispatch(Action::Inc).await.expect("dispatch");
        let _ = engine.dispatch(Action::Fail).await.unwrap_err();
    }

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let engine = ReducerEngine::<ReducerImpl>::new_persistent(
        ReducerImpl::default(),
        State { counter: 0 },
        PersistenceConfig {
            key: key.clone(),
            min_interval: std::time::Duration::from_millis(0),
        },
    )
    .await
    .unwrap();
    let snap = engine.current().await;
    assert_eq!(snap.revision, 0);
    assert_eq!(snap.state.counter, 1);
}
