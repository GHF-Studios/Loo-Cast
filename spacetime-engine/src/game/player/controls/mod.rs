//! Local input adapters for the player body.
//!
//! These systems translate devices into components understood by lower-level
//! simulation. They intentionally do not own camera collision, item semantics
//! or movement tuning.

use avian3d::prelude::LinearVelocity;
use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

use crate::{
    physics::character::{
        CharacterControlFrame, CharacterGroundState, CharacterLocomotionFrame, CharacterMotor,
        CharacterMovementInput,
    },
    spatial::{SpatialDemandSource, SpatialScale, UsfViewFrame},
};

use super::{
    Player, PlayerAim, PlayerController, PlayerDead, PlayerNoclip, PlayerStance,
    cursor::CursorCapture,
};

pub(super) fn gameplay_suppressed(
    keyboard: &ButtonInput<KeyCode>,
    capture: &CursorCapture,
) -> bool {
    !capture.active()
        || keyboard.just_pressed(KeyCode::Tab)
        || keyboard.just_pressed(KeyCode::Escape)
}

mod modes;
mod movement;
mod view;

pub(super) use modes::{toggle_noclip, toggle_spatial_demand};
pub(super) use movement::{movement, noclip_movement};
pub(super) use view::{look, zoom_spatial_view};
