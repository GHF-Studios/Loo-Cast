use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use crate::physics::character::CharacterDimensions;

use super::{
    CameraMode,
    Player,
    PlayerAim,
    PlayerCamera,
    PlayerModel,
    PlayerStance,
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

/// Bevy stores perspective FOV vertically. Keep the requested gameplay FOV
/// horizontal and derive the vertical value from the current window aspect.
pub fn sync_player_fov(
    window: Single<&Window, With<PrimaryWindow>>,
    settings: Single<&PlayerCamera>,
    mut projection: Single<&mut Projection, With<PlayerCamera>>,
) {
    let Projection::Perspective(perspective) = projection.as_mut() else {
        return;
    };

    let height = window.height();
    if height <= 0.0 {
        return;
    }

    let aspect = window.width() / height;
    if aspect <= 0.0 {
        return;
    }

    let horizontal = settings
        .horizontal_fov_degrees
        .clamp(1.0, 179.0)
        .to_radians();

    perspective.fov =
        2.0 * ((horizontal * 0.5).tan() / aspect).atan();
}

pub fn sync_player_model(
    camera: Single<&PlayerCamera>,
    player: Single<&PlayerStance, With<Player>>,
    mut models: Query<(&mut Visibility, &mut Transform), With<PlayerModel>>,
) {
    let stance = player.into_inner();
    let visibility = match camera.mode {
        CameraMode::FirstPerson => Visibility::Hidden,
        CameraMode::ThirdPerson => Visibility::Inherited,
    };

    let height_scale = if stance.crouched {
        CharacterDimensions::CROUCH_HEIGHT / CharacterDimensions::HULL_HEIGHT
    } else {
        1.0
    };

    for (mut model_visibility, mut transform) in &mut models {
        *model_visibility = visibility;
        transform.scale = Vec3::new(1.0, height_scale, 1.0);
    }
}
