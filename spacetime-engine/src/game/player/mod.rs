//! Local-player gameplay and presentation.

mod camera;
mod components;
mod controls;
pub mod cursor;
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
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                cursor::update_cursor_capture
                    .in_set(InputSet::Cursor),
            )
            .add_systems(
                Update,
                (
                    controls::look,
                    controls::movement,
                    camera::toggle_camera_mode,
                )
                    .in_set(InputSet::Gameplay),
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
    let position =
        Vec3::new(0.0, 1.5, 8.0);

    let model =
        model::create_model(
            &mut meshes,
            &mut materials,
        );

    let player =
        commands
            .spawn((
                Name::new("Player"),
                Player,
                PlayerController::default(),
                Weapon::default(),
                PortalTraveler::new(position),
                Transform::from_translation(
                    position,
                ),
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
        RenderLayers::layer(0)
            .with(MAIN_PORTAL_LAYER),
        Transform::from_translation(
            position,
        ),
    ));
}
