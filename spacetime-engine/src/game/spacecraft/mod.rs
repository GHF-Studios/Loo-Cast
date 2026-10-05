//! Reference spacecraft built from generic USF/control primitives.
//!
//! The spacecraft is not the player. The player is a semantic constituent while
//! piloting it, and local control authority may transfer back to the body.

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
            FlightContactState, FlightLandingOpportunity, FlightSafetyProfile, FlightSafetyState,
            FlightTelemetry, TraversalPolicy,
        },
        locomotion::{
            ControlledSubjectLocomotion, DetailedBodyScale, FlightControlIntent,
            LocomotionCapabilities, LocomotionEnabled, LocomotionInhibition,
            LocomotionInhibitionReason, LocomotionRegime, LocomotionSet, ScaleInteractionProxy,
        },
        navigation::{
            AdaptiveCruise, ApproachRefinementState, PrimaryBodyContext, TravelEnvelope,
            TravelPace, TravelProfile, TravelState,
        },
        player::{Player, PlayerAction, PlayerInputFrame, ViewCameraProfile},
        surface::SurfaceContext,
    },
    physics::{
        PhysicalBoxHull,
        character::{
            CharacterControlFrame, CharacterGroundState, CharacterLocomotionFrame, CharacterMotor,
            CharacterMovementConfig, CharacterMovementInput, GravityAlignedLocomotionFrame,
        },
        collision_query::UsfCollisionQueryDemand,
        gravity::{GravitySample, RadialGravitySource},
        slice::UsfPhysicsSlices,
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

use crate::game::GameWorld;
use crate::spatial::UsfNavigationContext;

const SHIP_SIZE: Vec3 = Vec3::new(4.0, 2.0, 8.0);
const SHIP_PROXY_CLEARANCE_METRES: f32 = 0.08;
// Authored in physical metres. Recharting S0 -> S+1 must not turn this into a
// ~960x640x960-metre half-extent.
const SHIP_DEMAND_HALF_EXTENT: Vec3 = Vec3::new(96.0, 64.0, 96.0);
const SHIP_DEMAND_PRIORITY: i32 = 120;
const LANDING_PROBE_METRES: f32 = 2.0;
const LANDING_PROBE_LIFT_METRES: f32 = LANDING_PROBE_METRES;
const LANDING_SEPARATION_SPEED_EPSILON_METRES_PER_SECOND: f64 = 0.25;
const ENTER_DISTANCE_METRES: f32 = 12.0;
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

use boarding::{
    enforce_embarked_player_hidden, handle_landing_actions, handle_ship_entry, handle_ship_exit,
};
use landing::{SpacecraftLandingSolution, detect_landing};
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
                    handle_landing_actions,
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
                (enforce_embarked_player_hidden, sync_spacecraft_orbit)
                    .chain()
                    .in_set(GameSet::Presentation),
            );
    }
}
