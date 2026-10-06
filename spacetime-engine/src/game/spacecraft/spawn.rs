//! Reference ship semantic creation and initial control transfer.

use super::*;

const SHIP_SIZE: Vec3 = Vec3::new(4.0, 2.0, 8.0);
const SHIP_PROXY_CLEARANCE_METRES: f32 = 0.08;
// Authored in physical metres. Recharting S0 -> S+1 must not turn this into a
// ~960x640x960-metre half-extent.
const SHIP_DEMAND_HALF_EXTENT: Vec3 = Vec3::new(96.0, 64.0, 96.0);
const SHIP_DEMAND_PRIORITY: i32 = 120;

pub(super) fn spawn_reference_spacecraft(
    world: Res<State<GameWorld>>,
    existing_ships: Query<(), With<SpacecraftManifestation>>,
    ownership: UsfOwnershipQuery,
    mut commands: Commands,
    mut control_transfers: MessageWriter<LocalControlTransferRequest>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    body: Single<
        (
            Entity,
            &Transform,
            &UsfScaleLayer,
            &mut Visibility,
            &mut SpatialDemandSource,
            &mut LocomotionEnabled,
            &DetailedBodyScale,
            &SurfaceContext,
        ),
        (With<Player>, With<LocalControlSubject>),
    >,
    semantic_positions: Query<&UsfPosition>,
) {
    if existing_ships.iter().next().is_some() {
        return;
    }

    let (
        body_entity,
        body_transform,
        body_layer,
        mut body_visibility,
        mut body_demand,
        mut body_enabled,
        detailed_body,
        surface,
    ) = body.into_inner();

    // Fixture bootstrap owns the arrival transaction. Do not steal local
    // control or disable its spatial demand until the player's detailed chart
    // and actual local collision realization are ready.
    if *world.get() == GameWorld::CelestialFixture
        && (body_layer.scale() != detailed_body.0 || !surface.collision_ready())
    {
        return;
    }

    let Some(player_semantic) = ownership.semantic_of(body_entity) else {
        return;
    };
    let Ok(&semantic_position) = semantic_positions.get(player_semantic) else {
        return;
    };

    let semantic_ship = commands
        .spawn((
            Name::new("Reference Spacecraft"),
            UsfEntity,
            Spacecraft,
            ControlledBy(player_semantic),
            semantic_position,
        ))
        .id();

    // Control/focus now resolves runtime subjects through the generic USF
    // ownership graph, so the spacecraft must participate before transfer.
    let ship_partition = commands
        .spawn((
            Name::new("Reference Spacecraft Authority Partition"),
            UsfAuthorityPartitionOf(semantic_ship),
        ))
        .id();

    let ship = spawn_ship_manifestation(&mut commands, ship_partition, body_layer, body_transform);
    spawn_ship_model(&mut commands, ship, &mut meshes, &mut materials);

    commands
        .entity(player_semantic)
        .insert(UsfConstituentOf(semantic_ship));
    commands
        .entity(body_entity)
        .remove::<Collider>()
        .remove::<CharacterMotor>();

    body_demand.set_enabled(false);
    body_enabled.0 = false;
    *body_visibility = Visibility::Hidden;

    // Initial piloting is a normal control transaction. Vehicle code never
    // mutates global LocalControlSubject / LocalViewTarget / UsfViewAnchor /
    // UsfInteractionProjection ownership directly.
    control_transfers.write(LocalControlTransferRequest::new(player_semantic, ship));
}

/// Assemble the initial runtime realization. Component groups remain explicit
/// here so identity, movement, physics and travel share the same manifestation.
fn spawn_ship_manifestation(
    commands: &mut Commands,
    ship_partition: Entity,
    body_layer: &UsfScaleLayer,
    body_transform: &Transform,
) -> Entity {
    let mut locomotion = ControlledSubjectLocomotion::default();
    locomotion.request_automatic();
    let mut actuation = FlightActuation::default();
    actuation.set_thrusters_enabled(true);
    actuation.set_rcs_enabled(true);

    // Human body S0 -> small spacecraft S+1. Spawn placement may initially use
    // the player's current chart; control transfer explicitly recharts it.
    let ship_interaction_scale =
        SpatialScale::new(1).expect("reference spacecraft S+1 is a valid USF Scale");

    commands
        .spawn((
            (
                Name::new("Reference Spacecraft Manifestation"),
                SpacecraftManifestation,
                Visibility::Inherited,
                UsfLogicalRealizationOf(ship_partition),
                UsfScaleLayer::new(body_layer.scale()),
                UsfInteractionScaleAffinity::new(ship_interaction_scale)
                    .requiring(UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::COLLISION)),
                SpatialDemandSource::cuboid_metres(SHIP_DEMAND_HALF_EXTENT)
                    .with_priority(SHIP_DEMAND_PRIORITY),
                SpatialRefinementDemand::cuboid_metres(SHIP_DEMAND_HALF_EXTENT),
                VoxelMaterializationDemand,
            ),
            (
                ViewCameraProfile::spacecraft(14.0),
                LocomotionCapabilities::spacecraft(),
                LocomotionEnabled(true),
                (
                    PhysicalBoxHull::from_size_metres(SHIP_SIZE),
                    UsfCollisionQueryDemand::new(2.0, f64::from(SHIP_SIZE.length() * 0.5), 0.25),
                    ScaleInteractionProxy::new(SHIP_PROXY_CLEARANCE_METRES),
                ),
                FlightControlIntent::default(),
                UsfCanonicalMotion::default(),
                locomotion,
                DetailedBodyScale(ship_interaction_scale),
                TravelProfile::spacecraft(),
                TravelEnvelope::default(),
                ApproachRefinementState::default(),
                AdaptiveCruise::default(),
                TravelState::default(),
                PrimaryBodyContext::default(),
                SurfaceContext::default(),
            ),
            (
                UsfTravelNeighborhood::default(),
                (GravitySample::default(), GravityAlignedLocomotionFrame),
                UsfNavigationContext::default(),
                CharacterControlFrame::default(),
                CharacterLocomotionFrame::default(),
                CharacterMovementConfig::default(),
                CharacterMovementInput::default(),
                CharacterGroundState::default(),
                SpacecraftOrbit::default(),
                RigidBody::Kinematic,
                CustomPositionIntegration,
                CustomVelocityIntegration,
                LinearVelocity::ZERO,
                PhysicalBoxHull::from_size_metres(SHIP_SIZE).collider(body_layer.scale()),
            ),
            (
                LocomotionInhibition::default(),
                FlightContactState::default(),
                FlightLandingOpportunity::default(),
                SpacecraftLandingSolution::default(),
                FlightSafetyProfile::spacecraft(),
                FlightSafetyState::default(),
                FlightTelemetry::default(),
                PilotAttitudeLaw::FollowView,
                AttitudeAutopilot::default(),
                TraversalPolicy::Physical,
                TravelAssistanceState::default(),
                MotionExecution::default(),
                actuation,
            ),
            (
                PortalTraveler::new(body_transform.translation),
                Transform::from_translation(body_transform.translation)
                    .with_rotation(body_transform.rotation),
            ),
        ))
        .id()
}

/// Keep the visible model disposable and parented to the ship realization.
fn spawn_ship_model(
    commands: &mut Commands,
    ship: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    commands.entity(ship).with_children(|parent| {
        parent.spawn((
            Name::new("Reference Spacecraft Model"),
            ViewSubjectPresentation,
            UsfPresentationProjectionOf(ship),
            UsfLocalScalePresentation::metres(SpatialScale::MAX),
            Mesh3d(meshes.add(Cuboid::new(SHIP_SIZE.x, SHIP_SIZE.y, SHIP_SIZE.z))),
            MeshMaterial3d(materials.add(Color::srgb(0.68, 0.70, 0.76))),
            Transform::IDENTITY,
            // ViewSubjectPresentation self-visibility is layer-driven. Keep the
            // controlled ship model alive on the derived-view layer just like
            // the player model; presentation sync promotes it to the ordinary
            // world layer when the active camera mode should show the subject.
            RenderLayers::layer(DERIVED_VIEW_LAYER),
            Visibility::Inherited,
        ));
    });
}
