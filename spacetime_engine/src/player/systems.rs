use std::panic;

use crate::chunk::actor::components::ChunkActor;
use crate::chunk::actor::events::*;
use crate::chunk::actor::position::structs::ChunkActorPosition;
use crate::chunk::actor::resources::ChunkActorRequestRegistry;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::loader::components::ChunkLoader;
use crate::chunk::loader::events::*;
use crate::chunk::loader::resources::ChunkLoaderRequestRegistry;
use crate::chunk::position::structs::ChunkPosition;
use crate::entity::resources::*;
use crate::physics::components::*;
use bevy::prelude::*;
use super::components::Player;
use super::events::*;
use super::resources::*;
use super::functions;
use super::constants::*;

pub(super) fn pre_start(
    mut create_player_entity_event_writer: EventWriter<CreatePlayerEntity>,
    mut player_request_registry: ResMut<PlayerRequestRegistry>,
) {
    let player_request_id = player_request_registry.get_unused_player_request_id();

    create_player_entity_event_writer.send(CreatePlayerEntity {
        player_request_id,
        world_position: Vec2::new(0.0, 0.0),
    });
}

pub(super) fn start_phase1(
    mut commands: Commands,
    mut created_player_entity_event_reader: EventReader<CreatedPlayerEntity>,
    mut promote__chunk_loader_entity_event_writer: EventWriter<PromoteToChunkLoaderEntity>,
    player_query: Query<Entity, Added<Player>>,
    mut chunk_loader_request_registry: ResMut<ChunkLoaderRequestRegistry>,
    entity_registry: Res<EntityRegistry>,
) {
    let mut created_player_entity_events = Vec::new();
    for created_player_entity_event in created_player_entity_event_reader.read() {
        created_player_entity_events.push(created_player_entity_event);
    }

    for created_player_entity_event in created_player_entity_events {
        info!("Starting player [Phase 1] ...");

        let (player_request_id, player_id, player_entity_id, world_position) = match created_player_entity_event {
            CreatedPlayerEntity::Success { player_request_id, player_id, player_entity_id, world_position } => {
                (player_request_id, player_id, player_entity_id, world_position)
            },
            CreatedPlayerEntity::Failure { player_request_id, world_position } => {
                panic!("The request for creating the player entity has been cancelled due to the player entity creation failing!");
            }
        };

        let player_entity_reference = match entity_registry.get_loaded_entity_reference(player_entity_id) {
            Some(player_entity_reference) => player_entity_reference,
            None => {
                panic!("The request for upgrading the player entity '{:?}' to a chunk loader entity has been cancelled due to the player entity reference not being found!", player_entity_id);
            }
        };

        if entity_registry.get_loaded_entity_id(&player_entity_reference).is_none() {
            panic!("The request for upgrading the player entity '{:?}' to a chunk loader entity has been cancelled due to the player entity id not being found!", player_entity_reference);
        }

        let chunk_loader_request_id = chunk_loader_request_registry.get_unused_chunk_loader_request_id();
        let player_entity_id = match entity_registry.get_loaded_entity_id(&player_entity_reference) {
            Some(player_entity_id) => player_entity_id,
            None => {
                // TODO: Make this better
                panic!("The request for upgrading the player entity '{:?}' to a chunk loader entity has been cancelled due to the player entity id not being found!", player_entity_reference);
            }
        };

        info!("Upgrading player '{:?}' to a chunk loader entity ...", player_entity_id);
        promote__chunk_loader_entity_event_writer.send(PromoteToChunkLoaderEntity {
            chunk_loader_request_id,
            target_entity_id: player_entity_id,
        });
    }

    for player_entity_reference in player_query.iter() {
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn start_phase2(
    mut promoted_to_chunk_loader_entity_event_reader: EventReader<PromotedToChunkLoaderEntity>,
    mut promote__chunk_actor_entity_event_writer: EventWriter<PromoteToChunkActorEntity>,
    player_query: Query<(Entity, &Transform), (With<Player>, With<ChunkLoader>)>,
    mut chunk_actor_request_registry: ResMut<ChunkActorRequestRegistry>,
    entity_registry: Res<EntityRegistry>,
) {
    let mut promoted_to_chunk_loader_entity_events = Vec::new();
    for promoted_to_chunk_loader_entity_event in promoted_to_chunk_loader_entity_event_reader.read() {
        promoted_to_chunk_loader_entity_events.push(promoted_to_chunk_loader_entity_event);
    }

    'outer: for promoted_to_chunk_loader_entity_event in promoted_to_chunk_loader_entity_events {
        info!("Starting player [Phase 2] ...");

        let (_, _, target_entity_id) = match promoted_to_chunk_loader_entity_event {
            PromotedToChunkLoaderEntity::Success { chunk_loader_request_id, chunk_loader_id, target_entity_id } => {
                (chunk_loader_request_id, chunk_loader_id, target_entity_id)
            },
            PromotedToChunkLoaderEntity::Failure { target_entity_id, .. } => {
                // TODO: Make this better
                panic!("The request for upgrading the player entity '{:?}' to a chunk loader entity has been cancelled due to the promote failing!", target_entity_id);
            }
        };

        for (player_entity_reference, player_transform) in player_query.iter() {
            let player_entity_id = match entity_registry.get_loaded_entity_id(&player_entity_reference) {
                Some(player_entity_id) => player_entity_id,
                None => { 
                    // TODO: Make this better
                    panic!("The request for upgrading the player entity '{:?}' to a chunk loader entity has been cancelled due to the player entity id not being found!", player_entity_reference);
                }
            };

            if player_entity_id != *target_entity_id {
                continue;
            }

            let player_position = player_transform.translation.truncate();
            let player_chunk_actor_position: ChunkActorPosition = player_position.into();
            let player_chunk_position: ChunkPosition = player_chunk_actor_position.into();
            let player_chunk_id: ChunkID = player_chunk_position.into();

            let chunk_actor_request_id = chunk_actor_request_registry.get_unused_chunk_actor_request_id();

            info!("Upgrading player entity '{:?}' to a chunk actor entity in chunk '{:?}' ...", player_entity_id, player_chunk_id);
            promote__chunk_actor_entity_event_writer.send(PromoteToChunkActorEntity {
                chunk_actor_request_id,
                target_entity_id: player_entity_id,
            });

            continue 'outer;
        }

        continue;
    }
}

pub(super) fn start_phase3(
    mut commands: Commands,
    mut promoted_to_chunk_actor_entity_event_reader: EventReader<PromotedToChunkActorEntity>,
    mut started_player_event_writer: EventWriter<StartedPlayer>,
    player_query: Query<(Entity, &Transform, &Player), (With<ChunkLoader>, With<ChunkActor>)>,
    entity_registry: Res<EntityRegistry>,
    mut player_request_registry: ResMut<PlayerRequestRegistry>,
) {
    let mut promoted_to_chunk_actor_entity_events = Vec::new();
    for promoted_to_chunk_actor_entity_event in promoted_to_chunk_actor_entity_event_reader.read() {
        promoted_to_chunk_actor_entity_events.push(promoted_to_chunk_actor_entity_event);
    }

    'outer: for promoted_to_chunk_actor_entity_event in promoted_to_chunk_actor_entity_events {
        info!("Starting player [Phase 3] ...");

        let (_, _, target_entity_id, _) = match promoted_to_chunk_actor_entity_event { 
            PromotedToChunkActorEntity::Success { chunk_actor_request_id, chunk_actor_id, target_entity_id, chunk_id, world_position: _ } => {
                (chunk_actor_request_id, chunk_actor_id, target_entity_id, chunk_id)
            },
            PromotedToChunkActorEntity::Failure { target_entity_id, .. } => {
                // TODO: Make this better
                panic!("The request for upgrading the player entity '{:?}' to a chunk actor entity has been cancelled due to the promote failing!", target_entity_id);
            }
        };

        for (player_entity_reference, player_transform, player) in player_query.iter() {
            let player_entity_id = match entity_registry.get_loaded_entity_id(&player_entity_reference) {
                Some(player_entity_id) => player_entity_id,
                None => { 
                    // TODO: Make this better
                    panic!("Starting the player has been cancelled due to the player entity '{:?}' id not being found!", player_entity_reference);
                }
            };

            if player_entity_id != *target_entity_id {
                continue;
            }

            info!("Adding secondary components to player entity '{:?}' ...", player_entity_id);
            commands
                .entity(player_entity_reference)
                .insert(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.1, 0.1, 1.0),
                        custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                        ..default()
                    },
                    transform: *player_transform,
                    ..default()
                })
                .insert(ProxyRigidBody::Dynamic)
                .insert(ProxyCollider::Circle { radius: 15.0 })
                .insert(ProxyVelocity::linear(Vec2::new(0.0, 0.0)));

            let player_request_id = player_request_registry.get_unused_player_request_id();

            info!("Successfully started player '{:?}'!", player_entity_id);

            started_player_event_writer.send(StartedPlayer::Success {
                player_request_id,
                player_id: player.id
            });

            continue 'outer;
        }

        continue;
    }

}

pub(in crate) fn change_player_chunk_load_radius(
    mut chunk_loader_query: Query<(&mut ChunkLoader, &Player)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut chunk_loader, _) in chunk_loader_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            *chunk_loader.load_radius_mut() = (chunk_loader.load_radius() as i16 - 1).max(0) as u16;
        }
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            *chunk_loader.load_radius_mut() = (chunk_loader.load_radius() as i16 + 1) as u16;
        }
    }
}

pub(super) fn handle_create_player_entity_events(
    mut commands: Commands,
    mut create_player_entity_event_reader: EventReader<CreatePlayerEntity>,
    mut created_player_entity_event_writer: EventWriter<CreatedPlayerEntity>,
    mut player_registry: ResMut<PlayerRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    let mut create_player_entity_events = Vec::new();
    for create_player_entity_event in create_player_entity_event_reader.read() {
        create_player_entity_events.push(create_player_entity_event);
    }

    for create_player_entity_event in create_player_entity_events {
        let player_request_id = create_player_entity_event.player_request_id;
        let player_entity_id = entity_registry.register_entity();
        let player_id = player_registry.register_player();
        let world_position = create_player_entity_event.world_position;

        info!("Creating player entity '{:?}' at world position '{:?}'...", player_entity_id, world_position);

        let player_entity_reference = functions::new_player_entity(&mut commands, player_id, world_position);

        info!("Successfully created player entity '{:?}' at world position '{:?}'!", player_entity_reference, world_position);

        entity_registry.load_entity(player_entity_id, player_entity_reference);
        player_registry.load_player(player_id, player_entity_reference);

        created_player_entity_event_writer.send(CreatedPlayerEntity::Success {
            player_request_id,
            player_id,
            player_entity_id,
            world_position
        });
    }
}

pub(super) fn handle_destroy_player_entity_events(
    mut commands: Commands,
    mut destroy_player_entity_event_reader: EventReader<DestroyPlayerEntity>,
    mut destroyed_player_entity_event_writer: EventWriter<DestroyedPlayerEntity>,
    mut player_registry: ResMut<PlayerRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    let mut destroy_player_entity_events = Vec::new();
    for destroy_player_entity_event in destroy_player_entity_event_reader.read() {
        destroy_player_entity_events.push(destroy_player_entity_event);
    }

    for destroy_player_entity_event in destroy_player_entity_events {
        let player_request_id = destroy_player_entity_event.player_request_id;
        let player_id = destroy_player_entity_event.player_id;

        let player_entity_reference = match player_registry.get_loaded_player(player_id) {
            Some(player_entity) => player_entity,
            None => {
                error!("The request for destroying the player entity '{:?}' has been cancelled due to the player not being loaded!", player_id);

                destroyed_player_entity_event_writer.send(DestroyedPlayerEntity::Failure {
                    player_request_id,
                    player_id
                });

                continue;
            }
        };

        let player_entity_id = match entity_registry.get_loaded_entity_id(&player_entity_reference) {
            Some(player_entity_id) => player_entity_id,
            None => {
                error!("The request for destroying the player entity '{:?}' has been cancelled due to the respective player entity id not being found!", player_id);

                destroyed_player_entity_event_writer.send(DestroyedPlayerEntity::Failure {
                    player_request_id,
                    player_id
                });

                continue;
            }
        };

        player_registry.unload_player(player_id);
        entity_registry.unload_entity(player_entity_id);

        player_registry.unregister_player(player_id);
        entity_registry.unregister_entity(player_entity_id);

        commands.entity(player_entity_reference).despawn();

        destroyed_player_entity_event_writer.send(DestroyedPlayerEntity::Success {
            player_request_id,
            player_id,
        });
    }
}

pub(super) fn handle_promote__player_entity_events(
    mut commands: Commands,
    mut promote__player_entity_event_reader: EventReader<PromoteToPlayerEntity>,
    mut promoted_to_player_entity_event_writer: EventWriter<PromotedToPlayerEntity>,
    mut player_registry: ResMut<PlayerRegistry>,
    entity_registry: Res<EntityRegistry>,
    mut ineligible_entity_query_0: Query<Entity, Without<Transform>>,
    mut ineligible_entity_query_1: Query<Entity, With<Player>>,
    mut eligible_entity_query: Query<Entity, (With<Transform>, Without<Player>)>,
) {
    let mut promote__player_entity_events = Vec::new();
    for promote__player_entity_event in promote__player_entity_event_reader.read() {
        promote__player_entity_events.push(promote__player_entity_event);
    }

    for promote__player_entity_event in promote__player_entity_events {
        let player_request_id = promote__player_entity_event.player_request_id;
        let target_entity_id = promote__player_entity_event.target_entity_id;
        let player_id = player_registry.register_player();

        info!("Upgrading entity '{:?}' to a player entity '{:?}'...", target_entity_id, player_id);

        let target_entity_reference = match entity_registry.get_loaded_entity_reference(&target_entity_id) {
            Some(target_entity) => target_entity,
            None => {
                error!("The request for upgrading entity '{:?}' to a player entity has been cancelled due to the entity reference not being found!", target_entity_id);

                player_registry.unregister_player(player_id);

                promoted_to_player_entity_event_writer.send(PromotedToPlayerEntity::Failure {
                    player_request_id,
                    target_entity_id,
                });

                continue;
            }
        };

        let player_entity_reference = match functions::promote__player_entity(
            &mut commands, 
            player_id, 
            target_entity_reference,
            &mut ineligible_entity_query_0,
            &mut ineligible_entity_query_1,
            &mut eligible_entity_query
        ) {
            Ok(player_entity_reference) => player_entity_reference,
            Err(_) => {
                error!("The request for upgrading entity '{:?}' to a player entity has been cancelled due to the promote failing!", target_entity_id);

                player_registry.unregister_player(player_id);

                promoted_to_player_entity_event_writer.send(PromotedToPlayerEntity::Failure {
                    player_request_id,
                    target_entity_id,
                });

                continue;
            }
        
        };

        player_registry.load_player(player_id, player_entity_reference);

        promoted_to_player_entity_event_writer.send(PromotedToPlayerEntity::Success {
            player_request_id,
            player_id,
            target_entity_id,
        });
    }
}
