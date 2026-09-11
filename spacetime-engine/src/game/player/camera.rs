use bevy::prelude::*;

use super::{
    CameraMode,
    Player,
    PlayerAim,
    PlayerCamera,
};

pub fn toggle_camera_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut PlayerCamera>,
) {
    if !keyboard.just_pressed(KeyCode::F5) {
        return;
    }

    camera.mode = match camera.mode {
        CameraMode::FirstPerson => CameraMode::ThirdPerson,
        CameraMode::ThirdPerson => CameraMode::FirstPerson,
    };
}

/// Resolves presentation after simulation/topology.
pub fn sync_player_camera(
    player: Single<
        (&Transform, &PlayerAim),
        (With<Player>, Without<PlayerCamera>),
    >,
    camera: Single<
        (&PlayerCamera, &mut Transform),
        (With<PlayerCamera>, Without<Player>),
    >,
) {
    let (body, aim) = player.into_inner();
    let (camera, mut camera_transform) = camera.into_inner();

    *camera_transform = camera.resolve_transform(body, aim);
}
