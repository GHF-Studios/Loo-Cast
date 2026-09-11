//! Local-player gameplay and presentation.
//!
//! The player and camera are deliberately separate entities:
//!
//! - [`Player`] owns gameplay position, aim, weapon and portal traversal.
//! - [`PlayerCamera`] only observes that gameplay entity.

mod camera;
mod components;
mod controls;
mod cursor;
mod model;

pub use components::{
    CameraMode,
    Player,
    PlayerCamera,
    PlayerController,
};
pub use model::PlayerModel;

use bevy::{
    camera::visibility::RenderLayers,
    prelude::*,
};

use super::{
    GameSet,
    PresentationSet,
    combat::Weapon,
    portal::{
        MAIN_PORTAL_LAYER,
        PortalTraveler,
        PortalView,
    },
};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PlayerInputSet {
    Cursor,
    Control,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<cursor::CursorCapture>()
            .configure_sets(
                Update,
                (
                    PlayerInputSet::Cursor,
                    PlayerInputSet::Control,
                )
                    .chain()
                    .in_set(GameSet::Input),
            )
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                cursor::update_cursor_capture
                    .in_set(PlayerInputSet::Cursor),
            )
            .add_systems(
                Update,
                (
                    controls::look,
                    controls::movement,
                    camera::toggle_camera_mode,
                    controls::request_fire,
                    controls::request_target_spawn,
                )
                    .in_set(PlayerInputSet::Control),
            )
            .add_systems(
                Update,
                camera::sync_player_camera
                    .in_set(PresentationSet::PrimaryView),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let position = Vec3::new(0.0, 1.5, 8.0);

    let model =
        model::create_model(&mut meshes, &mut materials);

    let player = commands
        .spawn((
            Name::new("Player"),
            Player,
            PlayerController::default(),
            Weapon::default(),
            PortalTraveler::new(position),
            Transform::from_translation(position),
        ))
        .id();

    commands
        .entity(player)
        .with_children(|parent| {
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
