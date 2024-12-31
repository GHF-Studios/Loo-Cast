use bevy::prelude::*;

use crate::oneshot_systems::MainOneshotSystems;

use super::{components::PlayerComponent, constants::PLAYER_MOVEMENT_SPEED};

pub(in crate) fn update_player_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform), With<PlayerComponent>>,
    keys: Res<ButtonInput<KeyCode>>,
    main_oneshot_systems: Res<MainOneshotSystems>,
    time: Res<Time>,
) {
    if player_query.is_empty() {
        // Create player if no player exists and space has just been pressed
        if keys.just_pressed(KeyCode::Space) {
            let id = main_oneshot_systems.0["spawn_main_player"];
            commands.run_system(id);
        }
        return;
    }

    for (player_entity, mut transform) in player_query.iter_mut() {
        // Initialize a movement direction vector
        let mut direction = Vec3::ZERO;

        // Adjust direction based on key input
        if keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
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

        // Delete player if space has just been pressed
        if keys.just_pressed(KeyCode::Space) {
            commands.entity(player_entity).despawn_recursive();
        }
    }
}
