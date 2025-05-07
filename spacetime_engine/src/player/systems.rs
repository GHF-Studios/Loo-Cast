use bevy::prelude::*;
use spacetime_engine_macros::{composite_workflow, composite_workflow_return};
use tokio::task::JoinHandle;

use crate::{config::statics::CONFIG, workflow::{composite_workflow_context::ScopedCompositeWorkflowContext, functions::handle_composite_workflow_return_now}};

#[derive(Resource, Default)]
pub enum PlayerLifecycle {
    #[default]
    None,
    Spawning(Option<JoinHandle<ScopedCompositeWorkflowContext>>),
    PendingActivation(Entity),
    Active(Entity),
}
impl std::fmt::Debug for PlayerLifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerLifecycle::None => write!(f, "None"),
            PlayerLifecycle::Spawning(_) => write!(f, "Spawning"),
            PlayerLifecycle::PendingActivation(entity) => write!(f, "PendingActivation({:?})", entity),
            PlayerLifecycle::Active(entity) => write!(f, "Active({:?})", entity),
        }
    }
}

pub(crate) fn update_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerLifecycle>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
) {
    match *player_state {
        PlayerLifecycle::None => {
            if keys.just_pressed(KeyCode::Space) {
                let entity = Entity::from_raw(0);
                let handle = composite_workflow!(mut entity: Entity, JustDoIt {
                    let spawn_player_output = workflow!(OE, Player::SpawnPlayer);
                    entity = spawn_player_output.player_entity;
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
            println!("Pending activation of player entity: {:?}", entity);
            if transforms.contains(entity) {
                *player_state = PlayerLifecycle::Active(entity);
            }
        }
        PlayerLifecycle::Active(entity) => {
            if keys.just_pressed(KeyCode::Space) {
                commands.entity(entity).despawn_recursive();
                *player_state = PlayerLifecycle::None;
                return;
            }

            if let Ok(mut transform) = transforms.get_mut(entity) {
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
                    transform.translation +=
                        direction * CONFIG.get::<f32>("player/movement_speed") * time.delta_seconds();
                }
            } else {
                // Entity not found? Maybe it was deleted outside this system.
                *player_state = PlayerLifecycle::None;
                warn!(
                    "Player entity not found in update_player_system. The player entity should not be manually despawned! Resetting player state..",
                );
            }
        }
    }
}
