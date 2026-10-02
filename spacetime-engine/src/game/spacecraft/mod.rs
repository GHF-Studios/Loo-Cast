//! Reference spacecraft built from generic USF/control primitives.
//!
//! The spacecraft is not the player. The player is a semantic constituent while
//! piloting it, and local control authority may transfer back to the body.

use avian3d::prelude::{
    Collider, CustomPositionIntegration, CustomVelocityIntegration, LinearVelocity,
    RigidBody, ShapeCastConfig, SpatialQuery,
};
use bevy::{math::DVec3, prelude::*};

use crate::{
    ecs::{
        UsfAuthorityPartitionOf, UsfConstituentOf, UsfEntity, 
        UsfLogicalRealizationOf, 
        UsfOwnershipQuery, UsfPresentationProjectionOf,
    },
    game::{
        GameSet,
        flight::{
            FlightContactState, FlightLandingOpportunity, FlightSafetyProfile,
            FlightSafetyState, FlightTelemetry, TraversalPolicy,
        },
        control::{
            ControlActionSet, ControlledBy, LocalControlSubject, LocalControlTransferRequest,
        },
        locomotion::{
            ControlledSubjectLocomotion, DetailedBodyScale, FlightControlIntent,
            LocomotionCapabilities, LocomotionEnabled,
            LocomotionInhibition, LocomotionInhibitionReason, LocomotionRegime, LocomotionSet,
            ScaleInteractionProxy,
        },
        navigation::{
            AdaptiveCruise, ApproachRefinementState, PrimaryBodyContext, TravelEnvelope,
            TravelProfile, TravelState,
        },
        player::{Player, PlayerAction, PlayerInputFrame, ViewCameraProfile},
        surface::SurfaceContext,
    },
    physics::{
        PhysicalBoxHull,
        collision_query::UsfCollisionQueryDemand,
        slice::UsfPhysicsSlices,
        gravity::{GravitySample, RadialGravitySource},
        character::{
            CharacterControlFrame, CharacterGroundState, CharacterLocomotionFrame,
            CharacterMovementConfig, CharacterMovementInput, CharacterMotor,
            GravityAlignedLocomotionFrame,
        },
        topology::KinematicQueryExclusions,
    },
    portal::PortalTraveler,
    spatial::{
        SpatialDemandSet, SpatialDemandSource, SpatialRefinementDemand, SpatialScale,
        UsfCanonicalMotion, UsfLocalScalePresentation, UsfPosition, UsfScaleLayer,
        UsfSpatialFrame, UsfTravelNeighborhood,
    },
    view::ViewSubjectPresentation,
    voxel::VoxelMaterializationDemand,
};

use crate::spatial::UsfNavigationContext;
use crate::game::GameWorld;

const SHIP_SIZE: Vec3 = Vec3::new(4.0, 2.0, 8.0);
const SHIP_PROXY_CLEARANCE_METRES: f32 = 0.08;
const SHIP_DEMAND_HALF_EXTENT: Vec3 = Vec3::new(96.0, 64.0, 96.0);
const SHIP_DEMAND_PRIORITY: i32 = 120;
const LANDING_PROBE_METRES: f32 = 2.0;
const LANDING_PROBE_LIFT_METRES: f32 = LANDING_PROBE_METRES;
const LANDING_SEPARATION_SPEED_EPSILON_METRES_PER_SECOND: f64 = 0.25;
const ENTER_DISTANCE_METRES: f32 = 12.0;
// spacecraft-contact-handoff-v1
// Landing/disembark use actual hull shape casts; these constants are
// reach/search policy, not hard-coded standing or terrain offsets.

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Spacecraft;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct SpacecraftManifestation;

#[derive(Component, Debug, Default, Clone, Copy)]
struct SpacecraftLandingSolution {
    resolved: Option<(SpatialScale, Vec3, Quat)>,
}

impl SpacecraftLandingSolution {
    fn clear(&mut self) { self.resolved = None; }
    fn set(&mut self, scale: SpatialScale, translation: Vec3, rotation: Quat) {
        self.resolved = Some((scale, translation, rotation));
    }
    fn at_scale(self, scale: SpatialScale) -> Option<(Vec3, Quat)> {
        let (s, t, r) = self.resolved?;
        (s == scale).then_some((t, r))
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct SpacecraftOrbit {
    pub valid: bool,
    pub bound: bool,
    pub altitude_metres: f64,
    pub speed_metres_per_second: f64,
    pub semi_major_axis_metres: f64,
    pub eccentricity: f64,
    pub inclination_radians: f64,
    pub longitude_ascending_node_radians: f64,
    pub argument_periapsis_radians: f64,
    pub true_anomaly_radians: f64,
    pub periapsis_altitude_metres: f64,
    pub apoapsis_altitude_metres: f64,
}

impl Default for SpacecraftOrbit {
    fn default() -> Self {
        Self {
            valid: false,
            bound: false,
            altitude_metres: 0.0,
            speed_metres_per_second: 0.0,
            semi_major_axis_metres: f64::INFINITY,
            eccentricity: 0.0,
            inclination_radians: 0.0,
            longitude_ascending_node_radians: 0.0,
            argument_periapsis_radians: 0.0,
            true_anomaly_radians: 0.0,
            periapsis_altitude_metres: f64::INFINITY,
            apoapsis_altitude_metres: f64::INFINITY,
        }
    }
}

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
                    handle_spacecraft_actions,
                )
                    .chain()
                    .in_set(ControlActionSet::Request)
                    .after(SpatialDemandSet::Collect),
            )
            .add_systems(
                FixedUpdate,
                detect_landing.after(LocomotionSet::Motion),
            )
            .add_systems(
                Update,
                sync_spacecraft_orbit.in_set(GameSet::Presentation),
            );
    }
}

fn spawn_reference_spacecraft(
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

    let mut locomotion = ControlledSubjectLocomotion::default();
    locomotion.request_automatic();
    locomotion.set_thrusters_enabled(true);
    locomotion.set_rcs_enabled(true);

    let ship = commands
        .spawn((
            (
                Name::new("Reference Spacecraft Manifestation"),
                SpacecraftManifestation,
                Visibility::Inherited,
                
                
                UsfLogicalRealizationOf(ship_partition),
                UsfScaleLayer::new(body_layer.scale()),
                SpatialDemandSource::cuboid(SHIP_DEMAND_HALF_EXTENT)
                    .with_priority(SHIP_DEMAND_PRIORITY),
                SpatialRefinementDemand::cuboid(SHIP_DEMAND_HALF_EXTENT),
                VoxelMaterializationDemand,
            ),
            (
                ViewCameraProfile::spacecraft(14.0),
                LocomotionCapabilities::spacecraft(),
                LocomotionEnabled(true),
                (
                    PhysicalBoxHull::from_size_metres(SHIP_SIZE),
                    UsfCollisionQueryDemand::new(
                        2.0,
                        f64::from(SHIP_SIZE.length() * 0.5),
                        0.25,
                    ),
                    ScaleInteractionProxy::new(SHIP_PROXY_CLEARANCE_METRES),
                ),
                FlightControlIntent::default(),
                UsfCanonicalMotion::default(),
                locomotion,
                DetailedBodyScale::default(),
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
                (
                    GravitySample::default(),
                    GravityAlignedLocomotionFrame,
                ),
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
                PhysicalBoxHull::from_size_metres(SHIP_SIZE)
                    .collider(body_layer.scale()),
            ),
            (
                LocomotionInhibition::default(),
                FlightContactState::default(),
                FlightLandingOpportunity::default(),
                SpacecraftLandingSolution::default(),
                FlightSafetyProfile::spacecraft(),
                FlightSafetyState::default(),
                FlightTelemetry::default(),
                TraversalPolicy::Physical,
            ),
            (
                PortalTraveler::new(body_transform.translation),
                Transform::from_translation(body_transform.translation)
                    .with_rotation(body_transform.rotation),
            ),
        ))
        .id();

    commands.entity(ship).with_children(|parent| {
        parent.spawn((
            Name::new("Reference Spacecraft Model"),
            ViewSubjectPresentation,
            UsfPresentationProjectionOf(ship),
            UsfLocalScalePresentation::metres(SpatialScale::MAX),
            Mesh3d(meshes.add(Cuboid::new(SHIP_SIZE.x, SHIP_SIZE.y, SHIP_SIZE.z))),
            MeshMaterial3d(materials.add(Color::srgb(0.68, 0.70, 0.76))),
            Transform::IDENTITY,
            Visibility::Inherited,
        ));
    });

    commands.entity(player_semantic).insert(UsfConstituentOf(semantic_ship));
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
    control_transfers.write(LocalControlTransferRequest::new(
        player_semantic,
        ship,
    ));
}

pub(crate) fn detect_landing(
    spatial_query: SpatialQuery,
    physics_charts: UsfPhysicsSlices,
    mut ships: Query<
        (
            Entity,
            &Transform,
            &UsfScaleLayer,
            &DetailedBodyScale,
            &CharacterLocomotionFrame,
            &CharacterMovementConfig,
            &Collider,
            Option<&KinematicQueryExclusions>,
            &UsfCanonicalMotion,
            &ControlledSubjectLocomotion,
            &FlightSafetyProfile,
            &FlightContactState,
            &mut FlightLandingOpportunity,
            &mut SpacecraftLandingSolution,
        ),
        (With<SpacecraftManifestation>, With<LocalControlSubject>, Without<Player>),
    >,
) {
    let Ok((entity, transform, layer, detailed, locomotion_frame, movement,
        collider, exclusions, motion, locomotion, safety, contact,
        mut opportunity, mut solution)) = ships.single_mut() else { return; };

    opportunity.set_available(false);
    solution.clear();

    if contact.is_landed()
        || layer.scale() != detailed.0
        || locomotion.regime() != LocomotionRegime::LocalFlight
        || motion.speed_metres_per_second() > safety.preferred_contact_speed_metres_per_second()
    { return; }

    let up = locomotion_frame.up();
    let up_si = DVec3::new(f64::from(up.x), f64::from(up.y), f64::from(up.z));
    if motion.velocity_metres_per_second().dot(up_si)
        > LANDING_SEPARATION_SPEED_EPSILON_METRES_PER_SECOND
    { return; }

    let Ok(direction) = Dir3::new(-up) else { return; };
    let lift = layer.scale().metres_to_native_f32(LANDING_PROBE_LIFT_METRES);
    let probe_origin = transform.translation + up * lift;
    let max_distance = layer.scale().metres_to_native_f32(
        LANDING_PROBE_LIFT_METRES + LANDING_PROBE_METRES,
    );
    let filter = physics_charts.filter_for_scale(
        layer.scale(),
        std::iter::once(entity).chain(exclusions.into_iter().flat_map(|items| items.iter())),
    );
    let config = ShapeCastConfig { max_distance, ignore_origin_penetration: false, ..default() };
    let aligned = locomotion_frame.aligned_rotation(transform.rotation);
    let Some(hit) = spatial_query.cast_shape(
        collider, probe_origin, aligned, direction, &config, &filter,
    ) else { return; };
    if hit.normal1.dot(up) < movement.min_ground_dot { return; }

    let settled = probe_origin - up * hit.distance.max(0.0);
    opportunity.set_available(true);
    solution.set(layer.scale(), settled, aligned);
}

fn handle_spacecraft_actions(
    input: Res<PlayerInputFrame>,
    frame: Res<UsfSpatialFrame>,
    spatial_query: SpatialQuery,
    physics_charts: UsfPhysicsSlices,
    ownership: UsfOwnershipQuery,
    mut commands: Commands,
    mut control_transfers: MessageWriter<LocalControlTransferRequest>,
    player: Single<
        (
            Entity,
            &mut Transform,
            &UsfScaleLayer,
            &mut Visibility,
            &mut SpatialDemandSource,
            &mut LocomotionEnabled,
            &mut ControlledSubjectLocomotion,
            &mut CharacterControlFrame,
            &mut CharacterLocomotionFrame,
            &PhysicalBoxHull,
            &CharacterMovementConfig,
            Option<&KinematicQueryExclusions>,
            &mut PortalTraveler,
        ),
        (With<Player>, Without<SpacecraftManifestation>),
    >,
    mut semantic_positions: Query<&mut UsfPosition>,
    mut controlled_ship: Query<
        (
            Entity,
            &mut Transform,
            &UsfScaleLayer,
            &CharacterLocomotionFrame,
            &PhysicalBoxHull,
            &mut SpatialDemandSource,
            &mut ControlledSubjectLocomotion,
            &mut LinearVelocity,
            &mut UsfCanonicalMotion,
            &mut LocomotionInhibition,
            &mut FlightContactState,
            &FlightLandingOpportunity,
            &SpacecraftLandingSolution,
            &mut PortalTraveler,
        ),
        (With<SpacecraftManifestation>, With<LocalControlSubject>, Without<Player>),
    >,
    player_controlled: Query<(), (With<Player>, With<LocalControlSubject>)>,
    mut ships: Query<
        (
            Entity,
            &Transform,
            &UsfScaleLayer,
            &FlightContactState,
            &mut SpatialDemandSource,
        ),
        (With<SpacecraftManifestation>, Without<LocalControlSubject>, Without<Player>),
    >,
) {
    if let Ok((
        ship_entity,
        mut ship_transform,
        ship_layer,
        ship_frame,
        ship_hull,
        mut ship_demand,
        mut ship_locomotion,
        mut ship_velocity,
        mut ship_motion,
        mut ship_inhibition,
        mut ship_contact,
        ship_landing,
        ship_landing_solution,
        mut ship_traveler,
    )) = controlled_ship.single_mut()
    {
        if !ship_contact.is_landed()
            && input.gameplay_active()
            && input.just_pressed(PlayerAction::ToggleLanding)
            && ship_landing.available()
            && let Some((settled_translation, aligned)) =
                ship_landing_solution.at_scale(ship_layer.scale())
        {
            let Ok(settled_semantic) = frame.origin()
                .translated_at_scale(ship_layer.scale(), settled_translation)
            else { return; };
            let Some(semantic_ship) = ownership.semantic_of(ship_entity) else { return; };
            let Ok(mut semantic_position) = semantic_positions.get_mut(semantic_ship) else { return; };

            ship_transform.translation = settled_translation;
            ship_transform.rotation = aligned;
            ship_traveler.commit_position(settled_translation);
            *semantic_position = settled_semantic;
            ship_velocity.0 = Vec3::ZERO;
            ship_motion.stop();
            ship_contact.land();
            ship_inhibition.set(LocomotionInhibitionReason::SurfaceContact, true);
            ship_locomotion.request_automatic();
            ship_locomotion.set_thrusters_enabled(false);
            ship_locomotion.set_rcs_enabled(false);
            return;
        }

        if ship_contact.is_landed()
            && input.gameplay_active()
            && input.just_pressed(PlayerAction::TakeOff)
        {
            ship_contact.launch();
            ship_inhibition.set(LocomotionInhibitionReason::SurfaceContact, false);
            ship_locomotion.request_regime(LocomotionRegime::LocalFlight);
            ship_locomotion.set_thrusters_enabled(true);
            ship_locomotion.set_rcs_enabled(true);
            ship_velocity.0 =
                ship_frame.up() * ship_layer.scale().metres_to_native_f32(5.0);
            ship_motion.set_from_native_velocity(ship_layer.scale(), ship_velocity.0);
            return;
        }

        if !ship_contact.is_landed()
            || !input.gameplay_active()
            || !input.just_pressed(PlayerAction::Interact)
        {
            return;
        }

        let (
            player_entity,
            mut player_transform,
            _player_layer,
            mut player_visibility,
            mut player_demand,
            mut player_enabled,
            mut player_locomotion,
            mut player_control,
            mut player_frame,
            player_hull,
            player_movement,
            player_exclusions,
            mut player_traveler,
        ) = player.into_inner();

        // Vehicle exit is a real standing-pose query. The player does not
        // inherit the ship-center altitude: search for walkable support beside
        // the landed hull using the player's own detailed body.
        player_frame.up = ship_frame.up();
        let aligned_player = player_frame.aligned_rotation(ship_transform.rotation);

        let exit_offset =
            ship_transform.rotation * Vec3::X * ship_layer.scale().metres_to_native_f32(4.0);
        let exit_column = ship_transform.translation + exit_offset;

        let ship_support_metres =
            ship_hull.projection_radius_metres(ship_transform.rotation, ship_frame.up());
        let player_support_metres =
            player_hull.projection_radius_metres(aligned_player, ship_frame.up());
        let probe_lift_metres = player_support_metres + LANDING_PROBE_METRES;
        let probe_distance_metres =
            ship_support_metres + player_support_metres + LANDING_PROBE_METRES * 2.0;

        let exit_probe_start = exit_column
            + ship_frame.up()
                * ship_layer.scale().metres_to_native_f32(probe_lift_metres);
        let Ok(exit_direction) = Dir3::new(-ship_frame.up()) else {
            return;
        };
        let exit_filter = physics_charts.filter_for_scale(
            ship_layer.scale(),
            std::iter::once(player_entity)
                .chain(std::iter::once(ship_entity))
                .chain(
                    player_exclusions
                        .into_iter()
                        .flat_map(|items| items.iter()),
                ),
        );
        let exit_config = ShapeCastConfig {
            max_distance: ship_layer
                .scale()
                .metres_to_native_f32(probe_distance_metres),
            ignore_origin_penetration: true,
            ..default()
        };
        let player_collider = player_hull.collider(ship_layer.scale());
        let Some(exit_hit) = spatial_query.cast_shape(
            &player_collider,
            exit_probe_start,
            aligned_player,
            exit_direction,
            &exit_config,
            &exit_filter,
        ) else {
            return;
        };
        if exit_hit.normal1.dot(ship_frame.up()) < player_movement.min_ground_dot {
            return;
        }

        let exit_local =
            exit_probe_start - ship_frame.up() * exit_hit.distance.max(0.0);
        let Ok(exit_semantic) = frame
            .origin()
            .translated_at_scale(ship_layer.scale(), exit_local)
        else {
            return;
        };

        let Some(player_semantic) = ownership.semantic_of(player_entity) else {
            return;
        };
        if let Ok(mut semantic) = semantic_positions.get_mut(player_semantic) {
            *semantic = exit_semantic;
        }
        player_transform.translation = exit_local;

        // Vehicle exit is a pose transaction. The radial control frame and
        // standing support pose were resolved above before control changes.
        player_transform.rotation = aligned_player;
        player_control.snap_to(aligned_player);

        // spacecraft-contact-handoff-layer-access-v4
        // Scale-Slice reassignment is part of the deferred control/handoff
        // transaction. The action system only needs to read the current player
        // layer, which keeps its collision-query SystemParams alias-safe.
        commands
            .entity(player_entity)
            .insert(UsfScaleLayer::new(ship_layer.scale()));
        player_traveler.commit_position(exit_local);
        *player_visibility = Visibility::Inherited;
        player_demand.set_enabled(true);
        player_enabled.0 = true;
        player_locomotion.request_automatic();
        player_locomotion.set_thrusters_enabled(false);
        player_locomotion.set_rcs_enabled(false);

        ship_demand.set_enabled(false);

        commands
            .entity(player_semantic)
            .remove::<UsfConstituentOf>();

        control_transfers.write(LocalControlTransferRequest::new(
            player_semantic,
            player_entity,
        ));
        return;
    }

    if !input.gameplay_active() || !input.just_pressed(PlayerAction::Interact) {
        return;
    }

    let (
        player_entity,
        player_transform,
        player_layer,
        mut player_visibility,
        mut player_demand,
        mut player_enabled,
        _,
        _,
        _,
        _,
        _,
        _,
        _,
    ) = player.into_inner();

    if !player_controlled.contains(player_entity) {
        return;
    }

    let Some(player_semantic) = ownership.semantic_of(player_entity) else {
        return;
    };

    for (
        ship_entity,
        ship_transform,
        ship_layer,
        ship_contact,
        mut ship_demand,
    ) in &mut ships
    {
        if !ship_contact.is_landed() || ship_layer.scale() != player_layer.scale()
        {
            continue;
        }

        let distance_native = player_transform
            .translation
            .distance(ship_transform.translation);
        let distance_metres =
            f64::from(distance_native) * player_layer.scale().scale0_units_per_native();
        if distance_metres > f64::from(ENTER_DISTANCE_METRES) {
            continue;
        }

        *player_visibility = Visibility::Hidden;
        player_demand.set_enabled(false);
        player_enabled.0 = false;
        ship_demand.set_enabled(true);

        commands
            .entity(player_entity)
            .remove::<Collider>()
            .remove::<CharacterMotor>();
        let Some(ship_semantic) = ownership.semantic_of(ship_entity) else {
            continue;
        };
        commands
            .entity(player_semantic)
            .insert(UsfConstituentOf(ship_semantic));

        control_transfers.write(LocalControlTransferRequest::new(
            player_semantic,
            ship_entity,
        ));
        return;
    }
}

fn sync_spacecraft_orbit(
    ownership: UsfOwnershipQuery,
    semantic_positions: Query<&UsfPosition>,
    gravity_sources: Query<&RadialGravitySource>,
    mut ships: Query<
        (
            Entity,
            &UsfCanonicalMotion,
            &PrimaryBodyContext,
            &mut SpacecraftOrbit,
        ),
        With<SpacecraftManifestation>,
    >,
) {
    for (ship_entity, motion, primary, mut orbit) in &mut ships {
        orbit.valid = false;
        let Some(primary_entity) = primary.entity() else {
            continue;
        };
        let Ok(gravity) = gravity_sources.get(primary_entity).copied() else {
            continue;
        };
        if gravity.surface_gravity_metres_per_second2() <= 0.0 {
            continue;
        }

        let Some(ship_semantic) = ownership.semantic_of(ship_entity) else {
            continue;
        };
        let Ok(position) = semantic_positions.get(ship_semantic).copied() else {
            continue;
        };

        let field_scale = gravity.field_scale();
        let bound = gravity.radius_metres() * 16.0;
        let bound_native = field_scale
            .scale0_to_native_f64(bound)
            .min(f64::from(f32::MAX)) as f32;
        let Ok(relative) = position.relative_at_scale_bounded(
            &primary.center(),
            field_scale,
            bound_native,
        ) else {
            continue;
        };

        let field_metres_per_native = field_scale.scale0_units_per_native();
        let r = DVec3::new(
            f64::from(relative.x),
            f64::from(relative.y),
            f64::from(relative.z),
        ) * field_metres_per_native;
        let radius_from_center = r.length();
        if radius_from_center <= f64::EPSILON {
            continue;
        }

        let v = motion.velocity_metres_per_second();
        let body_radius = gravity.radius_metres();
        let mu =
            f64::from(gravity.surface_gravity_metres_per_second2()) * body_radius.powi(2);
        if !mu.is_finite() || mu <= f64::EPSILON {
            continue;
        }

        let h = r.cross(v);
        let h_len = h.length();
        if h_len <= f64::EPSILON {
            continue;
        }

        let e_vec = v.cross(h) / mu - r / radius_from_center;
        let eccentricity = e_vec.length();
        let specific_energy = 0.5 * v.length_squared() - mu / radius_from_center;
        let semi_major_axis = if specific_energy.abs() > 1.0e-12 {
            -mu / (2.0 * specific_energy)
        } else {
            f64::INFINITY
        };
        let bound = specific_energy < 0.0 && eccentricity < 1.0;

        let reference_normal = DVec3::Y;
        let node = reference_normal.cross(h);
        let node_len = node.length();
        let inclination =
            (h.dot(reference_normal) / h_len).clamp(-1.0, 1.0).acos();

        let longitude_ascending_node = if node_len > 1.0e-12 {
            node.z.atan2(node.x).rem_euclid(std::f64::consts::TAU)
        } else {
            0.0
        };

        let argument_periapsis = if node_len > 1.0e-12 && eccentricity > 1.0e-12 {
            let mut angle =
                (node.dot(e_vec) / (node_len * eccentricity)).clamp(-1.0, 1.0).acos();
            if e_vec.y < 0.0 {
                angle = std::f64::consts::TAU - angle;
            }
            angle
        } else {
            0.0
        };

        let true_anomaly = if eccentricity > 1.0e-12 {
            let mut angle = (e_vec.dot(r) / (eccentricity * radius_from_center))
                .clamp(-1.0, 1.0)
                .acos();
            if r.dot(v) < 0.0 {
                angle = std::f64::consts::TAU - angle;
            }
            angle
        } else {
            0.0
        };

        let periapsis_radius = if semi_major_axis.is_finite() {
            semi_major_axis * (1.0 - eccentricity)
        } else {
            h_len * h_len / (mu * (1.0 + eccentricity))
        };
        let apoapsis_radius = if bound && semi_major_axis.is_finite() {
            semi_major_axis * (1.0 + eccentricity)
        } else {
            f64::INFINITY
        };

        *orbit = SpacecraftOrbit {
            valid: true,
            bound,
            altitude_metres: radius_from_center - body_radius,
            speed_metres_per_second: v.length(),
            semi_major_axis_metres: semi_major_axis,
            eccentricity,
            inclination_radians: inclination,
            longitude_ascending_node_radians: longitude_ascending_node,
            argument_periapsis_radians: argument_periapsis,
            true_anomaly_radians: true_anomaly,
            periapsis_altitude_metres: periapsis_radius - body_radius,
            apoapsis_altitude_metres: if apoapsis_radius.is_finite() {
                apoapsis_radius - body_radius
            } else {
                f64::INFINITY
            },
        };
    }
}
