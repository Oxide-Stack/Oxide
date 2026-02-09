use oxide_core::{CoreResult, InitContext, Reducer, ReducerEngine, StateChange};

#[derive(Debug, Clone, PartialEq, Eq)]
struct SlicedState {
    a: u64,
    b: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlicedStateSlice {
    A,
    B,
}

#[derive(Clone)]
enum SlicedAction {
    IncA,
    IncB,
    ExplicitA,
    Full,
}

enum SlicedSideEffect {}

#[derive(Default)]
struct SlicedReducer;

impl Reducer<SlicedStateSlice> for SlicedReducer {
    type State = SlicedState;
    type Action = SlicedAction;
    type SideEffect = SlicedSideEffect;

    async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<'_, Self::Action, Self::State, SlicedStateSlice>,
    ) -> CoreResult<StateChange<SlicedStateSlice>> {
        match ctx.input {
            SlicedAction::IncA => {
                state.a = state.a.saturating_add(1);
                Ok(StateChange::Infer)
            }
            SlicedAction::IncB => {
                state.b = state.b.saturating_add(1);
                Ok(StateChange::Infer)
            }
            SlicedAction::ExplicitA => {
                state.a = state.a.saturating_add(1);
                Ok(StateChange::Slices(&[SlicedStateSlice::A]))
            }
            SlicedAction::Full => {
                state.a = state.a.saturating_add(1);
                Ok(StateChange::Full)
            }
        }
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<'_, Self::SideEffect, Self::State, SlicedStateSlice>,
    ) -> CoreResult<StateChange<SlicedStateSlice>> {
        Ok(StateChange::None)
    }

    fn infer_slices(&self, before: &Self::State, after: &Self::State) -> Vec<SlicedStateSlice> {
        let mut slices = Vec::new();
        if before.a != after.a {
            slices.push(SlicedStateSlice::A);
        }
        if before.b != after.b {
            slices.push(SlicedStateSlice::B);
        }
        slices
    }
}

fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
    static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
        std::sync::OnceLock::new();
    POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
}

#[tokio::test]
async fn infer_emits_changed_slice() {
    let _ = oxide_core::runtime::init(thread_pool);

    let engine = ReducerEngine::<SlicedReducer, SlicedStateSlice>::new(
        SlicedReducer::default(),
        SlicedState { a: 0, b: 0 },
    )
    .await
    .unwrap();

    let snap = engine.dispatch(SlicedAction::IncA).await.unwrap();
    assert_eq!(snap.state, SlicedState { a: 1, b: 0 });
    assert_eq!(snap.slices, vec![SlicedStateSlice::A]);

    let snap = engine.dispatch(SlicedAction::IncB).await.unwrap();
    assert_eq!(snap.state, SlicedState { a: 1, b: 1 });
    assert_eq!(snap.slices, vec![SlicedStateSlice::B]);
}

#[tokio::test]
async fn explicit_slices_bypass_inference() {
    let _ = oxide_core::runtime::init(thread_pool);

    let engine = ReducerEngine::<SlicedReducer, SlicedStateSlice>::new(
        SlicedReducer::default(),
        SlicedState { a: 0, b: 0 },
    )
    .await
    .unwrap();

    let snap = engine.dispatch(SlicedAction::ExplicitA).await.unwrap();
    assert_eq!(snap.state, SlicedState { a: 1, b: 0 });
    assert_eq!(snap.slices, vec![SlicedStateSlice::A]);
}

#[tokio::test]
async fn full_emits_empty_slices() {
    let _ = oxide_core::runtime::init(thread_pool);

    let engine = ReducerEngine::<SlicedReducer, SlicedStateSlice>::new(
        SlicedReducer::default(),
        SlicedState { a: 0, b: 0 },
    )
    .await
    .unwrap();

    let full = engine.dispatch(SlicedAction::Full).await.unwrap();
    assert_eq!(full.state, SlicedState { a: 1, b: 0 });
    assert!(full.slices.is_empty());
}
