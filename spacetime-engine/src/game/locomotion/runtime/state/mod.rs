//! Regime resolution and runtime collision/motor realization.
//!
//! ## Module map
//!
//! - `policy`: Resolve regime eligibility, canonical authority, and collision contract.
//! - `systems`: Apply locomotion policy to the controlled ECS subject and its runtime body.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

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
        navigation::{TravelAssistance, TravelAssistanceState},
    },
    physics::{
        DetailedBodyCollision, PhysicalBoxHull,
        character::{CharacterGroundState, CharacterMotor, CharacterMovementIntent},
    },
    spatial::{SpatialScale, UsfCanonicalMotion, UsfMotionAuthority, UsfPosition, UsfScaleLayer},
};
use avian3d::prelude::{Collider, LinearVelocity};
use bevy::prelude::*;

mod policy;
mod systems;

pub(in crate::game::locomotion) use systems::{resolve_locomotion_state, sync_locomotion_runtime};
