use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::events::*;
use crate::chunk::resources::*;
use crate::chunk::functions;
use crate::entity::resources::*;

pub(in crate) fn handle_create_chunk_events(
    mut create_chunk_event_reader: EventReader<CreateChunk>,
    mut create_chunk_internal_event_writer: EventWriter<CreateChunkInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_ids = Vec::new();
    for create_chunk_event in create_chunk_event_reader.read() {
        chunk_ids.push(create_chunk_event.chunk_id);
    }

    for chunk_id in chunk_ids {
        info!("Trying to create chunk '{:?}' ...", chunk_id);

        if chunk_registry.is_creating_chunk(chunk_id) {
            error!("Chunk '{:?}' is already being created!", chunk_id);

            continue;
        }
        
        chunk_registry.start_creating_chunk(chunk_id);
        create_chunk_internal_event_writer.send(CreateChunkInternal { chunk_id });
    }
}

pub(in crate) fn handle_destroy_chunk_events(
    mut destroy_chunk_event_reader: EventReader<DestroyChunk>,
    mut destroy_chunk_internal_event_writer: EventWriter<DestroyChunkInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_ids = Vec::new();
    for destroy_chunk_event in destroy_chunk_event_reader.read() {
        chunk_ids.push(destroy_chunk_event.chunk_id);
    }

    for chunk_id in chunk_ids {
        info!("Trying to destroy chunk '{:?}' ...", chunk_id);

        if chunk_registry.is_destroying_chunk(chunk_id) {
            error!("Chunk '{:?}' is already being destroyed!", chunk_id);

            continue;
        }

        chunk_registry.start_destroying_chunk(chunk_id);
        destroy_chunk_internal_event_writer.send(DestroyChunkInternal { chunk_id });
    }
}

pub(in crate) fn handle_load_chunk_events(
    mut load_chunk_event_reader: EventReader<LoadChunk>,
    mut load_chunk_internal_event_writer: EventWriter<LoadChunkInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_ids = Vec::new();
    for load_chunk_event in load_chunk_event_reader.read() {
        chunk_ids.push(load_chunk_event.chunk_id);
    }

    for chunk_id in chunk_ids {
        info!("Trying to load chunk '{:?}' ...", chunk_id);

        if chunk_registry.is_loading_chunk(chunk_id) {
            error!("Chunk '{:?}' is already being loaded!", chunk_id);

            continue;
        }
        
        chunk_registry.start_loading_chunk(chunk_id);
        load_chunk_internal_event_writer.send(LoadChunkInternal { chunk_id });
    }
}

pub(in crate) fn handle_unload_chunk_events(
    mut unload_chunk_event_reader: EventReader<UnloadChunk>,
    mut unload_chunk_internal_event_writer: EventWriter<UnloadChunkInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_ids = Vec::new();
    for unload_chunk_event in unload_chunk_event_reader.read() {
        chunk_ids.push(unload_chunk_event.chunk_id);
    }

    for chunk_id in chunk_ids {
        info!("Trying to unload chunk '{:?}' ...", chunk_id);

        if chunk_registry.is_unloading_chunk(chunk_id) {
            error!("Chunk '{:?}' is already being unloaded!", chunk_id);

            continue;
        }

        chunk_registry.start_unloading_chunk(chunk_id);
        unload_chunk_internal_event_writer.send(UnloadChunkInternal { chunk_id });
    }
}

pub(in crate) fn handle_create_chunk_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<CreateChunkInternal>,
        EventWriter<CreatedChunkInternal>,
    )>,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<EntityRegistry>,
    )>,
) {
    let mut create_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut create_chunk_events: Vec<CreateChunkInternal> = Vec::new();
    for create_chunk_event in create_chunk_event_reader.read() {
        create_chunk_events.push(create_chunk_event.clone());
    }

    for create_chunk_event in create_chunk_events {
        let chunk_id = create_chunk_event.chunk_id;

        let (chunk_registry, _) = registry_parameters.get_mut(world);

        let is_chunk_registered = chunk_registry.is_chunk_registered(chunk_id);
        let is_chunk_loaded = chunk_registry.is_chunk_loaded(chunk_id);

        if is_chunk_registered {
            warn!("Chunk '{:?}' is already registered!", chunk_id);
            
            let mut created_chunk_event_writer = event_parameters.get_mut(world).1;
            created_chunk_event_writer.send(CreatedChunkInternal { chunk_id, success: false });

            continue;
        }

        if is_chunk_loaded {
            warn!("Chunk '{:?}' is already loaded!", chunk_id);

            let mut created_chunk_event_writer = event_parameters.get_mut(world).1;
            created_chunk_event_writer.send(CreatedChunkInternal { chunk_id, success: false });

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

        let mut created_chunk_event_writer = event_parameters.get_mut(world).1;
        created_chunk_event_writer.send(CreatedChunkInternal { chunk_id, success: true });
    }
}

pub(in crate) fn handle_destroy_chunk_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<DestroyChunkInternal>,
        EventWriter<DestroyedChunkInternal>,
    )>,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<EntityRegistry>,
    )>,
) {
    let mut destroy_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut destroy_chunk_events: Vec<DestroyChunkInternal> = Vec::new();
    for destroy_chunk_event in destroy_chunk_event_reader.read() {
        destroy_chunk_events.push(destroy_chunk_event.clone());
    }

    for destroy_chunk_event in destroy_chunk_events {
        let chunk_id = destroy_chunk_event.chunk_id;

        let (mut chunk_registry, mut entity_registry) = registry_parameters.get_mut(world);

        let is_chunk_registered = chunk_registry.is_chunk_registered(chunk_id);
        let is_chunk_loaded = chunk_registry.is_chunk_loaded(chunk_id);

        if !is_chunk_loaded {
            warn!("Chunk '{:?}' is already unloaded!", chunk_id);

            let mut destroyed_chunk_event_writer = event_parameters.get_mut(world).1;
            destroyed_chunk_event_writer.send(DestroyedChunkInternal { chunk_id, success: false });

            continue;
        }

        if !is_chunk_registered {
            warn!("Chunk '{:?}' is already unregistered!", chunk_id);
            
            let mut destroyed_chunk_event_writer = event_parameters.get_mut(world).1;
            destroyed_chunk_event_writer.send(DestroyedChunkInternal { chunk_id, success: false });

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

        let mut destroyed_chunk_event_writer = event_parameters.get_mut(world).1;
        destroyed_chunk_event_writer.send(DestroyedChunkInternal { chunk_id, success: true });
    }
}

pub(in crate) fn handle_load_chunk_internal_events(
    world: &mut World,
    registry_parameter: &mut SystemState<(
        ResMut<ChunkRegistry>,
    )>,
    event_parameters: &mut SystemState<(
        EventReader<LoadChunkInternal>,
        EventWriter<LoadedChunkInternal>,
    )>
) {
    let mut load_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut load_chunk_events: Vec<LoadChunkInternal> = Vec::new();
    for load_chunk_event in load_chunk_event_reader.read() {
        load_chunk_events.push(load_chunk_event.clone());
    }

    for load_chunk_event in load_chunk_events {
        let chunk_id = load_chunk_event.chunk_id;

        let chunk_registry = registry_parameter.get_mut(world).0;

        let is_chunk_registered = chunk_registry.is_chunk_registered(chunk_id);
        let is_chunk_loaded = chunk_registry.is_chunk_loaded(chunk_id);

        if !is_chunk_registered {
            warn!("Chunk '{:?}' is not registered!", chunk_id);

            let mut loaded_chunk_event_writer = event_parameters.get_mut(world).1;
            loaded_chunk_event_writer.send(LoadedChunkInternal { chunk_id, success: false });

            continue;
        }

        if is_chunk_loaded {
            warn!("Chunk '{:?}' is already loaded!", chunk_id);

            let mut loaded_chunk_event_writer = event_parameters.get_mut(world).1;
            loaded_chunk_event_writer.send(LoadedChunkInternal { chunk_id, success: false });

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

        let mut loaded_chunk_event_writer = event_parameters.get_mut(world).1;
        loaded_chunk_event_writer.send(LoadedChunkInternal { chunk_id, success: true });
    }
}

pub(in crate) fn handle_unload_chunk_internal_events(
    world: &mut World,
    registry_parameter: &mut SystemState<(
        ResMut<ChunkRegistry>,
    )>,
    event_parameters: &mut SystemState<(
        EventReader<UnloadChunkInternal>,
        EventWriter<UnloadedChunkInternal>,
    )>
) {
    let mut unload_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut unload_chunk_events: Vec<UnloadChunkInternal> = Vec::new();
    for unload_chunk_event in unload_chunk_event_reader.read() {
        unload_chunk_events.push(unload_chunk_event.clone());
    }

    for unload_chunk_event in unload_chunk_events {
        let chunk_id = unload_chunk_event.chunk_id;

        let chunk_registry = registry_parameter.get_mut(world).0;

        let is_chunk_registered = chunk_registry.is_chunk_registered(chunk_id);
        let is_chunk_loaded = chunk_registry.is_chunk_loaded(chunk_id);

        if !is_chunk_loaded {
            warn!("Chunk '{:?}' is already unloaded!", chunk_id);

            let mut unloaded_chunk_event_writer = event_parameters.get_mut(world).1;
            unloaded_chunk_event_writer.send(UnloadedChunkInternal { chunk_id, success: false });

            continue;
        }

        if !is_chunk_registered {
            warn!("Chunk '{:?}' is already unregistered!", chunk_id);

            let mut unloaded_chunk_event_writer = event_parameters.get_mut(world).1;
            unloaded_chunk_event_writer.send(UnloadedChunkInternal { chunk_id, success: false });

            continue;
        }

        let serialized_chunk = functions::serialize_chunk(world, registry_parameter, chunk_id);

        let mut chunk_registry = registry_parameter.get_mut(world).0;

        chunk_registry.serialize_chunk(chunk_id, serialized_chunk);

        chunk_registry.unload_chunk(chunk_id);

        chunk_registry.stop_unloading_chunk(chunk_id);

        let mut unloaded_chunk_event_writer = event_parameters.get_mut(world).1;
        unloaded_chunk_event_writer.send(UnloadedChunkInternal { chunk_id, success: true });
    }
}

pub(in crate) fn handle_created_chunk_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<CreatedChunkInternal>,
        EventWriter<CreatedChunk>,
    )>,
) {
    let mut created_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut created_chunk_events: Vec<CreatedChunkInternal> = Vec::new();
    for created_chunk_event in created_chunk_event_reader.read() {
        created_chunk_events.push(created_chunk_event.clone());
    }

    for created_chunk_event in created_chunk_events {
        let chunk_id = created_chunk_event.chunk_id;
        let success = created_chunk_event.success;

        match success {
            true => info!("Chunk '{:?}' has been created successfully!", chunk_id),
            false => error!("Chunk '{:?}' has failed to be created!", chunk_id),
        };

        let mut created_chunk_event_writer = event_parameters.get_mut(world).1;

        created_chunk_event_writer.send(CreatedChunk { chunk_id, success });
    }
}

pub(in crate) fn handle_destroyed_chunk_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<DestroyedChunkInternal>,
        EventWriter<DestroyedChunk>,
    )>,
) {
    let mut destroyed_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut destroyed_chunk_events: Vec<DestroyedChunkInternal> = Vec::new();
    for destroyed_chunk_event in destroyed_chunk_event_reader.read() {
        destroyed_chunk_events.push(destroyed_chunk_event.clone());
    }

    for destroyed_chunk_event in destroyed_chunk_events {
        let chunk_id = destroyed_chunk_event.chunk_id;
        let success = destroyed_chunk_event.success;

        match success {
            true => info!("Chunk '{:?}' has been destroyed successfully!", chunk_id),
            false => error!("Chunk '{:?}' has failed to be destroyed!", chunk_id),
        };

        let mut destroyed_chunk_event_writer = event_parameters.get_mut(world).1;

        destroyed_chunk_event_writer.send(DestroyedChunk { chunk_id, success });
    }
}

pub(in crate) fn handle_loaded_chunk_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<LoadedChunkInternal>,
        EventWriter<LoadedChunk>,
    )>,
) {
    let mut loaded_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut loaded_chunk_events: Vec<LoadedChunkInternal> = Vec::new();
    for loaded_chunk_event in loaded_chunk_event_reader.read() {
        loaded_chunk_events.push(loaded_chunk_event.clone());
    }

    for loaded_chunk_event in loaded_chunk_events {
        let chunk_id = loaded_chunk_event.chunk_id;
        let success = loaded_chunk_event.success;

        match success {
            true => info!("Chunk '{:?}' has been loaded successfully!", chunk_id),
            false => error!("Chunk '{:?}' has failed to be loaded!", chunk_id),
        };

        let mut loaded_chunk_event_writer = event_parameters.get_mut(world).1;

        loaded_chunk_event_writer.send(LoadedChunk { chunk_id, success });
    }
}

pub(in crate) fn handle_unloaded_chunk_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<UnloadedChunkInternal>,
        EventWriter<UnloadedChunk>,
    )>,
) {
    let mut unloaded_chunk_event_reader = event_parameters.get_mut(world).0;

    let mut unloaded_chunk_events: Vec<UnloadedChunkInternal> = Vec::new();
    for unloaded_chunk_event in unloaded_chunk_event_reader.read() {
        unloaded_chunk_events.push(unloaded_chunk_event.clone());
    }

    for unloaded_chunk_event in unloaded_chunk_events {
        let chunk_id = unloaded_chunk_event.chunk_id;
        let success = unloaded_chunk_event.success;

        match success {
            true => info!("Chunk '{:?}' has been unloaded successfully!", chunk_id),
            false => error!("Chunk '{:?}' has failed to be unloaded!", chunk_id),
        };

        let mut unloaded_chunk_event_writer = event_parameters.get_mut(world).1;

        unloaded_chunk_event_writer.send(UnloadedChunk { chunk_id, success });
    }
}