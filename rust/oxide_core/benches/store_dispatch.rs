use criterion::{Criterion, black_box, criterion_group, criterion_main};

use oxide_core::{InitContext, Reducer, ReducerEngine, StateChange};

#[derive(Clone)]
struct BenchReducer;

#[derive(Clone)]
enum BenchAction {
    Increment,
}

impl Reducer for BenchReducer {
    type State = u64;
    type Action = BenchAction;
    type SideEffect = ();

    async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<'_, Self::Action, Self::State, ()>,
    ) -> oxide_core::CoreResult<StateChange> {
        match ctx.input {
            BenchAction::Increment => {
                *state = state.saturating_add(1);
                Ok(StateChange::Full)
            }
        }
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<'_, Self::SideEffect, Self::State, ()>,
    ) -> oxide_core::CoreResult<StateChange> {
        Ok(StateChange::None)
    }
}

fn bench_store_dispatch(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build Tokio runtime");

    let engine = runtime.block_on(async {
        fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
            static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
                std::sync::OnceLock::new();
            POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
        }
        let _ = oxide_core::runtime::init(thread_pool);
        ReducerEngine::<BenchReducer>::new(BenchReducer, 0)
            .await
            .unwrap()
    });

    c.bench_function("store_dispatch_increment", |b| {
        b.iter(|| {
            let snapshot = runtime.block_on(engine.dispatch(BenchAction::Increment));
            black_box(snapshot)
        })
    });
}

criterion_group!(benches, bench_store_dispatch);
criterion_main!(benches);
