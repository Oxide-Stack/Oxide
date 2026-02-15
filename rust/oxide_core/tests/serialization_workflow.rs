#![cfg(feature = "state-persistence")]

use oxide_core::{CoreResult, InitContext, Reducer};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct StructState {
    value: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum EnumState {
    Idle,
    Active { count: u64 },
}

#[derive(Debug, Clone)]
enum Action {
    Inc,
}

enum SideEffect {}

#[derive(Default)]
struct TestReducer;

impl Reducer for TestReducer {
    type State = StructState;
    type Action = Action;
    type SideEffect = SideEffect;

    async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        state: &mut Self::State,
        action: Self::Action,
    ) -> CoreResult<oxide_core::StateChange> {
        match action {
            Action::Inc => {
                state.value = state.value.saturating_add(1);
            }
        }
        Ok(oxide_core::StateChange::Full)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _effect: Self::SideEffect,
    ) -> CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

#[test]
fn bincode_round_trip_struct_state() {
    let state = StructState { value: 123 };
    let bytes = oxide_core::persistence::encode(&state).expect("encode");
    let decoded: StructState = oxide_core::persistence::decode(&bytes).expect("decode");
    assert_eq!(decoded, state);
}

#[test]
fn bincode_round_trip_enum_state() {
    let state = EnumState::Active { count: 7 };
    let bytes = oxide_core::persistence::encode(&state).expect("encode");
    let decoded: EnumState = oxide_core::persistence::decode(&bytes).expect("decode");
    assert_eq!(decoded, state);
}

#[tokio::test]
async fn dispatch_then_encode_decode_snapshot() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = oxide_core::runtime::init(thread_pool);

    let engine = oxide_core::ReducerEngine::<TestReducer>::new(
        TestReducer::default(),
        StructState { value: 0 },
    )
    .await
    .unwrap();
    let snapshot = engine.dispatch(Action::Inc).await.expect("dispatch");
    assert_eq!(snapshot.revision, 1);
    assert_eq!(snapshot.state.value, 1);

    let bytes = oxide_core::persistence::encode(&snapshot).expect("encode snapshot");
    let decoded: oxide_core::StateSnapshot<StructState> =
        oxide_core::persistence::decode(&bytes).expect("decode snapshot");

    assert_eq!(decoded, snapshot);
}
