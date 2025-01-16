use bevy::prelude::*;

use crate::{config::statics::CONFIG, oneshot_systems::MainOneshotSystems};

use super::{components::PlayerComponent, resources::{PlayerAction, PlayerActionQueue}};

pub(in crate) fn update_player_system(
    mut queue: ResMut<PlayerActionQueue>,
    mut player_query: Query<(Entity, &mut Transform), With<PlayerComponent>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if player_query.is_empty() {
        if keys.just_pressed(KeyCode::Space) {
            queue.0.push(PlayerAction::Spawn);
        }
        return;
    }

    for (player_entity, mut transform) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;

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

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * CONFIG.get::<f32>("player/movement_speed") * time.delta_seconds();

        if keys.just_pressed(KeyCode::Space) {
            queue.0.push(PlayerAction::Despawn(player_entity));
        }
    }
}

pub(in crate) fn process_player_action_queue(
    mut commands: Commands,
    mut queue: ResMut<PlayerActionQueue>,
    main_oneshot_systems: Res<MainOneshotSystems>,
) {
    for action in queue.0.drain(..) {
        match action {
            PlayerAction::Spawn => {
                let id = main_oneshot_systems.0["spawn_main_player"];
                commands.run_system(id);
            }
            PlayerAction::Despawn(entity) => {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}