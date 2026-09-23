//! Human-player controller adapters.
//!
//! This module owns device -> intent/request adaptation only. Navigation,
//! locomotion resolution, collision realization and motion kernels live in
//! their generic domains.

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

use crate::{
    game::{
        control::{LocalControlSubject, LocalViewTarget},
        locomotion::{
            CharacterStance, ControlledSubjectLocomotion, DetailedInteractionScale,
            FlightAttitudeCommand, FlightControlIntent, LocomotionRegime,
            LocomotionRequest, MotionKernel,
        },
        navigation::{
            AdaptiveCruise, NavigationPresentationState, TravelPace, TravelState,
        },
    },
    physics::character::{
        CharacterControlFrame, CharacterGroundState, CharacterLocomotionFrame,
        CharacterMovementConfig, CharacterMovementInput,
    },
    spatial::{
        SpatialDemandSource, UsfScaleLayer, UsfViewRenderAnchor,
    },
    view::PrimaryViewPresentation,
};

use super::{
    Player, PlayerAim, PlayerController, PlayerDead, ViewCameraProfile,
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

pub(super) use modes::{
    toggle_adaptive_cruise, toggle_local_flight, toggle_local_flight_thrusters,
    toggle_spatial_demand,
};
pub(super) use movement::{movement, sample_flight_control_intent};
pub(super) use view::{look, zoom_spatial_view};
