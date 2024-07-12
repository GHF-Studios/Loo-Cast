use bevy::ecs::entity::EntityHashMap;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::scene::ron;
use bevy::scene::serde::{SceneSerializer, SceneDeserializer};
use crate::chunk::components::*;
use crate::chunk::constants::*;
use crate::chunk::id::structs::ChunkID as ChunkID;
use crate::chunk::position::structs::ChunkPosition as ChunkPosition;
use crate::chunk::actor::position::structs::ChunkActorPosition as ChunkActorPosition;
use crate::chunk::events::*;
use crate::math::structs::I16Vec2;
use serde::de::DeserializeSeed;
use super::actor::components::ChunkActor;
use super::loader::components::ChunkLoader;
use super::{ChunkRequestRegistry, ChunkRegistry};

// TODO: The addition of secondary components as shown below should be done in each location where a new chunk actor is created and needs custom components apart from the basics like a Transform and a ChunkActor component.
// TODO: Implement and integrate chunk entity upgrading
pub(in crate) fn new_chunk_entity(world: &mut World, chunk_id: ChunkID) -> Entity {
    let chunk_position: ChunkPosition = chunk_id.into();
    let chunk_chunk_actor_position: ChunkActorPosition = chunk_position.into();
    let chunk_position = chunk_position.0;
    let world_position = chunk_chunk_actor_position.0;
    let world_position = Vec3::new(world_position.x, world_position.y, CHUNK_Z_INDEX);

    let chunk_color = if (chunk_position.0 + chunk_position.1) % 2 == 0 {
        Color::rgb(0.25, 0.25, 0.25)
    } else {
        Color::rgb(0.75, 0.75, 0.75)
    };

    let chunk_entity = world.spawn((
        Chunk::new(chunk_id),
        SpriteBundle {
            sprite: Sprite {
                color: chunk_color,
                custom_size: Some(Vec2::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32)),
                ..default()
            },
            transform: Transform::from_translation(world_position),
            ..default()
        },
    )).id();

    chunk_entity
}

pub(in crate) fn deserialize_chunk(
    world: &mut World,
    serialized_chunk: String,
) -> Entity {
    let deserialized_chunk_scene = {
        let type_registry_rwlock = &world.resource::<AppTypeRegistry>().0.read();

        let deserializer = SceneDeserializer {
            type_registry: type_registry_rwlock,
        };

        let mut ron_deserializer = ron::de::Deserializer::from_str(&serialized_chunk).unwrap();

        deserializer.deserialize(&mut ron_deserializer).unwrap()
    };

    let mut entity_map: EntityHashMap<Entity> = default();

    deserialized_chunk_scene.write_to_world(world, &mut entity_map).unwrap();

    let mut chunk_entity = None;
    for (_, entity_id) in entity_map {
        let entity = match world.get_entity(entity_id) {
                Some(entity) => {
                    entity
                },
                None => {
                    panic!("Entity '{:?}' does not exist!", entity_id);
                },
        };

        if entity.contains::<Chunk>() {
                match chunk_entity {
                    Some(_) => {
                        panic!("Multiple chunks detected!");
                    },
                    None => {
                        chunk_entity = Some(entity_id);
                    },
                }
            }
    }

    match chunk_entity {
        Some(chunk_entity) => chunk_entity,
        None => panic!("No chunk detected!"),
    }
}

pub(in crate) fn serialize_chunk(
    world: &mut World,
    registry_parameter: &mut SystemState<(
        ResMut<ChunkRegistry>,
    )>,
    chunk_id: ChunkID
) -> String {
    debug!("Serializing chunk '{:?}'...", chunk_id);
    
    let mut entities = world
            .query::<(Entity, &ChunkActor)>()
            .iter(world)
            .filter(|(_, chunk_actor)| chunk_actor.current_chunk() == chunk_id)
            .map(|(entity, _)| entity)
            .collect::<Vec<_>>();

    for entity in entities.iter() {
        match world.get_entity(*entity) {
            Some(_) => {},
            None => {
                panic!("Tried to unload non-existent entity!");
            },
        }
    }

    let chunk_registry = registry_parameter.get_mut(world).0;

    let chunk_entity = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
        Some(chunk_entity) => chunk_entity,
        None => panic!("Chunk Entity '{:?}' is not loaded!", chunk_id)
    };

    match world.get_entity(chunk_entity) {
        Some(_) => {},
        None => {
            panic!("Chunk Entity '{:?}' does not exist!", chunk_entity);
        },
    }

    entities.push(chunk_entity);

    let mut builder = DynamicSceneBuilder::from_world(world);
    
    builder = builder.extract_entities(entities.clone().into_iter());

    let dyn_scene = builder.build();

    let type_registry_arc = &world.resource::<AppTypeRegistry>().0;

    let serializer = SceneSerializer::new(&dyn_scene, type_registry_arc);

    let serialized_chunk = ron::to_string(&serializer).unwrap();

    for entity in entities.iter() {
        world.entity_mut(*entity).despawn_recursive();
    }

    serialized_chunk
}

pub(in crate) fn detect_chunks(
    chunk_loader_transform: &Transform,
    chunk_loader_load_radius: u16,
) -> Vec<ChunkID> {
    let chunk_loader_chunk_actor_position: ChunkActorPosition = chunk_loader_transform.translation.into();
    let current_chunk_position: ChunkPosition = chunk_loader_chunk_actor_position.into();
    let load_radius = chunk_loader_load_radius as i16;
    
    let mut detected_chunk_ids = Vec::new();
    for x_offset in -load_radius..=load_radius {
        for y_offset in -load_radius..=load_radius {
            let chunk_position = current_chunk_position + ChunkPosition(I16Vec2(x_offset, y_offset));
            let chunk_id: ChunkID = chunk_position.into();

            detected_chunk_ids.push(chunk_id);
        }
    }

    detected_chunk_ids
}

pub(in crate) fn categorize_chunks(
    chunk_registry: &mut ResMut<ChunkRegistry>,
    detected_chunk_ids: Vec<ChunkID>,
) -> (Vec<ChunkID>, Vec<ChunkID>, Vec<ChunkID>) {
    let mut old_chunks: Vec<ChunkID> = Vec::new();
    let mut unchanged_chunks: Vec<ChunkID> = Vec::new();
    let mut new_chunks: Vec<ChunkID> = Vec::new();

    for loaded_chunk_id in chunk_registry.loaded_chunk_ids() {
        if !detected_chunk_ids.contains(&loaded_chunk_id) {
            old_chunks.push(loaded_chunk_id);
        }
    }

    for detected_chunk_id in detected_chunk_ids {
        if chunk_registry.is_chunk_loaded(detected_chunk_id) {
            unchanged_chunks.push(detected_chunk_id);
        } else {
            new_chunks.push(detected_chunk_id);
        }
    }

    let old_chunks = old_chunks.into_iter().filter(|old_chunk_id| {
        !chunk_registry.is_chunk_allocated(*old_chunk_id)
    }).collect::<Vec<ChunkID>>();

    let new_chunks = new_chunks.into_iter().filter(|new_chunk_id| {
        !chunk_registry.is_chunk_allocated(*new_chunk_id)
    }).collect::<Vec<ChunkID>>();

    (old_chunks, unchanged_chunks, new_chunks)
}

pub(in crate) fn start_chunks(
    mut create_chunk_event_writer: EventWriter<CreateChunkEntity>,
    mut load_chunk_event_writer: EventWriter<LoadChunkEntity>,
    chunk_loader: &mut Mut<ChunkLoader>,
    chunk_registry: &mut ResMut<ChunkRegistry>,
    chunk_request_registry: &mut ResMut<ChunkRequestRegistry>,
    start_chunk_ids: &Vec<ChunkID>,
) {
    for start_chunk_id in start_chunk_ids {
        debug!("Start chunk '{:?}' detected!", start_chunk_id);

        let chunk_id = *start_chunk_id;
        let chunk_request_id = chunk_request_registry.get_unused_chunk_request_id();

        if chunk_registry.is_chunk_registered(chunk_id) {
            if chunk_loader.currently_loading_chunks().contains(&chunk_id) {
                debug!("Start chunk '{:?}' is already loading!", chunk_id);
    
                continue;
            }

            if chunk_registry.is_loading_chunk(chunk_id) {
                debug!("Start chunk '{:?}' is already loading!", chunk_id);

                continue;
            }

            if !chunk_registry.try_allocate_chunk(chunk_id) {
                panic!("Failed to allocate start chunk '{:?}'!", chunk_id);
            }

            chunk_loader.start_loading_chunk(chunk_id);
            chunk_registry.start_loading_chunk(chunk_id);

            load_chunk_event_writer.send(LoadChunkEntity { 
                chunk_request_id,
                chunk_id
            });
        } else {
            if chunk_loader.currently_creating_chunks().contains(&chunk_id) {
                debug!("Start chunk '{:?}' is already creating!", chunk_id);
    
                continue;
            }

            if chunk_registry.is_creating_chunk(chunk_id) {
                debug!("Start chunk '{:?}' is already creating!", chunk_id);

                continue;
            }

            if !chunk_registry.try_allocate_chunk(chunk_id) {
                panic!("Failed to allocate start chunk '{:?}'!", chunk_id);
            }

            chunk_loader.start_creating_chunk(chunk_id);
            chunk_registry.start_creating_chunk(chunk_id);

            create_chunk_event_writer.send(CreateChunkEntity { 
                chunk_request_id,
                chunk_id
            });
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(in crate) fn update_chunks(
    mut create_chunk_event_writer: EventWriter<CreateChunkEntity>,
    mut load_chunk_event_writer: EventWriter<LoadChunkEntity>,
    mut unload_chunk_event_writer: EventWriter<UnloadChunkEntity>,
    chunk_loader: &mut Mut<ChunkLoader>,
    chunk_registry: &mut ResMut<ChunkRegistry>,
    chunk_request_registry: &mut ResMut<ChunkRequestRegistry>,
    old_chunk_ids: Vec<ChunkID>,
    new_chunk_ids: Vec<ChunkID>,
) {
    for old_chunk_id in old_chunk_ids {
        debug!("Old chunk '{:?}' detected!", old_chunk_id);

        let chunk_request_id = chunk_request_registry.get_unused_chunk_request_id();
        let chunk_id = old_chunk_id;

        if chunk_loader.currently_unloading_chunks().contains(&chunk_id) {
            debug!("Loader: Old chunk '{:?}' is already unloading!", chunk_id);

            continue;
        }

        if chunk_registry.is_unloading_chunk(chunk_id) {
            debug!("Registry: Old chunk '{:?}' is already unloading!", chunk_id);

            continue;
        }

        if !chunk_registry.try_allocate_chunk(chunk_id) {
            panic!("Failed to allocate old chunk '{:?}'!", chunk_id);
        }

        chunk_loader.start_unloading_chunk(chunk_id);
        chunk_registry.start_unloading_chunk(chunk_id);

        unload_chunk_event_writer.send(UnloadChunkEntity {
            chunk_request_id,
            chunk_id: old_chunk_id
        });
    }

    for new_chunk_id in new_chunk_ids.iter() {
        debug!("New chunk '{:?}' detected!", new_chunk_id);
        let chunk_request_id = chunk_request_registry.get_unused_chunk_request_id();
        let chunk_id = *new_chunk_id;

        if !chunk_registry.try_allocate_chunk(chunk_id) {
            error!("Failed to allocate new chunk '{:?}'!", chunk_id);

            continue;
        }

        if chunk_registry.is_chunk_registered(chunk_id) {
            debug!("New chunk '{:?}' is registered, thus it will be loaded.", chunk_id);

            if chunk_loader.currently_loading_chunks().contains(&chunk_id) {
                debug!("New chunk '{:?}' is already loading!", chunk_id);
    
                continue;
            }

            if chunk_registry.is_loading_chunk(chunk_id) {
                debug!("New chunk '{:?}' is already loading!", chunk_id);

                continue;
            }

            chunk_loader.start_loading_chunk(chunk_id);
            chunk_registry.start_loading_chunk(chunk_id);

            load_chunk_event_writer.send(LoadChunkEntity {
                chunk_request_id,
                chunk_id
            });
        } else {
            debug!("New chunk '{:?}' is not registered, thus it will be created.", chunk_id);
        
            if chunk_loader.currently_creating_chunks().contains(&chunk_id) {
                debug!("New chunk '{:?}' is already creating!", chunk_id);
    
                continue;
            }

            if chunk_registry.is_creating_chunk(chunk_id) {
                debug!("New chunk '{:?}' is already creating!", chunk_id);

                continue;
            }

            chunk_loader.start_creating_chunk(chunk_id);
            chunk_registry.start_creating_chunk(chunk_id);

            create_chunk_event_writer.send(CreateChunkEntity {
                chunk_request_id,
                chunk_id
            });
        }
    }
}

pub(in crate) fn handle_updated_chunks(
    mut created_chunk_internal_event_reader: EventReader<CreatedChunkEntityInternal>,
    mut loaded_chunk_internal_event_reader: EventReader<LoadedChunkEntityInternal>,
    mut unloaded_chunk_internal_event_reader: EventReader<UnloadedChunkEntityInternal>,
    chunk_loader: &mut Mut<ChunkLoader>,
    chunk_registry: &mut ResMut<ChunkRegistry>,
) {
    let mut created_chunk_internal_events = Vec::new();
    for created_chunk_event in created_chunk_internal_event_reader.read() {
        created_chunk_internal_events.push(created_chunk_event.clone());
    }

    let mut loaded_chunk_internal_events = Vec::new();
    for loaded_chunk_event in loaded_chunk_internal_event_reader.read() {
        loaded_chunk_internal_events.push(loaded_chunk_event.clone());
    }

    let mut unloaded_chunk_internal_events = Vec::new();
    for unloaded_chunk_event in unloaded_chunk_internal_event_reader.read() {
        unloaded_chunk_internal_events.push(unloaded_chunk_event.clone());
    }

    for created_chunk_internal_event in created_chunk_internal_events {
        let chunk_id = match created_chunk_internal_event {
            CreatedChunkEntityInternal::Success { chunk_id, .. } => chunk_id,
            CreatedChunkEntityInternal::Failure { chunk_id, .. } => chunk_id,
        };

        if chunk_loader.currently_creating_chunks().contains(&chunk_id) {
            chunk_loader.stop_creating_chunk(chunk_id);
        }

        if chunk_registry.is_creating_chunk(chunk_id) {
            chunk_registry.stop_creating_chunk(chunk_id);
        }
    }

    for loaded_chunk_internal_event in loaded_chunk_internal_events {
        let chunk_id = match loaded_chunk_internal_event {
            LoadedChunkEntityInternal::Success { chunk_id, .. } => chunk_id,
            LoadedChunkEntityInternal::Failure { chunk_id, .. } => chunk_id,
        };

        if chunk_loader.currently_loading_chunks().contains(&chunk_id) {
            chunk_loader.stop_loading_chunk(chunk_id);
        }

        if chunk_registry.is_loading_chunk(chunk_id) {
            chunk_registry.stop_loading_chunk(chunk_id);
        }
    }

    for unloaded_chunk_internal_event in unloaded_chunk_internal_events {
        let chunk_id = match unloaded_chunk_internal_event {
            UnloadedChunkEntityInternal::Success { chunk_id, .. } => chunk_id,
            UnloadedChunkEntityInternal::Failure { chunk_id, .. } => chunk_id,
        
        };

        if chunk_loader.currently_unloading_chunks().contains(&chunk_id) {
            chunk_loader.stop_unloading_chunk(chunk_id);
        }

        if chunk_registry.is_unloading_chunk(chunk_id) {
            chunk_registry.stop_unloading_chunk(chunk_id);
        }
    }
}