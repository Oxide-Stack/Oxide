use oxide_core::runtime::spawn;
use oxide_core::{CoreResult, InitContext, Reducer, ReducerCtx, StateChange};
use oxide_generator_rs::reducer;
use tokio::sync::mpsc::UnboundedSender;

use crate::api::settings::side_effects::extract_main_color;
pub use crate::api::settings::{
    actions::SettingsAction, side_effects::SettingsSideEffect, state::SettingsState,
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
        ctx: ReducerCtx<'_, Self::Action, Self::State>,
    ) -> CoreResult<StateChange> {
        match ctx.input {
            SettingsAction::SetTheme(theme) => {
                state.theme = theme.clone();
                Ok(StateChange::Full)
            }
            SettingsAction::SetMainColor(color) => {
                state.main_color = color.clone();
                Ok(StateChange::Full)
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
        }
    }

    fn effect(
        &mut self,
        state: &mut Self::State,
        ctx: ReducerCtx<'_, Self::SideEffect, Self::State>,
    ) -> CoreResult<StateChange> {
        match ctx.input {
            SettingsSideEffect::SetMainColor(color) => {
                state.main_color = color.clone();
                Ok(StateChange::Full)
            }
        }
    }
}
