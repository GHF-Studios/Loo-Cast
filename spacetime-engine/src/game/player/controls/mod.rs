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
        CharacterMovementConfig, CharacterMovementInput,
    },
    spatial::{
        SpatialDemandSource, SpatialScale, UsfScaleLayer, UsfScaleLayerFrames, UsfTravelInfluence,
        UsfTravelInfluenceKind, UsfTravelNeighborhood, UsfViewContext, UsfViewRenderAnchor,
    },
    view::PrimaryViewPresentation,
};

use super::{
    Player, PlayerAdaptiveCruise, PlayerAim, PlayerController, PlayerDead, PlayerNoclip,
    PlayerScaleNavigation, PlayerStance, PlayerTravelSpeed, cursor::CursorCapture,
};

pub(super) fn gameplay_suppressed(
    keyboard: &ButtonInput<KeyCode>,
    capture: &CursorCapture,
) -> bool {
    !capture.active()
        || keyboard.just_pressed(KeyCode::Tab)
        || keyboard.just_pressed(KeyCode::Escape)
}

mod cruise;
mod modes;
mod movement;
mod view;

pub(super) use cruise::adaptive_cruise_movement;
pub(super) use modes::{
    sync_locomotion_mode, toggle_adaptive_cruise, toggle_noclip, toggle_spatial_demand,
};
pub(super) use movement::{movement, noclip_movement, scale_navigation_movement};
pub(super) use view::{look, zoom_spatial_view};
