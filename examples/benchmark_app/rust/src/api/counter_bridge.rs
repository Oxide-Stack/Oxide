use flutter_rust_bridge::frb;
use oxide_generator_rs::reducer;
use std::sync::OnceLock;
use std::time::Duration;

use crate::util::fnv1a_mix_u64;
use crate::state::counter_action::CounterAction;
use crate::state::counter_state::CounterState;
pub use crate::OxideError;

static NAV_BOOTSTRAP: OnceLock<()> = OnceLock::new();

#[reducer(
    engine = CounterEngine,
    snapshot = CounterStateSnapshot,
    initial = CounterState::new(),
)]
impl oxide_core::Reducer for CounterRootReducer {
    type State = CounterState;
    type Action = CounterAction;
    type SideEffect = CounterSideEffect;

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {
        #[cfg(feature = "navigation-binding")]
        NAV_BOOTSTRAP.get_or_init(|| {
            let _ = oxide_core::init_navigation();
            let runtime = oxide_core::navigation_runtime().ok();
            oxide_core::tokio::spawn(async move {
                if let Some(runtime) = runtime {
                    runtime.set_current_route(Some(oxide_core::navigation::NavRoute {
                        kind: "Splash".into(),
                        payload: serde_json::to_value(crate::routes::SplashRoute {})
                            .unwrap_or(serde_json::Value::Null),
                        extras: None,
                    }));
                }

                oxide_core::tokio::time::sleep(Duration::from_millis(450)).await;

                if let Some(runtime) = runtime {
                    runtime.reset(vec![oxide_core::navigation::NavRoute {
                        kind: "Home".into(),
                        payload: serde_json::to_value(crate::routes::HomeRoute {})
                            .unwrap_or(serde_json::Value::Null),
                        extras: None,
                    }]);
                }
            });
        });
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<'_, Self::Action, Self::State, ()>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        let CounterAction::Run { iterations } = ctx.input;
        if *iterations == 0 {
            return Ok(oxide_core::StateChange::None);
        }

        for _ in 0..*iterations {
            state.counter = state.counter.saturating_add(1);
            state.checksum = fnv1a_mix_u64(state.checksum, state.counter);
        }

        Ok(oxide_core::StateChange::Full)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<'_, Self::SideEffect, Self::State, ()>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}
#[frb(ignore)]
enum CounterSideEffect {}
#[frb(ignore)]
#[derive(Default)]
struct CounterRootReducer {}
