use oxide_core::{CoreResult, InitContext, OxideError, Reducer, ReducerEngine, StateChange};

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    value: u64,
}

enum Action {
    Increment,
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
            Action::Increment => {
                state.value = state.value.saturating_add(1);
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
async fn reducer_engine_round_trip() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = oxide_core::runtime::init(thread_pool);

    let engine = ReducerEngine::<ReducerImpl>::new(ReducerImpl::default(), State { value: 0 })
        .await
        .unwrap();
    let snapshot = engine.dispatch(Action::Increment).await.unwrap();
    assert_eq!(snapshot.revision, 1);
    assert_eq!(snapshot.state, State { value: 1 });
}

#[tokio::test]
async fn reducer_engine_dispatch_returns_error() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = oxide_core::runtime::init(thread_pool);

    let engine = ReducerEngine::<ReducerImpl>::new(ReducerImpl::default(), State { value: 0 })
        .await
        .unwrap();
    let err = engine.dispatch(Action::Fail).await.unwrap_err();
    assert!(matches!(err, OxideError::Internal { .. }));

    let snapshot = engine.current().await;
    assert_eq!(snapshot.revision, 0);
    assert_eq!(snapshot.state, State { value: 0 });
}
