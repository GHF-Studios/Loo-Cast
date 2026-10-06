//! Generic controlled-subject locomotion.
//!
//! This domain knows nothing about `Player` identity. Humans, spacecraft,
//! vehicles, NPCs or modded subjects can expose capabilities and participate in
//! the same request -> resolve -> one authoritative motion-kernel contract.

use bevy::{app::RunFixedMainLoop, prelude::*};

use crate::{
    physics::character::CharacterMovementSet,
    spatial::{SpatialScale, UsfSpatialSet},
};

mod runtime;

mod capability;
mod flight;
mod state;

pub use capability::{
    CharacterStance, DetailedBodyScale, LocomotionCapabilities, LocomotionEnabled,
    LocomotionInhibition, LocomotionInhibitionReason, ScaleInteractionProxy,
};
pub use flight::{FlightAttitudeCommand, FlightControlIntent};
pub use state::{
    CollisionPolicy, ControlledSubjectLocomotion, ControlledSubjectLocomotionChanged,
    LocomotionRegime, LocomotionRegimeOverride, LocomotionRequest, LocomotionTransitionReason,
    MotionKernel, VelocitySemantics,
};

/// Stable generic locomotion runtime extension points.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocomotionSet {
    Resolve,
    Realize,
    Motion,
}

pub struct LocomotionPlugin;

impl Plugin for LocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LocomotionRegime>()
            .register_type::<LocomotionRequest>()
            .register_type::<LocomotionRegimeOverride>()
            .register_type::<LocomotionTransitionReason>()
            .register_type::<MotionKernel>()
            .register_type::<CollisionPolicy>()
            .register_type::<VelocitySemantics>()
            .register_type::<ControlledSubjectLocomotion>()
            .register_type::<LocomotionCapabilities>()
            .register_type::<LocomotionInhibitionReason>()
            .register_type::<LocomotionInhibition>()
            .register_type::<LocomotionEnabled>()
            .register_type::<CharacterStance>()
            .register_type::<FlightAttitudeCommand>()
            .register_type::<FlightControlIntent>()
            .register_type::<ScaleInteractionProxy>()
            .register_type::<DetailedBodyScale>()
            .add_message::<ControlledSubjectLocomotionChanged>()
            .add_systems(
                RunFixedMainLoop,
                runtime::resolve_locomotion_state.in_set(LocomotionSet::Resolve),
            )
            .add_systems(
                RunFixedMainLoop,
                runtime::sync_locomotion_runtime.in_set(LocomotionSet::Realize),
            )
            .add_systems(
                FixedUpdate,
                runtime::flight_movement
                    .in_set(LocomotionSet::Motion)
                    .after(CharacterMovementSet::Simulate),
            )
            .add_systems(
                PostUpdate,
                (
                    runtime::resolve_locomotion_state,
                    runtime::sync_locomotion_runtime,
                )
                    .chain()
                    .after(UsfSpatialSet::SyncSemantic),
            );
    }
}
