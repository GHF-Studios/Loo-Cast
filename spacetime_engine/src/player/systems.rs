use bevy::prelude::*;

use crate::{config::statics::CONFIG, oneshot_systems::MainOneshotSystems};

use super::{
    components::PlayerComponent,
    resources::{PlayerWorkflow, PlayerWorkflowQueue},
};

pub(crate) fn update_player_system(
    mut queue: ResMut<PlayerWorkflowQueue>,
    mut player_query: Query<(Entity, &mut Transform), With<PlayerComponent>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if player_query.is_empty() {
        if keys.just_pressed(KeyCode::Space) {
            queue.0.push(PlayerWorkflow::Spawn);
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

        transform.translation +=
            direction * CONFIG.get::<f32>("player/movement_speed") * time.delta_seconds();

        if keys.just_pressed(KeyCode::Space) {
            queue.0.push(PlayerWorkflow::Despawn(player_entity));
        }
    }
}

pub(crate) fn process_player_workflow_queue(
    mut commands: Commands,
    mut queue: ResMut<PlayerWorkflowQueue>,
    main_oneshot_systems: Res<MainOneshotSystems>,
) {
    for workflow in queue.0.drain(..) {
        match workflow {
            PlayerWorkflow::Spawn => {
                let id = main_oneshot_systems.0["spawn_main_player"];
                commands.run_system(id);
            }
            PlayerWorkflow::Despawn(entity) => {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
