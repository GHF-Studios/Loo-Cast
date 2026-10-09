//! Generic controlled-subject locomotion.
//!
//! This domain knows nothing about `Player` identity. Humans, spacecraft,
//! vehicles, NPCs or modded subjects can expose capabilities and participate in
//! the same request -> resolve -> one authoritative motion-kernel contract.
//!
//! ## Integration
//!
//! Capabilities and requests select a regime, policy resolves the physical contract, and
//! MotionExecution selects one motion kernel. Player and spacecraft code adapt into this subject-
//! owned facility.
//!
//! ## Module map
//!
//! - `runtime`: Controlled-subject locomotion runtime.
//! - `capability`: Runtime capability, inhibition, and collision-representation policy.
//! - `execution`: Resolved physical executor and collision handoff for a controlled subject.
//! - `flight`: Device-independent flight control intent.
//! - `state`: Controlled-subject locomotion request and resolved motion state.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

use bevy::{app::RunFixedMainLoop, prelude::*};

use crate::{
    physics::character::CharacterMovementSet,
    spatial::{SpatialScale, UsfSpatialSet},
};

mod runtime;

mod capability;
mod execution;
mod flight;
mod state;

pub use capability::{
    CharacterStance, DetailedBodyScale, LocomotionCapabilities, LocomotionEnabled,
    LocomotionInhibition, LocomotionInhibitionReason, ScaleInteractionProxy,
};
pub use execution::{
    CollisionPolicy, DeveloperMotionOverride, MotionAuthorityReason, MotionExecution, MotionKernel,
    VelocitySemantics,
};
pub use flight::{FlightActuation, FlightAttitudeCommand, FlightControlIntent, FlightThrottle};
pub use state::{
    ControlledSubjectLocomotion, ControlledSubjectLocomotionChanged, LocomotionRegime,
    LocomotionRegimeOverride, LocomotionRequest, LocomotionTransitionReason, MotionHandoffSnapshot,
};

/// Stable generic locomotion runtime extension points.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocomotionSet {
    Resolve,
    Realize,
    Prepare,
    Motion,
}

pub struct LocomotionPlugin;

impl Plugin for LocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LocomotionRegime>()
            .init_resource::<runtime::PreparedFlightMotion>()
            .register_type::<LocomotionRequest>()
            .register_type::<LocomotionRegimeOverride>()
            .register_type::<LocomotionTransitionReason>()
            .register_type::<MotionKernel>()
            .register_type::<MotionExecution>()
            .register_type::<MotionAuthorityReason>()
            .register_type::<DeveloperMotionOverride>()
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
            .register_type::<FlightThrottle>()
            .register_type::<FlightActuation>()
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
                runtime::prepare_flight_movement
                    .in_set(LocomotionSet::Prepare)
                    .after(CharacterMovementSet::Simulate),
            )
            .add_systems(
                FixedUpdate,
                runtime::flight_movement
                    .in_set(LocomotionSet::Motion)
                    .after(crate::physics::collision_query::UsfCollisionQuerySet::Finalize),
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
