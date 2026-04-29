use bevy::ecs::entity::EntityHashMap;
use bevy::prelude::*;
use bevy::scene::ron;
use bevy::scene::serde::{SceneSerializer, SceneDeserializer};
use serde::de::DeserializeSeed;
use crate::chunk_actor::components::ChunkActor;
use crate::core::singletons::*;
use crate::core::structs::NumericID;
use crate::chunk::wrappers::ChunkInstanceRegistry;
use super::components::Chunk;

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
    chunk_id: NumericID<Chunk>
) -> String {
    debug!("Serializing chunk '{:?}'...", chunk_id);

    let main_type_registry = MAIN_TYPE_REGISTRY.lock().unwrap();

    let chunk_instance_registry = main_type_registry.get_data::<Chunk, ChunkInstanceRegistry>().unwrap();

    let chunk_entity = match chunk_instance_registry.get(chunk_id) {
        Some(chunk_entity) => chunk_entity,
        None => panic!("Chunk Entity '{:?}' is not loaded!", chunk_id)
    };

    match world.get_entity(*chunk_entity) {
        Some(_) => {},
        None => {
            panic!("Chunk Entity '{:?}' does not exist!", chunk_entity);
        },
    }
    
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

    entities.push(*chunk_entity);

    let mut builder = DynamicSceneBuilder::from_world(world);
    
    builder = builder.extract_entities(entities.clone().into_iter());

    let dyn_scene = builder.build();

    let bevy_type_registry_arc = &world.resource::<AppTypeRegistry>().0;

    let bevy_type_registry = bevy_type_registry_arc.read();

    let serializer = SceneSerializer::new(&dyn_scene, &bevy_type_registry);

    let serialized_chunk = match ron::to_string(&serializer) {
        Ok(serialized_chunk) => serialized_chunk.clone(),
        Err(error) => {
            panic!("Failed to serialize chunk '{:?}'! Error: {:?}", chunk_id, error);
        },
    };

    drop(bevy_type_registry);

    for entity in entities.iter() {
        world.entity_mut(*entity).despawn_recursive();
    }

    serialized_chunk
}