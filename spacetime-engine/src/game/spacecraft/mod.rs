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
            ControlledSubjectHull, ControlledSubjectLocomotion, DetailedInteractionScale,
            FlightControlIntent, LocomotionCapabilities, LocomotionEnabled,
            LocomotionInhibition, LocomotionInhibitionReason, LocomotionRegime, LocomotionSet,
        },
        navigation::{
            AdaptiveCruise, ApproachRefinementState, PrimaryBodyContext, TravelEnvelope,
            TravelProfile, TravelState,
        },
        player::{Player, ViewCameraProfile},
    },
    physics::{
        chart::UsfPhysicsCharts,
        character::{
            CharacterControlFrame, CharacterGroundState, CharacterLocomotionFrame,
            CharacterMovementConfig, CharacterMovementInput, CharacterMotor,
        },
        topology::{KinematicQueryExclusions, SpatialSplitBox},
    },
    portal::PortalTraveler,
    spatial::{
        SpatialDemandSource, SpatialRefinementDemand, SpatialScale,
        UsfCanonicalMotion, UsfLocalScalePresentation, UsfPosition, UsfScaleLayer,
        UsfSpatialAnchor, UsfSpatialFrame, UsfTravelNeighborhood,
    },
    view::ViewSubjectPresentation,
    voxel::VoxelMaterializationDemand,
};

use crate::spatial::UsfNavigationContext;

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
            .add_systems(PostStartup, spawn_reference_spacecraft)
            .add_systems(Update, handle_spacecraft_actions.in_set(ControlActionSet::Request))
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
        ),
        With<Player>,
    >,
    semantic_positions: Query<&UsfPosition>,
) {
    let (
        body_entity,
        body_manifestation,
        body_transform,
        body_layer,
        mut body_visibility,
        mut body_demand,
        mut body_enabled,
    ) = body.into_inner();

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
                UsfSpatialAnchor,
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
                ControlledSubjectHull::cuboid(SHIP_SIZE, SHIP_PROXY_RADIUS_NATIVE),
                FlightControlIntent::default(),
                UsfCanonicalMotion::default(),
                locomotion,
                DetailedInteractionScale::default(),
                TravelProfile::spacecraft(),
                TravelEnvelope::default(),
                ApproachRefinementState::default(),
                AdaptiveCruise::default(),
                TravelState::default(),
                PrimaryBodyContext::default(),
            ),
            (
                UsfTravelNeighborhood::default(),
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
                SpatialSplitBox::from_size(SHIP_SIZE),
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
            UsfLocalScalePresentation::new(SpatialScale::MAX),
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
            &Transform,
            &UsfScaleLayer,
            &DetailedInteractionScale,
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
        transform,
        layer,
        detailed,
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

    if contact.is_landed()
        || layer.scale() != detailed.0
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

    velocity.0 = Vec3::ZERO;
    motion.stop();
    contact.land();
    inhibition.set(LocomotionInhibitionReason::SurfaceContact, true);
    locomotion.request_automatic();
    locomotion.set_thrusters_enabled(false);
}

fn handle_spacecraft_actions(
    keyboard: Res<ButtonInput<KeyCode>>,
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
        if ship_contact.is_landed() && keyboard.just_pressed(KeyCode::Space) {
            ship_contact.launch();
            ship_inhibition.set(LocomotionInhibitionReason::SurfaceContact, false);
            ship_locomotion.request_regime(LocomotionRegime::LocalFlight);
            ship_locomotion.set_thrusters_enabled(true);
            ship_velocity.0 =
                ship_frame.up() * ship_layer.scale().metres_to_native_f32(5.0);
            ship_motion.set_from_native_velocity(ship_layer.scale(), ship_velocity.0);
            return;
        }

        if !ship_contact.is_landed() || !keyboard.just_pressed(KeyCode::KeyE)
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
        player_transform.rotation = ship_transform.rotation;
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

    if !keyboard.just_pressed(KeyCode::KeyE) {
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
        if !primary.is_resolved() || primary.surface_gravity_metres_per_second2() <= 0.0 {
            continue;
        }

        let Ok(position) = semantic_positions.get(manifestation.0).copied() else {
            continue;
        };

        let field_scale = primary.field_scale();
        let bound = primary.radius_metres() * 16.0;
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
        let body_radius = primary.radius_metres();
        let mu = f64::from(primary.surface_gravity_metres_per_second2()) * body_radius.powi(2);
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
