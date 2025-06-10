use bevy::prelude::*;
use core_engine_macros::{composite_workflow, composite_workflow_return};

use crate::{
    chunk_loader::components::ChunkLoaderComponent, config::statics::CONFIG, player::types::PlayerLifecycle, utils::InitHook,
    workflow::functions::handle_composite_workflow_return_now,
};

pub(super) fn update_player_system(
    mut commands: Commands,
    chunk_loader_init_hook_query: Query<(&ChunkLoaderComponent, &InitHook<ChunkLoaderComponent>)>,
    mut transform_query: Query<&mut Transform>,
    mut player_state: ResMut<PlayerLifecycle>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let can_player_transition_state = if let Some((_, init_hook)) = chunk_loader_init_hook_query.iter().find(|(l, _)| l.chunk_owner_id().id() == "player") {
        init_hook.has_fired()
    } else {
        true
    };

    match *player_state {
        PlayerLifecycle::None => {
            if can_player_transition_state && keys.just_pressed(KeyCode::Space) {
                let handle = composite_workflow!(move out entity: Entity, {
                    let spawn_player_output = workflow!(OE, Player::SpawnPlayer);
                    let entity = spawn_player_output.player_entity;
                });

                *player_state = PlayerLifecycle::Spawning(Some(handle));
            }
        }
        PlayerLifecycle::Spawning(ref mut handle) => {
            if handle.as_ref().unwrap().is_finished() {
                let handle = handle.take().unwrap();
                handle_composite_workflow_return_now(handle, |ctx| {
                    composite_workflow_return!(entity: Entity);
                    *player_state = PlayerLifecycle::PendingActivation(entity);
                });
            }
        }
        PlayerLifecycle::PendingActivation(entity) => {
            if transform_query.contains(entity) {
                *player_state = PlayerLifecycle::Active(entity);
            }
        }
        PlayerLifecycle::Active(entity) => {
            if can_player_transition_state && keys.just_pressed(KeyCode::Space) {
                commands.entity(entity).despawn_recursive();
                *player_state = PlayerLifecycle::None;
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
                *player_state = PlayerLifecycle::None;
                warn!("Player entity not found in update_player_system. The player entity should not be manually despawned! Resetting player state..",);
            }
        }
    }
}
