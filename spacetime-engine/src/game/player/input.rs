//! Local-human device bindings and per-frame semantic input snapshot.
//!
//! Hardware state is sampled exactly once after focus/cursor arbitration.
//! Gameplay systems consume [`PlayerInputFrame`] and never inspect keys or
//! mouse buttons directly.

use std::collections::HashMap;

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

use crate::input_focus::InputFocus;

use super::cursor::CursorCapture;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PlayerAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    Ascend,
    Crouch,
    Descend,
    Sprint,
    Boost,
    ToggleLocalFlight,
    ToggleThrusters,
    ToggleCruise,
    ToggleSpatialDemand,
    ToggleCameraMode,
    ViewScaleModifier,
    FastModifier,
    ItemPrimary,
    ItemSecondary,
    ItemReload,
    EraseObject,
    ToggleCreativeMenu,
    Interact,
    TakeOff,
    Hotbar1,
    Hotbar2,
    Hotbar3,
    Hotbar4,
    Hotbar5,
    Hotbar6,
    Hotbar7,
    Hotbar8,
    Hotbar9,
}

impl PlayerAction {
    const ALL: [Self; 33] = [
        Self::MoveForward,
        Self::MoveBackward,
        Self::MoveLeft,
        Self::MoveRight,
        Self::Jump,
        Self::Ascend,
        Self::Crouch,
        Self::Descend,
        Self::Sprint,
        Self::Boost,
        Self::ToggleLocalFlight,
        Self::ToggleThrusters,
        Self::ToggleCruise,
        Self::ToggleSpatialDemand,
        Self::ToggleCameraMode,
        Self::ViewScaleModifier,
        Self::FastModifier,
        Self::ItemPrimary,
        Self::ItemSecondary,
        Self::ItemReload,
        Self::EraseObject,
        Self::ToggleCreativeMenu,
        Self::Interact,
        Self::TakeOff,
        Self::Hotbar1,
        Self::Hotbar2,
        Self::Hotbar3,
        Self::Hotbar4,
        Self::Hotbar5,
        Self::Hotbar6,
        Self::Hotbar7,
        Self::Hotbar8,
        Self::Hotbar9,
    ];

    const HOTBAR: [Self; 9] = [
        Self::Hotbar1,
        Self::Hotbar2,
        Self::Hotbar3,
        Self::Hotbar4,
        Self::Hotbar5,
        Self::Hotbar6,
        Self::Hotbar7,
        Self::Hotbar8,
        Self::Hotbar9,
    ];

    const fn bit(self) -> u64 {
        1_u64 << self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlayerInputButton {
    Key(KeyCode),
    Mouse(MouseButton),
}

#[derive(Resource, Debug, Clone)]
pub(crate) struct PlayerInputBindings {
    bindings: HashMap<PlayerAction, Vec<PlayerInputButton>>,
}

impl Default for PlayerInputBindings {
    fn default() -> Self {
        use PlayerAction as A;
        use PlayerInputButton::{Key, Mouse};

        let mut bindings = HashMap::new();
        let mut bind = |action, buttons: &[PlayerInputButton]| {
            bindings.insert(action, buttons.to_vec());
        };

        bind(A::MoveForward, &[Key(KeyCode::KeyW)]);
        bind(A::MoveBackward, &[Key(KeyCode::KeyS)]);
        bind(A::MoveLeft, &[Key(KeyCode::KeyA)]);
        bind(A::MoveRight, &[Key(KeyCode::KeyD)]);

        bind(A::Jump, &[Key(KeyCode::Space)]);
        bind(A::Ascend, &[Key(KeyCode::Space)]);
        bind(A::TakeOff, &[Key(KeyCode::Space)]);
        bind(A::Crouch, &[Key(KeyCode::ControlLeft), Key(KeyCode::ControlRight)]);
        bind(A::Descend, &[Key(KeyCode::ControlLeft), Key(KeyCode::ControlRight)]);
        bind(A::Sprint, &[Key(KeyCode::ShiftLeft), Key(KeyCode::ShiftRight)]);
        bind(A::Boost, &[Key(KeyCode::ShiftLeft), Key(KeyCode::ShiftRight)]);
        bind(A::FastModifier, &[Key(KeyCode::ShiftLeft), Key(KeyCode::ShiftRight)]);

        bind(A::ToggleLocalFlight, &[Key(KeyCode::KeyV)]);
        bind(A::ToggleThrusters, &[Key(KeyCode::KeyX)]);
        bind(A::ToggleCruise, &[Key(KeyCode::KeyC)]);
        bind(A::ToggleSpatialDemand, &[Key(KeyCode::KeyL)]);
        bind(A::ToggleCameraMode, &[Key(KeyCode::F5)]);
        bind(A::ViewScaleModifier, &[Key(KeyCode::AltLeft), Key(KeyCode::AltRight)]);

        bind(A::ItemPrimary, &[Mouse(MouseButton::Left)]);
        bind(A::ItemSecondary, &[Mouse(MouseButton::Right)]);
        bind(A::EraseObject, &[Mouse(MouseButton::Middle)]);
        bind(A::ItemReload, &[Key(KeyCode::KeyR)]);

        bind(A::ToggleCreativeMenu, &[Key(KeyCode::Tab)]);
        bind(A::Interact, &[Key(KeyCode::KeyE)]);

        for (action, key) in A::HOTBAR.into_iter().zip([
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
            KeyCode::Digit7,
            KeyCode::Digit8,
            KeyCode::Digit9,
        ]) {
            bind(action, &[Key(key)]);
        }

        Self { bindings }
    }
}

impl PlayerInputBindings {
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
            .any(|button| match button {
                PlayerInputButton::Key(key) => keyboard.pressed(key),
                PlayerInputButton::Mouse(button) => mouse.pressed(button),
            })
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
            .any(|button| match button {
                PlayerInputButton::Key(key) => keyboard.just_pressed(key),
                PlayerInputButton::Mouse(button) => mouse.just_pressed(button),
            })
    }

    fn has_mouse_binding(&self, action: PlayerAction) -> bool {
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

    pub(crate) fn hotbar_range_label(&self) -> String {
        format!(
            "{}–{}",
            self.label(PlayerAction::Hotbar1),
            self.label(PlayerAction::Hotbar9)
        )
    }
}

fn key_label(key: KeyCode) -> String {
    match key {
        KeyCode::KeyW => "W".into(),
        KeyCode::KeyA => "A".into(),
        KeyCode::KeyS => "S".into(),
        KeyCode::KeyD => "D".into(),
        KeyCode::KeyC => "C".into(),
        KeyCode::KeyE => "E".into(),
        KeyCode::KeyL => "L".into(),
        KeyCode::KeyR => "R".into(),
        KeyCode::KeyV => "V".into(),
        KeyCode::KeyX => "X".into(),
        KeyCode::Space => "SPACE".into(),
        KeyCode::ControlLeft | KeyCode::ControlRight => "CTRL".into(),
        KeyCode::ShiftLeft | KeyCode::ShiftRight => "SHIFT".into(),
        KeyCode::AltLeft | KeyCode::AltRight => "ALT".into(),
        KeyCode::Tab => "TAB".into(),
        KeyCode::F5 => "F5".into(),
        KeyCode::Digit1 => "1".into(),
        KeyCode::Digit2 => "2".into(),
        KeyCode::Digit3 => "3".into(),
        KeyCode::Digit4 => "4".into(),
        KeyCode::Digit5 => "5".into(),
        KeyCode::Digit6 => "6".into(),
        KeyCode::Digit7 => "7".into(),
        KeyCode::Digit8 => "8".into(),
        KeyCode::Digit9 => "9".into(),
        other => format!("{other:?}"),
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub(crate) struct PlayerInputFrame {
    pressed: u64,
    just_pressed: u64,
    look_delta: Vec2,
    scroll_y: f32,
    gameplay_active: bool,
}

impl PlayerInputFrame {
    pub(crate) const fn gameplay_active(&self) -> bool {
        self.gameplay_active
    }

    pub(crate) const fn pressed(&self, action: PlayerAction) -> bool {
        self.pressed & action.bit() != 0
    }

    pub(crate) const fn just_pressed(&self, action: PlayerAction) -> bool {
        self.just_pressed & action.bit() != 0
    }

    pub(crate) const fn look_delta(&self) -> Vec2 {
        self.look_delta
    }

    pub(crate) const fn scroll_y(&self) -> f32 {
        self.scroll_y
    }

    pub(crate) fn digital_axis(&self, negative: PlayerAction, positive: PlayerAction) -> f32 {
        (self.pressed(positive) as i8 - self.pressed(negative) as i8) as f32
    }

    pub(crate) fn hotbar_slot_just_pressed(&self) -> Option<usize> {
        PlayerAction::HOTBAR
            .into_iter()
            .position(|action| self.just_pressed(action))
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum PlayerInputSet {
    Cursor,
    Sample,
}

pub(super) fn sample_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    bindings: Res<PlayerInputBindings>,
    focus: Res<InputFocus>,
    capture: Res<CursorCapture>,
    mut frame: ResMut<PlayerInputFrame>,
) {
    frame.reset();

    frame.gameplay_active = capture.active() && !focus.gameplay_claimed();
    if !frame.gameplay_active {
        return;
    }

    for action in PlayerAction::ALL {
        if bindings.pressed_raw(action, &keyboard, &mouse) {
            frame.pressed |= action.bit();
        }

        let pointer_edge_allowed =
            !bindings.has_mouse_binding(action) || capture.accepts_gameplay_click();
        if pointer_edge_allowed && bindings.just_pressed_raw(action, &keyboard, &mouse) {
            frame.just_pressed |= action.bit();
        }
    }

    frame.look_delta = mouse_motion.delta;
    frame.scroll_y = scroll.delta.y;
}
