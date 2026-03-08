use serde::{Deserialize, Serialize};
// navigation actions were originally defined here, but chart navigation
// is now executed directly in Dart rather than via Rust.  The reducer and
// associated bridges are kept only for historical reference and could be
// removed entirely in future cleanup.
#[derive(Debug, Clone, Copy)]
#[flutter_rust_bridge::frb(ignore)]
pub enum BenchNavAction {
    // no-op placeholder
    _Unused,
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
        // no actions are currently handled; navigation is performed in
        // Dart so we don't need to react to any Rust nav commands here.
        match ctx.input {
            _ => Ok(oxide_core::StateChange::None),
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
