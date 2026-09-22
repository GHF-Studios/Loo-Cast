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
    game::{
        control::LocalControlSubject,
        locomotion::{
            CollisionPolicy, ControlledSubjectHull, ControlledSubjectLocomotion,
            ControlledSubjectLocomotionChanged, DetailedInteractionScale,
            LocomotionCapabilities, LocomotionEnabled, LocomotionRegime,
            LocomotionRequest, MotionKernel, ScaleInteractionProxy, VelocitySemantics,
        },
        navigation::{
            AdaptiveCruise, ApproachRefinementState, TravelEnvelope, TravelPace,
            TravelState,
        },
    },
    physics::{
        character::{
            CharacterControlFrame, CharacterDimensions, CharacterGroundState,
            CharacterLocomotionFrame, CharacterMotor, CharacterMovementConfig,
            CharacterMovementInput,
        },
    },
    spatial::{
        SpatialDemandSource, SpatialRefinementDemand, SpatialScale, UsfApproachRefinement,
        UsfNavigationContext,
        UsfNavigationContextKind, UsfRadialGravitySource, UsfScaleLayer, UsfScaleRoleMask,
        UsfSpatialFrame,
        UsfSpatialTransition, UsfSpatialTransitionQueue, UsfTransitionVelocity,
        UsfTravelInfluence, UsfTravelInfluenceKind, UsfTravelNeighborhood, UsfViewContext,
        UsfViewRenderAnchor,
    },
    view::PrimaryViewPresentation,
};

use super::{
    Player,
    PlayerAim,
    PlayerController, PlayerDead,
    PlayerStance,
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
mod travel;
mod view;

pub(super) use cruise::adaptive_cruise_movement;
pub(super) use modes::{
    resolve_locomotion_state, sync_locomotion_runtime, toggle_adaptive_cruise,
    toggle_local_flight, toggle_local_flight_thrusters, toggle_spatial_demand,
};
pub(super) use movement::{
    inertial_flight_movement, local_flight_movement, movement,
    orbital_flight_movement, scale_navigation_movement,
};
pub(super) use navigation::{
    plan_approach_refinement, request_approach_interaction_handoff,
    sync_approach_presentation, sync_navigation_context, sync_planetary_gravity,
    sync_travel_state,
};
pub(super) use travel::{
    critical_dropout_clearance, local_flight_capture_clearance,
    local_flight_release_clearance, planetary_handoff_clearance,
    planetary_release_clearance, sync_travel_envelope,
};
pub(super) use view::{look, zoom_spatial_view};
