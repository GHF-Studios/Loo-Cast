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
use super::{ChunkEventRegistry, ChunkRegistry};

pub(in crate) fn new_chunk_entity(world: &mut World, chunk_id: ChunkID) -> Entity {
    let chunk_position: ChunkPosition = chunk_id.into();
    let chunk_chunk_actor_position: ChunkActorPosition = chunk_position.into();
    let chunk_position = chunk_position.0;
    let world_position = chunk_chunk_actor_position.0;

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
    chunk_registry: &Res<ChunkRegistry>,
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

    (old_chunks, unchanged_chunks, new_chunks)
}

pub(in crate) fn start_chunks(
    mut create_chunk_event_writer: EventWriter<CreateChunkEntity>,
    mut load_chunk_event_writer: EventWriter<LoadChunkEntity>,
    chunk_registry: &Res<ChunkRegistry>,
    chunk_event_registry: &mut ResMut<ChunkEventRegistry>,
    detected_chunk_ids: &Vec<ChunkID>,
) {
    for detected_chunk_id in detected_chunk_ids {
        let chunk_id = *detected_chunk_id;
        let chunk_event_id = chunk_event_registry.get_unused_chunk_event_id();

        if chunk_registry.is_chunk_registered(chunk_id) {
            load_chunk_event_writer.send(LoadChunkEntity { 
                chunk_event_id,
                chunk_id
            });
        } else {
            create_chunk_event_writer.send(CreateChunkEntity { 
                chunk_event_id,
                chunk_id
            });
        }
    }
}

pub(in crate) fn update_chunks(
    mut create_chunk_event_writer: EventWriter<CreateChunkEntity>,
    mut load_chunk_event_writer: EventWriter<LoadChunkEntity>,
    mut unload_chunk_event_writer: EventWriter<UnloadChunkEntity>,
    chunk_registry: &Res<ChunkRegistry>,
    chunk_event_registry: &mut ResMut<ChunkEventRegistry>,
    old_chunk_ids: Vec<ChunkID>,
    new_chunk_ids: Vec<ChunkID>,
) {
    for old_chunk_id in old_chunk_ids {
        let chunk_event_id = chunk_event_registry.get_unused_chunk_event_id();

        unload_chunk_event_writer.send(UnloadChunkEntity {
            chunk_event_id,
            chunk_id: old_chunk_id
        });
    }

    for new_chunk_id in new_chunk_ids.iter() {
        let chunk_event_id = chunk_event_registry.get_unused_chunk_event_id();
        let chunk_id = *new_chunk_id;
        
        if chunk_registry.is_chunk_registered(chunk_id) {
            load_chunk_event_writer.send(LoadChunkEntity {
                chunk_event_id,
                chunk_id
            });
        } else {
            create_chunk_event_writer.send(CreateChunkEntity {
                chunk_event_id,
                chunk_id
            });
        }
    }
}