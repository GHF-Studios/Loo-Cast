//! Local input adapters for the player body.
//!
//! These systems translate devices into components understood by lower-level
//! simulation. They intentionally do not own camera collision, item semantics
//! or movement tuning.

use avian3d::prelude::{Collider, LinearVelocity};
use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

use crate::{
    physics::character::{
        CharacterControlFrame, CharacterDimensions, CharacterGroundState,
        CharacterLocomotionFrame, CharacterMotor, CharacterMovementConfig, CharacterMovementInput,
    },
    spatial::{
        SpatialDemandSource, SpatialScale, UsfApproachRefinement, UsfNavigationContext,
        UsfRadialGravitySource, UsfScaleLayer, UsfSpatialFrame, UsfTravelInfluence,
        UsfTravelInfluenceKind, UsfTravelNeighborhood, UsfViewContext, UsfViewRenderAnchor,
    },
    view::PrimaryViewPresentation,
};

use super::{
    Player, PlayerAdaptiveCruise, PlayerAim, PlayerController, PlayerDead, PlayerNoclip,
    PlayerStance, PlayerThrusters, PlayerTravelMode, PlayerTravelSpeed, PlayerTravelState,
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

mod cruise;
mod modes;
mod movement;
mod navigation;
mod view;

pub(super) use cruise::adaptive_cruise_movement;
pub(super) use modes::{
    sync_locomotion_mode, toggle_adaptive_cruise, toggle_noclip, toggle_spatial_demand,
    toggle_thrusters,
};
pub(super) use movement::{movement, noclip_movement, scale_navigation_movement};
pub(super) use navigation::{
    sync_approach_refinement_view, sync_navigation_context, sync_planetary_gravity,
    sync_travel_state,
};
pub(super) use view::{look, zoom_spatial_view};
