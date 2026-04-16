use crate::config::enums::theme_type::Theme;
use oxide_generator_rs::state;

#[state]
pub struct SettingsState {
    pub theme: Theme,
    pub main_color: String,
}
