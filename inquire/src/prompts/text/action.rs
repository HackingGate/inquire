use crate::{
    ui::{Key, KeyModifiers},
    InnerAction, InputAction,
};

use super::config::TextConfig;

/// Set of actions for a TextPrompt.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum TextPromptAction {
    /// Action on the value text input handler.
    ValueInput(InputAction),
    /// When a suggestion list exists, moves the cursor to the option above.
    MoveToSuggestionAbove,
    /// When a suggestion list exists, moves the cursor to the option below.
    MoveToSuggestionBelow,
    /// When a suggestion list exists, moves the cursor to the page above.
    MoveToSuggestionPageUp,
    /// When a suggestion list exists, moves the cursor to the page below.
    MoveToSuggestionPageDown,
    /// When a suggestion list exists, autocompletes using the standard key (Tab).
    UseCurrentSuggestion,
    /// Requests contextual help for the current field (Ctrl+H).
    ShowHelp,
}

impl InnerAction for TextPromptAction {
    type Config = TextConfig;

    fn from_key(key: Key, _config: &TextConfig) -> Option<Self> {
        let action = match key {
            Key::Up(KeyModifiers::NONE) | Key::Char('p', KeyModifiers::CONTROL) => {
                Self::MoveToSuggestionAbove
            }
            Key::PageUp(_) => Self::MoveToSuggestionPageUp,

            Key::Down(KeyModifiers::NONE) | Key::Char('n', KeyModifiers::CONTROL) => {
                Self::MoveToSuggestionBelow
            }
            Key::PageDown(_) => Self::MoveToSuggestionPageDown,

            Key::Tab => Self::UseCurrentSuggestion,
            Key::Char('h' | 'H', KeyModifiers::CONTROL) => Self::ShowHelp,

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
    use super::{TextConfig, TextPromptAction};
    use crate::prompts::action::InnerAction;
    use crate::ui::Key;

    #[test]
    fn ctrl_h_maps_to_help_suggestion_action() {
        let cfg = TextConfig { page_size: 7 };
        let action = TextPromptAction::from_key(Key::Char('h', crate::ui::KeyModifiers::CONTROL), &cfg);
        assert_eq!(action, Some(TextPromptAction::ShowHelp));
    }
}
