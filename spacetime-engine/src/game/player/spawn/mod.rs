//! Construction of the local player's semantic entity and runtime manifestations.

use super::*;

use crate::{
    game::{
        control::{LocalControlSubject, LocalController, LocalViewTarget},
        flight::{
            AttitudeAutopilot, FlightCapabilities, FlightTelemetry, PilotAttitudeLaw,
            TraversalPolicy,
        },
        locomotion::{
            CharacterStance, ControlledSubjectLocomotion, DetailedBodyScale, FlightActuation,
            FlightControlIntent, LocomotionCapabilities, LocomotionEnabled, LocomotionInhibition,
            MotionExecution, ScaleInteractionProxy,
        },
        navigation::{
            AdaptiveCruise, ApproachRefinementState, NavigationCapabilities,
            NavigationPresentationProfile, NavigationPresentationState, PrimaryBodyContext,
            TravelAssistanceState, TravelEnvelope, TravelPace, TravelProfile, TravelState,
        },
        surface::SurfaceContext,
    },
    physics::gravity::GravitySample,
    spatial::{
        UsfCanonicalMotion, UsfInteractionScaleAffinity, UsfNavigationContext, UsfScaleRoleMask,
    },
};

pub(super) fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player Transform is the physical standing-hull center, not the eye.
    let position = Vec3::new(0.0, CharacterDimensions::HALF_HEIGHT + 0.01, 8.0);

    // Semantic metres are independent from the current +35 observer chart.
    // Runtime coordinates are only the bounded projection of this identity.
    let semantic_position = UsfPosition::from_metres_local(position)
        .expect("initial player semantic position must be canonical");
    let runtime_position = semantic_position
        .relative_at_scale_bounded(
            &UsfPosition::zero(SpatialScale::ZERO),
            SpatialScale::MAX,
            1.0,
        )
        .expect("initial player position must project into the root runtime chart");

    let (_, partition) = spawn_semantic_player(&mut commands, semantic_position);
    let player = spawn_player_manifestation(&mut commands, partition, runtime_position);
    let peer = spawn_split_solver_peer(&mut commands, player, runtime_position);
    commands.entity(player).insert((
        PortalSplitTraveler::new(Transform::from_translation(runtime_position), peer),
        KinematicQueryExclusions::from_entities([peer]),
    ));
    let primary_view = spawn_primary_view(&mut commands, runtime_position);
    attach_player_models(
        &mut commands,
        &mut meshes,
        &mut materials,
        player,
        peer,
        primary_view,
    );
    spawn_projection_view(&mut commands, runtime_position);
}

/// Semantic identity and authority partition outlive all runtime manifestations.
fn spawn_semantic_player(
    commands: &mut Commands,
    semantic_position: UsfPosition,
) -> (Entity, Entity) {
    let semantic_player = commands
        .spawn((
            Name::new("Player Entity"),
            UsfEntity,
            semantic_position,
            Health::new(100.0),
            ThermalBody::ambient(8_000.0, 25.0),
            ThermalInjury::human_like(),
            LocalController,
        ))
        .id();

    let player_partition = commands
        .spawn((
            Name::new("Player Authority Partition"),
            UsfAuthorityPartitionOf(semantic_player),
        ))
        .id();

    (semantic_player, player_partition)
}

fn spawn_player_manifestation(
    commands: &mut Commands,
    player_partition: Entity,
    runtime_position: Vec3,
) -> Entity {
    let player = commands
        .spawn((
            (
                Name::new("Player Manifestation"),
                Visibility::Inherited,
                Player,
                UsfLogicalRealizationOf(player_partition),
            ),
            (
                UsfSpatialAnchor,
                UsfViewAnchor,
                UsfScaleLayer::new(SpatialScale::MAX),
                UsfInteractionProjection,
                UsfInteractionScaleAffinity::new(SpatialScale::ZERO)
                    .requiring(UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::COLLISION)),
                ThermalSpatialSample,
                // The parent-module constant is authored in physical metres.
                SpatialDemandSource::cuboid_metres(PLAYER_SPATIAL_DEMAND_HALF_EXTENT)
                    .with_priority(PLAYER_SPATIAL_DEMAND_PRIORITY),
                VoxelMaterializationDemand,
                PlayerController::default(),
                PlayerAim::default(),
            ),
            (LocalControlSubject, LocalViewTarget),
            (
                ViewCameraProfile::character(),
                SpatialRefinementDemand::cuboid_metres(PLAYER_SPATIAL_DEMAND_HALF_EXTENT),
                LocomotionCapabilities::character(),
                FlightCapabilities::default(),
                NavigationCapabilities::default(),
                LocomotionEnabled(true),
                CharacterStance::default(),
                ControlledSubjectLocomotion::default(),
                FlightControlIntent::default(),
                UsfCanonicalMotion::default(),
                ScaleInteractionProxy::default(),
                DetailedBodyScale::default(),
            ),
            (
                TravelPace::default(),
                TravelProfile::character(),
                TravelEnvelope::default(),
                ApproachRefinementState::default(),
                AdaptiveCruise::default(),
            ),
            (
                TravelState::default(),
                TravelAssistanceState::default(),
                MotionExecution::default(),
                FlightActuation::default(),
                PilotAttitudeLaw::Hold,
                AttitudeAutopilot::default(),
                PrimaryBodyContext::default(),
                SurfaceContext::default(),
                UsfTravelNeighborhood::default(),
                UsfNavigationContext::default(),
            ),
            (
                LocomotionInhibition::default(),
                TraversalPolicy::Physical,
                FlightTelemetry::default(),
            ),
            (
                // Controlled-manifestation motion state persists across Scale
                // Slices. CharacterMotor is only one detailed-body solver and
                // must not be the component that implicitly creates the state
                // needed by coarse navigation, cruise or input adapters.
                (GravitySample::default(), GravityAlignedLocomotionFrame),
                CharacterControlFrame::default(),
                CharacterLocomotionFrame::default(),
                CharacterMovementConfig::default(),
                CharacterMovementIntent::default(),
                CharacterGroundState::default(),
                RigidBody::Kinematic,
                CustomPositionIntegration,
                CustomVelocityIntegration,
                LinearVelocity::ZERO,
                CharacterDimensions::standing_hull()
                    .bounding_sphere_collider(SpatialScale::MAX, 0.0),
                CharacterDimensions::standing_hull(),
                Weapon::default(),
                PortalTraveler::new(runtime_position),
                Transform::from_translation(runtime_position),
            ),
        ))
        .id();

    player
}

fn spawn_split_solver_peer(
    commands: &mut Commands,
    player: Entity,
    runtime_position: Vec3,
) -> Entity {
    let split_solver_peer = commands
        .spawn((
            (
                Name::new("Player Portal Split Solver Peer"),
                Visibility::Inherited,
                // Pairwise portal/Avian solver slot only. Generic logical
                // realization ownership is attached temporarily while split.
                UsfScaleLayer::new(SpatialScale::MAX),
                UsfInteractionProjection,
                SpatialSplitPeer { authority: player },
                ActiveCollisionHooks::FILTER_PAIRS,
            ),
            (
                ThermalSpatialSample,
                RigidBody::Kinematic,
                CustomPositionIntegration,
                CustomVelocityIntegration,
                LinearVelocity::ZERO,
                CharacterDimensions::standing_hull()
                    .bounding_sphere_collider(SpatialScale::MAX, 0.0),
                CollisionLayers::NONE,
                Transform::from_translation(runtime_position),
            ),
        ))
        .id();

    split_solver_peer
}

fn spawn_primary_view(commands: &mut Commands, runtime_position: Vec3) -> Entity {
    let primary_view = commands
        .spawn((
            Name::new("Player Local Camera"),
            PlayerCamera::default(),
            PrimaryGameView,
            PortalView,
            UsfPresentationView,
            Camera3d::default(),
            bevy::render::view::NoIndirectDrawing,
            Camera {
                order: 1,
                clear_color: bevy::camera::ClearColorConfig::None,
                ..default()
            },
            IsDefaultUiCamera,
            RenderLayers::layer(0).with(MAIN_PORTAL_LAYER),
            Transform::from_translation(runtime_position),
        ))
        .id();

    primary_view
}

fn attach_player_models(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    player: Entity,
    split_solver_peer: Entity,
    primary_view: Entity,
) {
    commands.entity(player).with_children(|parent| {
        parent.spawn((
            model::create_model(meshes, materials),
            UsfPresentationProjectionOf(player),
            UsfPresentationViewOf(primary_view),
        ));
    });
    commands.entity(split_solver_peer).with_children(|parent| {
        parent.spawn((
            model::create_model(meshes, materials),
            UsfPresentationProjectionOf(split_solver_peer),
            UsfPresentationViewOf(primary_view),
        ));
    });
}

fn spawn_projection_view(commands: &mut Commands, runtime_position: Vec3) {
    commands.spawn((
        Name::new("USF Projection Camera"),
        camera::UsfProjectionCamera,
        UsfPresentationView,
        UsfViewRenderAnchor,
        UsfViewContext::default(),
        NavigationPresentationProfile::default(),
        NavigationPresentationState::default(),
        Camera3d::default(),
        bevy::render::view::NoIndirectDrawing,
        Camera {
            order: 0,
            ..default()
        },
        RenderLayers::layer(crate::view::USF_PRESENTATION_LAYER),
        Transform::from_translation(runtime_position),
    ));
}
