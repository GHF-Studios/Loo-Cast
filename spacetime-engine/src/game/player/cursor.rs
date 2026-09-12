//! Focus-aware mouse capture.

use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
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

pub fn update_cursor_capture(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut capture: ResMut<CursorCapture>,
) {
    capture.just_captured = false;

    if keyboard.just_pressed(KeyCode::Escape) {
        capture.release();
    }

    if mouse.just_pressed(MouseButton::Left)
        && window.focused
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
