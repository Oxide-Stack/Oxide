#![cfg(feature = "state-persistence")]

//! Regression test: `ReducerEngine::new_persistent` must return a validation
//! error instead of panicking when the Oxide runtime has never been
//! initialized.
//!
//! This file intentionally lives in its own integration-test binary so no other
//! test can initialize the process-wide runtime first.

use oxide_core::{CoreResult, InitContext, OxideError, Reducer, ReducerEngine, StateChange};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
async fn new_persistent_before_runtime_init_returns_validation_error_without_panicking() {
    let result: CoreResult<ReducerEngine<ReducerImpl>> = ReducerEngine::new_persistent(
        ReducerImpl::default(),
        State { counter: 0 },
        oxide_core::persistence::PersistenceConfig {
            key: "oxide_core.test.new_persistent_uninitialized.v1".to_string(),
            min_interval: std::time::Duration::from_millis(0),
        },
    )
    .await;

    let Err(err) = result else {
        panic!("expected new_persistent to fail before runtime init")
    };
    match err {
        OxideError::Validation { message } => {
            assert!(message.contains("runtime not initialized"));
        }
        other => panic!("expected Validation error before runtime init, got {other:?}"),
    }
}
