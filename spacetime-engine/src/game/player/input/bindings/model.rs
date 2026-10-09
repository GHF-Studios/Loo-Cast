//! Mutable binding state and semantic action lookup.

use super::{
    DEFAULT_PLAYER_BINDINGS, PlayerAction, PlayerInputButton, bind_target_actions, key_label,
};
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Debug, Clone)]
pub(crate) struct PlayerInputBindings {
    bindings: HashMap<PlayerAction, Vec<PlayerInputButton>>,
    console_bindings: HashMap<PlayerInputButton, String>,
    binding_labels: HashMap<PlayerInputButton, String>,
}

impl Default for PlayerInputBindings {
    fn default() -> Self {
        let mut result = Self::empty();
        for &(button, target) in DEFAULT_PLAYER_BINDINGS {
            result
                .bind_named(button, target)
                .expect("compiled default input binding must be valid");
        }
        result
    }
}

impl PlayerInputBindings {
    fn empty() -> Self {
        Self {
            bindings: HashMap::new(),
            console_bindings: HashMap::new(),
            binding_labels: HashMap::new(),
        }
    }

    pub(super) fn console_commands(&self) -> impl Iterator<Item = (&PlayerInputButton, &String)> {
        self.console_bindings.iter()
    }

    pub(crate) fn bind_named(&mut self, button: &str, target: &str) -> Result<String, String> {
        let Some(button) = PlayerInputButton::parse(button) else {
            return Err(format!("unknown bindable input `{button}`"));
        };
        self.bind_button(button, target)
    }

    pub(crate) fn bind_button(
        &mut self,
        button: PlayerInputButton,
        target: &str,
    ) -> Result<String, String> {
        let target = target.trim();
        if target.is_empty() {
            return Err("binding target must not be empty".to_string());
        }

        self.unbind_button(button);
        let canonical = if let Some((canonical, actions)) = bind_target_actions(target) {
            for &action in actions {
                self.bindings.entry(action).or_default().push(button);
            }
            canonical.to_string()
        } else {
            self.console_bindings.insert(button, target.to_string());
            target.to_string()
        };
        self.binding_labels.insert(button, canonical.clone());
        Ok(canonical)
    }

    pub(crate) fn unbind_named(&mut self, button: &str) -> Result<Option<String>, String> {
        let Some(button) = PlayerInputButton::parse(button) else {
            return Err(format!("unknown bindable input `{button}`"));
        };
        Ok(self.unbind_button(button))
    }

    pub(crate) fn unbind_button(&mut self, button: PlayerInputButton) -> Option<String> {
        for buttons in self.bindings.values_mut() {
            buttons.retain(|bound| *bound != button);
        }
        self.console_bindings.remove(&button);
        self.binding_labels.remove(&button)
    }

    pub(crate) fn clear_all(&mut self) {
        self.bindings.clear();
        self.console_bindings.clear();
        self.binding_labels.clear();
    }

    pub(crate) fn binding_for_name(&self, button: &str) -> Result<Option<&str>, String> {
        let Some(button) = PlayerInputButton::parse(button) else {
            return Err(format!("unknown bindable input `{button}`"));
        };
        Ok(self.binding_labels.get(&button).map(String::as_str))
    }

    pub(crate) fn binding_lines(&self) -> Vec<String> {
        let mut entries = self
            .binding_labels
            .iter()
            .map(|(button, target)| (button.canonical_name(), target.as_str()))
            .collect::<Vec<_>>();
        entries.sort_by(|left, right| left.0.cmp(&right.0));
        entries
            .into_iter()
            .map(|(button, target)| format!("{button:<12} {target}"))
            .collect()
    }

    pub(crate) fn pressed_raw(
        &self,
        action: PlayerAction,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        self.bindings
            .get(&action)
            .into_iter()
            .flatten()
            .copied()
            .any(|button| button.pressed(keyboard, mouse))
    }

    pub(crate) fn just_pressed_raw(
        &self,
        action: PlayerAction,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        self.bindings
            .get(&action)
            .into_iter()
            .flatten()
            .copied()
            .any(|button| button.just_pressed(keyboard, mouse))
    }

    pub(in crate::game::player::input) fn has_mouse_binding(&self, action: PlayerAction) -> bool {
        self.bindings
            .get(&action)
            .into_iter()
            .flatten()
            .any(|button| matches!(button, PlayerInputButton::Mouse(_)))
    }

    pub(crate) fn hotbar_slot_just_pressed_raw(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> Option<usize> {
        PlayerAction::HOTBAR
            .into_iter()
            .position(|action| self.just_pressed_raw(action, keyboard, mouse))
    }

    pub(crate) fn label(&self, action: PlayerAction) -> String {
        self.bindings
            .get(&action)
            .and_then(|bindings| bindings.first())
            .map(|button| match button {
                PlayerInputButton::Mouse(MouseButton::Left) => "LMB".to_string(),
                PlayerInputButton::Mouse(MouseButton::Right) => "RMB".to_string(),
                PlayerInputButton::Mouse(MouseButton::Middle) => "MMB".to_string(),
                PlayerInputButton::Mouse(other) => format!("{other:?}"),
                PlayerInputButton::Key(key) => key_label(*key),
            })
            .unwrap_or_else(|| "UNBOUND".to_string())
    }

    pub(crate) fn movement_cluster_label(&self) -> String {
        [
            PlayerAction::MoveForward,
            PlayerAction::MoveLeft,
            PlayerAction::MoveBackward,
            PlayerAction::MoveRight,
        ]
        .into_iter()
        .map(|action| self.label(action))
        .collect::<String>()
    }
}
