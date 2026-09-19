//! Device helper for conventional numeric hotbar selection.

use bevy::prelude::*;

use super::HOTBAR_SIZE;

pub(in crate::game) fn pressed_hotbar_slot(keyboard: &ButtonInput<KeyCode>) -> Option<usize> {
    const KEYS: [KeyCode; HOTBAR_SIZE] = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];

    KEYS.into_iter().position(|key| keyboard.just_pressed(key))
}
