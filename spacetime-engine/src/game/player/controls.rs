use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
};

use super::{
    Player,
    PlayerController,
    cursor::CursorCapture,
};

pub fn look(
    mouse: Res<AccumulatedMouseMotion>,
    capture: Res<CursorCapture>,
    player: Single<
        (&PlayerController, &mut Transform),
        With<Player>,
    >,
) {
    if !capture.active() {
        return;
    }

    let (controller, mut transform) =
        player.into_inner();

    let (mut yaw, mut pitch, roll) =
        transform.rotation.to_euler(EulerRot::YXZ);

    yaw -= mouse.delta.x * controller.look_sensitivity;
    pitch -= mouse.delta.y * controller.look_sensitivity;
    pitch = pitch.clamp(-1.5, 1.5);

    transform.rotation =
        Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}

pub fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    time: Res<Time>,
    player: Single<
        (&PlayerController, &mut Transform),
        With<Player>,
    >,
) {
    if !capture.active() {
        return;
    }

    let (controller, mut transform) =
        player.into_inner();

    let forward =
        transform.rotation * Vec3::NEG_Z;

    let right =
        transform.rotation * Vec3::X;

    let forward =
        Vec3::new(forward.x, 0.0, forward.z)
            .normalize_or_zero();

    let right =
        Vec3::new(right.x, 0.0, right.z)
            .normalize_or_zero();

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction += forward;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        direction -= forward;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        direction += right;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        direction -= right;
    }

    transform.translation +=
        direction.normalize_or_zero()
            * controller.move_speed
            * time.delta_secs();
}
