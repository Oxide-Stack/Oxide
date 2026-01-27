use criterion::{Criterion, black_box, criterion_group, criterion_main};

use oxide_core::{Reducer, ReducerEngine, StateChange};

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

    fn init(
        &mut self,
        _sideeffect_tx: oxide_core::tokio::sync::mpsc::UnboundedSender<Self::SideEffect>,
    ) {
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        action: Self::Action,
    ) -> oxide_core::CoreResult<StateChange> {
        match action {
            BenchAction::Increment => {
                *state = state.saturating_add(1);
                Ok(StateChange::FullUpdate)
            }
        }
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<StateChange> {
        Ok(StateChange::None)
    }
}

fn bench_store_dispatch(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build Tokio runtime");

    let engine = {
        let _guard = runtime.enter();
        ReducerEngine::<BenchReducer>::new(BenchReducer, 0)
    };

    c.bench_function("store_dispatch_increment", |b| {
        b.iter(|| {
            let snapshot = runtime.block_on(engine.dispatch(BenchAction::Increment));
            black_box(snapshot)
        })
    });
}

criterion_group!(benches, bench_store_dispatch);
criterion_main!(benches);
