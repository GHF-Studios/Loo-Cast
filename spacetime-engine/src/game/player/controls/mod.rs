//! Human-player controller adapters.
//!
//! This module owns device -> intent/request adaptation only. Navigation,
//! locomotion resolution, collision realization and motion kernels live in
//! their generic domains.

use bevy::prelude::*;

use crate::{
    game::{
        control::{LocalControlSubject, LocalViewTarget},
        flight::PilotAttitudeLaw,
        locomotion::{
            CharacterStance, ControlledSubjectLocomotion, FlightActuation, FlightAttitudeCommand,
            FlightControlIntent, LocomotionCapabilities, LocomotionRegime, MotionExecution,
            MotionKernel,
        },
        navigation::{
            AdaptiveCruise, NavigationPresentationState, TravelAssistance, TravelAssistanceState,
            TravelAssistanceTransitionReason, TravelPace, TravelState,
        },
    },
    physics::character::{
        CharacterControlFrame, CharacterGroundState, CharacterLocomotionFrame,
        CharacterMovementConfig, CharacterMovementInput,
    },
    spatial::{SpatialDemandSource, UsfViewRenderAnchor},
    view::PrimaryViewPresentation,
};

use super::{
    Player, PlayerAim, PlayerController, PlayerDead, ViewCameraProfile,
    input::{PlayerAction, PlayerInputFrame},
};

mod modes;
mod movement;
mod view;

pub(super) use modes::{
    toggle_adaptive_cruise, toggle_attitude_law, toggle_rcs, toggle_spatial_demand,
    toggle_thrusters,
};
pub(super) use movement::{adjust_flight_travel_pace, movement, sample_flight_control_intent};
pub(super) use view::{look, zoom_spatial_view};
