//! Construction of the local player's semantic entity and runtime manifestations.

use super::*;

use crate::{
    game::{
        control::{LocalController, LocalControlSubject, LocalViewTarget},
        flight::{FlightTelemetry, TraversalPolicy},
        locomotion::{
            CharacterStance, ControlledSubjectLocomotion, DetailedBodyScale,
            FlightControlIntent, LocomotionCapabilities, LocomotionEnabled,
            LocomotionInhibition, ScaleInteractionProxy,
        },
        navigation::{
            AdaptiveCruise, ApproachRefinementState, NavigationPresentationProfile,
            NavigationPresentationState, PrimaryBodyContext, TravelEnvelope, TravelPace,
            TravelProfile, TravelState,
        },
        surface::SurfaceContext,
    },
    physics::gravity::GravitySample,
    spatial::{UsfCanonicalMotion, UsfNavigationContext},
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
    let semantic_position = UsfPosition::from_scale0_local(position)
        .expect("initial player semantic position must be canonical");
    let runtime_position = semantic_position
        .relative_at_scale_bounded(
            &UsfPosition::zero(SpatialScale::ZERO),
            SpatialScale::MAX,
            1.0,
        )
        .expect("initial player position must project into the root runtime chart");

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

    let player = commands
        .spawn((
            (
                Name::new("Player Manifestation"),
                Visibility::Inherited,
                Player,
                UsfManifestationOf(semantic_player),
                UsfManifestationAuthority,
                UsfLogicalProjection,
                UsfLogicalRealizationOf(player_partition),
            ),
            (
                UsfSpatialAnchor,
                UsfViewAnchor,
                UsfScaleLayer::new(SpatialScale::MAX),
                UsfInteractionProjection,
                ThermalSpatialSample,
                SpatialDemandSource::cuboid(PLAYER_SPATIAL_DEMAND_HALF_EXTENT)
                    .with_priority(PLAYER_SPATIAL_DEMAND_PRIORITY),
                VoxelMaterializationDemand,
                PlayerController::default(),
                PlayerAim::default(),
            ),
            (
                LocalControlSubject,
                LocalViewTarget,
            ),
            (
                ViewCameraProfile::character(),
                SpatialRefinementDemand::cuboid(PLAYER_SPATIAL_DEMAND_HALF_EXTENT),
                LocomotionCapabilities::character(),
                LocomotionEnabled(true),
                CharacterStance::default(),
                ControlledSubjectLocomotion::default(),
                FlightControlIntent::default(),
                UsfCanonicalMotion::default(),
                ScaleInteractionProxy::default(),
                DetailedBodyScale::default(),
                TravelPace::default(),
                TravelProfile::character(),
                TravelEnvelope::default(),
                ApproachRefinementState::default(),
                AdaptiveCruise::default(),
            ),
            (
                TravelState::default(),
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
                (
                    GravitySample::default(),
                    GravityAlignedLocomotionFrame,
                ),
                CharacterControlFrame::default(),
                CharacterLocomotionFrame::default(),
                CharacterMovementConfig::default(),
                CharacterMovementInput::default(),
                CharacterGroundState::default(),
                RigidBody::Kinematic,
                CustomPositionIntegration,
                CustomVelocityIntegration,
                LinearVelocity::ZERO,
                Collider::sphere(ScaleInteractionProxy::DEFAULT_RADIUS_NATIVE),
                CharacterDimensions::standing_hull(),
                Weapon::default(),
                PortalTraveler::new(runtime_position),
                Transform::from_translation(runtime_position),
            ),
        ))
        .id();

    let split_solver_peer = commands
        .spawn((
            (
                Name::new("Player Portal Split Solver Peer"),
                Visibility::Inherited,
                // Pairwise portal/Avian solver slot only. Generic logical
                // realization ownership is attached temporarily while split.
                UsfLogicalProjection,
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
                Collider::sphere(ScaleInteractionProxy::DEFAULT_RADIUS_NATIVE),
                CollisionLayers::NONE,
                Transform::from_translation(runtime_position),
            ),
        ))
        .id();

    commands.entity(player).insert((
        PortalSplitTraveler::new(Transform::from_translation(runtime_position), split_solver_peer),
        KinematicQueryExclusions::from_entities([split_solver_peer]),
    ));

    let primary_view = commands
        .spawn((
            Name::new("Player Local Camera"),
            PlayerCamera::default(),
            PrimaryGameView,
            PortalView,
            UsfPresentationView,
            Camera3d::default(),
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

    commands.entity(player).with_children(|parent| {
        parent.spawn((
            model::create_model(&mut meshes, &mut materials),
            UsfPresentationProjectionOf(player),
            UsfPresentationViewOf(primary_view),
        ));
    });
    commands
        .entity(split_solver_peer)
        .with_children(|parent| {
            parent.spawn((
                model::create_model(&mut meshes, &mut materials),
                UsfPresentationProjectionOf(split_solver_peer),
                UsfPresentationViewOf(primary_view),
            ));
        });

    commands.spawn((
        Name::new("USF Projection Camera"),
        camera::UsfProjectionCamera,
        UsfPresentationView,
        UsfViewRenderAnchor,
        UsfViewContext::default(),
        NavigationPresentationProfile::default(),
        NavigationPresentationState::default(),
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        RenderLayers::layer(crate::view::USF_PRESENTATION_LAYER),
        Transform::from_translation(runtime_position),
    ));
}
