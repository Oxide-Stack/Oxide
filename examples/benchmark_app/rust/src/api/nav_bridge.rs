use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy)]
#[flutter_rust_bridge::frb(ignore)]
pub enum BenchNavAction {
    OpenCharts,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[flutter_rust_bridge::frb(ignore)]
pub struct BenchNavState {}

pub type BenchNavEngine = oxide_core::ReducerEngine<BenchNavReducer>;

impl oxide_core::Reducer for BenchNavReducer {
    type State = BenchNavState;
    type Action = BenchNavAction;
    type SideEffect = ();

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        _state: &mut Self::State,
        ctx: oxide_core::ReducerCtx<'_, Self::Action, Self::State>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match ctx.input {
            BenchNavAction::OpenCharts => {
                if let Ok(runtime) = oxide_core::navigation_runtime() {
                    let _ = runtime.push(crate::routes::ChartsRoute {});
                }
                Ok(oxide_core::StateChange::None)
            }
        }
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::ReducerCtx<'_, Self::SideEffect, Self::State>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

#[flutter_rust_bridge::frb(ignore)]
#[derive(Default)]
pub struct BenchNavReducer {}

pub async fn nav_engine() -> Result<BenchNavEngine, oxide_core::OxideError> {
    static ENGINE: oxide_core::tokio::sync::OnceCell<BenchNavEngine> =
        oxide_core::tokio::sync::OnceCell::const_new();
    let engine = ENGINE
        .get_or_try_init(|| async {
            BenchNavEngine::new(BenchNavReducer::default(), BenchNavState::default()).await
        })
        .await?;
    Ok(engine.clone())
}
