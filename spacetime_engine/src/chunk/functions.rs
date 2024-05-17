use bevy::ecs::entity::EntityHashMap;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::scene::ron;
use bevy::scene::serde::{SceneSerializer, SceneDeserializer};
use crate::chunk::components::*;
use crate::chunk::constants::*;
use crate::chunk::id::structs::ChunkID as ChunkID;
use crate::chunk::coordinate::structs::ChunkCoordinate as ChunkCoordinate;
use crate::chunk::actor::coordinate::structs::ChunkActorCoordinate as ChunkActorCoordinate;
use serde::de::DeserializeSeed;
use super::actor::components::ChunkActor;
use super::ChunkRegistry;

pub(in crate) fn new_chunk_entity(world: &mut World, chunk_id: ChunkID) -> Entity {
    let chunk_coordinate: ChunkCoordinate = chunk_id.into();
    let chunk_chunk_actor_coordinate: ChunkActorCoordinate = chunk_coordinate.into();
    let chunk_pos = chunk_coordinate.0;
    let world_pos = chunk_chunk_actor_coordinate.0;

    let chunk_color = if (chunk_pos.0 + chunk_pos.1) % 2 == 0 {
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
            transform: Transform::from_translation(world_pos),
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