//! Reference spacecraft built from generic USF/control primitives.
//!
//! The spacecraft is not the player. The player is a semantic constituent while
//! piloting it, and local control authority may transfer back to the body.
//!
//! ## Module map
//!
//! - `boarding`: Enter/exit control transactions and embarked presentation state.
//! - `landing`: Hull-based landing opportunity and resolved placement.
//! - `orbit`: Derived orbital telemetry for spacecraft presentation.
//! - `spawn`: Reference ship semantic creation and initial control transfer.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

use avian3d::prelude::{
    Collider, CustomPositionIntegration, CustomVelocityIntegration, LinearVelocity, RigidBody,
    ShapeCastConfig, SpatialQuery,
};
use bevy::{camera::visibility::RenderLayers, math::DVec3, prelude::*};

use crate::{
    ecs::{
        UsfAuthorityPartitionOf, UsfConstituentOf, UsfEntity, UsfLogicalRealizationOf,
        UsfOwnershipQuery, UsfPresentationProjectionOf,
    },
    game::{
        GameSet,
        control::{
            ControlActionSet, ControlledBy, LocalControlSubject, LocalControlTransferRequest,
        },
        flight::{
            AttitudeAutopilot, FlightCapabilities, FlightContactState, FlightControlCommand,
            FlightControlRequest, FlightLandingOpportunity, FlightSafetyProfile, FlightSafetyState,
            FlightTelemetry, PilotAttitudeLaw, TraversalPolicy,
        },
        locomotion::{
            ControlledSubjectLocomotion, DetailedBodyScale, FlightActuation, FlightControlIntent,
            LocomotionCapabilities, LocomotionEnabled, LocomotionInhibition,
            LocomotionInhibitionReason, LocomotionRegime, LocomotionSet, MotionExecution,
            ScaleInteractionProxy,
        },
        navigation::{
            AdaptiveCruise, ApproachRefinementState, NavigationCapabilities, PrimaryBodyContext,
            TravelAssistanceState, TravelEnvelope, TravelPace, TravelProfile, TravelState,
        },
        player::{Player, PlayerAction, PlayerInputFrame, ViewCameraProfile},
        surface::SurfaceContext,
    },
    physics::{
        PhysicalBoxHull,
        character::{
            CharacterControlFrame, CharacterGroundState, CharacterLocomotionFrame, CharacterMotor,
            CharacterMovementConfig, CharacterMovementIntent, GravityAlignedLocomotionFrame,
        },
        collision_query::UsfCollisionQueryDemand,
        gravity::{GravitySample, RadialGravitySource},
        slice::UsfPhysicsSliceQuery,
        topology::KinematicQueryExclusions,
    },
    portal::{DERIVED_VIEW_LAYER, PortalTraveler},
    spatial::{
        SpatialDemandSet, SpatialDemandSource, SpatialRefinementDemand, SpatialScale,
        UsfCanonicalMotion, UsfInteractionScaleAffinity, UsfLocalScalePresentation, UsfPosition,
        UsfRuntimeChartState, UsfScaleLayer, UsfScaleRoleMask, UsfTravelNeighborhood,
    },
    view::ViewSubjectPresentation,
    voxel::VoxelMaterializationDemand,
};

use crate::game::GameScenario;
use crate::spatial::UsfNavigationContext;

// Shared landing and disembark search reach, expressed in physical metres.
const LANDING_PROBE_METRES: f32 = 2.0;
// Landing/disembark use actual hull shape casts; these constants are
// reach/search policy, not hard-coded standing or terrain offsets.

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Spacecraft;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct SpacecraftManifestation;

mod boarding;
mod landing;
mod orbit;
mod spawn;

use boarding::{handle_ship_entry, handle_ship_exit, sync_embarked_player_visibility};
use landing::{SpacecraftLandingSolution, detect_landing, resolve_landing_actions};
pub use orbit::SpacecraftOrbit;
use orbit::sync_spacecraft_orbit;
use spawn::spawn_reference_spacecraft;

pub struct SpacecraftPlugin;

impl Plugin for SpacecraftPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Spacecraft>()
            .register_type::<SpacecraftManifestation>()
            .register_type::<SpacecraftOrbit>()
            .add_systems(
                Update,
                (
                    spawn_reference_spacecraft,
                    resolve_landing_actions,
                    handle_ship_exit,
                    handle_ship_entry,
                )
                    .chain()
                    .in_set(ControlActionSet::Request)
                    .after(SpatialDemandSet::Collect),
            )
            .add_systems(FixedUpdate, detect_landing.after(LocomotionSet::Motion))
            .add_systems(
                Update,
                (sync_embarked_player_visibility, sync_spacecraft_orbit)
                    .chain()
                    .in_set(GameSet::Presentation),
            );
    }
}
