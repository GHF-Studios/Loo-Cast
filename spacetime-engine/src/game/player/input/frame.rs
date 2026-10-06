//! One gameplay input snapshot sampled after focus and cursor arbitration.

use super::super::cursor::CursorCapture;
use super::{PlayerAction, bindings::PlayerInputBindings};
use crate::input_focus::InputFocus;
use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

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

pub(in crate::game::player) fn sample_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    bindings: Res<PlayerInputBindings>,
    focus: Res<InputFocus>,
    freecam: Res<super::super::DebugFreecam>,
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
