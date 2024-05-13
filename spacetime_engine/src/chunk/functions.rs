use bevy::ecs::entity::EntityHashMap;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::scene::ron;
use bevy::scene::serde::{SceneSerializer, SceneDeserializer};
use bevy_rapier2d::dynamics::Velocity;
use crate::chunk::components::*;
use crate::chunk::constants::*;
use crate::chunk::id::structs::ChunkID as ChunkID;
use crate::chunk::coordinate::structs::ChunkCoordinate as ChunkCoordinate;
use crate::chunk::actor::coordinate::structs::ChunkActorCoordinate as ChunkActorCoordinate;
use crate::physics::components::*;
use serde::de::DeserializeSeed;
use super::actor::components::ChunkActor;
use super::ChunkRegistry;

pub(in crate) fn create_chunk_entity(commands: &mut Commands, chunk_id: ChunkID) -> Entity {
    // Gather the chunk coordinate and chunk actor coordinate
    let chunk_coordinate: ChunkCoordinate = chunk_id.into();
    let chunk_chunk_actor_coordinate: ChunkActorCoordinate = chunk_coordinate.into();
    let chunk_pos = chunk_coordinate.0;
    let world_pos = chunk_chunk_actor_coordinate.0;

    // Determine the chunk color
    let chunk_color = if (chunk_pos.0 + chunk_pos.1) % 2 == 0 {
        Color::rgb(0.25, 0.25, 0.25)
    } else {
        Color::rgb(0.75, 0.75, 0.75)
    };

    // Spawn the chunk entity
    let chunk_entity = commands.spawn((
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

    // Return the chunk entity
    chunk_entity
}

pub(in crate) fn deserialize_chunk(
    world: &mut World,
    serialized_chunk: String,
) -> Entity {
    // Deserialize the chunk scene
    let deserialized_chunk_scene = {
        // Gather the type registry arc
        let type_registry_rwlock = &world.resource::<AppTypeRegistry>().0.read();

        // Create the scene deserializer
        let deserializer = SceneDeserializer {
            type_registry: type_registry_rwlock,
        };

        // Create the RON deserializer from the serialized chunk scene
        let mut ron_deserializer = ron::de::Deserializer::from_str(&serialized_chunk).unwrap();

        // Deserialize and return the chunk scene
        deserializer.deserialize(&mut ron_deserializer).unwrap()
    };

    // Create an entity map
    let mut entity_map: EntityHashMap<Entity> = default();

    // Write the deserialized chunk scene to the world and populate the entity map
    deserialized_chunk_scene.write_to_world(world, &mut entity_map).unwrap();

    // Find the chunk entity
    let mut chunk_entity = None;
    for (_, entity_id) in entity_map {
        // Get the entity
        let entity = match world.get_entity(entity_id) {
                Some(entity) => {
                    entity
                },
                None => {
                    panic!("Entity '{:?}' does not exist!", entity_id);
                },
        };

        // Check if the entity is a chunk entity
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

    // Check if chunk entity exists
    let chunk_entity = match chunk_entity {
        Some(chunk_entity) => chunk_entity,
        None => panic!("No chunk detected!"),
    };

    // Return the chunk entity
    chunk_entity
}

pub(in crate) fn serialize_chunk(
    world: &mut World,
    registry_parameter: &mut SystemState<(
        ResMut<ChunkRegistry>,
    )>,
    chunk_id: ChunkID
) -> String {
    // Gather chunk actor entities
    let mut entities = world
            .query::<(Entity, &ChunkActor)>()
            .iter(world)
            .filter(|(_, chunk_actor)| chunk_actor.current_chunk() == chunk_id)
            .map(|(entity, _)| entity)
            .collect::<Vec<_>>();

    // Check if chunk actor entities exist
    for entity in entities.iter() {
        match world.get_entity(*entity) {
            Some(_) => {},
            None => {
                panic!("Tried to unload non-existent entity!");
            },
        }
    }

    // Gather chunk registry
    let chunk_registry = registry_parameter.get_mut(world).0;

    // Check if chunk entity exists
    let chunk_entity = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
        Some(chunk_entity) => chunk_entity,
        None => panic!("Chunk Entity '{:?}' does not exist!", chunk_id)
    };

    // Check if chunk entity exists
    match world.get_entity(chunk_entity) {
        Some(_) => {},
        None => {
            panic!("Chunk Entity '{:?}' does not exist!", chunk_entity);
        },
    }

    // Add chunk entity to chunk actor entities, constituting all to-be-serialized entities
    entities.push(chunk_entity);

    // Create a dynamic scene builder from the world
    let mut builder = DynamicSceneBuilder::from_world(world);
    
    // Extract entities into the builder
    builder = builder.extract_entities(entities.clone().into_iter());

    // Build the dynamic scene
    let dyn_scene = builder.build();

    // Gather the type registry arc
    let type_registry_arc = &world.resource::<AppTypeRegistry>().0;

    // Create the scene serializer
    let serializer = SceneSerializer::new(&dyn_scene, type_registry_arc);

    // Serialize the scene
    let serialized_chunk = ron::to_string(&serializer).unwrap();

    // Despawn all serialized entities
    for entity in entities.iter() {
        world.entity_mut(*entity).despawn_recursive();
    }

    // Return the serialized chunk
    serialized_chunk
}