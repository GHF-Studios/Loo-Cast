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
        UsfConstituentOf, UsfEntity, UsfLogicalProjection, UsfManifestationAuthority,
        UsfManifestationOf, UsfPresentationProjectionOf,
    },
    game::{
        GameSet,
        flight::{
            FlightContactState, FlightSafetyProfile, FlightSafetyState, FlightTelemetry,
            TraversalPolicy,
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
        chart::UsfPhysicsCharts,
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
const SHIP_PROXY_RADIUS_NATIVE: f32 = 0.08;
const SHIP_DEMAND_HALF_EXTENT: Vec3 = Vec3::new(96.0, 64.0, 96.0);
const SHIP_DEMAND_PRIORITY: i32 = 120;
const LANDING_PROBE_METRES: f32 = 2.0;
const ENTER_DISTANCE_METRES: f32 = 12.0;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Spacecraft;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct SpacecraftManifestation;

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
    mut commands: Commands,
    mut control_transfers: MessageWriter<LocalControlTransferRequest>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    body: Single<
        (
            Entity,
            &UsfManifestationOf,
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
        body_manifestation,
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

    let player_semantic = body_manifestation.0;
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

    let mut locomotion = ControlledSubjectLocomotion::default();
    locomotion.request_automatic();
    locomotion.set_thrusters_enabled(true);

    let ship = commands
        .spawn((
            (
                Name::new("Reference Spacecraft Manifestation"),
                SpacecraftManifestation,
                Visibility::Inherited,
                UsfManifestationOf(semantic_ship),
                UsfManifestationAuthority,
                UsfLogicalProjection,
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
                    ScaleInteractionProxy::new(SHIP_PROXY_RADIUS_NATIVE),
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
                Collider::sphere(SHIP_PROXY_RADIUS_NATIVE),
            ),
            (
                LocomotionInhibition::default(),
                FlightContactState::default(),
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
    physics_charts: UsfPhysicsCharts,
    mut ships: Query<
        (
            Entity,
            &mut Transform,
            &UsfScaleLayer,
            &DetailedBodyScale,
            &mut CharacterControlFrame,
            &CharacterLocomotionFrame,
            &Collider,
            Option<&KinematicQueryExclusions>,
            &mut LinearVelocity,
            &mut UsfCanonicalMotion,
            &mut ControlledSubjectLocomotion,
            &FlightSafetyProfile,
            &mut LocomotionInhibition,
            &mut FlightContactState,
        ),
        (With<SpacecraftManifestation>, With<LocalControlSubject>, Without<Player>),
    >,
) {
    let Ok((
        entity,
        mut transform,
        layer,
        detailed,
        mut control,
        frame,
        collider,
        exclusions,
        mut velocity,
        mut motion,
        mut locomotion,
        safety,
        mut inhibition,
        mut contact,
    )) = ships.single_mut()
    else {
        return;
    };

    if contact.is_landed() {
        let aligned = frame.aligned_rotation(transform.rotation);
        transform.rotation = aligned;
        control.snap_to(aligned);
        return;
    }

    if layer.scale() != detailed.0
        || locomotion.regime() != LocomotionRegime::LocalFlight
    {
        return;
    }

    let speed_metres = motion.speed_metres_per_second();
    if speed_metres > safety.preferred_contact_speed_metres_per_second() {
        return;
    }

    let Ok(direction) = Dir3::new(-frame.up()) else {
        return;
    };
    let max_distance = layer.scale().metres_to_native_f32(LANDING_PROBE_METRES);
    let filter = physics_charts.filter_for_scale(
        layer.scale(),
        std::iter::once(entity)
            .chain(exclusions.into_iter().flat_map(|items| items.iter())),
    );
    let config = ShapeCastConfig {
        max_distance,
        ignore_origin_penetration: true,
        ..default()
    };

    if spatial_query
        .cast_shape(
            collider,
            transform.translation,
            transform.rotation,
            direction,
            &config,
            &filter,
        )
        .is_none()
    {
        return;
    }

    // Contact has been accepted: resolve the hull into the local surface
    // tangent frame once. This fixes landed camera/body orientation without
    // imposing auto-level behavior during free flight.
    let aligned = frame.aligned_rotation(transform.rotation);
    transform.rotation = aligned;
    control.snap_to(aligned);

    velocity.0 = Vec3::ZERO;
    motion.stop();
    contact.land();
    inhibition.set(LocomotionInhibitionReason::SurfaceContact, true);
    locomotion.request_automatic();
    locomotion.set_thrusters_enabled(false);
}

fn handle_spacecraft_actions(
    input: Res<PlayerInputFrame>,
    frame: Res<UsfSpatialFrame>,
    mut commands: Commands,
    mut control_transfers: MessageWriter<LocalControlTransferRequest>,
    player: Single<
        (
            Entity,
            &UsfManifestationOf,
            &mut Transform,
            &mut UsfScaleLayer,
            &mut Visibility,
            &mut SpatialDemandSource,
            &mut LocomotionEnabled,
            &mut ControlledSubjectLocomotion,
            &mut CharacterControlFrame,
            &mut CharacterLocomotionFrame,
            &mut PortalTraveler,
        ),
        (With<Player>, Without<SpacecraftManifestation>),
    >,
    mut semantic_positions: Query<&mut UsfPosition>,
    mut controlled_ship: Query<
        (
            Entity,
            &UsfManifestationOf,
            &Transform,
            &UsfScaleLayer,
            &CharacterLocomotionFrame,
            &mut SpatialDemandSource,
            &mut ControlledSubjectLocomotion,
            &mut LinearVelocity,
            &mut UsfCanonicalMotion,
            &mut LocomotionInhibition,
            &mut FlightContactState,
        ),
        (With<SpacecraftManifestation>, With<LocalControlSubject>, Without<Player>),
    >,
    player_controlled: Query<(), (With<Player>, With<LocalControlSubject>)>,
    mut ships: Query<
        (
            Entity,
            &UsfManifestationOf,
            &Transform,
            &UsfScaleLayer,
            &FlightContactState,
            &mut SpatialDemandSource,
        ),
        (With<SpacecraftManifestation>, Without<LocalControlSubject>, Without<Player>),
    >,
) {
    if let Ok((
        _ship_entity,
        _ship_manifestation,
        ship_transform,
        ship_layer,
        ship_frame,
        mut ship_demand,
        mut ship_locomotion,
        mut ship_velocity,
        mut ship_motion,
        mut ship_inhibition,
        mut ship_contact,
    )) = controlled_ship.single_mut()
    {
        if ship_contact.is_landed()
            && input.gameplay_active()
            && input.just_pressed(PlayerAction::TakeOff)
        {
            ship_contact.launch();
            ship_inhibition.set(LocomotionInhibitionReason::SurfaceContact, false);
            ship_locomotion.request_regime(LocomotionRegime::LocalFlight);
            ship_locomotion.set_thrusters_enabled(true);
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
            player_manifestation,
            mut player_transform,
            mut player_layer,
            mut player_visibility,
            mut player_demand,
            mut player_enabled,
            mut player_locomotion,
            mut player_control,
            mut player_frame,
            mut player_traveler,
        ) = player.into_inner();

        let exit_offset =
            ship_transform.rotation * Vec3::X * ship_layer.scale().metres_to_native_f32(4.0);
        let exit_local = ship_transform.translation + exit_offset;
        let Ok(exit_semantic) = frame
            .origin()
            .translated_at_scale(ship_layer.scale(), exit_local)
        else {
            return;
        };

        if let Ok(mut semantic) = semantic_positions.get_mut(player_manifestation.0) {
            *semantic = exit_semantic;
        }
        player_transform.translation = exit_local;

        // Vehicle exit is a pose transaction. Seed the character's radial
        // locomotion/control frame from the landed ship before control changes,
        // otherwise grounding can start from a sideways collider and never
        // reach the later "grounded => align" repair path.
        player_frame.up = ship_frame.up();
        let aligned_player = player_frame.aligned_rotation(ship_transform.rotation);
        player_transform.rotation = aligned_player;
        player_control.snap_to(aligned_player);

        *player_layer = UsfScaleLayer::new(ship_layer.scale());
        player_traveler.commit_position(exit_local);
        *player_visibility = Visibility::Inherited;
        player_demand.set_enabled(true);
        player_enabled.0 = true;
        player_locomotion.request_automatic();
        player_locomotion.set_thrusters_enabled(false);

        ship_demand.set_enabled(false);

        commands
            .entity(player_manifestation.0)
            .remove::<UsfConstituentOf>();

        control_transfers.write(LocalControlTransferRequest::new(
            player_manifestation.0,
            player_entity,
        ));
        return;
    }

    if !input.gameplay_active() || !input.just_pressed(PlayerAction::Interact) {
        return;
    }

    let (
        player_entity,
        player_manifestation,
        player_transform,
        player_layer,
        mut player_visibility,
        mut player_demand,
        mut player_enabled,
        _,
        _,
        _,
        _,
    ) = player.into_inner();

    if !player_controlled.contains(player_entity) {
        return;
    }

    for (
        ship_entity,
        ship_manifestation,
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
        commands
            .entity(player_manifestation.0)
            .insert(UsfConstituentOf(ship_manifestation.0));

        control_transfers.write(LocalControlTransferRequest::new(
            player_manifestation.0,
            ship_entity,
        ));
        return;
    }
}

fn sync_spacecraft_orbit(
    semantic_positions: Query<&UsfPosition>,
    gravity_sources: Query<&RadialGravitySource>,
    mut ships: Query<
        (
            &UsfManifestationOf,
            &UsfCanonicalMotion,
            &PrimaryBodyContext,
            &mut SpacecraftOrbit,
        ),
        With<SpacecraftManifestation>,
    >,
) {
    for (manifestation, motion, primary, mut orbit) in &mut ships {
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

        let Ok(position) = semantic_positions.get(manifestation.0).copied() else {
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
