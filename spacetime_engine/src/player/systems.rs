use bevy::prelude::*;
use spacetime_engine_macros::{workflow_path, run_workflow, define_composite_workflow};

use crate::config::statics::CONFIG;

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
    mut queue: ResMut<PlayerWorkflowQueue>
) {
    for workflow in queue.0.drain(..) {
        match workflow {
            PlayerWorkflow::Spawn => {
                define_composite_workflow!(Spawn {
                    #workflow(#path(Player::SpawnPlayer));
                })
            
                crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME.lock().unwrap().spawn_fallible(Box::pin(spawn()));
            }
            PlayerWorkflow::Despawn(entity) => {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
