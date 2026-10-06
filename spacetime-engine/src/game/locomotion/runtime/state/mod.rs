//! Regime resolution and runtime collision/motor realization.

use super::super::{
    CollisionPolicy, ControlledSubjectLocomotion, ControlledSubjectLocomotionChanged,
    DetailedBodyScale, LocomotionCapabilities, LocomotionEnabled, LocomotionInhibition,
    LocomotionRegime, LocomotionRegimeOverride, LocomotionRequest, MotionKernel,
    ScaleInteractionProxy, VelocitySemantics,
};
use crate::{
    game::{
        control::LocalControlSubject,
        navigation::{TravelProfile, TravelState},
    },
    physics::{
        DetailedBodyCollision, PhysicalBoxHull,
        character::{CharacterGroundState, CharacterMotor, CharacterMovementInput},
    },
    spatial::{SpatialScale, UsfCanonicalMotion, UsfScaleLayer},
};
use avian3d::prelude::Collider;
use bevy::prelude::*;

mod policy;
mod systems;

pub(in crate::game::locomotion) use systems::{resolve_locomotion_state, sync_locomotion_runtime};
