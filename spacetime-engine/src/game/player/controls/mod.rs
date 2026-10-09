//! Human-player controller adapters.
//!
//! This module owns device -> intent/request adaptation only. Navigation,
//! locomotion resolution, collision realization and motion kernels live in
//! their generic domains.
//!
//! ## Module map
//!
//! - `modes`: Human input -> controlled-subject locomotion requests.
//! - `movement`: Human device input -> generic controlled-subject intent.
//! - `view`: Look and observer-scale input adapters.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use bevy::prelude::*;

use crate::{
    game::{
        control::LocalControlSubject,
        flight::{
            FlightCapabilities, FlightControlCommand, FlightControlRequest, PilotAttitudeLaw,
        },
        locomotion::{
            CharacterStance, ControlledSubjectLocomotion, DeveloperMotionOverride, FlightActuation,
            FlightAttitudeCommand, FlightControlIntent, LocomotionRegime, MotionExecution,
            MotionKernel,
        },
        navigation::{
            NavigationPresentationState, TravelAssistance, TravelAssistanceRequest,
            TravelAssistanceState, TravelPace,
        },
    },
    physics::character::{
        CharacterControlFrame, CharacterLocomotionFrame, CharacterMovementConfig,
        CharacterMovementIntent,
    },
    spatial::{SpatialDemandSource, UsfViewRenderAnchor},
    view::PrimaryViewPresentation,
};

use super::{
    CameraMode, Player, PlayerAim, PlayerCamera, PlayerController, PlayerDead,
    input::{PlayerAction, PlayerInputFrame},
};

mod modes;
mod movement;
mod view;

pub(super) use modes::{
    toggle_adaptive_cruise, toggle_attitude_law, toggle_flight_stabilizers, toggle_spatial_demand,
    toggle_thrusters,
};
pub(super) use movement::{
    adjust_ship_scroll_controls, sample_flight_control_intent, write_character_movement_intent,
};
pub(super) use view::{adjust_view_scale_bias, write_player_view_intent};
