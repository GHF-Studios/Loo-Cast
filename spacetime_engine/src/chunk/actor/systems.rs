use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::actor::position::structs::ChunkActorPosition;
use crate::chunk::events::CreatedChunkEntity;
use crate::chunk::actor::components::*;
use crate::chunk::actor::resources::*;
use crate::chunk::components::*;
use crate::chunk::events::LoadedChunkEntity;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::position::structs::ChunkPosition;
use crate::chunk::resources::*;
use crate::entity::resources::EntityRegistry;
use super::events::*;
use super::functions;

pub(super) fn start(
    mut started_chunk_actor_event_writer: EventWriter<StartedChunkActor>,
    chunk_actor_query: Query<&ChunkActor, Added<ChunkActor>>,
    mut chunk_actor_event_registry: ResMut<ChunkActorEventRegistry>,
) {
    for chunk_actor in chunk_actor_query.iter() {
        let chunk_actor_event_id = chunk_actor_event_registry.get_unused_chunk_actor_event_id();

        started_chunk_actor_event_writer.send(StartedChunkActor::Success {
            chunk_actor_event_id,
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
    mut create_chunk_actor_entity_event_reader: EventReader<CreateChunkActorEntity>,
    mut create_chunk_actor_entity_internal_event_writer: EventWriter<CreateChunkActorEntityInternal>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    let mut create_chunk_actor_entity_events = Vec::new();
    for create_chunk_actor_entity_event in create_chunk_actor_entity_event_reader.read() {
        create_chunk_actor_entity_events.push(create_chunk_actor_entity_event.clone());
    }

    for create_chunk_actor_entity_event in create_chunk_actor_entity_events {
        let chunk_actor_event_id = create_chunk_actor_entity_event.chunk_actor_event_id;
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

        create_chunk_actor_entity_internal_event_writer.send(CreateChunkActorEntityInternal {
            chunk_actor_event_id,
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
) {
    let mut destroy_chunk_actor_entity_events = Vec::new();
    for destroy_chunk_actor_entity_event in destroy_chunk_actor_entity_event_reader.read() {
        destroy_chunk_actor_entity_events.push(destroy_chunk_actor_entity_event.clone());
    }

    for destroy_chunk_actor_entity_event in destroy_chunk_actor_entity_events {
        let chunk_actor_id = destroy_chunk_actor_entity_event.chunk_actor_id;
        let chunk_actor_event_id = destroy_chunk_actor_entity_event.chunk_actor_event_id;

        info!("Destroying chunk actor entity '{:?}' immediately ...", chunk_actor_id);

        destroy_chunk_actor_entity_internal_event_writer.send(DestroyChunkActorEntityInternal {
            chunk_actor_event_id,
            chunk_actor_id
        });
    }
}

pub(super) fn handle_upgrade_to_chunk_actor_entity_events() {}

pub(super) fn handle_create_chunk_actor_entity_internal_events() {}

pub(super) fn handle_destroy_chunk_actor_entity_internal_events() {}

pub(super) fn handle_upgrade_to_chunk_actor_entity_internal_events() {}

pub(super) fn handle_created_chunk_actor_entity_internal_events() {}

pub(super) fn handle_destroyed_chunk_actor_entity_internal_events() {}

pub(super) fn handle_upgraded_to_chunk_actor_entity_internal_events() {}

#[allow(clippy::too_many_arguments)]
#[deprecated]
pub(super) fn handle_create_chunk_actor_entity_events_OLD(
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
        let chunk_actor_event_id = create_chunk_actor_entity_event.chunk_actor_event_id;
        let chunk_id = {
            let world_position = create_chunk_actor_entity_event.world_position;
            let chunk_actor_position: ChunkActorPosition = world_position.into();
            let chunk_position: ChunkPosition = chunk_actor_position.into();
            let chunk_id: ChunkID = chunk_position.into();

            chunk_id
        };
        let world_position = create_chunk_actor_entity_event.world_position;

        info!("Trying to create chunk actor entity '{:?}' at world position '{:?}'...", chunk_actor_entity_id, world_position);

        if let Some(chunk_entity) = chunk_registry.get_loaded_chunk_entity(chunk_id) {
            info!("Chunk '{:?}' loaded; creating chunk actor entity '{:?}' immediately ...", chunk_id, chunk_actor_entity_id);

            let mut chunk = match chunk_query.get_mut(chunk_entity) {
                Ok(chunk) => chunk,
                Err(_) => {
                    error!("The request for creating the chunk actor entity '{:?}' has been cancelled due to the chunk '{:?}' failing to load!", chunk_actor_entity_id, chunk_id);

                    chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);
                    entity_registry.unregister_entity(chunk_actor_entity_id);

                    created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Failure {
                        chunk_actor_event_id,
                        world_position,
                    });

                    continue;
                }
            };

            let chunk_actor_entity_reference = functions::new_chunk_actor_entity(&mut commands, chunk_actor_id, chunk_id, world_position);

            entity_registry.load_entity(chunk_actor_entity_id, chunk_actor_entity_reference);
            chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity_reference);

            chunk.add_chunk_actor(chunk_actor_id);

            info!("Successfully created chunk actor entity '{:?}'!", chunk_actor_entity_id);

            created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Success {
                chunk_actor_event_id,
                chunk_actor_id,
                chunk_actor_entity_id,
                world_position,
            });
        } else {
            info!("Chunk '{:?}' not loaded; issuing request to create chunk actor entity '{:?}' whenever the chunk is created or loaded ...", chunk_id, chunk_actor_entity_id);

            if chunk_actor_registry.is_chunk_actor_creating(chunk_actor_id) {
                error!("The request for creating the chunk actor entity (chunk actor id '{:?}' | entity id '{:?}') has been cancelled due to the request already being issued!", chunk_actor_id, chunk_actor_entity_id);

                chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);
                entity_registry.unregister_entity(chunk_actor_entity_id);

                created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Failure {
                    chunk_actor_event_id,
                    world_position,
                });

                continue;
            }
            
            chunk_actor_registry.start_creating_chunk_actor(
                CreateChunkActorEntityRequest {
                    chunk_actor_event_id,
                    chunk_actor_id,
                    chunk_actor_entity_id,
                    chunk_id,
                    world_position,
                }
            );

            info!("Request to create chunk actor entity '{:?}' issued!", chunk_actor_entity_id);
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[deprecated]
pub(super) fn handle_destroy_chunk_actor_entity_events_OLD(
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
        let chunk_actor_event_id = destroy_chunk_actor_entity_event.chunk_actor_event_id;

        info!("Destroying chunk actor entity '{:?}' immediately...", chunk_actor_id);

        let chunk_actor_entity_reference = match chunk_actor_registry.get_loaded_chunk_actor(chunk_actor_id) {
            Some(chunk_actor_entity) => chunk_actor_entity,
            None => {
                error!("The request for destroying the chunk actor entity '{:?}' has been cancelled due to the chunk actor not being loaded!", chunk_actor_id);

                destroyed_chunk_actor_entity_event_writer.send(DestroyedChunkActorEntity::Failure {
                    chunk_actor_event_id,
                    chunk_actor_id 
                });

                continue;
            }
        };

        let chunk_actor = match chunk_actor_query.get(chunk_actor_entity_reference) {
            Ok(chunk_actor) => chunk_actor,
            Err(_) => {
                error!("The request for destroying the chunk actor entity '{:?}' has been cancelled due to the chunk actor failing to be queried!", chunk_actor_id);

                destroyed_chunk_actor_entity_event_writer.send(DestroyedChunkActorEntity::Failure { 
                    chunk_actor_event_id,
                    chunk_actor_id
                });

                continue;
            }
        };

        let chunk_actor_entity_id = match entity_registry.get_loaded_entity_id(&chunk_actor_entity_reference) {
            Some(chunk_actor_entity_id) => chunk_actor_entity_id,
            None => {
                error!("The request for destroying the chunk actor entity '{:?}' has been cancelled due to the respective chunk actor entity id not being found!", chunk_actor_id);

                destroyed_chunk_actor_entity_event_writer.send(DestroyedChunkActorEntity::Failure { 
                    chunk_actor_event_id,
                    chunk_actor_id
                });

                continue;
            }
        };

        let chunk_id = chunk_actor.current_chunk();

        let chunk_entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => {
                error!("The request for destroying the chunk actor entity '{:?}' has been cancelled due to the chunk '{:?}' not being loaded!", chunk_actor_id, chunk_id);

                destroyed_chunk_actor_entity_event_writer.send(DestroyedChunkActorEntity::Failure { 
                    chunk_actor_event_id,
                    chunk_actor_id
                });

                continue;
            }
        };

        let mut chunk = match chunk_query.get_mut(chunk_entity_reference) {
            Ok(chunk) => chunk,
            Err(_) => {
                error!("The request for destroying the chunk actor entity '{:?}' has been cancelled due to the chunk '{:?}' failing to be queried!", chunk_actor_id, chunk_id);

                destroyed_chunk_actor_entity_event_writer.send(DestroyedChunkActorEntity::Failure { 
                    chunk_actor_event_id,
                    chunk_actor_id
                });

                continue;
            }
        };

        chunk.remove_chunk_actor(chunk_actor_id);

        chunk_actor_registry.unload_chunk_actor(chunk_actor_id);
        entity_registry.unload_entity(chunk_actor_entity_id);

        chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);
        entity_registry.unregister_entity(chunk_actor_entity_id);

        commands.entity(chunk_actor_entity_reference).despawn();

        info!("Successfully destroyed chunk actor entity '{:?}'!", chunk_actor_id);

        destroyed_chunk_actor_entity_event_writer.send(DestroyedChunkActorEntity::Success {
            chunk_actor_event_id,
            chunk_actor_id,
        });
    }
}

#[allow(clippy::too_many_arguments)]
#[deprecated]
pub(super) fn handle_upgrade_to_chunk_actor_entity_events_OLD(
    mut commands: Commands,
    mut upgrade_to_chunk_actor_entity_event_reader: EventReader<UpgradeToChunkActorEntity>,
    mut upgraded_to_chunk_actor_entity_event_writer: EventWriter<UpgradedToChunkActorEntity>,
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
        let chunk_actor_event_id = upgrade_to_chunk_actor_entity_event.chunk_actor_event_id;

        info!("Trying to upgrade entity '{:?}' to a chunk actor entity ...", target_entity_id);

        if let Some(chunk_entity) = chunk_registry.get_loaded_chunk_entity(chunk_id) {
            info!("Chunk '{:?}' loaded; upgrading entity '{:?}' to a chunk actor entity immediately ...", chunk_id, target_entity_id);

            let mut chunk = match chunk_query.get_mut(chunk_entity) {
                Ok(chunk) => chunk,
                Err(_) => {
                    error!("The request for upgrading entity '{:?}' to a chunk actor entity has been cancelled due to the chunk '{:?}' failing to load!", target_entity_id, chunk_id);

                    chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                        chunk_actor_event_id,
                        target_entity_id,
                        chunk_id,
                    });

                    continue;
                }
            };

            let target_entity_reference = match entity_registry.get_loaded_entity_reference(&target_entity_id) {
                Some(target_entity) => target_entity,
                None => {
                    error!("The request for upgrading entity '{:?}' to a chunk actor entity has been cancelled due to the entity reference not being found!", target_entity_id);

                    chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                        chunk_actor_event_id,
                        target_entity_id,
                        chunk_id,
                    });

                    continue;
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
                    error!("The request for upgrading entity '{:?}' to a chunk actor entity has been cancelled due to the upgrade failing!", target_entity_id);

                    chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                        chunk_actor_event_id,
                        target_entity_id,
                        chunk_id,
                    });

                    continue;
                }
            
            };

            chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity_reference);

            chunk.add_chunk_actor(chunk_actor_id);

            info!("Successfully upgraded entity '{:?}' to a chunk actor entity immediately!", target_entity_id);

            upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Success {
                chunk_actor_event_id,
                chunk_actor_id,
                target_entity_id,
                chunk_id,
            });
        } else {
            info!("Chunk '{:?}' not loaded, issuing request to upgrade entity '{:?}' to a chunk actor entity whenever the appropriate chunk is loaded ...", chunk_id, target_entity_id);

            if chunk_actor_registry.is_chunk_actor_entity_being_upgraded_to(chunk_actor_id) {
                error!("The chunk actor upgrade request for target entity '{:?}' has been cancelled due to the request already being issued!", target_entity_id);

                chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);

                upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                    chunk_actor_event_id,
                    target_entity_id,
                    chunk_id,
                });

                continue;
            }
            
            chunk_actor_registry.start_upgrading_to_chunk_actor_entity(
                UpgradeToChunkActorEntityRequest {
                    chunk_actor_event_id,
                    chunk_actor_id,
                    target_entity_id,
                    chunk_id,
                }
            );

            info!("Request to upgrade entity '{:?}' to a chunk actor entity issued!", target_entity_id);
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[deprecated]
pub(super) fn process_create_chunk_actor_entity_requests_OLD(
    mut commands: Commands,
    mut created_chunk_entity_event_reader: EventReader<CreatedChunkEntity>,
    mut loaded_chunk_entity_event_reader: EventReader<LoadedChunkEntity>,
    mut created_chunk_actor_entity_event_writer: EventWriter<CreatedChunkActorEntity>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    chunk_registry: ResMut<ChunkRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
    mut chunk_query: Query<&mut Chunk>,
) {
    let mut created_chunk_entity_events = Vec::new();
    for created_chunk_entity_event in created_chunk_entity_event_reader.read() {
        created_chunk_entity_events.push(created_chunk_entity_event.clone());
    }

    let mut loaded_chunk_entity_events = Vec::new();
    for loaded_chunk_entity_event in loaded_chunk_entity_event_reader.read() {
        loaded_chunk_entity_events.push(loaded_chunk_entity_event.clone());
    }

    for created_chunk_entity_event in created_chunk_entity_events {
        let (_, created_chunk_id) = match created_chunk_entity_event {
            CreatedChunkEntity::Success { chunk_event_id, chunk_id } => {
                (chunk_event_id, chunk_id)
            },
            CreatedChunkEntity::Failure { chunk_id, .. } => {
                error!("The chunk actor entity creation requests related to chunk '{:?}' have been cancelled due to the chunk failing to be loaded!", chunk_id);
    
                let requests = chunk_actor_registry.create_chunk_actor_entity_requests().clone();
                requests.values().filter(|request| request.chunk_id == chunk_id).for_each(|request| {
                    entity_registry.unregister_entity(request.chunk_actor_entity_id);
                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);
    
                    chunk_actor_registry.stop_creating_chunk_actor_entity(request.chunk_actor_id);
    
                    created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Failure {
                        chunk_actor_event_id: request.chunk_actor_event_id,
                        chunk_id: request.chunk_id,
                        world_position: request.world_position,
                    });
                });
    
                continue;
            },
        };

        let chunk_entity_reference = match chunk_registry.get_loaded_chunk_entity(created_chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => {
                error!("The chunk actor entity creation requests related to chunk '{:?}' have been cancelled due to the chunk not being loaded!", created_chunk_id);

                let requests = chunk_actor_registry.create_chunk_actor_entity_requests().clone();
                requests.values().filter(|request| request.chunk_id == created_chunk_id).for_each(|request| {
                    entity_registry.unregister_entity(request.chunk_actor_entity_id);
                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_creating_chunk_actor_entity(request.chunk_actor_id);

                    created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Failure {
                        chunk_actor_event_id: request.chunk_actor_event_id,
                        chunk_id: request.chunk_id,
                        world_position: request.world_position,
                    });
                });

                continue;
            }
        };

        let mut chunk = match chunk_query.get_mut(chunk_entity_reference) {
            Ok(chunk) => chunk,
            Err(_) => {
                error!("The chunk actor entity creation requests related to chunk '{:?}' have been cancelled due to the chunk failing to be queried!", created_chunk_id);

                let requests = chunk_actor_registry.create_chunk_actor_entity_requests().clone();
                requests.values().filter(|request| request.chunk_id == created_chunk_id).for_each(|request| {
                    entity_registry.unregister_entity(request.chunk_actor_entity_id);
                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_creating_chunk_actor_entity(request.chunk_actor_id);

                    created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Failure {
                        chunk_actor_event_id: request.chunk_actor_event_id,
                        chunk_id: request.chunk_id,
                        world_position: request.world_position,
                    });
                });

                continue;
            }
        };

        let create_chunk_actor_entity_requests = chunk_actor_registry.create_chunk_actor_entity_requests().clone();
        for create_chunk_actor_entity_request in create_chunk_actor_entity_requests.values() {
            let chunk_actor_event_id = create_chunk_actor_entity_request.chunk_actor_event_id;
            let chunk_actor_id = create_chunk_actor_entity_request.chunk_actor_id;
            let chunk_actor_entity_id = create_chunk_actor_entity_request.chunk_actor_entity_id;
            let chunk_id = create_chunk_actor_entity_request.chunk_id;
            let world_position = create_chunk_actor_entity_request.world_position;

            if chunk_id != created_chunk_id {
                continue;
            }

            let chunk_actor_entity = functions::new_chunk_actor_entity(&mut commands, chunk_actor_id, created_chunk_id, world_position);

            entity_registry.load_entity(chunk_actor_entity_id, chunk_actor_entity);
            chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity);

            chunk.add_chunk_actor(chunk_actor_id);

            chunk_actor_registry.stop_creating_chunk_actor_entity(chunk_actor_id);

            created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Success {
                chunk_actor_event_id,
                chunk_actor_id,
                chunk_actor_entity_id,
                chunk_id: created_chunk_id,
                world_position,
            });
        }
    }

    for loaded_chunk_entity_event in loaded_chunk_entity_events {
        let (_, created_chunk_id) = match loaded_chunk_entity_event {
            LoadedChunkEntity::Success { chunk_event_id, chunk_id } => (chunk_event_id, chunk_id),
            LoadedChunkEntity::Failure { chunk_id, .. } => {
                error!("The chunk actor entity creation requests related to chunk '{:?}' have been cancelled due to the chunk failing to be loaded!", chunk_id);

                let requests = chunk_actor_registry.create_chunk_actor_entity_requests().clone();
                requests.values().filter(|request| request.chunk_id == chunk_id).for_each(|request| {
                    entity_registry.unregister_entity(request.chunk_actor_entity_id);
                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_creating_chunk_actor_entity(request.chunk_actor_id);

                    created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Failure {
                        chunk_actor_event_id: request.chunk_actor_event_id,
                        chunk_id: request.chunk_id,
                        world_position: request.world_position,
                    });
                });

                continue;
            },
        };

        let chunk_entity_reference = match chunk_registry.get_loaded_chunk_entity(created_chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => {
                error!("The chunk actor entity creation requests related to chunk '{:?}' have been cancelled due to the chunk not being loaded!", created_chunk_id);

                let requests = chunk_actor_registry.create_chunk_actor_entity_requests().clone();
                requests.values().filter(|request| request.chunk_id == created_chunk_id).for_each(|request| {
                    entity_registry.unregister_entity(request.chunk_actor_entity_id);
                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_creating_chunk_actor_entity(request.chunk_actor_id);

                    created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Failure {
                        chunk_actor_event_id: request.chunk_actor_event_id,
                        chunk_id: request.chunk_id,
                        world_position: request.world_position,
                    });
                });

                continue;
            }
        };

        let mut chunk = match chunk_query.get_mut(chunk_entity_reference) {
            Ok(chunk) => chunk,
            Err(_) => {
                error!("The chunk actor entity creation requests related to chunk '{:?}' have been cancelled due to the chunk failing to be queried!", created_chunk_id);

                let requests = chunk_actor_registry.create_chunk_actor_entity_requests().clone();
                requests.values().filter(|request| request.chunk_id == created_chunk_id).for_each(|request| {
                    entity_registry.unregister_entity(request.chunk_actor_entity_id);
                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_creating_chunk_actor_entity(request.chunk_actor_id);

                    created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Failure {
                        chunk_actor_event_id: request.chunk_actor_event_id,
                        chunk_id: request.chunk_id,
                        world_position: request.world_position,
                    });
                });

                continue;
            }
        };

        let create_chunk_actor_entity_requests = chunk_actor_registry.create_chunk_actor_entity_requests().clone();
        for create_chunk_actor_entity_request in create_chunk_actor_entity_requests.values() {
            let chunk_actor_event_id = create_chunk_actor_entity_request.chunk_actor_event_id;
            let chunk_actor_id = create_chunk_actor_entity_request.chunk_actor_id;
            let chunk_actor_entity_id = create_chunk_actor_entity_request.chunk_actor_entity_id;
            let chunk_id = create_chunk_actor_entity_request.chunk_id;
            let world_position = create_chunk_actor_entity_request.world_position;

            if chunk_id != created_chunk_id {
                continue;
            }

            let chunk_actor_entity = functions::new_chunk_actor_entity(&mut commands, chunk_actor_id, chunk_id, world_position);

            entity_registry.load_entity(chunk_actor_entity_id, chunk_actor_entity);
            chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity);

            chunk.add_chunk_actor(chunk_actor_id);

            chunk_actor_registry.stop_creating_chunk_actor_entity(chunk_actor_id);

            created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity::Success {
                chunk_actor_event_id,
                chunk_actor_id,
                chunk_actor_entity_id,
                chunk_id,
                world_position,
            });
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[deprecated]
pub(super) fn process_upgrade_to_chunk_actor_entity_requests_OLD(
    mut commands: Commands,
    mut created_chunk_entity_event_reader: EventReader<CreatedChunkEntity>,
    mut loaded_chunk_entity_event_reader: EventReader<LoadedChunkEntity>,
    mut upgraded_to_chunk_actor_entity_event_writer: EventWriter<UpgradedToChunkActorEntity>,
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

    let mut loaded_chunk_entity_events = Vec::new();
    for loaded_chunk_entity_event in loaded_chunk_entity_event_reader.read() {
        loaded_chunk_entity_events.push(loaded_chunk_entity_event.clone());
    }

    for created_chunk_entity_event in created_chunk_entity_events {
        let (_, created_chunk_id) = match created_chunk_entity_event {
            CreatedChunkEntity::Success { chunk_event_id, chunk_id } => {
                (chunk_event_id, chunk_id)
            },
            CreatedChunkEntity::Failure { chunk_id, .. } => {
                let requests = chunk_actor_registry.upgrade_to_chunk_actor_entity_requests().clone();
                for request in requests.values() {
                    if request.chunk_id != chunk_id {
                        error!("The request for upgrading entity '{:?}' to a chunk actor entity has been cancelled due to the starting chunk '{:?}' failing to load!", request.target_entity_id, request.chunk_id);
                        
                        chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);
    
                        chunk_actor_registry.stop_creating_chunk_actor_entity(request.chunk_actor_id);
    
                        upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                            chunk_actor_event_id: request.chunk_actor_event_id,
                            target_entity_id: request.target_entity_id,
                            chunk_id: request.chunk_id,
                        });
                    }
                }
    
                continue;
            },
        };

        let chunk_entity_reference = match chunk_registry.get_loaded_chunk_entity(created_chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => {
                error!("The chunk actor entity creation requests related to chunk '{:?}' have been cancelled due to the chunk not being loaded!", created_chunk_id);

                let requests = chunk_actor_registry.upgrade_to_chunk_actor_entity_requests().clone();
                requests.values().filter(|request| request.chunk_id == created_chunk_id).for_each(|request| {
                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_upgrading_to_chunk_actor_entity(request.chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                        chunk_actor_event_id: request.chunk_actor_event_id,
                        target_entity_id: request.target_entity_id,
                        chunk_id: request.chunk_id,
                    });
                });

                continue;
            }
        };

        let mut chunk = match chunk_query.get_mut(chunk_entity_reference) {
            Ok(chunk) => chunk,
            Err(_) => {
                error!("The chunk actor entity creation requests related to chunk '{:?}' have been cancelled due to the chunk failing to be queried!", created_chunk_id);

                let requests = chunk_actor_registry.upgrade_to_chunk_actor_entity_requests().clone();
                requests.values().filter(|request| request.chunk_id == created_chunk_id).for_each(|request| {
                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_upgrading_to_chunk_actor_entity(request.chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                        chunk_actor_event_id: request.chunk_actor_event_id,
                        target_entity_id: request.target_entity_id,
                        chunk_id: request.chunk_id,
                    });
                });

                continue;
            }
        };

        let upgrade_to_chunk_actor_entity_requests = chunk_actor_registry.upgrade_to_chunk_actor_entity_requests().clone();
        for upgrade_to_chunk_actor_entity_request in upgrade_to_chunk_actor_entity_requests.values() {
            let chunk_actor_event_id = upgrade_to_chunk_actor_entity_request.chunk_actor_event_id;
            let chunk_actor_id = upgrade_to_chunk_actor_entity_request.chunk_actor_id;
            let target_entity_id = upgrade_to_chunk_actor_entity_request.target_entity_id;
            let chunk_id = upgrade_to_chunk_actor_entity_request.chunk_id;

            if chunk_id != created_chunk_id {
                continue;
            }

            let target_entity_reference = match entity_registry.get_loaded_entity_reference(&target_entity_id) {
                Some(target_entity) => target_entity,
                None => {
                    error!("The request for upgrading entity '{:?}' to a chunk actor entity has been cancelled due to the entity reference not being found!", target_entity_id);

                    chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                        chunk_actor_event_id,
                        target_entity_id,
                        chunk_id,
                    });

                    continue;
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
                    error!("The request for upgrading entity '{:?}' to a chunk actor entity has been cancelled due to the upgrade failing!", target_entity_id);

                    chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                        chunk_actor_event_id,
                        target_entity_id,
                        chunk_id,
                    });

                    continue;
                }
            };

            chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity_reference);

            chunk.add_chunk_actor(chunk_actor_id);

            chunk_actor_registry.stop_upgrading_to_chunk_actor_entity(chunk_actor_id);

            info!("Successfully upgraded entity '{:?}' to a chunk actor entity after the associated chunk '{:?}' had been created!", target_entity_id, chunk_id);

            upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Success {
                chunk_actor_event_id,
                chunk_actor_id,
                target_entity_id,
                chunk_id,
            });
        }
    }

    for loaded_chunk_entity_event in loaded_chunk_entity_events {
        let (_, created_chunk_id) = match loaded_chunk_entity_event {
            LoadedChunkEntity::Success { chunk_event_id, chunk_id } => (chunk_event_id, chunk_id),
            LoadedChunkEntity::Failure { chunk_id, .. } => {
                let requests = chunk_actor_registry.upgrade_to_chunk_actor_entity_requests().clone();
                for request in requests.values() {
                    if request.chunk_id != chunk_id {
                        error!("The request for upgrading entity '{:?}' to a chunk actor entity has been cancelled due to the starting chunk '{:?}' failing to load!", request.target_entity_id, request.chunk_id);
                        
                        chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);
    
                        chunk_actor_registry.stop_creating_chunk_actor_entity(request.chunk_actor_id);
    
                        upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                            chunk_actor_event_id: request.chunk_actor_event_id,
                            target_entity_id: request.target_entity_id,
                            chunk_id: request.chunk_id,
                        });
                    }
                }
    
                continue;
            }
        };

        let chunk_entity_reference = match chunk_registry.get_loaded_chunk_entity(created_chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => {
                error!("The chunk actor entity creation requests related to chunk '{:?}' have been cancelled due to the chunk not being loaded!", created_chunk_id);

                let requests = chunk_actor_registry.upgrade_to_chunk_actor_entity_requests().clone();
                requests.values().filter(|request| request.chunk_id == created_chunk_id).for_each(|request| {
                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_upgrading_to_chunk_actor_entity(request.chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                        chunk_actor_event_id: request.chunk_actor_event_id,
                        target_entity_id: request.target_entity_id,
                        chunk_id: request.chunk_id,
                    });
                });

                continue;
            }
        };

        let mut chunk = match chunk_query.get_mut(chunk_entity_reference) {
            Ok(chunk) => chunk,
            Err(_) => {
                error!("The chunk actor entity creation requests related to chunk '{:?}' have been cancelled due to the chunk failing to be queried!", created_chunk_id);

                let requests = chunk_actor_registry.upgrade_to_chunk_actor_entity_requests().clone();
                requests.values().filter(|request| request.chunk_id == created_chunk_id).for_each(|request| {
                    chunk_actor_registry.unregister_chunk_actor(request.chunk_actor_id);

                    chunk_actor_registry.stop_upgrading_to_chunk_actor_entity(request.chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                        chunk_actor_event_id: request.chunk_actor_event_id,
                        target_entity_id: request.target_entity_id,
                        chunk_id: request.chunk_id,
                    });
                });

                continue;
            }
        };

        let upgrade_to_chunk_actor_entity_requests = chunk_actor_registry.upgrade_to_chunk_actor_entity_requests().clone();
        for upgrade_to_chunk_actor_entity_request in upgrade_to_chunk_actor_entity_requests.values() {
            let chunk_actor_event_id = upgrade_to_chunk_actor_entity_request.chunk_actor_event_id;
            let chunk_actor_id = upgrade_to_chunk_actor_entity_request.chunk_actor_id;
            let target_entity_id = upgrade_to_chunk_actor_entity_request.target_entity_id;
            let chunk_id = upgrade_to_chunk_actor_entity_request.chunk_id;

            if chunk_id != created_chunk_id {
                continue;
            }

            let target_entity_reference = match entity_registry.get_loaded_entity_reference(&target_entity_id) {
                Some(target_entity) => target_entity,
                None => {
                    error!("The request for upgrading entity '{:?}' to a chunk actor entity has been cancelled due to the entity reference not being found!", target_entity_id);

                    chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                        chunk_actor_event_id,
                        target_entity_id,
                        chunk_id,
                    });

                    continue;
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
                    error!("The request for upgrading entity '{:?}' to a chunk actor entity has been cancelled due to the upgrade failing!", target_entity_id);

                    chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);

                    upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Failure {
                        chunk_actor_event_id,
                        target_entity_id,
                        chunk_id,
                    });

                    continue;
                }
            };

            chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity_reference);

            chunk.add_chunk_actor(chunk_actor_id);

            chunk_actor_registry.stop_upgrading_to_chunk_actor_entity(chunk_actor_id);

            info!("Successfully upgraded entity '{:?}' to a chunk actor entity after the associated chunk '{:?}' had been loaded!", target_entity_id, chunk_id);

            upgraded_to_chunk_actor_entity_event_writer.send(UpgradedToChunkActorEntity::Success {
                chunk_actor_event_id,
                chunk_actor_id,
                target_entity_id,
                chunk_id,
            });
        }
    }
}
