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
    let _system_span = info_span!("update_system").entered();

    let is_player_input_allowed = {
        let condition_1 = if let Some((_, init_hook)) = chunk_loader_init_hook_query.iter().find(|(l, _)| l.chunk_owner_id().id() == "player") {
            init_hook.has_fired()
        } else {
            true
        };
        let condition_2 = !chunk_loader_drop_hook_query.iter().any(|(l, _)| l.chunk_owner_id().id() == "player");

        condition_1 && condition_2
    };

    let _fsm_span = info_span!("state_machine").entered();

    let player_state = std::mem::take(&mut *player_state_resource);
    match player_state {
        PlayerLifecycle::None => {
            let _fsm_case_span = info_span!("current_state: None").entered();

            if is_player_input_allowed && keys.just_pressed(KeyCode::Space) {
                let handle = composite_workflow!(move out entity: Entity, {
                    let spawn_player_output = workflow!(OE, Player::SpawnPlayer);
                    let entity = spawn_player_output.player_entity;
                });

                *player_state_resource = PlayerLifecycle::Spawning(Some(handle));
                info!("Player entity is being spawned.");
            }
        }
        PlayerLifecycle::Spawning(handle) => {
            let _fsm_case_span = info_span!("current_state: Spawning").entered();

            if let Some(handle) = handle {
                if !handle.is_finished() {
                    info!("Player entity is still spawning, waiting for completion.");
                    return;
                }

                handle_composite_workflow_return_now(handle, |ctx| {
                    composite_workflow_return!(entity: Entity);
                    *player_state_resource = PlayerLifecycle::PendingActivation(entity);
                    info!("Player entity has been spawned: {:?}", entity);
                });
            }
        }
        PlayerLifecycle::Despawning(handle) => {
            let _fsm_case_span = info_span!("current_state: Despawning").entered();

            if let Some(handle) = handle {
                if !handle.is_finished() {
                    return;
                }

                handle_composite_workflow_return_now(handle, |_ctx| {
                    composite_workflow_return!();
                    *player_state_resource = PlayerLifecycle::None;
                info!("Player entity has been despawned.");
                });
            }
        }
        PlayerLifecycle::PendingActivation(entity) => {
            let _fsm_case_span = info_span!("current_state: PendingActivation").entered();

            if transform_query.contains(entity) {
                *player_state_resource = PlayerLifecycle::Active(entity);
                info!("Player entity is now active: {:?}", entity);
            }
        }
        PlayerLifecycle::Active(entity) => {
            let _fsm_case_span = info_span!("current_state: Active").entered();

            if !is_player_input_allowed {
                warn!("Player input is not allowed at this time. The player entity will not be updated.");
                return;
            }

            if keys.just_pressed(KeyCode::Space) {
                let handle = composite_workflow!({
                    workflow!(E, Player::DespawnPlayer);
                });
                *player_state_resource = PlayerLifecycle::Despawning(Some(handle));
                info!("Player entity is being despawned.");
                return;
            }
            let _fsm_case_span = info_span!("movement").entered();

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
    };

    drop(_fsm_span);
}
