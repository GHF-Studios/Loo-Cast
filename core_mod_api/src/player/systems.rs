use bevy::prelude::*;
use core_mod_macros::{composite_workflow, composite_workflow_return};

use crate::{
    camera::resources::ZoomFactor, chunk_loader::components::ChunkLoader, config::statics::CONFIG, input::states::InputMode, player::resources::PlayerLifecycle, utils::lifecycle_hook::{DropHook, InitHook}, workflow::functions::handle_composite_workflow_return_now
};

#[tracing::instrument(skip_all)]
pub(super) fn update_player_system(
    chunk_loader_init_hook_query: Query<(&ChunkLoader, &InitHook<ChunkLoader>)>,
    chunk_loader_drop_hook_query: Query<(&ChunkLoader, &DropHook<ChunkLoader>)>,
    mut chunk_loader_query: Query<(&mut Transform, &mut ChunkLoader), (Without<InitHook<ChunkLoader>>, Without<DropHook<ChunkLoader>>)>,
    mut player_state_resource: ResMut<PlayerLifecycle>,
    keys: Res<ButtonInput<KeyCode>>,
    input_mode: Res<State<InputMode>>,
    time: Res<Time<Virtual>>,
    zoom_factor: Res<ZoomFactor>,
) {
    let is_player_update_allowed = {
        let condition_1 = if let Some((_, init_hook)) = chunk_loader_init_hook_query.iter().find(|(l, _)| l.id().id() == "player") {
            init_hook.has_fired()
        } else {
            true
        };
        let condition_2 = !chunk_loader_drop_hook_query.iter().any(|(l, _)| l.id().id() == "player");

        condition_1 && condition_2
    };

    let _fsm_span = trace_span!("state_machine").entered();

    let player_state = std::mem::take(&mut *player_state_resource);
    match player_state {
        PlayerLifecycle::None => {
            let _fsm_case_span = trace_span!("case: None").entered();

            if is_player_update_allowed && keys.just_pressed(KeyCode::F1) && input_mode.is_game() {
                let handle = composite_workflow!(SpawnPlayer, move out entity: Entity, {
                    warn!("Running composite workflow 'SpawnPlayer'");

                    let spawn_player_output = workflow!(OE, Player::SpawnPlayer);
                    let entity = spawn_player_output.player_entity;
                });

                *player_state_resource = PlayerLifecycle::Spawning(Some(handle));
            }
        }
        PlayerLifecycle::Spawning(handle) => {
            let _fsm_case_span = trace_span!("case: Spawning").entered();

            if let Some(handle) = handle {
                if !handle.is_finished() {
                    warn!("Waiting for composite workflow 'SpawnPlayer' to finish...");
                    *player_state_resource = PlayerLifecycle::Spawning(Some(handle));
                    return;
                }

                handle_composite_workflow_return_now(handle, |ctx| {
                    composite_workflow_return!(entity: Entity);
                    *player_state_resource = PlayerLifecycle::PendingActivation(entity);

                    warn!("Finished composite workflow 'SpawnPlayer'");
                });
            }
        }
        PlayerLifecycle::Despawning(handle) => {
            let _fsm_case_span = trace_span!("case: Despawning").entered();

            if let Some(handle) = handle {
                if !handle.is_finished() {
                    *player_state_resource = PlayerLifecycle::Despawning(Some(handle));
                    warn!("Waiting for composite workflow 'DespawnPlayer' to finish...");
                    return;
                }

                handle_composite_workflow_return_now(handle, |_ctx| {
                    composite_workflow_return!();
                    *player_state_resource = PlayerLifecycle::None;

                    warn!("Finished composite workflow 'DespawnPlayer'");
                });
            }
        }
        PlayerLifecycle::PendingActivation(entity) => {
            let _fsm_case_span = trace_span!("case: PendingActivation").entered();

            if chunk_loader_query.contains(entity) {
                *player_state_resource = PlayerLifecycle::Active(entity);
                warn!("Player entity is now active");
            } else {
                *player_state_resource = PlayerLifecycle::PendingActivation(entity);
                warn!("Player entity is pending activation, waiting for completion.");
            }
        }
        PlayerLifecycle::Active(entity) => {
            let _fsm_case_span = trace_span!("case: Active").entered();

            if !is_player_update_allowed {
                *player_state_resource = PlayerLifecycle::Active(entity);
                // warn!("Player update is not allowed at this time.");
                return;
            }

            if keys.just_pressed(KeyCode::F1) && input_mode.is_game() {
                let handle = composite_workflow!(DespawnPlayer, {
                    warn!("Running composite workflow 'DespawnPlayer'");

                    workflow!(E, Player::DespawnPlayer);
                });
                *player_state_resource = PlayerLifecycle::Despawning(Some(handle));
                return;
            }

            let _transformation_span = trace_span!("transformation").entered();
            if let Ok((mut transform, _chunk_loader)) = chunk_loader_query.get_mut(entity) {
                transform.scale = Vec3::splat(zoom_factor.0);

                let mut direction = Vec3::ZERO;

                if input_mode.is_game() {
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
                }

                if direction.length_squared() > 0.0 {
                    direction = direction.normalize();
                    let base_movement_speed = CONFIG().get::<f32>("player/base_movement_speed");
                    let sprint_multiplier = if keys.pressed(KeyCode::ShiftLeft) {
                        CONFIG().get::<f32>("player/sprint_multiplier")
                    } else {
                        1.0
                    };
                    transform.translation += direction * zoom_factor.0 * base_movement_speed * sprint_multiplier * time.delta_secs();
                }

                *player_state_resource = PlayerLifecycle::Active(entity);
            } else {
                // Entity not found? Maybe it was deleted outside this system.
                *player_state_resource = PlayerLifecycle::None;
                warn!("Player entity not found in update_player_system. The player entity should not be manually despawned! Resetting player lifecycle...");
            }

            let _scale_zoom_span = trace_span!("scale_zoom").entered();
            if let Ok((mut transform, mut chunk_loader)) = chunk_loader_query.get_mut(entity) {
                if input_mode.is_game() {
                    if keys.just_pressed(KeyCode::NumpadAdd) {
                        transform.translation = chunk_loader.suggest_zoom_in(transform.translation);
                    } else if keys.just_pressed(KeyCode::NumpadSubtract) {
                        chunk_loader.suggest_zoom_out();
                    }
                }
            }
        }
    };

    drop(_fsm_span);
}
