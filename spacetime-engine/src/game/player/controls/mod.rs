//! Local input adapters for the player body.
//!
//! Device input requests locomotion state. One resolver then selects the active
//! motion kernel/collision policy; movement systems never infer authority from
//! independent mode flags.

use avian3d::prelude::{Collider, LinearVelocity};
use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

use crate::{
    ecs::UsfManifestationOf,
    physics::{
        character::{
            CharacterControlFrame, CharacterDimensions, CharacterGroundState,
            CharacterLocomotionFrame, CharacterMotor, CharacterMovementConfig,
            CharacterMovementInput,
        },
    },
    spatial::{
        SpatialDemandSource, SpatialScale, UsfApproachRefinement, UsfNavigationContext,
        UsfRadialGravitySource, UsfScaleLayer, UsfScaleRoleMask, UsfSpatialFrame,
        UsfSpatialTransition, UsfSpatialTransitionQueue, UsfTransitionVelocity,
        UsfTravelInfluence, UsfTravelInfluenceKind, UsfTravelNeighborhood, UsfViewContext,
        UsfViewRenderAnchor,
    },
    view::PrimaryViewPresentation,
};

use super::{
    ControlledSubjectLocomotion, ControlledSubjectLocomotionChanged, Player,
    PlayerAdaptiveCruise, PlayerAim, PlayerApproachRefinementState, PlayerCollisionPolicy,
    PlayerController, PlayerDead,
    PlayerDetailedPhysicsScale, PlayerLocomotionRegime, PlayerLocomotionRequest,
    PlayerMotionKernel, PlayerScaleInteractionProxy, PlayerStance, PlayerTravelSpeed,
    PlayerTravelState, PlayerVelocitySemantics,
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
    resolve_locomotion_state, sync_locomotion_runtime, toggle_adaptive_cruise,
    toggle_local_flight, toggle_local_flight_thrusters, toggle_spatial_demand,
};
pub(super) use movement::{local_flight_movement, movement, scale_navigation_movement};
pub(super) use navigation::{
    sync_approach_refinement, sync_navigation_context, sync_planetary_gravity,
    sync_travel_state,
};
pub(super) use view::{look, zoom_spatial_view};
