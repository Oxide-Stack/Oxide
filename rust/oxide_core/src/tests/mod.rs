// Internal (crate-level) behavioral tests.
//
// these tests validate invariants that are easy to accidentally break during
// refactors (transactionality, snapshot emission rules, and error handling).
use crate::{CoreResult, InitContext, OxideError, Reducer, ReducerEngine, StateChange};

mod engine_behavior;
#[cfg(feature = "isolated-channels")]
mod isolated_channels;
#[cfg(feature = "state-persistence")]
mod persistence;
mod runtime;
mod sideeffects;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TestState {
    value: u64,
}

#[derive(Clone)]
enum TestAction {
    Increment,
    Noop,
    Infer,
    ExplicitSlices,
    MutateThenFail,
}

enum TestSideEffect {
    Increment,
    Noop,
    Infer,
    ExplicitSlices,
    Fail,
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
        state: &mut Self::State,
        ctx: crate::Context<'_, Self::Action, Self::State, ()>,
    ) -> CoreResult<StateChange> {
        match ctx.input {
            TestAction::Increment => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::Full)
            }
            TestAction::Noop => Ok(StateChange::None),
            TestAction::Infer => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::Infer)
            }
            TestAction::ExplicitSlices => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::Slices(&[]))
            }
            TestAction::MutateThenFail => {
                state.value = state.value.saturating_add(1);
                Err(OxideError::Internal {
                    message: "boom".to_string(),
                })
            }
        }
    }

    fn effect(
        &mut self,
        state: &mut Self::State,
        ctx: crate::Context<'_, Self::SideEffect, Self::State, ()>,
    ) -> CoreResult<StateChange> {
        match ctx.input {
            TestSideEffect::Increment => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::Full)
            }
            TestSideEffect::Noop => Ok(StateChange::None),
            TestSideEffect::Infer => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::Infer)
            }
            TestSideEffect::ExplicitSlices => {
                state.value = state.value.saturating_add(1);
                Ok(StateChange::Slices(&[]))
            }
            TestSideEffect::Fail => {
                state.value = state.value.saturating_add(1);
                Err(OxideError::Internal {
                    message: "effect boom".to_string(),
                })
            }
        }
    }
}

fn init_test_runtime() {
    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = crate::runtime::init(thread_pool);
    #[cfg(feature = "navigation-binding")]
    {
        let _ = crate::init_navigation();
    }
}
