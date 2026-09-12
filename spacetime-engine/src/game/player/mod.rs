//! Local-player gameplay and presentation.
//!
//! The player body is simulation state. Aim is body-local view intent. Camera
//! placement is presentation. Device input is an adapter that writes those
//! components. Keeping those layers explicit makes them independently
//! replaceable by mods, AI, replay/network input or a different camera rig.

mod camera;
mod components;
mod controls;
pub mod cursor;
mod model;
mod stance;

pub use camera::{
    CameraMode,
    PlayerCamera,
    ThirdPersonCamera,
};
pub use components::{
    Player,
    PlayerAim,
    PlayerController,
    PlayerNoclip,
    PlayerStance,
};
pub use model::PlayerModel;

use avian3d::prelude::Collider;
use bevy::{
    app::{RunFixedMainLoop, RunFixedMainLoopSystems},
    camera::visibility::RenderLayers,
    prelude::*,
};

use crate::physics::character::{
    CharacterDimensions,
    CharacterMotor,
};

use super::{
    InputSet,
    PresentationSet,
    combat::Weapon,
    portal::{
        MAIN_PORTAL_LAYER,
        PortalTraveler,
        PortalView,
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<cursor::CursorCapture>()
            .register_type::<Player>()
            .register_type::<PlayerController>()
            .register_type::<PlayerAim>()
            .register_type::<PlayerStance>()
            .register_type::<PlayerNoclip>()
            .register_type::<PlayerCamera>()
            .register_type::<ThirdPersonCamera>()
            .register_type::<CameraMode>()
            .add_systems(Startup, spawn_player)
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
                    camera::toggle_camera_mode,
                    camera::zoom_third_person,
                )
                    .chain()
                    .in_set(InputSet::Gameplay),
            )
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
    let position = Vec3::new(
        0.0,
        CharacterDimensions::HALF_HEIGHT + 0.01,
        8.0,
    );

    let model = model::create_model(&mut meshes, &mut materials);

    let player = commands
        .spawn((
            Name::new("Player"),
            Player,
            PlayerController::default(),
            PlayerAim::default(),
            PlayerStance::default(),
            PlayerNoclip::default(),
            CharacterMotor,
            CharacterDimensions::standing_collider(),
            Weapon::default(),
            PortalTraveler::new(position),
            Transform::from_translation(position),
        ))
        .id();

    commands.entity(player).with_children(|parent| {
        parent.spawn(model);
    });

    commands.spawn((
        Name::new("Player Camera"),
        PlayerCamera::default(),
        PortalView,
        Camera3d::default(),
        IsDefaultUiCamera,
        RenderLayers::layer(0).with(MAIN_PORTAL_LAYER),
        Transform::from_translation(position),
    ));
}
