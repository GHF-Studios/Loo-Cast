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

use crate::{
    console::{ConsoleCommandSource, ConsoleTransport},
    input_focus::InputFocus,
};

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

// #56 source-style-runtime-keymap-v1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PlayerInputButton {
    Key(KeyCode),
    Mouse(MouseButton),
}

pub(crate) const BINDABLE_INPUT_NAMES: &[&str] = &[
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o",
    "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3",
    "4", "5", "6", "7", "8", "9", "space", "tab", "enter", "escape", "backquote",
    "lshift", "rshift", "lctrl", "rctrl", "lalt", "ralt", "up", "down", "left", "right",
    "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12",
    "mouse1", "mouse2", "mouse3",
];

pub(crate) const PLAYER_BIND_TARGETS: &[&str] = &[
    "+forward",
    "+back",
    "+moveleft",
    "+moveright",
    "+jump",
    "+duck",
    "+speed",
    "+viewscale",
    "+attack",
    "+attack2",
    "+use",
    "reload",
    "erase_object",
    "toggle_local_flight",
    "toggle_thrusters",
    "toggle_cruise",
    "toggle_spatial_demand",
    "toggle_camera",
    "toggle_creative_menu",
    "slot1",
    "slot2",
    "slot3",
    "slot4",
    "slot5",
    "slot6",
    "slot7",
    "slot8",
    "slot9",
];

impl PlayerInputButton {
    pub(crate) fn parse(raw: &str) -> Option<Self> {
        use PlayerInputButton::{Key, Mouse};
        let value = raw.trim().to_ascii_lowercase();
        Some(match value.as_str() {
            "a" => Key(KeyCode::KeyA),
            "b" => Key(KeyCode::KeyB),
            "c" => Key(KeyCode::KeyC),
            "d" => Key(KeyCode::KeyD),
            "e" => Key(KeyCode::KeyE),
            "f" => Key(KeyCode::KeyF),
            "g" => Key(KeyCode::KeyG),
            "h" => Key(KeyCode::KeyH),
            "i" => Key(KeyCode::KeyI),
            "j" => Key(KeyCode::KeyJ),
            "k" => Key(KeyCode::KeyK),
            "l" => Key(KeyCode::KeyL),
            "m" => Key(KeyCode::KeyM),
            "n" => Key(KeyCode::KeyN),
            "o" => Key(KeyCode::KeyO),
            "p" => Key(KeyCode::KeyP),
            "q" => Key(KeyCode::KeyQ),
            "r" => Key(KeyCode::KeyR),
            "s" => Key(KeyCode::KeyS),
            "t" => Key(KeyCode::KeyT),
            "u" => Key(KeyCode::KeyU),
            "v" => Key(KeyCode::KeyV),
            "w" => Key(KeyCode::KeyW),
            "x" => Key(KeyCode::KeyX),
            "y" => Key(KeyCode::KeyY),
            "z" => Key(KeyCode::KeyZ),
            "0" => Key(KeyCode::Digit0),
            "1" => Key(KeyCode::Digit1),
            "2" => Key(KeyCode::Digit2),
            "3" => Key(KeyCode::Digit3),
            "4" => Key(KeyCode::Digit4),
            "5" => Key(KeyCode::Digit5),
            "6" => Key(KeyCode::Digit6),
            "7" => Key(KeyCode::Digit7),
            "8" => Key(KeyCode::Digit8),
            "9" => Key(KeyCode::Digit9),
            "space" => Key(KeyCode::Space),
            "tab" => Key(KeyCode::Tab),
            "enter" | "return" => Key(KeyCode::Enter),
            "escape" | "esc" => Key(KeyCode::Escape),
            "backquote" | "grave" | "`" => Key(KeyCode::Backquote),
            "lshift" | "shift" => Key(KeyCode::ShiftLeft),
            "rshift" => Key(KeyCode::ShiftRight),
            "lctrl" | "ctrl" => Key(KeyCode::ControlLeft),
            "rctrl" => Key(KeyCode::ControlRight),
            "lalt" | "alt" => Key(KeyCode::AltLeft),
            "ralt" => Key(KeyCode::AltRight),
            "up" => Key(KeyCode::ArrowUp),
            "down" => Key(KeyCode::ArrowDown),
            "left" => Key(KeyCode::ArrowLeft),
            "right" => Key(KeyCode::ArrowRight),
            "f1" => Key(KeyCode::F1),
            "f2" => Key(KeyCode::F2),
            "f3" => Key(KeyCode::F3),
            "f4" => Key(KeyCode::F4),
            "f5" => Key(KeyCode::F5),
            "f6" => Key(KeyCode::F6),
            "f7" => Key(KeyCode::F7),
            "f8" => Key(KeyCode::F8),
            "f9" => Key(KeyCode::F9),
            "f10" => Key(KeyCode::F10),
            "f11" => Key(KeyCode::F11),
            "f12" => Key(KeyCode::F12),
            "mouse1" | "mouseleft" => Mouse(MouseButton::Left),
            "mouse2" | "mouseright" => Mouse(MouseButton::Right),
            "mouse3" | "mousemiddle" => Mouse(MouseButton::Middle),
            _ => return None,
        })
    }

    pub(crate) fn canonical_name(self) -> String {
        match self {
            Self::Mouse(MouseButton::Left) => "mouse1".to_string(),
            Self::Mouse(MouseButton::Right) => "mouse2".to_string(),
            Self::Mouse(MouseButton::Middle) => "mouse3".to_string(),
            Self::Mouse(other) => format!("{other:?}").to_ascii_lowercase(),
            Self::Key(key) => match key {
                KeyCode::KeyA => "a".into(),
                KeyCode::KeyB => "b".into(),
                KeyCode::KeyC => "c".into(),
                KeyCode::KeyD => "d".into(),
                KeyCode::KeyE => "e".into(),
                KeyCode::KeyF => "f".into(),
                KeyCode::KeyG => "g".into(),
                KeyCode::KeyH => "h".into(),
                KeyCode::KeyI => "i".into(),
                KeyCode::KeyJ => "j".into(),
                KeyCode::KeyK => "k".into(),
                KeyCode::KeyL => "l".into(),
                KeyCode::KeyM => "m".into(),
                KeyCode::KeyN => "n".into(),
                KeyCode::KeyO => "o".into(),
                KeyCode::KeyP => "p".into(),
                KeyCode::KeyQ => "q".into(),
                KeyCode::KeyR => "r".into(),
                KeyCode::KeyS => "s".into(),
                KeyCode::KeyT => "t".into(),
                KeyCode::KeyU => "u".into(),
                KeyCode::KeyV => "v".into(),
                KeyCode::KeyW => "w".into(),
                KeyCode::KeyX => "x".into(),
                KeyCode::KeyY => "y".into(),
                KeyCode::KeyZ => "z".into(),
                KeyCode::Digit0 => "0".into(),
                KeyCode::Digit1 => "1".into(),
                KeyCode::Digit2 => "2".into(),
                KeyCode::Digit3 => "3".into(),
                KeyCode::Digit4 => "4".into(),
                KeyCode::Digit5 => "5".into(),
                KeyCode::Digit6 => "6".into(),
                KeyCode::Digit7 => "7".into(),
                KeyCode::Digit8 => "8".into(),
                KeyCode::Digit9 => "9".into(),
                KeyCode::Space => "space".into(),
                KeyCode::Tab => "tab".into(),
                KeyCode::Enter => "enter".into(),
                KeyCode::Escape => "escape".into(),
                KeyCode::Backquote => "backquote".into(),
                KeyCode::ShiftLeft => "lshift".into(),
                KeyCode::ShiftRight => "rshift".into(),
                KeyCode::ControlLeft => "lctrl".into(),
                KeyCode::ControlRight => "rctrl".into(),
                KeyCode::AltLeft => "lalt".into(),
                KeyCode::AltRight => "ralt".into(),
                KeyCode::ArrowUp => "up".into(),
                KeyCode::ArrowDown => "down".into(),
                KeyCode::ArrowLeft => "left".into(),
                KeyCode::ArrowRight => "right".into(),
                KeyCode::F1 => "f1".into(),
                KeyCode::F2 => "f2".into(),
                KeyCode::F3 => "f3".into(),
                KeyCode::F4 => "f4".into(),
                KeyCode::F5 => "f5".into(),
                KeyCode::F6 => "f6".into(),
                KeyCode::F7 => "f7".into(),
                KeyCode::F8 => "f8".into(),
                KeyCode::F9 => "f9".into(),
                KeyCode::F10 => "f10".into(),
                KeyCode::F11 => "f11".into(),
                KeyCode::F12 => "f12".into(),
                other => format!("{other:?}").to_ascii_lowercase(),
            },
        }
    }

    fn pressed(
        self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        match self {
            Self::Key(key) => keyboard.pressed(key),
            Self::Mouse(button) => mouse.pressed(button),
        }
    }

    fn just_pressed(
        self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        match self {
            Self::Key(key) => keyboard.just_pressed(key),
            Self::Mouse(button) => mouse.just_pressed(button),
        }
    }

    fn just_released(
        self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        match self {
            Self::Key(key) => keyboard.just_released(key),
            Self::Mouse(button) => mouse.just_released(button),
        }
    }
}

fn bind_target_actions(target: &str) -> Option<(&'static str, &'static [PlayerAction])> {
    use PlayerAction as A;
    let normalized = target.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "+forward" => Some(("+forward", &[A::MoveForward])),
        "+back" | "+backward" => Some(("+back", &[A::MoveBackward])),
        "+moveleft" | "+left" => Some(("+moveleft", &[A::MoveLeft])),
        "+moveright" | "+right" => Some(("+moveright", &[A::MoveRight])),
        "+jump" => Some(("+jump", &[A::Jump, A::Ascend, A::TakeOff])),
        "+duck" | "+crouch" => Some(("+duck", &[A::Crouch, A::Descend])),
        "+speed" | "+sprint" => Some(("+speed", &[A::Sprint, A::Boost, A::FastModifier])),
        "+viewscale" => Some(("+viewscale", &[A::ViewScaleModifier])),
        "+attack" => Some(("+attack", &[A::ItemPrimary])),
        "+attack2" => Some(("+attack2", &[A::ItemSecondary])),
        "+use" => Some(("+use", &[A::Interact])),
        "reload" => Some(("reload", &[A::ItemReload])),
        "erase_object" => Some(("erase_object", &[A::EraseObject])),
        "toggle_local_flight" => Some(("toggle_local_flight", &[A::ToggleLocalFlight])),
        "toggle_thrusters" => Some(("toggle_thrusters", &[A::ToggleThrusters])),
        "toggle_cruise" => Some(("toggle_cruise", &[A::ToggleCruise])),
        "toggle_spatial_demand" => Some(("toggle_spatial_demand", &[A::ToggleSpatialDemand])),
        "toggle_camera" => Some(("toggle_camera", &[A::ToggleCameraMode])),
        "toggle_creative_menu" => Some(("toggle_creative_menu", &[A::ToggleCreativeMenu])),
        "slot1" => Some(("slot1", &[A::Hotbar1])),
        "slot2" => Some(("slot2", &[A::Hotbar2])),
        "slot3" => Some(("slot3", &[A::Hotbar3])),
        "slot4" => Some(("slot4", &[A::Hotbar4])),
        "slot5" => Some(("slot5", &[A::Hotbar5])),
        "slot6" => Some(("slot6", &[A::Hotbar6])),
        "slot7" => Some(("slot7", &[A::Hotbar7])),
        "slot8" => Some(("slot8", &[A::Hotbar8])),
        "slot9" => Some(("slot9", &[A::Hotbar9])),
        _ => None,
    }
}

#[derive(Resource, Debug, Clone)]
pub(crate) struct PlayerInputBindings {
    bindings: HashMap<PlayerAction, Vec<PlayerInputButton>>,
    console_bindings: HashMap<PlayerInputButton, String>,
    binding_labels: HashMap<PlayerInputButton, String>,
}

impl Default for PlayerInputBindings {
    fn default() -> Self {
        let mut result = Self {
            bindings: HashMap::new(),
            console_bindings: HashMap::new(),
            binding_labels: HashMap::new(),
        };

        for (button, target) in [
            ("w", "+forward"),
            ("s", "+back"),
            ("a", "+moveleft"),
            ("d", "+moveright"),
            ("space", "+jump"),
            ("lctrl", "+duck"),
            ("rctrl", "+duck"),
            ("lshift", "+speed"),
            ("rshift", "+speed"),
            ("v", "toggle_local_flight"),
            ("x", "toggle_thrusters"),
            ("c", "toggle_cruise"),
            ("l", "toggle_spatial_demand"),
            ("f5", "toggle_camera"),
            ("lalt", "+viewscale"),
            ("ralt", "+viewscale"),
            ("mouse1", "+attack"),
            ("mouse2", "+attack2"),
            ("mouse3", "erase_object"),
            ("r", "reload"),
            ("tab", "toggle_creative_menu"),
            ("e", "+use"),
            ("1", "slot1"),
            ("2", "slot2"),
            ("3", "slot3"),
            ("4", "slot4"),
            ("5", "slot5"),
            ("6", "slot6"),
            ("7", "slot7"),
            ("8", "slot8"),
            ("9", "slot9"),
        ] {
            result
                .bind_named(button, target)
                .expect("compiled default input binding must be valid");
        }

        result
    }
}

impl PlayerInputBindings {
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

pub(super) fn dispatch_bound_console_commands(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    bindings: Res<PlayerInputBindings>,
    focus: Res<InputFocus>,
    capture: Res<CursorCapture>,
    transport: Res<ConsoleTransport>,
) {
    if focus.gameplay_claimed() || !capture.active() {
        return;
    }

    for (button, command) in &bindings.console_bindings {
        if button.just_pressed(&keyboard, &mouse) {
            transport.submit(ConsoleCommandSource::Binding, command.clone());
        }
        if button.just_released(&keyboard, &mouse)
            && let Some(release) = command.strip_prefix('+')
        {
            transport.submit(ConsoleCommandSource::Binding, format!("-{release}"));
        }
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
    freecam: Res<super::DebugFreecam>,
    capture: Res<CursorCapture>,
    mut frame: ResMut<PlayerInputFrame>,
) {
    frame.reset();

    frame.gameplay_active = capture.active()
        && !focus.gameplay_claimed()
        && (!freecam.enabled() || !freecam.consumes_gameplay_input());
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

#[cfg(test)]
mod runtime_binding_tests {
    use super::*;

    #[test]
    fn bind_replaces_default_player_action_without_shadow_binding() {
        let mut bindings = PlayerInputBindings::default();
        bindings.bind_named("w", "debug freecam toggle").unwrap();

        assert_eq!(
            bindings.binding_for_name("w").unwrap(),
            Some("debug freecam toggle")
        );
        let w = PlayerInputButton::parse("w").unwrap();
        assert!(!bindings
            .bindings
            .get(&PlayerAction::MoveForward)
            .is_some_and(|values| values.contains(&w)));
    }

    #[test]
    fn grouped_source_style_actions_preserve_contextual_jump_semantics() {
        let bindings = PlayerInputBindings::default();
        assert_eq!(bindings.binding_for_name("space").unwrap(), Some("+jump"));
        let space = PlayerInputButton::parse("space").unwrap();
        assert!(bindings
            .bindings
            .get(&PlayerAction::Jump)
            .is_some_and(|values| values.contains(&space)));
        assert!(bindings
            .bindings
            .get(&PlayerAction::Ascend)
            .is_some_and(|values| values.contains(&space)));
        assert!(bindings
            .bindings
            .get(&PlayerAction::TakeOff)
            .is_some_and(|values| values.contains(&space)));
    }
}
