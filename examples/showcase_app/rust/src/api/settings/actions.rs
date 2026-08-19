use crate::config::enums::theme_type::Theme;
use oxide_generator_rs::actions;

#[actions]
pub enum SettingsAction {
    SetTheme(Theme),
    SetMainColor(String),     // color hex value
    ExtractMainColor(String), // Path to the image
    ResetState,
    OpenHome,
    OpenSettings,
    OpenChannels,
    OpenBackendMatrix,
    Pop,
}
