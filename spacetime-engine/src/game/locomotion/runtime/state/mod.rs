//! Regime resolution and runtime collision/motor realization.

use super::super::{
    CollisionPolicy, ControlledSubjectLocomotion, ControlledSubjectLocomotionChanged,
    DetailedBodyScale, DeveloperMotionOverride, FlightControlIntent, LocomotionCapabilities,
    LocomotionEnabled, LocomotionInhibition, LocomotionRegime, LocomotionRegimeOverride,
    LocomotionRequest, LocomotionTransitionReason, MotionExecution, MotionHandoffSnapshot,
    MotionKernel, ScaleInteractionProxy, VelocitySemantics,
};
use crate::{
    ecs::UsfOwnershipQuery,
    game::{
        control::LocalControlSubject,
        navigation::{
            TravelAssistance, TravelAssistanceState, TravelAssistanceTransitionReason, TravelState,
        },
    },
    physics::{
        DetailedBodyCollision, PhysicalBoxHull,
        character::{CharacterGroundState, CharacterMotor, CharacterMovementInput},
    },
    spatial::{SpatialScale, UsfCanonicalMotion, UsfPosition, UsfScaleLayer},
};
use avian3d::prelude::{Collider, LinearVelocity};
use bevy::prelude::*;

mod policy;
mod systems;

pub(in crate::game::locomotion) use systems::{resolve_locomotion_state, sync_locomotion_runtime};
