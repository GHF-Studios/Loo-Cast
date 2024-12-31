use bevy::prelude::*;

use super::{components::PlayerComponent, constants::PLAYER_MOVEMENT_SPEED};

pub(in crate) fn update_player_system(
    mut player_query: Query<&mut Transform, With<PlayerComponent>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in player_query.iter_mut() {
        // Initialize a movement direction vector
        let mut direction = Vec3::ZERO;

        // Adjust direction based on key input
        if keys.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        // Normalize direction to prevent diagonal movement speed boost
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        // Apply movement based on speed, direction, and delta time
        transform.translation += direction * PLAYER_MOVEMENT_SPEED * time.delta_seconds();
    }
}
