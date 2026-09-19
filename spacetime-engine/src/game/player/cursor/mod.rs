//! Focus-aware mouse capture.
//!
//! `requested` is the player's capture intent. `blocked` is a temporary claim
//! made by another interface surface. Keeping those separate means modal UI can
//! borrow the pointer without destroying the intent it should restore afterward.

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
    just_unblocked: bool,
    observed_gameplay_resume_epoch: u64,
}

impl Default for CursorCapture {
    fn default() -> Self {
        Self {
            requested: true,
            active: false,
            just_captured: false,
            blocked: false,
            just_unblocked: false,
            observed_gameplay_resume_epoch: 0,
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

    /// Requests gameplay capture as persistent user intent.
    ///
    /// A temporary [`InputFocus`] claim may prevent the request from becoming
    /// active immediately, but it must not erase the request itself.
    pub fn request(&mut self) {
        self.requested = true;
    }

    pub fn set_blocked(&mut self, blocked: bool) {
        self.just_unblocked = self.blocked && !blocked;
        self.blocked = blocked;

        if blocked {
            self.active = false;
        }
    }
}

pub fn apply_input_focus(focus: Res<InputFocus>, mut capture: ResMut<CursorCapture>) {
    let resume_epoch = focus.gameplay_resume_epoch();
    if capture.observed_gameplay_resume_epoch != resume_epoch {
        capture.observed_gameplay_resume_epoch = resume_epoch;
        capture.request();
    }

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
            // Entering the editor is an explicit mode transition, not a
            // temporary focus claim: leave gameplay capture off. Escape is the
            // explicit gameplay/editor pointer toggle while embedded.
            capture.release();
        } else {
            capture.request();
        }
        *previous_presentation = *presentation;
    }

    // Escape is the direct capture toggle. If Escape just dismissed a modal UI
    // (for example the F4 palette), that UI already returned its temporary focus
    // claim this frame; preserve the prior capture request instead of toggling it
    // a second time.
    if keyboard.just_pressed(KeyCode::Escape) && !capture.just_unblocked {
        if capture.requested {
            capture.release();
        } else if !capture.blocked {
            capture.request();
            capture.just_captured = true;
        }
    }

    let pointer_inside_game_view = window.cursor_position().is_some_and(|position| {
        ViewportSpace::new(&game_camera).contains_target_position(position)
    });

    if !presentation.is_embedded()
        && mouse.just_pressed(MouseButton::Left)
        && window.focused
        && pointer_inside_game_view
        && !capture.requested
        && !capture.blocked
    {
        capture.request();
        capture.just_captured = true;
    }

    capture.active = capture.requested && window.focused && !capture.blocked;

    cursor.visible = !capture.active;
    cursor.grab_mode = if capture.active {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };

    capture.just_unblocked = false;
}
