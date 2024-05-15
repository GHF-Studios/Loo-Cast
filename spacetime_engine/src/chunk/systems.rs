use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::id::structs::*;
use crate::chunk::coordinate::structs::*;
use crate::chunk::actor::coordinate::structs::*;
use crate::chunk::events::*;
use crate::chunk::resources::*;
use crate::chunk::loader::components::*;
use crate::chunk::functions;
use crate::entity::resources::EntityRegistry;
use crate::math::structs::*;
use crate::player::components::*;

pub(in crate) fn update(
    mut create_chunk_event_writer: EventWriter<CreateChunk>,
    mut destroy_chunk_event_writer: EventWriter<DestroyChunk>,
    mut load_chunk_event_writer: EventWriter<LoadChunk>,
    mut unload_chunk_event_writer: EventWriter<UnloadChunk>,
    mut chunk_loader_query: Query<(&Transform, &mut ChunkLoader)>,
    chunk_registry: Res<ChunkRegistry>,
) {
    let (chunk_loader_transform, mut chunk_loader) = chunk_loader_query.single_mut();
    let chunk_loader_chunk_actor_coordinate: ChunkActorCoordinate = chunk_loader_transform.translation.into();
    let current_chunk_coordinate: ChunkCoordinate = chunk_loader_chunk_actor_coordinate.into();
    let load_radius = chunk_loader.load_radius as i16;
    
    // Detect chunks around the player
    let mut detected_chunk_coordinates = Vec::new();
    for x_offset in -load_radius..=load_radius {
        for y_offset in -load_radius..=load_radius {
            detected_chunk_coordinates.push(current_chunk_coordinate + ChunkCoordinate(I16Vec2(x_offset, y_offset)));
        }
    }

    // Categorize the detected chunks
    let mut old_chunk_ids: Vec<ChunkID> = Vec::new();
    let mut unchanged_chunk_ids: Vec<ChunkID> = Vec::new();
    let mut new_chunk_ids: Vec<ChunkID> = Vec::new();
    for loaded_chunk_id in chunk_registry.loaded_chunk_ids().iter() {
        let loaded_chunk_coordinate: ChunkCoordinate = loaded_chunk_id.0;

        if !detected_chunk_coordinates.contains(&loaded_chunk_coordinate) {
            old_chunk_ids.push(*loaded_chunk_id);
        }
    }
    for detected_chunk_coordinate in detected_chunk_coordinates {
        let detected_chunk_id: ChunkID = detected_chunk_coordinate.into();
        if chunk_registry.is_chunk_loaded(detected_chunk_id) {
            unchanged_chunk_ids.push(detected_chunk_id);
        } else {
            new_chunk_ids.push(detected_chunk_id);
        }
    }

    // Handle old chunks
    for old_chunk_id in old_chunk_ids {
        unload_chunk_event_writer.send(UnloadChunk(old_chunk_id));
    }

    // Handle new chunks
    for new_chunk_id in new_chunk_ids.iter() {
        let new_chunk_id = *new_chunk_id;
        
        if chunk_registry.is_chunk_registered(new_chunk_id) {
            load_chunk_event_writer.send(LoadChunk(new_chunk_id));
        } else {
            create_chunk_event_writer.send(CreateChunk(new_chunk_id));
        }
    }

    // Update the current chunk IDs
    chunk_loader.current_chunk_ids = unchanged_chunk_ids;
    chunk_loader.current_chunk_ids.append(&mut new_chunk_ids);
}

pub(in crate) fn change_radius(
    mut chunk_loader_query: Query<(&mut ChunkLoader, &Player)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut chunk_loader, _) in chunk_loader_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            chunk_loader.load_radius = (chunk_loader.load_radius as i16 - 1).max(0) as u16;
        }
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            chunk_loader.load_radius = (chunk_loader.load_radius as i16 + 1) as u16;
        }
    }
}

pub(in crate) fn handle_create_chunk_events(
    mut create_chunk_event_reader: EventReader<CreateChunk>,
    mut create_chunk_internal_event_writer: EventWriter<CreateChunkInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_ids = Vec::new();
    for create_chunk_event in create_chunk_event_reader.read() {
        chunk_ids.push(create_chunk_event.0);
    }

    for chunk_id in chunk_ids {
        if chunk_registry.is_creating_chunk(chunk_id) {
            continue;
        }
        
        chunk_registry.start_creating_chunk(chunk_id);
        create_chunk_internal_event_writer.send(CreateChunkInternal(chunk_id));
    }
}

pub(in crate) fn handle_destroy_chunk_events(
    mut destroy_chunk_event_reader: EventReader<DestroyChunk>,
    mut destroy_chunk_internal_event_writer: EventWriter<DestroyChunkInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_ids = Vec::new();
    for destroy_chunk_event in destroy_chunk_event_reader.read() {
        chunk_ids.push(destroy_chunk_event.0);
    }

    for chunk_id in chunk_ids {
        if chunk_registry.is_destroying_chunk(chunk_id) {
            continue;
        }

        chunk_registry.start_destroying_chunk(chunk_id);
        destroy_chunk_internal_event_writer.send(DestroyChunkInternal(chunk_id));
    }
}

pub(in crate) fn handle_load_chunk_events(
    mut load_chunk_event_reader: EventReader<LoadChunk>,
    mut load_chunk_internal_event_writer: EventWriter<LoadChunkInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_ids = Vec::new();
    for load_chunk_event in load_chunk_event_reader.read() {
        chunk_ids.push(load_chunk_event.0);
    }

    for chunk_id in chunk_ids {
        if chunk_registry.is_loading_chunk(chunk_id) {
            continue;
        }
        
        chunk_registry.start_loading_chunk(chunk_id);
        load_chunk_internal_event_writer.send(LoadChunkInternal(chunk_id));
    }
}

pub(in crate) fn handle_unload_chunk_events(
    mut unload_chunk_event_reader: EventReader<UnloadChunk>,
    mut unload_chunk_internal_event_writer: EventWriter<UnloadChunkInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    let mut chunk_ids = Vec::new();
    for unload_chunk_event in unload_chunk_event_reader.read() {
        chunk_ids.push(unload_chunk_event.0);
    }

    for chunk_id in chunk_ids {
        if chunk_registry.is_unloading_chunk(chunk_id) {
            continue;
        }

        chunk_registry.start_unloading_chunk(chunk_id);
        unload_chunk_internal_event_writer.send(UnloadChunkInternal(chunk_id));
    }
}

pub(in crate) fn handle_create_chunk_internal_events(
    mut commands: Commands,
    mut create_chunk_event_reader: EventReader<CreateChunkInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    for create_chunk_event in create_chunk_event_reader.read() {
        let chunk_id = create_chunk_event.0;

        let entity_id = entity_registry.register_entity();
        chunk_registry.register_chunk(chunk_id);

        let new_chunk_entity = functions::new_chunk_entity(&mut commands, chunk_id);
        
        entity_registry.load_entity(entity_id, new_chunk_entity);
        chunk_registry.load_chunk(chunk_id, new_chunk_entity);

        chunk_registry.stop_creating_chunk(chunk_id);
    }
}

pub(in crate) fn handle_destroy_chunk_internal_events(
    mut commands: Commands,
    mut destroy_chunk_event_reader: EventReader<DestroyChunkInternal>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    for destroy_chunk_event in destroy_chunk_event_reader.read() {
        let chunk_id: ChunkID = destroy_chunk_event.0;

        let chunk_entity = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => continue,
        };
        let chunk_entity_id = match entity_registry.get_loaded_entity_id(&chunk_entity) {
            Some(entity_id) => entity_id,
            None => continue,
        };

        match chunk_registry.unload_chunk(chunk_id) {
            Some(_) => {},
            None => continue,
        }
        match entity_registry.unload_entity(chunk_entity_id) {
            Some(_) => {},
            None => continue,
        }

        commands.entity(chunk_entity).despawn_recursive();

        entity_registry.unregister_entity(chunk_entity_id);
        chunk_registry.unregister_chunk(chunk_id);

        chunk_registry.stop_destroying_chunk(chunk_id);
    }
}

pub(in crate) fn handle_load_chunk_internal_events(
    world: &mut World,
    registry_parameter: &mut SystemState<(
        ResMut<ChunkRegistry>,
    )>,
    event_reader_parameter: &mut SystemState<(
        EventReader<LoadChunkInternal>,
    )>
) {
    let mut load_chunk_event_reader = event_reader_parameter.get_mut(world).0;

    let mut load_chunk_events: Vec<LoadChunkInternal> = Vec::new();
    for load_chunk_event in load_chunk_event_reader.read() {
        load_chunk_events.push(load_chunk_event.clone());
    }

    for load_chunk_event in load_chunk_events {
        let chunk_id = load_chunk_event.0;
        
        let serialized_chunk = {
            let mut chunk_registry = registry_parameter.get_mut(world).0;

            chunk_registry.deserialize_chunk(chunk_id).unwrap()
        };

        let chunk_entity = functions::deserialize_chunk(world, serialized_chunk);

        {
            let mut chunk_registry = registry_parameter.get_mut(world).0;

            chunk_registry.load_chunk(chunk_id, chunk_entity);

            chunk_registry.stop_loading_chunk(chunk_id);
        }
    }
}

pub(in crate) fn handle_unload_chunk_internal_events(
    world: &mut World,
    registry_parameter: &mut SystemState<(
        ResMut<ChunkRegistry>,
    )>,
    event_reader_parameter: &mut SystemState<(
        EventReader<UnloadChunkInternal>,
    )>
) {
    let mut unload_chunk_event_reader = event_reader_parameter.get_mut(world).0;

    let mut unload_chunk_events: Vec<UnloadChunkInternal> = Vec::new();
    for unload_chunk_event in unload_chunk_event_reader.read() {
        unload_chunk_events.push(unload_chunk_event.clone());
    }

    for unload_chunk_event in unload_chunk_events {
        let chunk_id = unload_chunk_event.0;

        let serialized_chunk = functions::serialize_chunk(world, registry_parameter, chunk_id);

        {
            let mut chunk_registry = registry_parameter.get_mut(world).0;

            chunk_registry.serialize_chunk(chunk_id, serialized_chunk);

            chunk_registry.unload_chunk(chunk_id);
    
            chunk_registry.stop_unloading_chunk(chunk_id);
        };
    }
}