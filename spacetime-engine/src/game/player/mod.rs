//! Local-player gameplay and presentation.
//!
//! The player body is simulation state. Aim is control-frame-local view intent.
//! Camera placement is presentation. Device input is an adapter that writes
//! those components. Keeping those layers explicit makes them independently
//! replaceable by mods, AI, replay/network input or a different camera rig.

mod camera;
mod components;
mod controls;
pub mod cursor;
mod model;
mod stance;

pub use camera::{CameraMode, PlayerCamera, ThirdPersonCamera};
pub use components::{Player, PlayerAim, PlayerController, PlayerDead, PlayerNoclip, PlayerStance};
pub use model::PlayerModel;

use avian3d::prelude::{
    ActiveCollisionHooks, CollisionLayers, CustomPositionIntegration, CustomVelocityIntegration,
    LinearVelocity, RigidBody,
};
use bevy::{
    app::{RunFixedMainLoop, RunFixedMainLoopSystems},
    camera::visibility::RenderLayers,
    prelude::*,
};

use crate::{
    ecs::{
        UsfEntity, UsfLogicalProjection, UsfManifestationAuthority, UsfManifestationOf,
        UsfPresentationProjectionOf,
    },
    input_focus::{InputFocus, InputFocusSet},
    physics::{
        character::{
            CharacterDimensions, CharacterGroundState, CharacterMotor, CharacterMovementInput,
        },
        topology::{KinematicQueryExclusions, SpatialSplitBox, SpatialSplitPeer},
    },
    spatial::{SpatialDemandSource, UsfPosition, UsfSpatialAnchor, UsfViewAnchor},
    view::{PrimaryGameView, PrimaryViewPresentation},
    voxel::VoxelMaterializationDemand,
};

use super::{
    GameSet, InputSet, PresentationSet,
    combat::{Died, Health, Weapon},
    portal::{MAIN_PORTAL_LAYER, PortalSplitTraveler, PortalTraveler, PortalView},
    thermal::{ThermalBody, ThermalInjury, ThermalSpatialSample},
};

const PLAYER_SPATIAL_DEMAND_HALF_EXTENT: Vec3 = Vec3::splat(96.0);
const PLAYER_SPATIAL_DEMAND_PRIORITY: i32 = 100;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<cursor::CursorCapture>()
            .init_resource::<InputFocus>()
            .init_resource::<PrimaryViewPresentation>()
            .register_type::<Player>()
            .register_type::<PlayerController>()
            .register_type::<PlayerDead>()
            .register_type::<PlayerAim>()
            .register_type::<PlayerStance>()
            .register_type::<PlayerNoclip>()
            .register_type::<PlayerCamera>()
            .register_type::<ThirdPersonCamera>()
            .register_type::<CameraMode>()
            .add_systems(Startup, spawn_player)
            .add_systems(
                PreUpdate,
                cursor::apply_input_focus.in_set(InputFocusSet::Resolve),
            )
            .add_systems(
                RunFixedMainLoop,
                (
                    controls::look,
                    controls::toggle_noclip,
                    stance::update_stance,
                    controls::movement,
                    controls::noclip_movement,
                )
                    .chain()
                    .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
            )
            .add_systems(
                Update,
                cursor::update_cursor_capture.in_set(InputSet::Cursor),
            )
            .add_systems(
                Update,
                (
                    controls::toggle_spatial_demand,
                    controls::zoom_spatial_view,
                    camera::toggle_camera_mode,
                    camera::zoom_third_person,
                )
                    .chain()
                    .in_set(InputSet::Gameplay),
            )
            .add_systems(Update, handle_player_death.in_set(GameSet::Cleanup))
            .add_systems(
                Update,
                (
                    camera::sync_player_camera,
                    camera::sync_player_fov,
                    camera::sync_player_model,
                )
                    .chain()
                    .in_set(PresentationSet::PrimaryView),
            );
    }
}

fn spawn_player(
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
            UsfPosition::from_scale0_local(position)
                .expect("initial player position must fit USF spatial root"),
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
        PrimaryGameView,
        PortalView,
        Camera3d::default(),
        IsDefaultUiCamera,
        RenderLayers::layer(0).with(MAIN_PORTAL_LAYER),
        Transform::from_translation(position),
    ));
}

/// Adapts generic semantic death into local-player control state.
///
/// Respawning is intentionally a separate lifecycle mechanic; death cannot be
/// undone by toggling noclip or by another input adapter accidentally restoring
/// the character motor.
fn handle_player_death(
    mut commands: Commands,
    mut deaths: MessageReader<Died>,
    player: Single<
        (
            Entity,
            &UsfManifestationOf,
            &mut PlayerNoclip,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    let (entity, manifestation, mut noclip, mut input, mut ground, mut velocity) =
        player.into_inner();

    if !deaths.read().any(|death| death.entity == manifestation.0) {
        return;
    }

    noclip.active = false;
    input.clear();
    ground.grounded = false;
    ground.ground_entity = None;
    velocity.0 = Vec3::ZERO;

    commands
        .entity(entity)
        .remove::<CharacterMotor>()
        .insert(PlayerDead);
}
