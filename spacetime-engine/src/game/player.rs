//! Player entity and built-in first-person input.

use bevy::{
    camera::visibility::RenderLayers,
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
};

use super::{
    GameSet,
    combat::{FireWeapon, Weapon},
    portal::{
        MAIN_PORTAL_LAYER,
        PortalTraveler,
        PortalView,
    },
    target::SpawnTarget,
};

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerController {
    pub move_speed: f32,
    pub look_sensitivity: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            move_speed: 5.0,
            look_sensitivity: 0.002,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (look, movement)
                    .chain()
                    .in_set(GameSet::Input),
            )
            .add_systems(
                Update,
                (request_fire, request_target_spawn)
                    .in_set(GameSet::Input),
            );
    }
}

fn spawn_player(mut commands: Commands) {
    let position = Vec3::new(0.0, 1.5, 8.0);

    commands.spawn((
        Name::new("Player"),
        Player,
        PlayerController::default(),
        Weapon::default(),
        PortalView,
        PortalTraveler::new(position),
        Camera3d::default(),
        IsDefaultUiCamera,
        RenderLayers::layer(0).with(MAIN_PORTAL_LAYER),
        Transform::from_translation(position),
    ));
}

fn look(
    mouse: Res<AccumulatedMouseMotion>,
    player: Single<
        (&PlayerController, &mut Transform),
        With<Player>,
    >,
) {
    let (controller, mut transform) = player.into_inner();

    let (mut yaw, mut pitch, _) =
        transform.rotation.to_euler(EulerRot::YXZ);

    yaw -= mouse.delta.x * controller.look_sensitivity;
    pitch -= mouse.delta.y * controller.look_sensitivity;
    pitch = pitch.clamp(-1.5, 1.5);

    transform.rotation =
        Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<
        (&PlayerController, &mut Transform),
        With<Player>,
    >,
) {
    let (controller, mut transform) = player.into_inner();

    let forward = transform.rotation * Vec3::NEG_Z;
    let right = transform.rotation * Vec3::X;

    let forward =
        Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();

    let right =
        Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

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

fn request_fire(
    mouse: Res<ButtonInput<MouseButton>>,
    player: Single<Entity, With<Player>>,
    mut requests: MessageWriter<FireWeapon>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        requests.write(FireWeapon {
            wielder: *player,
        });
    }
}

fn request_target_spawn(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut requests: MessageWriter<SpawnTarget>,
) {
    if keyboard.just_pressed(KeyCode::KeyG) {
        requests.write(SpawnTarget);
    }
}