use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::events::*;
use crate::chunk::resources::*;
use crate::chunk::functions;
use crate::entity::resources::*;

pub(in crate) fn handle_create_chunk_events(
    mut create_chunk_event_reader: EventReader<CreateChunkEntity>,
    mut create_chunk_entity_internal_event_writer: EventWriter<CreateChunkEntityInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_infos = Vec::new();
    for create_chunk_event in create_chunk_event_reader.read() {
        chunk_infos.push((
            create_chunk_event.chunk_event_id,
            create_chunk_event.chunk_id, 
        ));
    }

    for (chunk_event_id, chunk_id) in chunk_infos {
        info!("Trying to create chunk '{:?}' ...", chunk_id);

        if chunk_registry.is_creating_chunk(chunk_id) {
            error!("Chunk '{:?}' is already being created!", chunk_id);

            continue;
        }
        
        chunk_registry.start_creating_chunk(chunk_id);
        create_chunk_entity_internal_event_writer.send(CreateChunkEntityInternal {
            chunk_event_id,
            chunk_id
        });
    }
}

pub(in crate) fn handle_destroy_chunk_events(
    mut destroy_chunk_event_reader: EventReader<DestroyChunkEntity>,
    mut destroy_chunk_entity_internal_event_writer: EventWriter<DestroyChunkEntityInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_infos = Vec::new();
    for destroy_chunk_event in destroy_chunk_event_reader.read() {
        chunk_infos.push((
            destroy_chunk_event.chunk_event_id,
            destroy_chunk_event.chunk_id
        ));
    }

    for (chunk_event_id, chunk_id) in chunk_infos {
        info!("Trying to destroy chunk '{:?}' ...", chunk_id);

        if chunk_registry.is_destroying_chunk(chunk_id) {
            error!("Chunk '{:?}' is already being destroyed!", chunk_id);

            continue;
        }

        chunk_registry.start_destroying_chunk(chunk_id);
        destroy_chunk_entity_internal_event_writer.send(DestroyChunkEntityInternal {
            chunk_event_id,
            chunk_id
        });
    }
}

pub(in crate) fn handle_load_chunk_events(
    mut load_chunk_event_reader: EventReader<LoadChunkEntity>,
    mut load_chunk_entity_internal_event_writer: EventWriter<LoadChunkEntityInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_infos = Vec::new();
    for load_chunk_event in load_chunk_event_reader.read() {
        chunk_infos.push((
            load_chunk_event.chunk_event_id,
            load_chunk_event.chunk_id
        ));
    }

    for (chunk_event_id, chunk_id) in chunk_infos {
        info!("Trying to load chunk '{:?}' ...", chunk_id);

        if chunk_registry.is_loading_chunk(chunk_id) {
            error!("Chunk '{:?}' is already being loaded!", chunk_id);

            continue;
        }
        
        chunk_registry.start_loading_chunk(chunk_id);
        load_chunk_entity_internal_event_writer.send(LoadChunkEntityInternal { 
            chunk_event_id,
            chunk_id
        });
    }
}

pub(in crate) fn handle_unload_chunk_events(
    mut unload_chunk_event_reader: EventReader<UnloadChunkEntity>,
    mut unload_chunk_entity_internal_event_writer: EventWriter<UnloadChunkEntityInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_infos = Vec::new();
    for unload_chunk_event in unload_chunk_event_reader.read() {
        chunk_infos.push((
            unload_chunk_event.chunk_event_id,
            unload_chunk_event.chunk_id
        ));
    }

    for (chunk_event_id, chunk_id) in chunk_infos {
        info!("Trying to unload chunk '{:?}' ...", chunk_id);

        if chunk_registry.is_unloading_chunk(chunk_id) {
            error!("Chunk '{:?}' is already being unloaded!", chunk_id);

            continue;
        }

        chunk_registry.start_unloading_chunk(chunk_id);
        unload_chunk_entity_internal_event_writer.send(UnloadChunkEntityInternal { 
            chunk_event_id,
            chunk_id
        });
    }
}

pub(in crate) fn handle_create_chunk_entity_internal_events(
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
    let mut create_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut create_chunk_events: Vec<CreateChunkEntityInternal> = Vec::new();
    for create_chunk_event in create_chunk_event_reader.read() {
        create_chunk_events.push(create_chunk_event.clone());
    }

    for create_chunk_event in create_chunk_events {
        let chunk_event_id = create_chunk_event.chunk_event_id;
        let chunk_id = create_chunk_event.chunk_id;

        let (chunk_registry, _) = registry_parameters.get_mut(world);

        let is_chunk_registered = chunk_registry.is_chunk_registered(chunk_id);
        let is_chunk_loaded = chunk_registry.is_chunk_loaded(chunk_id);

        if is_chunk_registered {
            warn!("Chunk '{:?}' is already registered!", chunk_id);
            
            let mut created_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            created_chunk_entity_event_writer.send(CreatedChunkEntityInternal::Failure { 
                chunk_event_id, 
                chunk_id
            });

            continue;
        }

        if is_chunk_loaded {
            warn!("Chunk '{:?}' is already loaded!", chunk_id);

            let mut created_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            created_chunk_entity_event_writer.send(CreatedChunkEntityInternal::Failure { 
                chunk_event_id,
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
            chunk_event_id,
            chunk_id
        });
    }
}

pub(in crate) fn handle_destroy_chunk_entity_internal_events(
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
    let mut destroy_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut destroy_chunk_events: Vec<DestroyChunkEntityInternal> = Vec::new();
    for destroy_chunk_event in destroy_chunk_event_reader.read() {
        destroy_chunk_events.push(destroy_chunk_event.clone());
    }

    for destroy_chunk_event in destroy_chunk_events {
        let chunk_event_id = destroy_chunk_event.chunk_event_id;
        let chunk_id = destroy_chunk_event.chunk_id;

        let (mut chunk_registry, mut entity_registry) = registry_parameters.get_mut(world);

        let is_chunk_registered = chunk_registry.is_chunk_registered(chunk_id);
        let is_chunk_loaded = chunk_registry.is_chunk_loaded(chunk_id);

        if !is_chunk_loaded {
            warn!("Chunk '{:?}' is already unloaded!", chunk_id);

            let mut destroyed_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            destroyed_chunk_entity_event_writer.send(DestroyedChunkEntityInternal::Failure { chunk_event_id, chunk_id });

            continue;
        }

        if !is_chunk_registered {
            warn!("Chunk '{:?}' is already unregistered!", chunk_id);
            
            let mut destroyed_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            destroyed_chunk_entity_event_writer.send(DestroyedChunkEntityInternal::Failure { chunk_event_id, chunk_id });

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
        destroyed_chunk_entity_event_writer.send(DestroyedChunkEntityInternal::Success { chunk_event_id, chunk_id });
    }
}

pub(in crate) fn handle_load_chunk_entity_internal_events(
    world: &mut World,
    registry_parameter: &mut SystemState<(
        ResMut<ChunkRegistry>,
    )>,
    event_parameters: &mut SystemState<(
        EventReader<LoadChunkEntityInternal>,
        EventWriter<LoadedChunkEntityInternal>,
    )>
) {
    let mut load_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut load_chunk_events: Vec<LoadChunkEntityInternal> = Vec::new();
    for load_chunk_event in load_chunk_event_reader.read() {
        load_chunk_events.push(load_chunk_event.clone());
    }

    for load_chunk_event in load_chunk_events {
        let chunk_event_id = load_chunk_event.chunk_event_id;
        let chunk_id = load_chunk_event.chunk_id;

        let chunk_registry = registry_parameter.get_mut(world).0;

        let is_chunk_registered = chunk_registry.is_chunk_registered(chunk_id);
        let is_chunk_loaded = chunk_registry.is_chunk_loaded(chunk_id);

        if !is_chunk_registered {
            warn!("Chunk '{:?}' is not registered!", chunk_id);

            let mut loaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            loaded_chunk_entity_event_writer.send(LoadedChunkEntityInternal::Failure { chunk_event_id, chunk_id });

            continue;
        }

        if is_chunk_loaded {
            warn!("Chunk '{:?}' is already loaded!", chunk_id);

            let mut loaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            loaded_chunk_entity_event_writer.send(LoadedChunkEntityInternal::Failure { chunk_event_id, chunk_id });

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
        loaded_chunk_entity_event_writer.send(LoadedChunkEntityInternal::Success { chunk_event_id, chunk_id });
    }
}

pub(in crate) fn handle_unload_chunk_entity_internal_events(
    world: &mut World,
    registry_parameter: &mut SystemState<(
        ResMut<ChunkRegistry>,
    )>,
    event_parameters: &mut SystemState<(
        EventReader<UnloadChunkEntityInternal>,
        EventWriter<UnloadedChunkEntityInternal>,
    )>
) {
    let mut unload_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut unload_chunk_events: Vec<UnloadChunkEntityInternal> = Vec::new();
    for unload_chunk_event in unload_chunk_event_reader.read() {
        unload_chunk_events.push(unload_chunk_event.clone());
    }

    for unload_chunk_event in unload_chunk_events {
        let chunk_event_id = unload_chunk_event.chunk_event_id;
        let chunk_id = unload_chunk_event.chunk_id;

        let chunk_registry = registry_parameter.get_mut(world).0;

        let is_chunk_registered = chunk_registry.is_chunk_registered(chunk_id);
        let is_chunk_loaded = chunk_registry.is_chunk_loaded(chunk_id);

        if !is_chunk_loaded {
            warn!("Chunk '{:?}' is already unloaded!", chunk_id);

            let mut unloaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            unloaded_chunk_entity_event_writer.send(UnloadedChunkEntityInternal::Failure { chunk_event_id, chunk_id });

            continue;
        }

        if !is_chunk_registered {
            warn!("Chunk '{:?}' is already unregistered!", chunk_id);

            let mut unloaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
            unloaded_chunk_entity_event_writer.send(UnloadedChunkEntityInternal::Failure { chunk_event_id, chunk_id });

            continue;
        }

        let serialized_chunk = functions::serialize_chunk(world, registry_parameter, chunk_id);

        let mut chunk_registry = registry_parameter.get_mut(world).0;

        chunk_registry.serialize_chunk(chunk_id, serialized_chunk);

        chunk_registry.unload_chunk(chunk_id);

        chunk_registry.stop_unloading_chunk(chunk_id);

        let mut unloaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;
        unloaded_chunk_entity_event_writer.send(UnloadedChunkEntityInternal::Success { chunk_event_id, chunk_id });
    }
}

pub(in crate) fn handle_created_chunk_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<CreatedChunkEntityInternal>,
        EventWriter<CreatedChunkEntity>,
    )>,
) {
    let mut created_chunk_entity_event_reader = event_parameters.get_mut(world).0;

    let mut created_chunk_entity_events: Vec<CreatedChunkEntityInternal> = Vec::new();
    for created_chunk_entity_event in created_chunk_entity_event_reader.read() {
        created_chunk_entity_events.push(created_chunk_entity_event.clone());
    }

    for created_chunk_entity_event in created_chunk_entity_events {
        let mut created_chunk_entity_event_writer = event_parameters.get_mut(world).1;

        match created_chunk_entity_event {
            CreatedChunkEntityInternal::Success { chunk_event_id, chunk_id } => {
                info!("Chunk '{:?}' has been created successfully!", chunk_id);

                created_chunk_entity_event_writer.send(CreatedChunkEntity::Success { chunk_event_id, chunk_id });
            },
            CreatedChunkEntityInternal::Failure { chunk_event_id, chunk_id } => {
                error!("Chunk '{:?}' has failed to be created!", chunk_id);

                created_chunk_entity_event_writer.send(CreatedChunkEntity::Failure { chunk_event_id, chunk_id });
            },
        }
    }
}

pub(in crate) fn handle_destroyed_chunk_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<DestroyedChunkEntityInternal>,
        EventWriter<DestroyedChunkEntity>,
    )>,
) {
    let mut destroyed_chunk_entity_event_reader = event_parameters.get_mut(world).0;

    let mut destroyed_chunk_entity_events: Vec<DestroyedChunkEntityInternal> = Vec::new();
    for destroyed_chunk_entity_event in destroyed_chunk_entity_event_reader.read() {
        destroyed_chunk_entity_events.push(destroyed_chunk_entity_event.clone());
    }

    for destroyed_chunk_entity_event in destroyed_chunk_entity_events {
        let mut destroyed_chunk_entity_event_writer = event_parameters.get_mut(world).1;

        match destroyed_chunk_entity_event {
            DestroyedChunkEntityInternal::Success { chunk_event_id, chunk_id } => {
                info!("Chunk '{:?}' has been destroyed successfully!", chunk_id);

                destroyed_chunk_entity_event_writer.send(DestroyedChunkEntity::Success { chunk_event_id, chunk_id });
            },
            DestroyedChunkEntityInternal::Failure { chunk_event_id, chunk_id } => {
                error!("Chunk '{:?}' has failed to be destroyed!", chunk_id);

                destroyed_chunk_entity_event_writer.send(DestroyedChunkEntity::Failure { chunk_event_id, chunk_id });
            },
        }
    }
}

pub(in crate) fn handle_loaded_chunk_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<LoadedChunkEntityInternal>,
        EventWriter<LoadedChunkEntity>,
    )>,
) {
    let mut loaded_chunk_entity_event_reader = event_parameters.get_mut(world).0;

    let mut loaded_chunk_entity_events: Vec<LoadedChunkEntityInternal> = Vec::new();
    for loaded_chunk_entity_event in loaded_chunk_entity_event_reader.read() {
        loaded_chunk_entity_events.push(loaded_chunk_entity_event.clone());
    }

    for loaded_chunk_entity_event in loaded_chunk_entity_events {
        let mut loaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;

        match loaded_chunk_entity_event {
            LoadedChunkEntityInternal::Success { chunk_event_id, chunk_id } => {
                info!("Chunk '{:?}' has been loaded successfully!", chunk_id);

                loaded_chunk_entity_event_writer.send(LoadedChunkEntity::Success { chunk_event_id, chunk_id });
            },
            LoadedChunkEntityInternal::Failure { chunk_event_id, chunk_id } => {
                error!("Chunk '{:?}' has failed to be loaded!", chunk_id);

                loaded_chunk_entity_event_writer.send(LoadedChunkEntity::Failure { chunk_event_id, chunk_id });
            },
        }
    }
}

pub(in crate) fn handle_unloaded_chunk_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<UnloadedChunkEntityInternal>,
        EventWriter<UnloadedChunkEntity>,
    )>,
) {
    let mut unloaded_chunk_entity_event_reader = event_parameters.get_mut(world).0;

    let mut unloaded_chunk_entity_events: Vec<UnloadedChunkEntityInternal> = Vec::new();
    for unloaded_chunk_entity_event in unloaded_chunk_entity_event_reader.read() {
        unloaded_chunk_entity_events.push(unloaded_chunk_entity_event.clone());
    }

    for unloaded_chunk_entity_event in unloaded_chunk_entity_events {
        let mut unloaded_chunk_entity_event_writer = event_parameters.get_mut(world).1;

        match unloaded_chunk_entity_event {
            UnloadedChunkEntityInternal::Success { chunk_event_id, chunk_id } => {
                info!("Chunk '{:?}' has been unloaded successfully!", chunk_id);

                unloaded_chunk_entity_event_writer.send(UnloadedChunkEntity::Success { chunk_event_id, chunk_id });
            },
            UnloadedChunkEntityInternal::Failure { chunk_event_id, chunk_id } => {
                error!("Chunk '{:?}' has failed to be unloaded!", chunk_id);

                unloaded_chunk_entity_event_writer.send(UnloadedChunkEntity::Failure { chunk_event_id, chunk_id });
            }
        }
    }
}