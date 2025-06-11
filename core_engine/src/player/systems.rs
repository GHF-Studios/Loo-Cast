use bevy::prelude::*;
use core_engine_macros::{composite_workflow, composite_workflow_return};

use crate::{
    chunk_loader::components::ChunkLoader,
    config::statics::CONFIG,
    player::types::PlayerLifecycle,
    utils::{DropHook, InitHook},
    workflow::functions::handle_composite_workflow_return_now,
};

pub(super) fn update_player_system(
    chunk_loader_init_hook_query: Query<(&ChunkLoader, &InitHook<ChunkLoader>)>,
    chunk_loader_drop_hook_query: Query<(&ChunkLoader, &DropHook<ChunkLoader>)>,
    mut transform_query: Query<&mut Transform>,
    mut player_state_resource: ResMut<PlayerLifecycle>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let is_player_input_allowed = {
        let condition_1 = if let Some((_, init_hook)) = chunk_loader_init_hook_query.iter().find(|(l, _)| l.chunk_owner_id().id() == "player") {
            init_hook.has_fired()
        } else {
            true
        };
        let condition_2 = !chunk_loader_drop_hook_query.iter().any(|(l, _)| l.chunk_owner_id().id() == "player");

        condition_1 && condition_2
    };

    let player_state = std::mem::take(&mut *player_state_resource);

    match player_state {
        PlayerLifecycle::None => {
            if is_player_input_allowed && keys.just_pressed(KeyCode::Space) {
                let handle = composite_workflow!(move out entity: Entity, {
                    let spawn_player_output = workflow!(OE, Player::SpawnPlayer);
                    let entity = spawn_player_output.player_entity;
                });

                *player_state_resource = PlayerLifecycle::Spawning(Some(handle));
            }
        }
        PlayerLifecycle::Spawning(handle) => {
            if let Some(handle) = handle {
                if !handle.is_finished() {
                    return;
                }

                handle_composite_workflow_return_now(handle, |ctx| {
                    composite_workflow_return!(entity: Entity);
                    *player_state_resource = PlayerLifecycle::PendingActivation(entity);
                });
            }
        }
        PlayerLifecycle::Despawning(handle) => {
            if let Some(handle) = handle {
                if !handle.is_finished() {
                    return;
                }

                handle_composite_workflow_return_now(handle, |_ctx| {
                    composite_workflow_return!();
                    *player_state_resource = PlayerLifecycle::None;
                });
            }
        }
        PlayerLifecycle::PendingActivation(entity) => {
            if transform_query.contains(entity) {
                *player_state_resource = PlayerLifecycle::Active(entity);
            }
        }
        PlayerLifecycle::Active(entity) => {
            if !is_player_input_allowed {
                return;
            }

            if keys.just_pressed(KeyCode::Space) {
                let handle = composite_workflow!({
                    workflow!(E, Player::DespawnPlayer);
                });
                *player_state_resource = PlayerLifecycle::Despawning(Some(handle));
                return;
            }

            if let Ok(mut transform) = transform_query.get_mut(entity) {
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
                    let movement_speed = CONFIG.get::<f32>("player/movement_speed");
                    let sprint_multiplier = if keys.pressed(KeyCode::ShiftLeft) {
                        CONFIG.get::<f32>("player/sprint_multiplier")
                    } else {
                        1.0
                    };
                    transform.translation += direction * movement_speed * sprint_multiplier * time.delta_seconds();
                }
            } else {
                // Entity not found? Maybe it was deleted outside this system.
                *player_state_resource = PlayerLifecycle::None;
                warn!("Player entity not found in update_player_system. The player entity should not be manually despawned! Resetting player state..",);
            }
        }
    }
}
