use bevy::prelude::*;

use super::{
    CameraMode,
    Player,
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

/// Resolves presentation after simulation.
///
/// Portal traversal moves the gameplay entity first. The local camera follows
/// that final transform, and recursive portal cameras are derived afterward.
pub fn sync_player_camera(
    player: Single<
        &Transform,
        (With<Player>, Without<PlayerCamera>),
    >,
    camera: Single<
        (&PlayerCamera, &mut Transform),
        (With<PlayerCamera>, Without<Player>),
    >,
) {
    let (camera, mut camera_transform) =
        camera.into_inner();

    let local_offset = match camera.mode {
        CameraMode::FirstPerson => camera.first_person_offset,
        CameraMode::ThirdPerson => camera.third_person_offset,
    };

    camera_transform.translation =
        player.translation + player.rotation * local_offset;

    camera_transform.rotation = player.rotation;
}
