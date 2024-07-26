use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::events::*;
use crate::chunk::resources::*;
use crate::chunk::functions;
use crate::entity;
use crate::entity::resources::*;

use super::components::Chunk;
use super::structs::ChunkRequest;

pub(super) fn handle_upgrade_to_chunk(
    world: &mut World,
    event_parameters: &mut SystemState<
        EventReader<UpgradeToChunk>,
    >,
    registry_parameters: &mut SystemState<
        ResMut<EntityRegistry>,
    >,
) {
    let mut upgrade_to_chunk_event_reader = event_parameters.get_mut(world);

    let mut upgrade_to_chunk_events = Vec::new();
    for upgrade_to_chunk_event in upgrade_to_chunk_event_reader.read() {
        upgrade_to_chunk_events.push(upgrade_to_chunk_event.clone());
    }

    for upgrade_to_chunk_event in upgrade_to_chunk_events {
        let upgrade_chunk_request = upgrade_to_chunk_event.0;

        let chunk_id = upgrade_chunk_request.chunk_id;
        let entity_id = upgrade_chunk_request.entity_id;

        let entity_reference = {
            let entity_registry = registry_parameters.get_mut(world);
            match entity_registry.get_loaded_entity_reference(&entity_id) {
                Some(entity_reference) => entity_reference.clone(),
                None => {
                    panic!("Entity reference associated with entity id '{:?}' not found!", entity_id);
                }
            }
        };

        world.entity_mut(entity_reference).insert(Chunk::new(chunk_id));
    }
}

pub(super) fn handle_downgrade_from_chunk(
    world: &mut World,
    event_parameters: &mut SystemState<
        EventReader<DowngradeFromChunk>,
    >,
    registry_parameters: &mut SystemState<
        ResMut<EntityRegistry>,
    >,
) {
    let mut downgrade_from_chunk_event_reader = event_parameters.get_mut(world);

    let mut downgrade_from_chunk_events = Vec::new();
    for downgrade_from_chunk_event in downgrade_from_chunk_event_reader.read() {
        downgrade_from_chunk_events.push(downgrade_from_chunk_event.clone());
    }

    for downgrade_from_chunk_event in downgrade_from_chunk_events {
        let downgrade_chunk_request = downgrade_from_chunk_event.0;

        let entity_id = downgrade_chunk_request.entity_id;

        let entity_reference = {
            let entity_registry = registry_parameters.get_mut(world);
            match entity_registry.get_loaded_entity_reference(&entity_id) {
                Some(entity_reference) => entity_reference.clone(),
                None => {
                    panic!("Entity reference associated with entity id '{:?}' not found!", entity_id);
                }
            }
        };

        world.entity_mut(entity_reference).remove::<Chunk>();
    }
}

pub(super) fn handle_load_chunk(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<LoadChunk>,
    )>,
    registry_parameters: &mut SystemState<
        ResMut<ChunkRegistry>,
    >,
) {
    let mut load_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut load_chunk_events = Vec::new();
    for load_chunk_event in load_chunk_event_reader.read() {
        load_chunk_events.push(load_chunk_event.clone());
    }

    for load_chunk_event in load_chunk_events {
        let load_chunk_request = load_chunk_event.0;

        let chunk_id = load_chunk_request.chunk_id;

        let serialized_chunk = {
            let mut chunk_registry = registry_parameters.get_mut(world);

            chunk_registry.deserialize_chunk(chunk_id).unwrap()
        };

        let _chunk_entity = functions::deserialize_chunk(world, serialized_chunk);
    }
}

pub(super) fn handle_save_chunk(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<SaveChunk>,
    )>,
    registry_parameter: &mut SystemState<
        ResMut<ChunkRegistry>,
    >,
) {
    let mut save_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut save_chunk_events = Vec::new();
    for save_chunk_event in save_chunk_event_reader.read() {
        save_chunk_events.push(save_chunk_event.clone());
    }

    for save_chunk_event in save_chunk_events {
        let save_chunk_request = save_chunk_event.0;

        let chunk_id = save_chunk_request.chunk_id;
        let entity_id = save_chunk_request.entity_id;

        let serialized_chunk = functions::serialize_chunk(world, registry_parameter, chunk_id);

        let mut chunk_registry = registry_parameter.get_mut(world).0;

        chunk_registry.serialize_chunk(chunk_id, serialized_chunk);
    }
}

pub(super) fn handle_create_chunk_entity_events(
    mut create_chunk_entity_event_reader: EventReader<CreateChunkEntity>,
    mut create_chunk_entity_internal_event_writer: EventWriter<CreateChunkEntityInternal>,
) {
    let mut chunk_infos = Vec::new();
    for create_chunk_entity_event in create_chunk_entity_event_reader.read() {
        chunk_infos.push((
            create_chunk_entity_event.chunk_request_id,
            create_chunk_entity_event.chunk_id, 
        ));
    }

    for (chunk_request_id, chunk_id) in chunk_infos {
        info!("Trying to create chunk '{:?}' ...", chunk_id);

        create_chunk_entity_internal_event_writer.send(CreateChunkEntityInternal {
            chunk_request_id,
            chunk_id
        });
    }
}

pub(super) fn handle_destroy_chunk_entity_events(
    mut destroy_chunk_entity_event_reader: EventReader<DestroyChunkEntity>,
    mut destroy_chunk_entity_internal_event_writer: EventWriter<DestroyChunkEntityInternal>,
) {
    let mut chunk_infos = Vec::new();
    for destroy_chunk_entity_event in destroy_chunk_entity_event_reader.read() {
        chunk_infos.push((
            destroy_chunk_entity_event.chunk_request_id,
            destroy_chunk_entity_event.chunk_id
        ));
    }

    for (chunk_request_id, chunk_id) in chunk_infos {
        info!("Trying to destroy chunk '{:?}' ...", chunk_id);

        destroy_chunk_entity_internal_event_writer.send(DestroyChunkEntityInternal {
            chunk_request_id,
            chunk_id
        });
    }
}

pub(super) fn handle_load_chunk_entity_events(
    mut load_chunk_entity_event_reader: EventReader<LoadChunkEntity>,
    mut load_chunk_entity_internal_event_writer: EventWriter<LoadChunkEntityInternal>,
) {
    let mut chunk_infos = Vec::new();
    for load_chunk_entity_event in load_chunk_entity_event_reader.read() {
        chunk_infos.push((
            load_chunk_entity_event.chunk_request_id,
            load_chunk_entity_event.chunk_id
        ));
    }

    for (chunk_request_id, chunk_id) in chunk_infos {
        info!("Trying to load chunk '{:?}' ...", chunk_id);

        load_chunk_entity_internal_event_writer.send(LoadChunkEntityInternal { 
            chunk_request_id,
            chunk_id
        });
    }
}

pub(super) fn handle_unload_chunk_entity_events(
    mut unload_chunk_entity_event_reader: EventReader<UnloadChunkEntity>,
    mut unload_chunk_entity_internal_event_writer: EventWriter<UnloadChunkEntityInternal>,
) {
    let mut chunk_infos = Vec::new();
    for unload_chunk_entity_event in unload_chunk_entity_event_reader.read() {
        chunk_infos.push((
            unload_chunk_entity_event.chunk_request_id,
            unload_chunk_entity_event.chunk_id
        ));
    }

    for (chunk_request_id, chunk_id) in chunk_infos {
        info!("Trying to unload chunk '{:?}' ...", chunk_id);

        unload_chunk_entity_internal_event_writer.send(UnloadChunkEntityInternal { 
            chunk_request_id,
            chunk_id
        });
    }
}

pub(super) fn handle_create_chunk_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<CreateChunkEntityInternal>,
        EventWriter<CreatedChunkEntityInternal>,
    )>,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<EntityRegistry>,
    )>,
) {
    let mut create_chunk_entity_event_reader = event_parameters.get_mut(world).0;

    let mut create_chunk_entity_events: Vec<CreateChunkEntityInternal> = Vec::new();
    for create_chunk_entity_event in create_chunk_entity_event_reader.read() {
        create_chunk_entity_events.push(create_chunk_entity_event.clone());
    }

    for create_chunk_entity_event in create_chunk_entity_events {
        let chunk_request_id = create_chunk_entity_event.chunk_request_id;
        let chunk_id = create_chunk_entity_event.chunk_id;

        let (chunk_registry, _) = registry_parameters.get_mut(world);

        let is_chunk_registered = chunk_registry.is_chunk_registered(chunk_id);
        if is_chunk_registered {
            warn!("The request for creating chunk '{:?}' has been cancelled due to the chunk already being registered!", chunk_id);
            
            let mut created_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            created_chunk_entity_event_writer.send(CreatedChunkEntityInternal::Failure { 
                chunk_request_id, 
                chunk_id
            });

            continue;
        }

        let is_chunk_loaded = chunk_registry.is_chunk_loaded(chunk_id);
        if is_chunk_loaded {
            warn!("The request for creating chunk '{:?}' has been cancelled due to the chunk already being loaded!", chunk_id);

            let mut created_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            created_chunk_entity_event_writer.send(CreatedChunkEntityInternal::Failure { 
                chunk_request_id,
                chunk_id
            });

            continue;
        }

        let (mut chunk_registry, mut entity_registry) = registry_parameters.get_mut(world);
        let entity_id = entity_registry.register_entity();
        chunk_registry.register_chunk(chunk_id);

        let new_chunk_entity = functions::new_chunk_entity(world, chunk_id);
        
        let (mut chunk_registry, mut entity_registry) = registry_parameters.get_mut(world);
        entity_registry.load_entity(entity_id, new_chunk_entity);
        chunk_registry.load_chunk(chunk_id, new_chunk_entity);

        chunk_registry.stop_creating_chunk(chunk_id);

        let mut created_chunk_entity_event_writer = event_parameters.get_mut(world).1;
        created_chunk_entity_event_writer.send(CreatedChunkEntityInternal::Success { 
            chunk_request_id,
            chunk_id
        });
    }
}

pub(super) fn handle_destroy_chunk_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<DestroyChunkEntityInternal>,
        EventWriter<DestroyedChunkEntityInternal>,
    )>,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<EntityRegistry>,
    )>,
) {
    let mut destroy_chunk_entity_event_reader = event_parameters.get_mut(world).0;

    let mut destroy_chunk_entity_events: Vec<DestroyChunkEntityInternal> = Vec::new();
    for destroy_chunk_entity_event in destroy_chunk_entity_event_reader.read() {
        destroy_chunk_entity_events.push(destroy_chunk_entity_event.clone());
    }

    for destroy_chunk_entity_event in destroy_chunk_entity_events {
        let chunk_request_id = destroy_chunk_entity_event.chunk_request_id;
        let chunk_id = destroy_chunk_entity_event.chunk_id;

        let (mut chunk_registry, mut entity_registry) = registry_parameters.get_mut(world);

        let is_chunk_loaded = chunk_registry.is_chunk_loaded(chunk_id);
        if !is_chunk_loaded {
            warn!("Chunk '{:?}' is already unloaded!", chunk_id);
            
            let mut destroyed_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            destroyed_chunk_entity_event_writer.send(DestroyedChunkEntityInternal::Failure { chunk_request_id, chunk_id });
            
            continue;
        }
        
        let is_chunk_registered = chunk_registry.is_chunk_registered(chunk_id);
        if !is_chunk_registered {
            warn!("Chunk '{:?}' is already unregistered!", chunk_id);
            
            let mut destroyed_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            destroyed_chunk_entity_event_writer.send(DestroyedChunkEntityInternal::Failure { chunk_request_id, chunk_id });

            continue;
        }

        let chunk_entity = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => continue,
        };
        let chunk_entity_id = match entity_registry.get_loaded_entity_id(&chunk_entity) {
            Some(entity_id) => entity_id,
            None => continue,
        };

        let _ = chunk_registry.unload_chunk(chunk_id);
        let _ = entity_registry.unload_entity(chunk_entity_id);

        world.despawn(chunk_entity);

        let (mut chunk_registry, mut entity_registry) = registry_parameters.get_mut(world);
        entity_registry.unregister_entity(chunk_entity_id);
        chunk_registry.unregister_chunk(chunk_id);

        chunk_registry.stop_destroying_chunk(chunk_id);

        let mut destroyed_chunk_entity_event_writer = event_parameters.get_mut(world).1;
        destroyed_chunk_entity_event_writer.send(DestroyedChunkEntityInternal::Success {
            chunk_request_id, 
            chunk_id
        });
    }
}

pub(super) fn handle_load_chunk_entity_internal_events(
    world: &mut World,
    registry_parameter: &mut SystemState<(
        ResMut<ChunkRegistry>,
    )>,
    event_parameters: &mut SystemState<(
        EventReader<LoadChunkEntityInternal>,
        EventWriter<LoadedChunkEntityInternal>,
    )>
) {
    let mut load_chunk_entity_event_reader = event_parameters.get_mut(world).0;

    let mut load_chunk_entity_events: Vec<LoadChunkEntityInternal> = Vec::new();
    for load_chunk_entity_event in load_chunk_entity_event_reader.read() {
        load_chunk_entity_events.push(load_chunk_entity_event.clone());
    }

    for load_chunk_entity_event in load_chunk_entity_events {
        let chunk_request_id = load_chunk_entity_event.chunk_request_id;
        let chunk_id = load_chunk_entity_event.chunk_id;

        let chunk_registry = registry_parameter.get_mut(world).0;

        let is_chunk_registered = chunk_registry.is_chunk_registered(chunk_id);
        let is_chunk_loaded = chunk_registry.is_chunk_loaded(chunk_id);

        if !is_chunk_registered {
            warn!("Chunk '{:?}' is not registered!", chunk_id);

            let mut loaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            loaded_chunk_entity_event_writer.send(LoadedChunkEntityInternal::Failure { chunk_request_id, chunk_id });

            continue;
        }

        if is_chunk_loaded {
            warn!("Chunk '{:?}' is already loaded!", chunk_id);

            let mut loaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            loaded_chunk_entity_event_writer.send(LoadedChunkEntityInternal::Failure { chunk_request_id, chunk_id });

            continue;
        }
        
        let serialized_chunk = {
            let mut chunk_registry = registry_parameter.get_mut(world).0;

            chunk_registry.deserialize_chunk(chunk_id).unwrap()
        };

        let chunk_entity = functions::deserialize_chunk(world, serialized_chunk);

        let mut chunk_registry = registry_parameter.get_mut(world).0;

        chunk_registry.load_chunk(chunk_id, chunk_entity);

        chunk_registry.stop_loading_chunk(chunk_id);

        let mut loaded_chunk_entity_event_writer: EventWriter<LoadedChunkEntityInternal> = event_parameters.get_mut(world).1;
        loaded_chunk_entity_event_writer.send(LoadedChunkEntityInternal::Success { chunk_request_id, chunk_id });
    }
}

pub(super) fn handle_unload_chunk_entity_internal_events(
    world: &mut World,
    registry_parameter: &mut SystemState<(
        ResMut<ChunkRegistry>,
    )>,
    event_parameters: &mut SystemState<(
        EventReader<UnloadChunkEntityInternal>,
        EventWriter<UnloadedChunkEntityInternal>,
    )>
) {
    let mut unload_chunk_entity_event_reader = event_parameters.get_mut(world).0;

    let mut unload_chunk_entity_events: Vec<UnloadChunkEntityInternal> = Vec::new();
    for unload_chunk_entity_event in unload_chunk_entity_event_reader.read() {
        unload_chunk_entity_events.push(unload_chunk_entity_event.clone());
    }

    for unload_chunk_entity_event in unload_chunk_entity_events {
        let chunk_request_id = unload_chunk_entity_event.chunk_request_id;
        let chunk_id = unload_chunk_entity_event.chunk_id;

        let chunk_registry = registry_parameter.get_mut(world).0;

        let is_chunk_registered = chunk_registry.is_chunk_registered(chunk_id);
        let is_chunk_loaded = chunk_registry.is_chunk_loaded(chunk_id);

        if !is_chunk_loaded {
            warn!("Chunk '{:?}' is already unloaded!", chunk_id);

            let mut unloaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            unloaded_chunk_entity_event_writer.send(UnloadedChunkEntityInternal::Failure { chunk_request_id, chunk_id });

            continue;
        }

        if !is_chunk_registered {
            warn!("Chunk '{:?}' is already unregistered!", chunk_id);

            let mut unloaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            unloaded_chunk_entity_event_writer.send(UnloadedChunkEntityInternal::Failure { chunk_request_id, chunk_id });

            continue;
        }

        let serialized_chunk = functions::serialize_chunk(world, registry_parameter, chunk_id);

        let mut chunk_registry = registry_parameter.get_mut(world).0;

        chunk_registry.serialize_chunk(chunk_id, serialized_chunk);

        chunk_registry.unload_chunk(chunk_id);

        chunk_registry.stop_unloading_chunk(chunk_id);

        let mut unloaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
        unloaded_chunk_entity_event_writer.send(UnloadedChunkEntityInternal::Success { chunk_request_id, chunk_id });
    }
}

pub(super) fn handle_created_chunk_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<CreatedChunkEntityInternal>,
        EventWriter<CreatedChunkEntity>,
    )>,
    registry_parameter: &mut SystemState<ResMut<ChunkRegistry>>,
) {
    let mut created_chunk_entity_event_reader = event_parameters.get_mut(world).0;

    let mut created_chunk_entity_events: Vec<CreatedChunkEntityInternal> = Vec::new();
    for created_chunk_entity_event in created_chunk_entity_event_reader.read() {
        created_chunk_entity_events.push(created_chunk_entity_event.clone());
    }

    for created_chunk_entity_event in created_chunk_entity_events {
        match created_chunk_entity_event {
            CreatedChunkEntityInternal::Success { chunk_request_id, chunk_id } => {
                info!("Chunk '{:?}' has been created successfully!", chunk_id);
                
                let mut chunk_registry = registry_parameter.get_mut(world);
                if !chunk_registry.try_deallocate_chunk(chunk_id) {
                    error!("Chunk '{:?}' has failed to be deallocated!", chunk_id);
                }

                let mut created_chunk_entity_event_writer = event_parameters.get_mut(world).1;
                created_chunk_entity_event_writer.send(CreatedChunkEntity::Success { chunk_request_id, chunk_id });
            },
            CreatedChunkEntityInternal::Failure { chunk_request_id, chunk_id } => {
                error!("Chunk '{:?}' has failed to be created!", chunk_id);
                
                let mut chunk_registry = registry_parameter.get_mut(world);
                if !chunk_registry.try_deallocate_chunk(chunk_id) {
                    error!("Chunk '{:?}' has failed to be deallocated!", chunk_id);
                }

                let mut created_chunk_entity_event_writer = event_parameters.get_mut(world).1;
                created_chunk_entity_event_writer.send(CreatedChunkEntity::Failure { chunk_request_id, chunk_id });
            },
        }
    }
}

pub(super) fn handle_destroyed_chunk_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<DestroyedChunkEntityInternal>,
        EventWriter<DestroyedChunkEntity>,
    )>,
    registry_parameter: &mut SystemState<ResMut<ChunkRegistry>>,
) {
    let mut destroyed_chunk_entity_event_reader = event_parameters.get_mut(world).0;

    let mut destroyed_chunk_entity_events: Vec<DestroyedChunkEntityInternal> = Vec::new();
    for destroyed_chunk_entity_event in destroyed_chunk_entity_event_reader.read() {
        destroyed_chunk_entity_events.push(destroyed_chunk_entity_event.clone());
    }

    for destroyed_chunk_entity_event in destroyed_chunk_entity_events {
        match destroyed_chunk_entity_event {
            DestroyedChunkEntityInternal::Success { chunk_request_id, chunk_id } => {
                info!("Chunk '{:?}' has been destroyed successfully!", chunk_id);

                let mut chunk_registry = registry_parameter.get_mut(world);
                if !chunk_registry.try_deallocate_chunk(chunk_id) {
                    error!("Chunk '{:?}' has failed to be deallocated!", chunk_id);
                }

                let mut destroyed_chunk_entity_event_writer = event_parameters.get_mut(world).1;
                destroyed_chunk_entity_event_writer.send(DestroyedChunkEntity::Success { chunk_request_id, chunk_id });
            },
            DestroyedChunkEntityInternal::Failure { chunk_request_id, chunk_id } => {
                error!("Chunk '{:?}' has failed to be destroyed!", chunk_id);

                let mut chunk_registry = registry_parameter.get_mut(world);
                if !chunk_registry.try_deallocate_chunk(chunk_id) {
                    error!("Chunk '{:?}' has failed to be deallocated!", chunk_id);
                }

                let mut destroyed_chunk_entity_event_writer = event_parameters.get_mut(world).1;
                destroyed_chunk_entity_event_writer.send(DestroyedChunkEntity::Failure { chunk_request_id, chunk_id });
            },
        }
    }
}

pub(super) fn handle_loaded_chunk_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<LoadedChunkEntityInternal>,
        EventWriter<LoadedChunkEntity>,
    )>,
    registry_parameter: &mut SystemState<ResMut<ChunkRegistry>>,
) {
    let mut loaded_chunk_entity_event_reader = event_parameters.get_mut(world).0;

    let mut loaded_chunk_entity_events: Vec<LoadedChunkEntityInternal> = Vec::new();
    for loaded_chunk_entity_event in loaded_chunk_entity_event_reader.read() {
        loaded_chunk_entity_events.push(loaded_chunk_entity_event.clone());
    }

    for loaded_chunk_entity_event in loaded_chunk_entity_events {
        match loaded_chunk_entity_event {
            LoadedChunkEntityInternal::Success { chunk_request_id, chunk_id } => {
                info!("Chunk '{:?}' has been loaded successfully!", chunk_id);

                let mut chunk_registry = registry_parameter.get_mut(world);
                if !chunk_registry.try_deallocate_chunk(chunk_id) {
                    error!("Chunk '{:?}' has failed to be deallocated!", chunk_id);
                }

                let mut loaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
                loaded_chunk_entity_event_writer.send(LoadedChunkEntity::Success { chunk_request_id, chunk_id });
            },
            LoadedChunkEntityInternal::Failure { chunk_request_id, chunk_id } => {
                error!("Chunk '{:?}' has failed to be loaded!", chunk_id);

                let mut chunk_registry = registry_parameter.get_mut(world);
                if !chunk_registry.try_deallocate_chunk(chunk_id) {
                    error!("Chunk '{:?}' has failed to be deallocated!", chunk_id);
                }

                let mut loaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
                loaded_chunk_entity_event_writer.send(LoadedChunkEntity::Failure { chunk_request_id, chunk_id });
            },
        }
    }
}

pub(super) fn handle_unloaded_chunk_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<UnloadedChunkEntityInternal>,
        EventWriter<UnloadedChunkEntity>,
    )>,
    registry_parameter: &mut SystemState<ResMut<ChunkRegistry>>,
) {
    let mut unloaded_chunk_entity_event_reader = event_parameters.get_mut(world).0;

    let mut unloaded_chunk_entity_events: Vec<UnloadedChunkEntityInternal> = Vec::new();
    for unloaded_chunk_entity_event in unloaded_chunk_entity_event_reader.read() {
        unloaded_chunk_entity_events.push(unloaded_chunk_entity_event.clone());
    }

    for unloaded_chunk_entity_event in unloaded_chunk_entity_events {
        match unloaded_chunk_entity_event {
            UnloadedChunkEntityInternal::Success { chunk_request_id, chunk_id } => {
                info!("Chunk '{:?}' has been unloaded successfully!", chunk_id);

                let mut chunk_registry = registry_parameter.get_mut(world);
                if !chunk_registry.try_deallocate_chunk(chunk_id) {
                    error!("Chunk '{:?}' has failed to be deallocated!", chunk_id);
                }

                let mut unloaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
                unloaded_chunk_entity_event_writer.send(UnloadedChunkEntity::Success { chunk_request_id, chunk_id });
            },
            UnloadedChunkEntityInternal::Failure { chunk_request_id, chunk_id } => {
                error!("Chunk '{:?}' has failed to be unloaded!", chunk_id);

                let mut chunk_registry = registry_parameter.get_mut(world);
                if !chunk_registry.try_deallocate_chunk(chunk_id) {
                    error!("Chunk '{:?}' has failed to be deallocated!", chunk_id);
                }

                let mut unloaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
                unloaded_chunk_entity_event_writer.send(UnloadedChunkEntity::Failure { chunk_request_id, chunk_id });
            }
        }
    }
}