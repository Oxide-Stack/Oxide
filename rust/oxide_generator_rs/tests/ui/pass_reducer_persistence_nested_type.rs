use oxide_generator_rs::{actions, reducer, state};

#[state]
pub enum Theme {
    Light,
    Dark,
}

#[state]
pub struct SettingsState {
    pub theme: Theme,
    pub count: u64,
}

#[actions]
pub enum SettingsAction {
    SetTheme(Theme),
}

pub enum SettingsSideEffect {}

#[derive(Default)]
pub struct SettingsReducer;

#[reducer(
    engine = SettingsEngine,
    snapshot = SettingsSnapshot,
    initial = SettingsState {
        theme: Theme::Light,
        count: 0
    },
    persist = "settings",
    no_frb
)]
impl oxide_core::Reducer for SettingsReducer {
    type State = SettingsState;
    type Action = SettingsAction;
    type SideEffect = SettingsSideEffect;

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<'_, Self::Action, Self::State, ()>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match ctx.input {
            SettingsAction::SetTheme(theme) => {
                state.theme = theme.clone();
                state.count = state.count.saturating_add(1);
            }
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

#[cfg(feature = "state-persistence")]
async fn smoke_nested_type_persistence_api() -> oxide_core::CoreResult<()> {
    let bytes = SettingsEngine::encode_state_value(&SettingsState {
        theme: Theme::Dark,
        count: 3,
    })?;
    let decoded = SettingsEngine::decode_state_value(&bytes)?;
    match decoded.theme {
        Theme::Light | Theme::Dark => {}
    }
    Ok(())
}

fn main() {}
