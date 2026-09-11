use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
};

use crate::game::{
    combat::FireWeapon,
    target::SpawnTarget,
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

    // Preserve any roll introduced by arbitrary portal orientation.
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

    // Locomotion intentionally remains world-up. Arbitrary portal orientation
    // is supported independently from arbitrary gravity/walking surfaces.
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

pub fn request_fire(
    mouse: Res<ButtonInput<MouseButton>>,
    capture: Res<CursorCapture>,
    player: Single<Entity, With<Player>>,
    mut requests: MessageWriter<FireWeapon>,
) {
    if mouse.just_pressed(MouseButton::Left)
        && capture.accepts_gameplay_click()
    {
        requests.write(FireWeapon {
            wielder: *player,
        });
    }
}

pub fn request_target_spawn(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut requests: MessageWriter<SpawnTarget>,
) {
    if keyboard.just_pressed(KeyCode::KeyG) {
        requests.write(SpawnTarget);
    }
}
