//! Focus-aware mouse capture.
//!
//! - losing window focus always releases the OS cursor;
//! - Escape releases capture while focused;
//! - left-clicking the focused game reacquires capture;
//! - the reacquire click is not also treated as a weapon click.

use bevy::{
    prelude::*,
    window::{
        CursorGrabMode,
        CursorOptions,
        PrimaryWindow,
    },
};

#[derive(Resource, Debug)]
pub struct CursorCapture {
    requested: bool,
    active: bool,
    just_captured: bool,
}

impl Default for CursorCapture {
    fn default() -> Self {
        Self {
            requested: true,
            active: false,
            just_captured: false,
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
        capture.requested = false;
    }

    if mouse.just_pressed(MouseButton::Left)
        && window.focused
        && !capture.requested
    {
        capture.requested = true;
        capture.just_captured = true;
    }

    // Focus always wins over requested capture. Alt-Tab therefore cannot leave
    // the pointer hidden/grabbed in another application.
    capture.active = capture.requested && window.focused;

    cursor.visible = !capture.active;
    cursor.grab_mode = if capture.active {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };
}
