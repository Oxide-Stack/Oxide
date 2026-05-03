use oxide_core::runtime::spawn;
use oxide_core::{InitContext, Reducer, ReducerCtx, StateChange};
use oxide_generator_rs::reducer;
use tokio::sync::mpsc::UnboundedSender;

use crate::api::settings::side_effects::extract_main_color;
pub use crate::api::settings::{
    actions::SettingsAction, side_effects::SettingsSideEffect, state::SettingsState,
    state::SettingsStateSlice,
};
use crate::config::enums::theme_type::Theme;

#[flutter_rust_bridge::frb(ignore)]
#[derive(Default)]
pub struct SettingsReducer {
    pub side_effect_tx: Option<UnboundedSender<SettingsSideEffect>>,
}

#[reducer(
  engine = SettingsEngine,
  snapshot = SettingsStateSnapshot,
  initial = SettingsState { theme: Theme::Light, main_color: String::new() },
  persist = "oxide.showcase.settings.v1",
  persist_min_interval_ms = 200,
)]
impl Reducer for SettingsReducer {
    type State = SettingsState;

    type Action = SettingsAction;

    type SideEffect = SettingsSideEffect;

    async fn init(&mut self, ctx: InitContext<Self::SideEffect>) {
        self.side_effect_tx = Some(ctx.sideeffect_tx);
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: ReducerCtx<'_, Self::Action, Self::State, SettingsStateSlice>,
    ) -> oxide_core::CoreResult<StateChange> {
        match ctx.input {
            SettingsAction::SetTheme(theme) => {
                if state.theme == *theme {
                    return Ok(StateChange::None);
                }
                state.theme = theme.clone();
                Ok(StateChange::Infer)
            }
            SettingsAction::SetMainColor(color) => {
                if state.main_color == *color {
                    return Ok(StateChange::None);
                }
                state.main_color = color.clone();
                Ok(StateChange::Infer)
            }
            SettingsAction::ExtractMainColor(path) => {
                let image_path = path.clone();
                let tx = self.side_effect_tx.clone();
                spawn(async move {
                    let result = extract_main_color(image_path).await;
                    if let Some(color) = result {
                        tx.as_ref()
                            .unwrap()
                            .send(SettingsSideEffect::SetMainColor(color))
                            .ok();
                    }
                });
                Ok(StateChange::None)
            }
            SettingsAction::ResetState => {
                let mut changed = false;
                if state.theme != Theme::Light {
                    state.theme = Theme::Light;
                    changed = true;
                }
                if !state.main_color.is_empty() {
                    state.main_color.clear();
                    changed = true;
                }
                if !changed {
                    return Ok(StateChange::None);
                }
                Ok(StateChange::Infer)
            }
            SettingsAction::OpenHome => {
                #[cfg(feature = "navigation-binding")]
                if let Ok(runtime) = oxide_core::navigation_runtime() {
                    let _ = runtime.reset(vec![crate::routes::RoutePayload::HomeScreen(
                        crate::routes::HomeScreen {},
                    )]);
                }
                Ok(StateChange::None)
            }
            SettingsAction::OpenSettings => {
                #[cfg(feature = "navigation-binding")]
                {
                    let _ = ctx.nav.push(crate::routes::SettingsScreen {});
                }
                Ok(StateChange::None)
            }
            SettingsAction::OpenChannels => {
                #[cfg(feature = "navigation-binding")]
                {
                    let _ = ctx.nav.push(crate::routes::ChannelsScreen {});
                }
                Ok(StateChange::None)
            }
            SettingsAction::OpenBackendMatrix => {
                #[cfg(feature = "navigation-binding")]
                {
                    let _ = ctx.nav.push(crate::routes::BackendMatrixScreen {});
                }
                Ok(StateChange::None)
            }
            SettingsAction::Pop => {
                #[cfg(feature = "navigation-binding")]
                {
                    let _ = ctx.nav.pop();
                }
                Ok(StateChange::None)
            }
        }
    }

    fn effect(
        &mut self,
        state: &mut Self::State,
        ctx: ReducerCtx<'_, Self::SideEffect, Self::State, SettingsStateSlice>,
    ) -> oxide_core::CoreResult<StateChange> {
        match ctx.input {
            SettingsSideEffect::SetMainColor(color) => {
                if state.main_color == *color {
                    return Ok(StateChange::None);
                }
                state.main_color = color.clone();
                Ok(StateChange::Infer)
            }
        }
    }
}
