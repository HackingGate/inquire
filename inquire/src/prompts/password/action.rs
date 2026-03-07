use crate::{
    ui::{Key, KeyModifiers},
    InnerAction, InputAction,
};

use super::config::PasswordConfig;

/// Set of actions for a PasswordPrompt.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PasswordPromptAction {
    /// Action on the value text input handler.
    ValueInput(InputAction),
    /// Toggles the display mode between plain text and the initial one.
    ToggleDisplayMode,
}

impl InnerAction for PasswordPromptAction {
    type Config = PasswordConfig;

    fn from_key(key: Key, config: &PasswordConfig) -> Option<Self> {
        let action = match key {
            Key::Char('r' | 'R', m)
                if m.contains(KeyModifiers::CONTROL) && config.enable_display_toggle =>
            {
                Self::ToggleDisplayMode
            }
            key => match InputAction::from_key(key, &()) {
                Some(action) => Self::ValueInput(action),
                None => return None,
            },
        };

        Some(action)
    }
}

#[cfg(test)]
mod test {
    use super::{PasswordConfig, PasswordPromptAction};
    use crate::PasswordDisplayMode;
    use crate::prompts::action::InnerAction;
    use crate::ui::{Key, KeyModifiers};

    #[test]
    fn ctrl_h_not_handled_by_inner_action() {
        // Ctrl+H is now handled at the top-level Action enum (Action::Help)
        let cfg = PasswordConfig {
            enable_display_toggle: true,
            display_mode: PasswordDisplayMode::Masked,
        };
        let action = PasswordPromptAction::from_key(
            Key::Char('h', KeyModifiers::CONTROL),
            &cfg,
        );
        assert_eq!(action, None);
    }

    #[test]
    fn ctrl_r_still_maps_to_toggle_display() {
        let cfg = PasswordConfig {
            enable_display_toggle: true,
            display_mode: PasswordDisplayMode::Masked,
        };
        let action = PasswordPromptAction::from_key(
            Key::Char('r', KeyModifiers::CONTROL),
            &cfg,
        );
        assert_eq!(action, Some(PasswordPromptAction::ToggleDisplayMode));
    }
}
