//! Construction of the local player's semantic entity and runtime manifestations.

use super::*;

pub(super) fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player Transform is the physical standing-hull center, not the eye.
    let position = Vec3::new(0.0, CharacterDimensions::HALF_HEIGHT + 0.01, 8.0);

    // Semantic identity carries canonical USF position but no local Transform.
    // The ordinary controlled body and reserved portal peer are spatial
    // manifestations of this one entity; local runtime coordinates remain on
    // those manifestations.
    let semantic_player = commands
        .spawn((
            Name::new("Player Entity"),
            UsfEntity,
            UsfPosition::zero(SpatialScale::MAX)
                .translated_native(position)
                .expect("initial player position must fit the root simulation layer"),
            Health::new(100.0),
            ThermalBody::ambient(8_000.0, 25.0),
            ThermalInjury::human_like(),
        ))
        .id();

    let player = commands
        .spawn((
            (
                Name::new("Player Manifestation"),
                Player,
                UsfManifestationOf(semantic_player),
                UsfManifestationAuthority,
                UsfLogicalProjection,
                UsfSpatialAnchor,
                UsfScaleLayer::new(SpatialScale::MAX),
                UsfFollowsActiveScale,
                ThermalSpatialSample,
                SpatialDemandSource::cuboid(PLAYER_SPATIAL_DEMAND_HALF_EXTENT)
                    .with_priority(PLAYER_SPATIAL_DEMAND_PRIORITY),
                VoxelMaterializationDemand,
                PlayerController::default(),
                PlayerAim::default(),
            ),
            (
                PlayerStance::default(),
                PlayerNoclip::default(),
                CharacterMotor,
                CharacterDimensions::standing_collider(),
                SpatialSplitBox::from_size(Vec3::new(
                    CharacterDimensions::HULL_WIDTH,
                    CharacterDimensions::HULL_HEIGHT,
                    CharacterDimensions::HULL_WIDTH,
                )),
                Weapon::default(),
                PortalTraveler::new(position),
                Transform::from_translation(position),
            ),
        ))
        .id();

    let split_manifestation = commands
        .spawn((
            Name::new("Player Split Manifestation"),
            UsfManifestationOf(semantic_player),
            UsfLogicalProjection,
            UsfScaleLayer::new(SpatialScale::MAX),
            UsfFollowsActiveScale,
            SpatialSplitPeer { authority: player },
            ActiveCollisionHooks::FILTER_PAIRS,
            ThermalSpatialSample,
            RigidBody::Kinematic,
            CustomPositionIntegration,
            CustomVelocityIntegration,
            LinearVelocity::ZERO,
            CharacterDimensions::standing_collider(),
            CollisionLayers::NONE,
            Transform::from_translation(position),
        ))
        .id();

    commands.entity(player).insert((
        PortalSplitTraveler::new(Transform::from_translation(position), split_manifestation),
        KinematicQueryExclusions::from_entities([split_manifestation]),
    ));

    commands.entity(player).with_children(|parent| {
        parent.spawn((
            model::create_model(&mut meshes, &mut materials),
            UsfPresentationProjectionOf(player),
        ));
    });
    commands
        .entity(split_manifestation)
        .with_children(|parent| {
            parent.spawn((
                model::create_model(&mut meshes, &mut materials),
                UsfPresentationProjectionOf(split_manifestation),
            ));
        });

    commands.spawn((
        Name::new("Player Camera"),
        PlayerCamera::default(),
        UsfViewAnchor,
        UsfScaleLayer::new(SpatialScale::MAX),
        UsfFollowsActiveScale,
        PrimaryGameView,
        PortalView,
        Camera3d::default(),
        IsDefaultUiCamera,
        RenderLayers::layer(0).with(MAIN_PORTAL_LAYER),
        Transform::from_translation(position),
    ));
}
