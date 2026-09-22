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
        control::{LocalControlState, LocalControlSubject},
        locomotion::{
            ControlledSubjectHull, LocomotionCapabilities, LocomotionEnabled,
        },
        player::{
            CameraMode, ControlledSubjectLocomotion, Player, PlayerAdaptiveCruise,
            PlayerAim, PlayerApproachRefinementState, PlayerCamera, PlayerController,
            PlayerDetailedPhysicsScale, PlayerLocomotionRegime, PlayerScaleInteractionProxy,
            PlayerStance, PlayerTravelEnvelope, PlayerTravelSpeed, PlayerTravelState,
        },
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
        UsfInteractionProjection, UsfLocalScalePresentation, UsfPosition,
        UsfRadialGravitySource, UsfScaleLayer, UsfSpatialAnchor, UsfSpatialFrame,
        UsfTravelNeighborhood, UsfViewAnchor,
    },
    voxel::VoxelMaterializationDemand,
};

use crate::spatial::UsfNavigationContext;

const SHIP_SIZE: Vec3 = Vec3::new(4.0, 2.0, 8.0);
const SHIP_PROXY_RADIUS_NATIVE: f32 = 0.08;
const SHIP_DEMAND_HALF_EXTENT: Vec3 = Vec3::new(96.0, 64.0, 96.0);
const SHIP_DEMAND_PRIORITY: i32 = 120;
const LANDING_PROBE_METRES: f32 = 2.0;
const LANDING_MAX_SPEED_METRES_PER_SECOND: f32 = 8.0;
const ENTER_DISTANCE_METRES: f32 = 12.0;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Spacecraft;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct SpacecraftManifestation;

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpacecraftFlightRegime {
    #[default]
    Cruise,
    Orbital,
    Local,
    Landed,
}

impl SpacecraftFlightRegime {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Cruise => "CRUISE",
            Self::Orbital => "ORBITAL",
            Self::Local => "LOCAL FLIGHT",
            Self::Landed => "LANDED",
        }
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy, Default)]
#[reflect(Component)]
pub struct SpacecraftFlightState {
    pub regime: SpacecraftFlightRegime,
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

#[derive(Component)]
struct SpacecraftModel;

pub struct SpacecraftPlugin;

impl Plugin for SpacecraftPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Spacecraft>()
            .register_type::<SpacecraftManifestation>()
            .register_type::<SpacecraftFlightRegime>()
            .register_type::<SpacecraftFlightState>()
            .register_type::<SpacecraftOrbit>()
            .add_systems(PostStartup, spawn_reference_spacecraft)
            .add_systems(Update, handle_spacecraft_actions.in_set(GameSet::Action))
            .add_systems(
                Update,
                (sync_spacecraft_flight_state, sync_spacecraft_orbit)
                    .chain()
                    .in_set(GameSet::Presentation),
            );
    }
}

fn spawn_reference_spacecraft(
    mut commands: Commands,
    mut control: ResMut<LocalControlState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut camera: Single<&mut PlayerCamera>,
    mut body: Single<
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
                LocalControlSubject,
                Visibility::Inherited,
                UsfManifestationOf(semantic_ship),
                UsfManifestationAuthority,
                UsfLogicalProjection,
                UsfSpatialAnchor,
                UsfViewAnchor,
                UsfInteractionProjection,
                UsfScaleLayer::new(body_layer.scale()),
                SpatialDemandSource::cuboid(SHIP_DEMAND_HALF_EXTENT)
                    .with_priority(SHIP_DEMAND_PRIORITY),
                SpatialRefinementDemand::cuboid(SHIP_DEMAND_HALF_EXTENT),
                VoxelMaterializationDemand,
            ),
            (
                LocomotionCapabilities::spacecraft(),
                LocomotionEnabled(true),
                ControlledSubjectHull::cuboid(SHIP_SIZE, SHIP_PROXY_RADIUS_NATIVE),
                PlayerController::default(),
                PlayerAim::default(),
                PlayerStance::default(),
                locomotion,
                PlayerScaleInteractionProxy::default(),
                PlayerDetailedPhysicsScale::default(),
                PlayerTravelSpeed::default(),
                PlayerTravelEnvelope::default(),
                PlayerApproachRefinementState::default(),
                PlayerAdaptiveCruise::default(),
                PlayerTravelState::default(),
            ),
            (
                UsfTravelNeighborhood::default(),
                UsfNavigationContext::default(),
                CharacterControlFrame::default(),
                CharacterLocomotionFrame::default(),
                CharacterMovementConfig::default(),
                CharacterMovementInput::default(),
                CharacterGroundState::default(),
                SpacecraftFlightState::default(),
                SpacecraftOrbit::default(),
                RigidBody::Kinematic,
                CustomPositionIntegration,
                CustomVelocityIntegration,
                LinearVelocity::ZERO,
                Collider::sphere(SHIP_PROXY_RADIUS_NATIVE),
                SpatialSplitBox::from_size(SHIP_SIZE),
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
            SpacecraftModel,
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
        .remove::<UsfViewAnchor>()
        .remove::<UsfInteractionProjection>()
        .remove::<Collider>()
        .remove::<CharacterMotor>();

    body_demand.set_enabled(false);
    body_enabled.0 = false;
    *body_visibility = Visibility::Hidden;

    control.set(player_semantic, semantic_ship, ship);

    camera.mode = CameraMode::ThirdPerson;
    camera.third_person.base_distance = 14.0;
    camera.third_person.maximum_distance = 40.0;
}

pub(crate) fn detect_landing(
    spatial_query: SpatialQuery,
    physics_charts: UsfPhysicsCharts,
    mut ships: Query<
        (
            Entity,
            &Transform,
            &UsfScaleLayer,
            &PlayerDetailedPhysicsScale,
            &CharacterLocomotionFrame,
            &Collider,
            Option<&KinematicQueryExclusions>,
            &mut LinearVelocity,
            &mut ControlledSubjectLocomotion,
            &mut LocomotionEnabled,
            &mut SpacecraftFlightState,
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
        mut locomotion,
        mut enabled,
        mut state,
    )) = ships.single_mut()
    else {
        return;
    };

    if !enabled.0
        || layer.scale() != detailed.0
        || locomotion.regime() != PlayerLocomotionRegime::LocalFlight
    {
        return;
    }

    let speed_metres =
        f64::from(velocity.0.length()) * layer.scale().scale0_units_per_native();
    if speed_metres > f64::from(LANDING_MAX_SPEED_METRES_PER_SECOND) {
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
    enabled.0 = false;
    state.regime = SpacecraftFlightRegime::Landed;
    locomotion.request_automatic();
    locomotion.set_thrusters_enabled(false);
}

fn sync_spacecraft_flight_state(
    mut ships: Query<
        (&LocomotionEnabled, &ControlledSubjectLocomotion, &mut SpacecraftFlightState),
        With<SpacecraftManifestation>,
    >,
) {
    for (enabled, locomotion, mut state) in &mut ships {
        if !enabled.0 {
            state.regime = SpacecraftFlightRegime::Landed;
            continue;
        }
        state.regime = match locomotion.regime() {
            PlayerLocomotionRegime::Cruise => SpacecraftFlightRegime::Cruise,
            PlayerLocomotionRegime::PlanetaryFlight => SpacecraftFlightRegime::Orbital,
            PlayerLocomotionRegime::LocalFlight | PlayerLocomotionRegime::OnFoot => {
                SpacecraftFlightRegime::Local
            }
        };
    }
}

fn handle_spacecraft_actions(
    keyboard: Res<ButtonInput<KeyCode>>,
    frame: Res<UsfSpatialFrame>,
    mut commands: Commands,
    mut control: ResMut<LocalControlState>,
    mut camera: Single<&mut PlayerCamera>,
    mut player: Single<
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
            &mut LocomotionEnabled,
            &mut ControlledSubjectLocomotion,
            &mut LinearVelocity,
            &mut SpacecraftFlightState,
        ),
        (With<SpacecraftManifestation>, With<LocalControlSubject>, Without<Player>),
    >,
    mut ships: Query<
        (
            Entity,
            &UsfManifestationOf,
            &Transform,
            &UsfScaleLayer,
            &SpacecraftFlightState,
            &mut SpatialDemandSource,
            &mut LocomotionEnabled,
        ),
        (With<SpacecraftManifestation>, Without<LocalControlSubject>, Without<Player>),
    >,
) {
    if let Ok((
        ship_entity,
        _,
        ship_transform,
        ship_layer,
        ship_frame,
        mut ship_demand,
        mut ship_enabled,
        mut ship_locomotion,
        mut ship_velocity,
        mut ship_state,
    )) = controlled_ship.single_mut()
    {
        if ship_state.regime == SpacecraftFlightRegime::Landed
            && keyboard.just_pressed(KeyCode::Space)
        {
            ship_enabled.0 = true;
            ship_state.regime = SpacecraftFlightRegime::Local;
            ship_locomotion.request_regime(PlayerLocomotionRegime::LocalFlight);
            ship_locomotion.set_thrusters_enabled(true);
            ship_velocity.0 =
                ship_frame.up() * ship_layer.scale().metres_to_native_f32(5.0);
            return;
        }

        if ship_state.regime != SpacecraftFlightRegime::Landed
            || !keyboard.just_pressed(KeyCode::KeyE)
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
            .entity(ship_entity)
            .remove::<LocalControlSubject>()
            .remove::<UsfViewAnchor>()
            .remove::<UsfInteractionProjection>();
        commands.entity(player_entity).insert((
            LocalControlSubject,
            UsfViewAnchor,
            UsfInteractionProjection,
        ));
        commands
            .entity(player_manifestation.0)
            .remove::<UsfConstituentOf>();

        control.set(
            player_manifestation.0,
            player_manifestation.0,
            player_entity,
        );
        camera.mode = CameraMode::FirstPerson;
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

    if control.manifestation() != Some(player_entity) {
        return;
    }

    for (
        ship_entity,
        ship_manifestation,
        ship_transform,
        ship_layer,
        ship_state,
        mut ship_demand,
        mut ship_enabled,
    ) in &mut ships
    {
        if ship_state.regime != SpacecraftFlightRegime::Landed
            || ship_layer.scale() != player_layer.scale()
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
        ship_enabled.0 = false;

        commands
            .entity(player_entity)
            .remove::<LocalControlSubject>()
            .remove::<UsfViewAnchor>()
            .remove::<UsfInteractionProjection>()
            .remove::<Collider>()
            .remove::<CharacterMotor>();
        commands.entity(ship_entity).insert((
            LocalControlSubject,
            UsfViewAnchor,
            UsfInteractionProjection,
        ));
        commands
            .entity(player_manifestation.0)
            .insert(UsfConstituentOf(ship_manifestation.0));

        control.set(
            player_manifestation.0,
            ship_manifestation.0,
            ship_entity,
        );
        camera.mode = CameraMode::ThirdPerson;
        camera.third_person.base_distance = 14.0;
        return;
    }
}

fn sync_spacecraft_orbit(
    frame: Res<UsfSpatialFrame>,
    gravity_sources: Query<&UsfRadialGravitySource>,
    mut ships: Query<
        (&Transform, &UsfScaleLayer, &LinearVelocity, &mut SpacecraftOrbit),
        With<SpacecraftManifestation>,
    >,
) {
    for (transform, layer, velocity, mut orbit) in &mut ships {
        orbit.valid = false;

        let Ok(position) = frame
            .origin()
            .translated_at_scale(layer.scale(), transform.translation)
        else {
            continue;
        };

        let mut selected = None::<(f64, DVec3, UsfRadialGravitySource)>;
        for source in &gravity_sources {
            let bound = source.radius_scale0() * 16.0;
            let bound_native = source.field_scale().scale0_to_native_f64(bound)
                .min(f64::from(f32::MAX)) as f32;
            let Ok(relative) = position.relative_at_scale_bounded(
                &source.center(),
                source.field_scale(),
                bound_native,
            ) else {
                continue;
            };

            let metres_per_native = source.field_scale().scale0_units_per_native();
            let r = DVec3::new(
                f64::from(relative.x),
                f64::from(relative.y),
                f64::from(relative.z),
            ) * metres_per_native;
            let distance = r.length();
            if selected.is_none_or(|(current, _, _)| distance < current) {
                selected = Some((distance, r, *source));
            }
        }

        let Some((radius_from_center, r, source)) = selected else {
            continue;
        };
        if radius_from_center <= f64::EPSILON {
            continue;
        }

        let metres_per_native = layer.scale().scale0_units_per_native();
        let v = DVec3::new(
            f64::from(velocity.0.x),
            f64::from(velocity.0.y),
            f64::from(velocity.0.z),
        ) * metres_per_native;

        let mu = f64::from(source.surface_gravity()) * source.radius_scale0().powi(2);
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
            altitude_metres: radius_from_center - source.radius_scale0(),
            speed_metres_per_second: v.length(),
            semi_major_axis_metres: semi_major_axis,
            eccentricity,
            inclination_radians: inclination,
            longitude_ascending_node_radians: longitude_ascending_node,
            argument_periapsis_radians: argument_periapsis,
            true_anomaly_radians: true_anomaly,
            periapsis_altitude_metres: periapsis_radius - source.radius_scale0(),
            apoapsis_altitude_metres: if apoapsis_radius.is_finite() {
                apoapsis_radius - source.radius_scale0()
            } else {
                f64::INFINITY
            },
        };
    }
}
