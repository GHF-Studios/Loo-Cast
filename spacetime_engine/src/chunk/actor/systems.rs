use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk;
use crate::chunk::events::CreatedChunk;
use crate::chunk::id::structs::*;
use crate::chunk::position::structs::*;
use crate::chunk::actor::position::structs::*;
use crate::chunk::actor::components::*;
use crate::chunk::actor::resources::*;
use crate::chunk::actor::structs::*;
use crate::chunk::components::*;
use crate::chunk::resources::*;
use crate::entity::resources::EntityRegistry;
use super::events::*;
use super::functions;


pub(in crate) fn update(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
) {
    let (updates, despawns) = collect_actor_updates(world, registry_parameters);

    apply_actor_updates(
        world,
        registry_parameters,
        updates,
        despawns,
    );
}

fn collect_actor_updates(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
) -> (Vec<UpdateChunkActorInfo>, Vec<DespawnChunkActorInfo>) {
    let mut chunk_actor_query = world.query::<(Entity, &Transform, &ChunkActor)>();
    let chunk_actor_query_size = chunk_actor_query.iter(world).count();
    let mut chunk_ids = Vec::new();
    let mut chunk_actor_ids = Vec::new();
    let mut chunk_actor_entities = Vec::new();
    let mut old_chunk_ids = Vec::new();

    for (chunk_actor_entity, chunk_actor_transform, chunk_actor) in chunk_actor_query.iter(world) {
        let actor_position: ChunkActorPosition = chunk_actor_transform.translation.into();
        let chunk_position: ChunkPosition = actor_position.into();
        let chunk_id: ChunkID = chunk_position.into();
        let chunk_actor_id = chunk_actor.id();
        let old_chunk_id = chunk_actor.current_chunk();

        chunk_ids.push(chunk_id);
        chunk_actor_ids.push(chunk_actor_id);
        chunk_actor_entities.push(chunk_actor_entity);
        old_chunk_ids.push(old_chunk_id);
    }

    let mut updates = Vec::new();
    let mut despawns = Vec::new();

    for i in 0..chunk_actor_query_size {
        let chunk_id = chunk_ids[i];
        let chunk_actor_id = chunk_actor_ids[i];
        let chunk_actor_entity = chunk_actor_entities[i];
        let old_chunk_id = old_chunk_ids[i];

        let (chunk_registry, _) = registry_parameters.get_mut(world);
        
        if !chunk_registry.is_chunk_loaded(chunk_id) {
            despawns.push(DespawnChunkActorInfo {
                actor_entity: chunk_actor_entity,
                actor_id: chunk_actor_id,
            });
        } else if old_chunk_id != chunk_id {
            updates.push(UpdateChunkActorInfo {
                actor_entity: chunk_actor_entity,
                old_chunk_id,
                new_chunk_id: chunk_id,
                actor_id: chunk_actor_id,
            });
        }
    }

    (updates, despawns)
}

fn apply_actor_updates(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
    updates: Vec<UpdateChunkActorInfo>,
    despawns: Vec<DespawnChunkActorInfo>,
) {
    let mut chunk_query = world.query::<&mut Chunk>();

    for update in updates {
        let (chunk_registry, _) = registry_parameters.get_mut(world);
        let old_chunk_entity = chunk_registry.get_loaded_chunk_entity(update.old_chunk_id).unwrap();
        let mut old_chunk = chunk_query.get_mut(world, old_chunk_entity).unwrap();
        old_chunk.remove_chunk_actor(update.actor_id);

        let (chunk_registry, _) = registry_parameters.get_mut(world);
        let new_chunk_entity = chunk_registry.get_loaded_chunk_entity(update.new_chunk_id).unwrap();
        let mut new_chunk = chunk_query.get_mut(world, new_chunk_entity).unwrap();
        new_chunk.add_chunk_actor(update.actor_id);
    }


    for despawn in despawns {
        world.despawn(despawn.actor_entity);
        let (_, mut chunk_actor_registry) = registry_parameters.get_mut(world);
        chunk_actor_registry.unload_chunk_actor(despawn.actor_id);
        chunk_actor_registry.unregister_chunk_actor(despawn.actor_id);
    }
}

pub(in crate) fn handle_create_chunk_actor_entity_events(
    mut commands: Commands,
    mut create_chunk_actor_entity_event_reader: EventReader<CreateChunkActorEntity>,
    mut created_chunk_actor_entity_event_writer: EventWriter<CreatedChunkActorEntity>,
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

            created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity {
                chunk_actor_id,
                chunk_actor_entity_id,
                chunk_id,
                world_position,
                success: true,
            });
        } else {
            info!("Chunk not loaded, issuing request to create chunk actor entity '{:?}' when the chunk is loaded ...", chunk_actor_entity_id);

            if chunk_actor_registry.is_chunk_actor_entity_creating(chunk_actor_id) {
                error!("The request for creating chunk actor entity '{:?}' has already been issued!", chunk_actor_entity_id);

                chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);
                entity_registry.unregister_entity(chunk_actor_entity_id);

                created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity {
                    chunk_actor_id,
                    chunk_actor_entity_id,
                    chunk_id,
                    world_position,
                    success: false,
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

pub(in crate) fn process_create_chunk_actor_requests(
    mut commands: Commands,
    mut created_chunk_event_reader: EventReader<CreatedChunk>,
    mut created_chunk_actor_entity_event_writer: EventWriter<CreatedChunkActorEntity>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    chunk_registry: ResMut<ChunkRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
    mut chunk_query: Query<&mut Chunk>,
) {
    let mut created_chunk_events = Vec::new();
    for created_chunk_event in created_chunk_event_reader.read() {
        created_chunk_events.push(created_chunk_event.clone());
    }

    for created_chunk_event in created_chunk_events {
        let chunk_id = created_chunk_event.chunk_id;
        let success = created_chunk_event.success;

        if !success {
            let requests = chunk_actor_registry.create_chunk_actor_entity_requests().clone();
            for request in requests.values() {
                if request.chunk_id != chunk_id {
                    warn!("The creation request for chunk actor entity '{:?}' has been cancelled due to the starting chunk '{:?}' failing to load!", request.chunk_actor_entity_id, request.chunk_id);

                    entity_registry.unregister_entity(request.chunk_actor_entity_id);
                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_creating_chunk_actor_entity(request.chunk_actor_id);

                    created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity {
                        chunk_actor_id: request.chunk_actor_id,
                        chunk_actor_entity_id: request.chunk_actor_entity_id,
                        chunk_id: request.chunk_id,
                        world_position: request.world_position,
                        success: false,
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

            created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity {
                chunk_actor_id,
                chunk_actor_entity_id,
                chunk_id,
                world_position,
                success: true,
            });
        }
    }
}

pub(in crate) fn handle_destroy_chunk_actor_entity_events(
    mut commands: Commands,
    mut destroy_chunk_actor_entity_event_reader: EventReader<DestroyChunkActorEntity>,
    mut destroyed_chunk_actor_entity_event_writer: EventWriter<DestroyedChunkActorEntity>,
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

                destroyed_chunk_actor_entity_event_writer.send(DestroyedChunkActorEntity {
                    chunk_actor_id,
                    success: false,
                });

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

                destroyed_chunk_actor_entity_event_writer.send(DestroyedChunkActorEntity {
                    chunk_actor_id,
                    success: false,
                });

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

        destroyed_chunk_actor_entity_event_writer.send(DestroyedChunkActorEntity {
            chunk_actor_id,
            success: true,
        });
    }
}

pub(in crate) fn handle_convert_to_chunk_actor_entity_events(
    mut commands: Commands,
    mut convert_to_chunk_actor_entity_event_reader: EventReader<ConvertToChunkActorEntity>,
    mut converted_to_chunk_actor_entity_event_writer: EventWriter<ConvertedToChunkActorEntity>,
    chunk_registry: ResMut<ChunkRegistry>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
    mut chunk_query: Query<&mut Chunk>,
    mut eligible_entity_query: Query<Entity, (With<Transform>, Without<ChunkActor>)>,
) {
    let mut convert_to_chunk_actor_entity_events = Vec::new();
    for convert_to_chunk_actor_entity_event in convert_to_chunk_actor_entity_event_reader.read() {
        convert_to_chunk_actor_entity_events.push(convert_to_chunk_actor_entity_event.clone());
    }

    for convert_to_chunk_actor_entity_event in convert_to_chunk_actor_entity_events {
        let target_entity_id = convert_to_chunk_actor_entity_event.target_entity_id;
        let chunk_actor_id = chunk_actor_registry.register_chunk_actor();
        let chunk_id = convert_to_chunk_actor_entity_event.chunk_id;

        info!("Trying to convert entity '{:?}' to a chunk actor entity ...", target_entity_id);

        if let Some(chunk_entity) = chunk_registry.get_loaded_chunk_entity(chunk_id) {
            info!("Chunk loaded, converting entity '{:?}' to a chunk actor entity immediately ...", target_entity_id);

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

            let chunk_actor_entity_reference = functions::convert_to_chunk_actor_entity(&mut commands, chunk_actor_id, chunk_id, target_entity_reference, &mut eligible_entity_query);

            chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity_reference);

            chunk.add_chunk_actor(chunk_actor_id);

            converted_to_chunk_actor_entity_event_writer.send(ConvertedToChunkActorEntity {
                chunk_actor_id,
                chunk_actor_entity_id: target_entity_id,
                chunk_id,
                success: true,
            });
        } else {
            info!("Chunk not loaded, issuing request to convert entity '{:?}' to a chunk actor entity when the chunk is loaded ...", target_entity_id);

            if chunk_actor_registry.is_chunk_actor_entity_converted_to(chunk_actor_id) {
                error!("The request for converting entity '{:?}' to a chunk actor entity has already been issued!", target_entity_id);

                chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);
                entity_registry.unregister_entity(target_entity_id);

                converted_to_chunk_actor_entity_event_writer.send(ConvertedToChunkActorEntity {
                    chunk_actor_id,
                    chunk_actor_entity_id: target_entity_id,
                    chunk_id,
                    success: false,
                });

                continue;
            }
            
            chunk_actor_registry.start_converting_to_chunk_actor_entity(
                ConvertToChunkActorEntityRequest {
                    chunk_actor_id,
                    target_entity_id,
                    chunk_id,
                }
            );
        }
    }
}

pub(in crate) fn process_convert_to_chunk_actor_requests(
    mut commands: Commands,
    mut created_chunk_event_reader: EventReader<CreatedChunk>,
    mut converted_to_chunk_actor_entity_event_writer: EventWriter<ConvertedToChunkActorEntity>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    chunk_registry: ResMut<ChunkRegistry>,
    entity_registry: Res<EntityRegistry>,
    mut chunk_query: Query<&mut Chunk>,
    mut eligible_entity_query: Query<Entity, (With<Transform>, Without<ChunkActor>)>,
) {
    let mut created_chunk_events = Vec::new();
    for created_chunk_event in created_chunk_event_reader.read() {
        created_chunk_events.push(created_chunk_event.clone());
    }

    for created_chunk_event in created_chunk_events {
        let chunk_id = created_chunk_event.chunk_id;
        let success = created_chunk_event.success;

        if !success {
            let requests = chunk_actor_registry.convert_to_chunk_actor_entity_requests().clone();
            for request in requests.values() {
                if request.chunk_id != chunk_id {
                    warn!("The chunk actor entity conversion request for entity '{:?}' has been cancelled due to the starting chunk '{:?}' failing to load!", request.target_entity_id, request.chunk_id);

                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_creating_chunk_actor_entity(request.chunk_actor_id);

                    converted_to_chunk_actor_entity_event_writer.send(ConvertedToChunkActorEntity {
                        chunk_actor_id: request.chunk_actor_id,
                        chunk_actor_entity_id: request.target_entity_id,
                        chunk_id: request.chunk_id,
                        success: false,
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

        let convert_to_chunk_actor_entity_requests = chunk_actor_registry.convert_to_chunk_actor_entity_requests().clone();
        for convert_to_chunk_actor_entity_request in convert_to_chunk_actor_entity_requests.values() {
            let chunk_actor_id = convert_to_chunk_actor_entity_request.chunk_actor_id;
            let target_entity_id = convert_to_chunk_actor_entity_request.target_entity_id;
            let chunk_id = convert_to_chunk_actor_entity_request.chunk_id;

            if chunk_id != chunk_id {
                continue;
            }

            let target_entity_reference = match entity_registry.get_loaded_entity_reference(&target_entity_id) {
                Some(target_entity) => target_entity,
                None => {
                    panic!("Entity '{:?}' is loaded, but the entity registry failed to get the entity!", target_entity_id);
                }
            };

            let chunk_actor_entity_reference = functions::convert_to_chunk_actor_entity(&mut commands, chunk_actor_id, chunk_id, target_entity_reference, &mut eligible_entity_query);
            let chunk_actor_entity_id = target_entity_id;

            chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity_reference);

            chunk.add_chunk_actor(chunk_actor_id);

            chunk_actor_registry.stop_converting_to_chunk_actor_entity(chunk_actor_id);

            converted_to_chunk_actor_entity_event_writer.send(ConvertedToChunkActorEntity {
                chunk_actor_id,
                chunk_actor_entity_id,
                chunk_id,
                success: true,
            });
        }
    }
}