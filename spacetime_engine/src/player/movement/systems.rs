use bevy::prelude::*;
use crate::physics::components::*;	
use crate::player::components::Player;
use super::constants::*;

pub(in crate) fn update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut ProxyVelocity, With<Player>>,
) {
    let mut player_velocity = Vec2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::KeyW) {
        player_velocity.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        player_velocity.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        player_velocity.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player_velocity.x += 1.0;
    }

    for (mut velocity) in query.iter_mut() {
        velocity.linvel = player_velocity.normalize_or_zero() * SPEED;
    }
}