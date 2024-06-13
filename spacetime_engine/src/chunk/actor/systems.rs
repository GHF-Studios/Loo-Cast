use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::events::CreatedChunkEntity;
use crate::chunk::actor::components::*;
use crate::chunk::actor::resources::*;
use crate::chunk::actor::structs::*;
use crate::chunk::components::*;
use crate::chunk::resources::*;
use crate::entity::resources::EntityRegistry;
use super::events::*;
use super::functions;

pub(super) fn start(
    mut started_chunk_actor_event_writer: EventWriter<StartChunkActorResult>,
    chunk_actor_query: Query<&ChunkActor, Added<ChunkActor>>,
) {
    for chunk_actor in chunk_actor_query.iter() {
        started_chunk_actor_event_writer.send(StartChunkActorResult::Success {
            chunk_actor_id: chunk_actor.id(),
        });
    }
}

pub(super) fn update(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
) {
    let (updates, despawns) = functions::collect_chunk_actor_updates(world, registry_parameters);

    functions::apply_chunk_actor_updates(
        world,
        registry_parameters,
        updates,
        despawns,
    );
}

pub(super) fn handle_create_chunk_actor_entity_events(
    mut commands: Commands,
    mut create_chunk_actor_entity_event_reader: EventReader<CreateChunkActorEntity>,
    mut created_chunk_actor_entity_event_writer: EventWriter<CreateChunkActorEntityResult>,
    chunk_registry: ResMut<ChunkRegistry>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
    mut chunk_query: Query<&mut Chunk>,
) {
    let mut create_chunk_actor_entity_events = Vec::new();
    for create_chunk_actor_entity_event in create_chunk_actor_entity_event_reader.read() {
        create_chunk_actor_entity_events.push(create_chunk_actor_entity_event.clone());
    }

    for create_chunk_actor_entity_event in create_chunk_actor_entity_events {
        let chunk_actor_entity_id = entity_registry.register_entity();
        let chunk_actor_id = chunk_actor_registry.register_chunk_actor();
        let chunk_id = create_chunk_actor_entity_event.chunk_id;
        let world_position = create_chunk_actor_entity_event.world_position;

        info!("Trying to create chunk actor entity '{:?}' ...", chunk_actor_entity_id);
        
        if let Some(chunk_entity) = chunk_registry.get_loaded_chunk_entity(chunk_id) {
            info!("Chunk loaded, creating chunk actor entity '{:?}' immediately ...", chunk_actor_entity_id);

            let mut chunk = match chunk_query.get_mut(chunk_entity) {
                Ok(chunk) => chunk,
                Err(_) => {
                    panic!("Chunk entity '{:?}' is loaded, but the chunk query failed to get the chunk!", chunk_entity);
                }
            };

            let chunk_actor_entity_reference = functions::new_chunk_actor_entity(&mut commands, chunk_actor_id, chunk_id, world_position);

            entity_registry.load_entity(chunk_actor_entity_id, chunk_actor_entity_reference);
            chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity_reference);

            chunk.add_chunk_actor(chunk_actor_id);

            created_chunk_actor_entity_event_writer.send(CreateChunkActorEntityResult::Success {
                chunk_actor_id,
                chunk_actor_entity_id,
                chunk_id,
                world_position,
            });
        } else {
            info!("Chunk not loaded, issuing request to create chunk actor entity '{:?}' when the chunk is loaded ...", chunk_actor_entity_id);

            if chunk_actor_registry.is_chunk_actor_entity_creating(chunk_actor_id) {
                error!("The request for creating chunk actor entity '{:?}' has already been issued!", chunk_actor_entity_id);

                chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);
                entity_registry.unregister_entity(chunk_actor_entity_id);

                created_chunk_actor_entity_event_writer.send(CreateChunkActorEntityResult::Failure {
                    chunk_id,
                    world_position,
                });

                continue;
            }
            
            chunk_actor_registry.start_creating_chunk_actor_entity(
                CreateChunkActorEntityRequest {
                    chunk_actor_id,
                    chunk_actor_entity_id,
                    chunk_id,
                    world_position,
                }
            );
        }
    }
}

pub(super) fn handle_destroy_chunk_actor_entity_events(
    mut commands: Commands,
    mut destroy_chunk_actor_entity_event_reader: EventReader<DestroyChunkActorEntity>,
    mut destroyed_chunk_actor_entity_event_writer: EventWriter<DestroyChunkActorEntityResult>,
    chunk_registry: ResMut<ChunkRegistry>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
    mut chunk_query: Query<&mut Chunk>,
    chunk_actor_query: Query<&ChunkActor>
) {
    let mut destroy_chunk_actor_entity_events = Vec::new();
    for destroy_chunk_actor_entity_event in destroy_chunk_actor_entity_event_reader.read() {
        destroy_chunk_actor_entity_events.push(destroy_chunk_actor_entity_event.clone());
    }

    for destroy_chunk_actor_entity_event in destroy_chunk_actor_entity_events {
        let chunk_actor_id = destroy_chunk_actor_entity_event.chunk_actor_id;

        let chunk_actor_entity_reference = match chunk_actor_registry.get_loaded_chunk_actor(chunk_actor_id) {
            Some(chunk_actor_entity) => chunk_actor_entity,
            None => {
                error!("Cannot destroy chunk actor entity '{:?}' because it is not loaded!", chunk_actor_id);

                destroyed_chunk_actor_entity_event_writer.send(DestroyChunkActorEntityResult::Failure { chunk_actor_id });

                continue;
            }
        };

        let chunk_actor = match chunk_actor_query.get(chunk_actor_entity_reference) {
            Ok(chunk_actor) => chunk_actor,
            Err(_) => {
                panic!("Chunk actor entity '{:?}' is loaded, but the chunk query failed to get the chunk actor!", chunk_actor_entity_reference);
            }
        };

        let chunk_actor_entity_id = match entity_registry.get_loaded_entity_id(&chunk_actor_entity_reference) {
            Some(chunk_actor_entity_id) => chunk_actor_entity_id,
            None => {
                panic!("Chunk actor entity '{:?}' is loaded, but the entity registry failed to get the entity id!", chunk_actor_entity_reference);
            }
        };

        let chunk_id = chunk_actor.current_chunk();

        let chunk_entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => {
                error!("Cannot destroy chunk actor entity '{:?}', because the chunk '{:?}' is not loaded!", chunk_actor_id, chunk_id);

                destroyed_chunk_actor_entity_event_writer.send(DestroyChunkActorEntityResult::Failure { chunk_actor_id });

                continue;
            }
        };

        let mut chunk = match chunk_query.get_mut(chunk_entity_reference) {
            Ok(chunk) => chunk,
            Err(_) => {
                panic!("Chunk entity '{:?}' is loaded, but the chunk query failed to get the chunk!", chunk_entity_reference);
            }
        };

        chunk.remove_chunk_actor(chunk_actor_id);

        chunk_actor_registry.unload_chunk_actor(chunk_actor_id);
        entity_registry.unload_entity(chunk_actor_entity_id);

        chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);
        entity_registry.unregister_entity(chunk_actor_entity_id);

        commands.entity(chunk_actor_entity_reference).despawn();

        destroyed_chunk_actor_entity_event_writer.send(DestroyChunkActorEntityResult::Success {
            chunk_actor_id,
        });
    }
}

pub(super) fn handle_upgrade_to_chunk_actor_entity_events(
    mut commands: Commands,
    mut upgrade_to_chunk_actor_entity_event_reader: EventReader<UpgradeToChunkActorEntity>,
    mut upgraded_to_chunk_actor_entity_event_writer: EventWriter<UpgradeToChunkActorEntityResult>,
    chunk_registry: ResMut<ChunkRegistry>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    entity_registry: Res<EntityRegistry>,
    mut chunk_query: Query<&mut Chunk>,
    mut ineligible_entity_query_0: Query<Entity, Without<Transform>>,
    mut ineligible_entity_query_1: Query<Entity, With<ChunkActor>>,
    mut eligible_entity_query: Query<Entity, (With<Transform>, Without<ChunkActor>)>,
) {
    let mut upgrade_to_chunk_actor_entity_events = Vec::new();
    for upgrade_to_chunk_actor_entity_event in upgrade_to_chunk_actor_entity_event_reader.read() {
        upgrade_to_chunk_actor_entity_events.push(upgrade_to_chunk_actor_entity_event.clone());
    }

    for upgrade_to_chunk_actor_entity_event in upgrade_to_chunk_actor_entity_events {
        let target_entity_id = upgrade_to_chunk_actor_entity_event.target_entity_id;
        let chunk_actor_id = chunk_actor_registry.register_chunk_actor();
        let chunk_id = upgrade_to_chunk_actor_entity_event.chunk_id;

        info!("Trying to upgrade entity '{:?}' to a chunk actor entity ...", target_entity_id);

        if let Some(chunk_entity) = chunk_registry.get_loaded_chunk_entity(chunk_id) {
            info!("Chunk loaded, upgrading entity '{:?}' to a chunk actor entity immediately ...", target_entity_id);

            let mut chunk = match chunk_query.get_mut(chunk_entity) {
                Ok(chunk) => chunk,
                Err(_) => {
                    panic!("Chunk entity '{:?}' is loaded, but the chunk query failed to get the chunk!", chunk_entity);
                }
            };

            let target_entity_reference = match entity_registry.get_loaded_entity_reference(&target_entity_id) {
                Some(target_entity) => target_entity,
                None => {
                    panic!("Entity '{:?}' is loaded, but the entity registry failed to get the entity!", target_entity_id);
                }
            };

            let chunk_actor_entity_reference = match functions::upgrade_to_chunk_actor_entity(
                &mut commands, 
                chunk_actor_id, 
                chunk_id, 
                target_entity_reference,
                &mut ineligible_entity_query_0,
                &mut ineligible_entity_query_1,
                &mut eligible_entity_query
            ) {
                Ok(chunk_actor_entity_reference) => chunk_actor_entity_reference,
                Err(_) => {
                    error!("Failed to upgrade entity '{:?}' to a chunk actor entity!", target_entity_id);

                    chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradeToChunkActorEntityResult::Failure {
                        target_entity_id,
                        chunk_id,
                    });

                    continue;
                }
            
            };

            chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity_reference);

            chunk.add_chunk_actor(chunk_actor_id);

            upgraded_to_chunk_actor_entity_event_writer.send(UpgradeToChunkActorEntityResult::Success {
                chunk_actor_id,
                target_entity_id,
                chunk_id,
            });
        } else {
            info!("Chunk not loaded, issuing request to upgrade entity '{:?}' to a chunk actor entity when the chunk is loaded ...", target_entity_id);

            if chunk_actor_registry.is_chunk_actor_entity_upgraded_to(chunk_actor_id) {
                error!("The request for upgrading entity '{:?}' to a chunk actor entity has already been issued!", target_entity_id);

                chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);

                upgraded_to_chunk_actor_entity_event_writer.send(UpgradeToChunkActorEntityResult::Failure {
                    target_entity_id,
                    chunk_id,
                });

                continue;
            }
            
            chunk_actor_registry.start_upgrading_to_chunk_actor_entity(
                UpgradeToChunkActorEntityRequest {
                    chunk_actor_id,
                    target_entity_id,
                    chunk_id,
                }
            );
        }
    }
}

pub(super) fn process_create_chunk_actor_entity_requests(
    mut commands: Commands,
    mut created_chunk_entity_event_reader: EventReader<CreatedChunkEntity>,
    mut created_chunk_actor_entity_event_writer: EventWriter<CreateChunkActorEntityResult>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    chunk_registry: ResMut<ChunkRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
    mut chunk_query: Query<&mut Chunk>,
) {
    let mut created_chunk_entity_events = Vec::new();
    for created_chunk_entity_event in created_chunk_entity_event_reader.read() {
        created_chunk_entity_events.push(created_chunk_entity_event.clone());
    }

    for created_chunk_entity_event in created_chunk_entity_events {
        let chunk_id = created_chunk_entity_event.chunk_id;
        let success = created_chunk_entity_event.success;

        if !success {
            let requests = chunk_actor_registry.create_chunk_actor_entity_requests().clone();
            for request in requests.values() {
                if request.chunk_id != chunk_id {
                    warn!("The creation request for chunk actor entity '{:?}' has been cancelled due to the starting chunk '{:?}' failing to load!", request.chunk_actor_entity_id, request.chunk_id);

                    entity_registry.unregister_entity(request.chunk_actor_entity_id);
                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_creating_chunk_actor_entity(request.chunk_actor_id);

                    created_chunk_actor_entity_event_writer.send(CreateChunkActorEntityResult::Failure {
                        chunk_id: request.chunk_id,
                        world_position: request.world_position,
                    });
                }
            }

            continue;
        }

        let chunk_entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => {
                panic!("Chunk '{:?}' is loaded, but the chunk registry failed to get the chunk entity!", chunk_id);
            }
        };

        let mut chunk = match chunk_query.get_mut(chunk_entity_reference) {
            Ok(chunk) => chunk,
            Err(_) => {
                panic!("Chunk entity '{:?}' is loaded, but the chunk query failed to get the chunk!", chunk_entity_reference);
            }
        };

        let create_chunk_actor_entity_requests = chunk_actor_registry.create_chunk_actor_entity_requests().clone();
        for create_chunk_actor_entity_request in create_chunk_actor_entity_requests.values() {
            let chunk_actor_id = create_chunk_actor_entity_request.chunk_actor_id;
            let chunk_actor_entity_id = create_chunk_actor_entity_request.chunk_actor_entity_id;
            let chunk_id = create_chunk_actor_entity_request.chunk_id;
            let world_position = create_chunk_actor_entity_request.world_position;

            if chunk_id != chunk_id {
                continue;
            }

            let chunk_actor_entity = functions::new_chunk_actor_entity(&mut commands, chunk_actor_id, chunk_id, world_position);

            entity_registry.load_entity(chunk_actor_entity_id, chunk_actor_entity);
            chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity);

            chunk.add_chunk_actor(chunk_actor_id);

            chunk_actor_registry.stop_creating_chunk_actor_entity(chunk_actor_id);

            created_chunk_actor_entity_event_writer.send(CreateChunkActorEntityResult::Success {
                chunk_actor_id,
                chunk_actor_entity_id,
                chunk_id,
                world_position,
            });
        }
    }
}

pub(super) fn process_upgrade_to_chunk_actor_requests(
    mut commands: Commands,
    mut created_chunk_entity_event_reader: EventReader<CreatedChunkEntity>,
    mut upgraded_to_chunk_actor_entity_event_writer: EventWriter<UpgradeToChunkActorEntityResult>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    chunk_registry: ResMut<ChunkRegistry>,
    entity_registry: Res<EntityRegistry>,
    mut chunk_query: Query<&mut Chunk>,
    mut ineligible_entity_query_0: Query<Entity, Without<Transform>>,
    mut ineligible_entity_query_1: Query<Entity, With<ChunkActor>>,
    mut eligible_entity_query: Query<Entity, (With<Transform>, Without<ChunkActor>)>,
) {
    let mut created_chunk_entity_events = Vec::new();
    for created_chunk_entity_event in created_chunk_entity_event_reader.read() {
        created_chunk_entity_events.push(created_chunk_entity_event.clone());
    }

    for created_chunk_entity_event in created_chunk_entity_events {
        let chunk_id = created_chunk_entity_event.chunk_id;
        let success = created_chunk_entity_event.success;

        if !success {
            let requests = chunk_actor_registry.upgrade_to_chunk_actor_entity_requests().clone();
            for request in requests.values() {
                if request.chunk_id != chunk_id {
                    warn!("The chunk actor entity conversion request for entity '{:?}' has been cancelled due to the starting chunk '{:?}' failing to load!", request.target_entity_id, request.chunk_id);

                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_creating_chunk_actor_entity(request.chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradeToChunkActorEntityResult::Failure {
                        target_entity_id: request.target_entity_id,
                        chunk_id: request.chunk_id,
                    });
                }
            }

            continue;
        }

        let chunk_entity = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => {
                panic!("Chunk '{:?}' is loaded, but the chunk registry failed to get the chunk entity!", chunk_id);
            }
        };

        let mut chunk = match chunk_query.get_mut(chunk_entity) {
            Ok(chunk) => chunk,
            Err(_) => {
                panic!("Chunk entity '{:?}' is loaded, but the chunk query failed to get the chunk!", chunk_entity);
            }
        };

        let upgrade_to_chunk_actor_entity_requests = chunk_actor_registry.upgrade_to_chunk_actor_entity_requests().clone();
        for upgrade_to_chunk_actor_entity_request in upgrade_to_chunk_actor_entity_requests.values() {
            let chunk_actor_id = upgrade_to_chunk_actor_entity_request.chunk_actor_id;
            let target_entity_id = upgrade_to_chunk_actor_entity_request.target_entity_id;
            let chunk_id = upgrade_to_chunk_actor_entity_request.chunk_id;

            if chunk_id != chunk_id {
                continue;
            }

            let target_entity_reference = match entity_registry.get_loaded_entity_reference(&target_entity_id) {
                Some(target_entity) => target_entity,
                None => {
                    panic!("Entity '{:?}' is loaded, but the entity registry failed to get the entity!", target_entity_id);
                }
            };

            let chunk_actor_entity_reference = match functions::upgrade_to_chunk_actor_entity(
                &mut commands, 
                chunk_actor_id, 
                chunk_id, 
                target_entity_reference, 
                &mut ineligible_entity_query_0,
                &mut ineligible_entity_query_1,
                &mut eligible_entity_query
            ) {
                Ok(chunk_actor_entity_reference) => chunk_actor_entity_reference,
                Err(_) => {
                    error!("Failed to upgrade entity '{:?}' to a chunk actor entity!", target_entity_id);

                    chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradeToChunkActorEntityResult::Failure {
                        target_entity_id,
                        chunk_id,
                    });

                    continue;
                }
            };

            chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity_reference);

            chunk.add_chunk_actor(chunk_actor_id);

            chunk_actor_registry.stop_upgrading_to_chunk_actor_entity(chunk_actor_id);

            upgraded_to_chunk_actor_entity_event_writer.send(UpgradeToChunkActorEntityResult::Success {
                chunk_actor_id,
                target_entity_id,
                chunk_id,
            });
        }
    }
}
