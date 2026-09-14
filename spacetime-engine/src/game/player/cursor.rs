//! Focus-aware mouse capture.

use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::{
    input_focus::InputFocus,
    view::{PrimaryGameView, PrimaryViewPresentation, ViewportSpace},
};

#[derive(Resource, Debug)]
pub struct CursorCapture {
    requested: bool,
    active: bool,
    just_captured: bool,
    blocked: bool,
}

impl Default for CursorCapture {
    fn default() -> Self {
        Self {
            requested: true,
            active: false,
            just_captured: false,
            blocked: false,
        }
    }
}

impl CursorCapture {
    pub fn active(&self) -> bool {
        self.active
    }

    pub fn accepts_gameplay_click(&self) -> bool {
        self.active && !self.just_captured
    }

    pub fn release(&mut self) {
        self.requested = false;
        self.active = false;
    }

    pub fn request(&mut self) {
        if !self.blocked {
            self.requested = true;
        }
    }

    pub fn set_blocked(&mut self, blocked: bool) {
        self.blocked = blocked;

        if blocked {
            self.release();
        }
    }
}

pub fn apply_input_focus(
    focus: Res<InputFocus>,
    mut capture: ResMut<CursorCapture>,
) {
    capture.set_blocked(focus.pointer_claimed() || focus.gameplay_claimed());
}

pub fn update_cursor_capture(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    presentation: Res<PrimaryViewPresentation>,
    window: Single<&Window, With<PrimaryWindow>>,
    game_camera: Single<&Camera, With<PrimaryGameView>>,
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut capture: ResMut<CursorCapture>,
    mut previous_presentation: Local<PrimaryViewPresentation>,
) {
    capture.just_captured = false;

    if *previous_presentation != *presentation {
        if presentation.is_embedded() {
            capture.release();
        } else {
            capture.request();
        }
        *previous_presentation = *presentation;
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        capture.release();
    }

    let pointer_inside_game_view = window.cursor_position().is_some_and(|position| {
        ViewportSpace::new(&game_camera).contains_target_position(position)
    });

    if mouse.just_pressed(MouseButton::Left)
        && window.focused
        && pointer_inside_game_view
        && !capture.requested
        && !capture.blocked
    {
        capture.requested = true;
        capture.just_captured = true;
    }

    capture.active = capture.requested && window.focused && !capture.blocked;

    cursor.visible = !capture.active;

    cursor.grab_mode = if capture.active {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };
}
