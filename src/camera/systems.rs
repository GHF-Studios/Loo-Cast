use crate::math::*;
use crate::player::components::*;

use bevy::prelude::*;

const CAMERA_SPEED: f32 = 10.0;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

pub fn lerp_to_player(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    let interpolation = CAMERA_SPEED * time.delta_seconds();

    let new_x = lerp(
        camera_transform.translation.x,
        player_transform.translation.x,
        interpolation,
    );
    let new_y = lerp(
        camera_transform.translation.y,
        player_transform.translation.y,
        interpolation,
    );

    camera_transform.translation.x = new_x;
    camera_transform.translation.y = new_y;
}
