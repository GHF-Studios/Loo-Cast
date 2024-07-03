use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::actor::position::structs::ChunkActorPosition;
use crate::chunk::actor::components::*;
use crate::chunk::actor::resources::*;
use crate::chunk::components::*;
use crate::chunk::events::CreatedChunkEntity;
use crate::chunk::events::LoadedChunkEntity;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::position::structs::ChunkPosition;
use crate::chunk::resources::*;
use crate::chunk::structs::ChunkActorCreateRequest;
use crate::chunk::structs::ChunkActorUpgradeRequest;
use crate::entity::resources::EntityRegistry;
use super::events::*;
use super::functions;
use super::id::structs::ChunkActorID;

pub(super) fn start(
    mut started_chunk_actor_event_writer: EventWriter<StartedChunkActor>,
    chunk_actor_query: Query<(&ChunkActor, &Transform), Added<ChunkActor>>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    mut chunk_actor_event_registry: ResMut<ChunkActorRequestRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    for (chunk_actor, chunk_actor_transform) in chunk_actor_query.iter() {
        let chunk_actor_request_id = chunk_actor_event_registry.get_unused_chunk_actor_request_id();
        let chunk_actor_id: ChunkActorID = chunk_actor.id();
        let chunk_actor_entity_id = {
            let chunk_actor_entity_reference = chunk_actor_registry.loaded_chunk_actor(chunk_actor_id);

            match entity_registry.get_loaded_entity_id(&chunk_actor_entity_reference) {
                Some(chunk_actor_entity_id) => chunk_actor_entity_id,
                None => {
                    panic!("The chunk actor entity id for chunk actor '{:?}' could not be found!", chunk_actor_id);
                }
            }
        };
        let world_position = chunk_actor_transform.translation.truncate();
        let chunk_id = {
            let chunk_actor_position: ChunkActorPosition = world_position.into();
            let chunk_position: ChunkPosition = chunk_actor_position.into();
            let chunk_id: ChunkID = chunk_position.into();

            chunk_id
        };

        started_chunk_actor_event_writer.send(StartedChunkActor::Success {
            chunk_actor_request_id,
            chunk_actor_id,
            chunk_actor_entity_id,
            chunk_id,
            world_position,
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
    mut create_chunk_actor_entity_event_reader: EventReader<CreateChunkActorEntity>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    let mut create_chunk_actor_entity_events = Vec::new();
    for create_chunk_actor_entity_event in create_chunk_actor_entity_event_reader.read() {
        create_chunk_actor_entity_events.push(create_chunk_actor_entity_event.clone());
    }

    for create_chunk_actor_entity_event in create_chunk_actor_entity_events {
        let chunk_actor_request_id = create_chunk_actor_entity_event.chunk_actor_request_id;
        let chunk_actor_entity_id = entity_registry.register_entity();
        let chunk_actor_id = chunk_actor_registry.register_chunk_actor();
        let chunk_id = {
            let world_position = create_chunk_actor_entity_event.world_position;
            let chunk_actor_position: ChunkActorPosition = world_position.into();
            let chunk_position: ChunkPosition = chunk_actor_position.into();
            let chunk_id: ChunkID = chunk_position.into();

            chunk_id
        };
        let world_position = create_chunk_actor_entity_event.world_position;

        info!("Trying to create chunk actor entity '{:?}' at world position '{:?}' in chunk '{:?}' ...", chunk_actor_entity_id, world_position, chunk_id);

        chunk_actor_registry.start_creating_chunk_actor(ChunkActorCreateRequest {
            chunk_actor_request_id,
            chunk_actor_id,
            chunk_actor_entity_id,
            chunk_id,
            world_position
        });
    }
}

pub(super) fn handle_destroy_chunk_actor_entity_events(
    mut destroy_chunk_actor_entity_event_reader: EventReader<DestroyChunkActorEntity>,
    mut destroy_chunk_actor_entity_internal_event_writer: EventWriter<DestroyChunkActorEntityInternal>,
    chunk_actor_registry: ResMut<ChunkActorRegistry>,
    entity_registry: ResMut<EntityRegistry>,
    target_entity_query: Query<&Transform, Without<ChunkActor>>,
) {
    let mut destroy_chunk_actor_entity_events = Vec::new();
    for destroy_chunk_actor_entity_event in destroy_chunk_actor_entity_event_reader.read() {
        destroy_chunk_actor_entity_events.push(destroy_chunk_actor_entity_event.clone());
    }

    for destroy_chunk_actor_entity_event in destroy_chunk_actor_entity_events {
        let chunk_actor_id = destroy_chunk_actor_entity_event.chunk_actor_id;
        let chunk_actor_request_id = destroy_chunk_actor_entity_event.chunk_actor_request_id;
        let chunk_actor_entity_id = {
            let chunk_actor_entity_reference = chunk_actor_registry.loaded_chunk_actor(chunk_actor_id);

            match entity_registry.get_loaded_entity_id(&chunk_actor_entity_reference) {
                Some(chunk_actor_entity_id) => chunk_actor_entity_id,
                None => {
                    panic!("The chunk actor entity id for chunk actor '{:?}' could not be found!", chunk_actor_id);
                }
            }
        };
        let (chunk_id, world_position) = {
            let target_entity_reference = match entity_registry.get_loaded_entity_reference(&chunk_actor_entity_id) {
                Some(target_entity_reference) => target_entity_reference,
                None => {
                    panic!("The target entity '{:?}' either does not exist or does not have a transform component!", chunk_actor_entity_id);
                }
            };
            let target_transform = match target_entity_query.get(target_entity_reference) {
                Ok(target_transform) => target_transform,
                Err(_) => {
                    panic!("The target entity '{:?}' either does not exist or does not have a transform component!", chunk_actor_entity_id);
                }
            };

            let world_position = target_transform.translation.truncate();
            let chunk_actor_position: ChunkActorPosition = world_position.into();
            let chunk_position: ChunkPosition = chunk_actor_position.into();
            let chunk_id: ChunkID = chunk_position.into();

            (chunk_id, world_position)
        };

        info!("Trying to destroy chunk actor entity '{:?}'  ...", chunk_actor_id);

        destroy_chunk_actor_entity_internal_event_writer.send(DestroyChunkActorEntityInternal {
            chunk_actor_request_id,
            chunk_actor_id,
            chunk_actor_entity_id,
            chunk_id,
            world_position
        });
    }
}

pub(super) fn handle_upgrade_to_chunk_actor_entity_events(
    mut upgrade_to_chunk_actor_entity_event_reader: EventReader<UpgradeToChunkActorEntity>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    entity_registry: Res<EntityRegistry>,
    target_entity_query: Query<&Transform, Without<ChunkActor>>,
) {
    let mut upgrade_to_chunk_actor_entity_events = Vec::new();
    for upgrade_to_chunk_actor_entity_event in upgrade_to_chunk_actor_entity_event_reader.read() {
        upgrade_to_chunk_actor_entity_events.push(upgrade_to_chunk_actor_entity_event.clone());
    }

    for upgrade_to_chunk_actor_entity_event in upgrade_to_chunk_actor_entity_events {
        
        let chunk_actor_id = chunk_actor_registry.register_chunk_actor();
        let chunk_actor_request_id = upgrade_to_chunk_actor_entity_event.chunk_actor_request_id;
        let target_entity_id = upgrade_to_chunk_actor_entity_event.target_entity_id;
        let (chunk_id, world_position) = {
            let target_entity_reference = match entity_registry.get_loaded_entity_reference(&upgrade_to_chunk_actor_entity_event.target_entity_id) {
                Some(target_entity_reference) => target_entity_reference,
                None => {
                    panic!("The target entity '{:?}' either does not exist or does not have a transform component!", upgrade_to_chunk_actor_entity_event.target_entity_id);
                }
            };
            let target_transform = match target_entity_query.get(target_entity_reference) {
                Ok(target_transform) => target_transform,
                Err(_) => {
                    panic!("The target entity '{:?}' either does not exist or does not have a transform component!", upgrade_to_chunk_actor_entity_event.target_entity_id);
                }
            };

            let world_position = target_transform.translation.truncate();
            let chunk_actor_position: ChunkActorPosition = world_position.into();
            let chunk_position: ChunkPosition = chunk_actor_position.into();
            let chunk_id: ChunkID = chunk_position.into();

            (chunk_id, world_position)
        };

        info!("Trying to upgrade entity '{:?}' to a chunk actor entity ...", target_entity_id);

        chunk_actor_registry.start_upgrading_to_chunk_actor(ChunkActorUpgradeRequest {
            chunk_actor_request_id,
            chunk_actor_id,
            target_entity_id,
            chunk_id,
            world_position,
        });
    }
}

pub(super) fn handle_create_chunk_actor_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<CreatedChunkEntity>,
        EventReader<LoadedChunkEntity>,
        EventWriter<CreatedChunkActorEntityInternal>,
    )>,
    registry_parameters: &mut SystemState<(
        Res<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
        ResMut<EntityRegistry>,
    )>,
) {
    // TODO: Refactor here so the code reacts to Created/Loaded chunks instead of just reading the external chunk actor creation events,
    //       which are also not being sent anymore; they're replaced by registry-managed and registry-saved create chunk actor requests.





    let mut create_chunk_actor_entity_event_reader = event_parameters.get_mut(world).0;

    let mut create_chunk_actor_entity_events: Vec<CreateChunkActorEntityInternal> = Vec::new();
    for create_chunk_actor_entity_event in create_chunk_actor_entity_event_reader.read() {
        create_chunk_actor_entity_events.push(create_chunk_actor_entity_event.clone());
    }

    for create_chunk_actor_entity_event in create_chunk_actor_entity_events {
        let chunk_actor_request_id = create_chunk_actor_entity_event.chunk_actor_request_id;
        let chunk_actor_id = create_chunk_actor_entity_event.chunk_actor_id;
        let chunk_actor_entity_id = create_chunk_actor_entity_event.chunk_actor_entity_id;
        let chunk_id = create_chunk_actor_entity_event.chunk_id;
        let world_position = create_chunk_actor_entity_event.world_position;

        let (chunk_registry, _, _) = registry_parameters.get_mut(world);

        let chunk_entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity_reference) => chunk_entity_reference,
            None => {
                panic!("The chunk entity reference for chunk '{:?}' could not be found!", chunk_id);
            }
        };

        let mut chunk = {
            let mut chunk_query_state = world.query::<&mut Chunk>();
        
            match chunk_query_state.get_mut(world, chunk_entity_reference) {
                Ok(chunk) => chunk,
                Err(_) => {
                    panic!("The chunk component for chunk '{:?}' could not be found!", chunk_id);
                }
            }
        };

        chunk.add_chunk_actor(chunk_actor_id);
        
        let chunk_actor_entity_reference = functions::new_chunk_actor_entity(world, chunk_actor_id, chunk_id, world_position);
        
        let (_, mut chunk_actor_registry, mut entity_registry) = registry_parameters.get_mut(world);
        
        entity_registry.load_entity(chunk_actor_entity_id, chunk_actor_entity_reference);
        chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity_reference);

        chunk_actor_registry.stop_creating_chunk_actor(chunk_actor_id);

        info!("Successfully created chunk actor entity '{:?}' at world position '{:?}'!", chunk_actor_entity_id, world_position);

        let mut created_chunk_actor_entity_event_writer = event_parameters.get_mut(world).2;
        created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntityInternal::Success {
            chunk_actor_request_id,
            chunk_actor_id,
            chunk_actor_entity_id,
            chunk_id,
            world_position
        });
    }
}

pub(super) fn handle_destroy_chunk_actor_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<DestroyChunkActorEntityInternal>,
        EventWriter<DestroyedChunkActorEntityInternal>,
    )>,
    registry_parameters: &mut SystemState<(
        Res<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
        ResMut<EntityRegistry>,
    )>,
) {
    let mut destroy_chunk_actor_entity_event_reader = event_parameters.get_mut(world).0;

    let mut destroy_chunk_actor_entity_events: Vec<DestroyChunkActorEntityInternal> = Vec::new();
    for destroy_chunk_actor_entity_event in destroy_chunk_actor_entity_event_reader.read() {
        destroy_chunk_actor_entity_events.push(destroy_chunk_actor_entity_event.clone());
    }

    for destroy_chunk_actor_entity_event in destroy_chunk_actor_entity_events {
        let chunk_actor_request_id = destroy_chunk_actor_entity_event.chunk_actor_request_id;
        let chunk_actor_id = destroy_chunk_actor_entity_event.chunk_actor_id;
        let chunk_actor_entity_id = destroy_chunk_actor_entity_event.chunk_actor_entity_id;
        let chunk_id = destroy_chunk_actor_entity_event.chunk_id;
        let world_position = destroy_chunk_actor_entity_event.world_position;

        let (chunk_registry, _, _) = registry_parameters.get_mut(world);

        let chunk_entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity_reference) => chunk_entity_reference,
            None => {
                panic!("The chunk entity reference for chunk '{:?}' could not be found!", chunk_id);
            }
        };
        
        let mut chunk = {
            let mut chunk_query_state = world.query::<&mut Chunk>();

            match chunk_query_state.get_mut(world, chunk_entity_reference) {
                Ok(chunk) => chunk,
                Err(_) => {
                    panic!("The chunk '{:?}' entity could not be queried!", chunk_id);
                }
            }
        };

        chunk.remove_chunk_actor(chunk_actor_id);

        let (_, mut chunk_actor_registry, mut entity_registry) = registry_parameters.get_mut(world);

        chunk_actor_registry.unload_chunk_actor(chunk_actor_id);
        entity_registry.unload_entity(chunk_actor_entity_id);

        chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);
        entity_registry.unregister_entity(chunk_actor_entity_id);

        info!("Successfully destroyed chunk actor entity '{:?}'!", chunk_actor_entity_id);

        let mut destroyed_chunk_actor_entity_event_writer = event_parameters.get_mut(world).1;
        destroyed_chunk_actor_entity_event_writer.send(DestroyedChunkActorEntityInternal::Success {
            chunk_actor_request_id,
            chunk_actor_id,
            chunk_actor_entity_id,
            chunk_id,
            world_position
        });

        continue;
    }
}

pub(super) fn handle_upgrade_to_chunk_actor_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<CreatedChunkEntity>,
        EventReader<LoadedChunkEntity>,
        EventWriter<UpgradedToChunkActorEntityInternal>,
    )>,
    registry_parameters: &mut SystemState<(
        Res<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
        Res<EntityRegistry>,
    )>,
) {
    // TODO: Refactor here so the code reacts to Created/Loaded chunks instead of just reading the external chunk actor upgrade_to events,
    //       which are also not being sent anymore; they're replaced by registry-managed and registry-saved upgrade_to chunk actor requests.





    let mut upgrade_to_chunk_actor_entity_event_reader = event_parameters.get_mut(world).0;

    let mut upgrade_to_chunk_actor_entity_events: Vec<UpgradeToChunkActorEntityInternal> = Vec::new();
    for upgrade_to_chunk_actor_entity_event in upgrade_to_chunk_actor_entity_event_reader.read() {
        upgrade_to_chunk_actor_entity_events.push(upgrade_to_chunk_actor_entity_event.clone());
    }

    for upgrade_to_chunk_actor_entity_event in upgrade_to_chunk_actor_entity_events {
        let chunk_actor_request_id = upgrade_to_chunk_actor_entity_event.chunk_actor_request_id;
        let chunk_actor_id = upgrade_to_chunk_actor_entity_event.chunk_actor_id;
        let target_entity_id = upgrade_to_chunk_actor_entity_event.target_entity_id;
        let chunk_id = upgrade_to_chunk_actor_entity_event.chunk_id;
        let world_position = upgrade_to_chunk_actor_entity_event.world_position;

        let (chunk_registry, _, entity_registry) = registry_parameters.get_mut(world);

        let target_entity_reference = match entity_registry.get_loaded_entity_reference(&target_entity_id) {
            Some(target_entity_reference) => target_entity_reference,
            None => {
                panic!("The target entity '{:?}' either does not exist or does not have a transform component!", target_entity_id);
            }
        };

        let chunk_entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity_reference) => chunk_entity_reference,
            None => {
                panic!("The chunk entity reference for chunk '{:?}' could not be found!", chunk_id);
            }
        };

        let mut chunk = {
            let mut chunk_query_state = world.query::<&mut Chunk>();

            match chunk_query_state.get_mut(world, chunk_entity_reference) {
                Ok(chunk) => chunk,
                Err(_) => {
                    panic!("The chunk '{:?}' entity could not be queried!", chunk_id);
                }
            }
        };

        chunk.add_chunk_actor(chunk_actor_id);

        let (_, mut chunk_actor_registry, _) = registry_parameters.get_mut(world);

        chunk_actor_registry.load_chunk_actor(chunk_actor_id, target_entity_reference);

        chunk_actor_registry.stop_creating_chunk_actor(chunk_actor_id);

        info!("Successfully upgraded entity '{:?}' to a chunk actor entity!", target_entity_id);

        let mut upgraded_to_chunk_actor_entity_event_writer = event_parameters.get_mut(world).1;
        upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntityInternal::Success {
            chunk_actor_request_id,
            chunk_actor_id,
            chunk_actor_entity_id: target_entity_id,
            chunk_id,
            world_position
        });
    }
}

pub(super) fn handle_created_chunk_actor_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<CreatedChunkActorEntityInternal>,
        EventWriter<CreatedChunkActorEntity>,
    )>,
) {
    let mut created_chunk_actor_entity_event_reader = event_parameters.get_mut(world).0;

    let mut created_chunk_actor_entity_events: Vec<CreatedChunkActorEntityInternal> = Vec::new();
    for created_chunk_actor_entity_event in created_chunk_actor_entity_event_reader.read() {
        created_chunk_actor_entity_events.push(created_chunk_actor_entity_event.clone());
    }

    for created_chunk_actor_entity_event in created_chunk_actor_entity_events {
        let mut created_chunk_actor_entity_event_writer = event_parameters.get_mut(world).1;

        match created_chunk_actor_entity_event {
            CreatedChunkActorEntityInternal::Success {
                chunk_actor_request_id,
                chunk_actor_id,
                chunk_actor_entity_id,
                chunk_id,
                world_position
            } => {
                info!("Successfully created chunk actor entity '{:?}' at world position '{:?}'!", chunk_actor_entity_id, world_position);

                created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Success {
                    chunk_actor_request_id,
                    chunk_actor_id,
                    chunk_actor_entity_id,
                    chunk_id,
                    world_position
                });
            },
            CreatedChunkActorEntityInternal::Failure {
                chunk_actor_request_id,
                chunk_actor_id,
                chunk_actor_entity_id,
                chunk_id,
                world_position
            } => {
                error!("Failed to create chunk actor entity '{:?}' at world position '{:?}'!", chunk_actor_entity_id, world_position);

                created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Failure {
                    chunk_actor_request_id,
                    chunk_actor_id,
                    chunk_actor_entity_id,
                    chunk_id,
                    world_position
                });
            }
        }
    }
}

pub(super) fn handle_destroyed_chunk_actor_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<DestroyedChunkActorEntityInternal>,
        EventWriter<DestroyedChunkActorEntity>,
    )>,
) {
    let mut destroyed_chunk_actor_entity_event_reader = event_parameters.get_mut(world).0;

    let mut destroyed_chunk_actor_entity_events: Vec<DestroyedChunkActorEntityInternal> = Vec::new();
    for destroyed_chunk_actor_entity_event in destroyed_chunk_actor_entity_event_reader.read() {
        destroyed_chunk_actor_entity_events.push(destroyed_chunk_actor_entity_event.clone());
    }

    for destroyed_chunk_actor_entity_event in destroyed_chunk_actor_entity_events {
        let mut destroyed_chunk_actor_entity_event_writer = event_parameters.get_mut(world).1;

        match destroyed_chunk_actor_entity_event {
            DestroyedChunkActorEntityInternal::Success {
                chunk_actor_request_id,
                chunk_actor_id,
                chunk_actor_entity_id,
                chunk_id,
                world_position
            } => {
                info!("Successfully destroyed chunk actor entity '{:?}'!", chunk_actor_entity_id);

                destroyed_chunk_actor_entity_event_writer.send(DestroyedChunkActorEntity::Success {
                    chunk_actor_request_id,
                    chunk_actor_id,
                    chunk_actor_entity_id,
                    chunk_id,
                    world_position
                });
            },
            DestroyedChunkActorEntityInternal::Failure {
                chunk_actor_request_id,
                chunk_actor_id,
                chunk_actor_entity_id,
                chunk_id,
                world_position
            } => {
                error!("Failed to destroy chunk actor entity '{:?}'!", chunk_actor_entity_id);

                destroyed_chunk_actor_entity_event_writer.send(DestroyedChunkActorEntity::Failure {
                    chunk_actor_request_id,
                    chunk_actor_id,
                    chunk_actor_entity_id,
                    chunk_id,
                    world_position
                });
            }
        }
    }
}

pub(super) fn handle_upgraded_to_chunk_actor_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<UpgradedToChunkActorEntityInternal>,
        EventWriter<UpgradedToChunkActorEntity>,
    )>,
) {
    let mut upgraded_to_chunk_actor_entity_event_reader = event_parameters.get_mut(world).0;

    let mut upgraded_to_chunk_actor_entity_events: Vec<UpgradedToChunkActorEntityInternal> = Vec::new();
    for upgraded_to_chunk_actor_entity_event in upgraded_to_chunk_actor_entity_event_reader.read() {
        upgraded_to_chunk_actor_entity_events.push(upgraded_to_chunk_actor_entity_event.clone());
    }

    for upgraded_to_chunk_actor_entity_event in upgraded_to_chunk_actor_entity_events {
        let mut upgraded_to_chunk_actor_entity_event_writer = event_parameters.get_mut(world).1;

        match upgraded_to_chunk_actor_entity_event {
            UpgradedToChunkActorEntityInternal::Success {
                chunk_actor_request_id,
                chunk_actor_id,
                chunk_actor_entity_id,
                chunk_id,
                world_position
            } => {
                info!("Successfully upgraded entity '{:?}' to a chunk actor entity!", chunk_actor_entity_id);

                upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Success {
                    chunk_actor_request_id,
                    chunk_actor_id,
                    target_entity_id: chunk_actor_entity_id,
                    chunk_id,
                    world_position
                });
            },
            UpgradedToChunkActorEntityInternal::Failure {
                chunk_actor_request_id,
                chunk_actor_id,
                target_entity_id,
                chunk_id,
                world_position
            } => {
                error!("Failed to upgrade entity '{:?}' to a chunk actor entity!", target_entity_id);

                upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                    chunk_actor_request_id,
                    chunk_actor_id,
                    target_entity_id,
                    chunk_id,
                    world_position
                });
            }
        }
    }
}
