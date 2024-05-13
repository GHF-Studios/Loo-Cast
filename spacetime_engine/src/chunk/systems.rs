use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::scene::ron;
use bevy::scene::serde::{SceneSerializer, SceneDeserializer};
use bevy_rapier2d::dynamics::{RigidBody, Velocity};
use bevy_rapier2d::geometry::Collider;
use crate::chunk::components::Chunk;
use crate::chunk::id::structs::*;
use crate::chunk::coordinate::structs::*;
use crate::chunk::actor::coordinate::structs::*;
use crate::chunk::actor::components::*;
use crate::chunk::events::*;
use crate::chunk::resources::*;
use crate::chunk::loader::components::*;
use crate::chunk::functions::*;
use crate::math::structs::*;
use crate::physics::components::*;
use crate::player::components::*;

pub(in crate) fn update(
    mut create_chunk_event_writer: EventWriter<CreateChunk>,
    mut destroy_chunk_event_writer: EventWriter<DestroyChunk>,
    mut load_chunk_event_writer: EventWriter<LoadChunk>,
    mut unload_chunk_event_writer: EventWriter<UnloadChunk>,
    mut chunk_loader_query: Query<(&Transform, &mut ChunkLoader)>,
    mut chunk_registry: ResMut<ChunkRegistry>,
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
        if let Some(_) = chunk_registry.unload_chunk(old_chunk_id) {
            unload_chunk_event_writer.send(UnloadChunk(old_chunk_id));
        }
    }

    // Handle new chunks
    for new_chunk_id in new_chunk_ids.clone() {
        if chunk_registry.is_chunk_registered(new_chunk_id) {
            if !chunk_registry.is_loading_chunk(new_chunk_id) {
                chunk_registry.start_loading_chunk(new_chunk_id);

                load_chunk_event_writer.send(LoadChunk(new_chunk_id));
            }
        } else if !chunk_registry.is_creating_chunk(new_chunk_id) {
            chunk_registry.start_creating_chunk(new_chunk_id);

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
    mut commands: Commands,
    mut create_chunk_event_reader: EventReader<CreateChunk>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    for create_chunk_event in create_chunk_event_reader.read() {
        let new_chunk_entity = new_chunk_entity(&mut commands, create_chunk_event.0);

        chunk_registry.register_chunk(create_chunk_event.0);
        
        chunk_registry.load_chunk(create_chunk_event.0, new_chunk_entity);

        chunk_registry.stop_creating_chunk(create_chunk_event.0);
    }
}

pub(in crate) fn handle_destroy_chunk_events(
    mut commands: Commands,
    mut destroy_chunk_event_reader: EventReader<DestroyChunk>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    for destroy_chunk_event in destroy_chunk_event_reader.read() {
        if let Some(loaded_chunk_entity) = chunk_registry.unload_chunk(destroy_chunk_event.0) {
            commands.entity(loaded_chunk_entity).despawn_recursive();
        }
        
        chunk_registry.unregister_chunk(destroy_chunk_event.0);

        chunk_registry.stop_destroying_chunk(destroy_chunk_event.0);
    }
}

pub(in crate) fn handle_load_chunk_events(
    world: &mut World,
    params: &mut SystemState<(
        EventReader<LoadChunk>,
        ResMut<ChunkRegistry>,
    )>
) {
    let (mut load_chunk_event_reader, _) = params.get_mut(world);
    let mut load_chunk_events: Vec<LoadChunk> = Vec::new();
    for load_chunk_event in load_chunk_event_reader.read() {
        load_chunk_events.push(load_chunk_event.clone());
    }

    for load_chunk_event in load_chunk_events {
        let (_, mut chunk_registry) = params.get_mut(world);

        let serialized = chunk_registry.deserialize_chunk(load_chunk_event.0).unwrap();

        let dyn_scene = {
            let type_registry_rwlock = &world.resource::<AppTypeRegistry>().0.read();
        
            let deserializer = SceneDeserializer {
                type_registry: type_registry_rwlock,
            };
        
            let mut ron_deserializer = ron::de::Deserializer::from_str(&serialized).unwrap();

            use serde::de::DeserializeSeed;
        
            deserializer.deserialize(&mut ron_deserializer).unwrap()
        };

        let mut entity_hash_map = default();

        dyn_scene
            .write_to_world(world, &mut entity_hash_map)
            .unwrap();

        println!("# of loading entities: {}", entity_hash_map.len());

        let mut chunk_entity = None;
        for (_, entity_id) in entity_hash_map {
            println!("Checking integrity of deserialized Entity: {:?}", entity_id);

            let entity = match world.get_entity(entity_id) {
                Some(entity) => {
                    println!("Entity '{:?}' exists!", entity_id);
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

            if entity.contains::<ChunkActor>() {
                println!("Chunk actor detected!");
            }

            if entity.contains::<ProxyRigidBody>() {
                let rigid_body = entity.get::<ProxyRigidBody>().unwrap();
                println!("ProxyRigidBody detected: {:?}", rigid_body);
            }

            if entity.contains::<RigidBody>() {
                let rigid_body = entity.get::<RigidBody>().unwrap();
                println!("RigidBody detected: {:?}", rigid_body);
            }

            if entity.contains::<ProxyCollider>() {
                let collider = entity.get::<ProxyCollider>().unwrap();
                println!("ProxyCollider detected: {:?}", collider);
            }

            if entity.contains::<Collider>() {
                let collider = entity.get::<Collider>().unwrap();
                println!("Collider detected: {:?}", collider);
            }

            if entity.contains::<ProxyVelocity>() {
                let velocity = entity.get::<ProxyVelocity>().unwrap();
                println!("ProxyVelocity detected: {:?}", velocity);
            }

            if entity.contains::<Velocity>() {
                let velocity = entity.get::<Velocity>().unwrap();
                println!("Velocity detected: {:?}", velocity);
            }
        }

        let chunk_entity = match chunk_entity {
            Some(chunk_entity) => chunk_entity,
            None => panic!("No chunk detected!"),
        };

        println!("Detected chunk entity: {:?}", chunk_entity);

        let (_, mut chunk_registry) = params.get_mut(world);

        chunk_registry.load_chunk(load_chunk_event.0, chunk_entity);

        chunk_registry.stop_loading_chunk(load_chunk_event.0);
    }
}

pub(in crate) fn handle_unload_chunk_events(
    world: &mut World,
    params: &mut SystemState<(
        EventReader<UnloadChunk>,
        ResMut<ChunkRegistry>,
    )>
) {
    let (mut unload_chunk_event_reader, _) = params.get_mut(world);
    let mut unload_chunk_events: Vec<UnloadChunk> = Vec::new();
    for unload_chunk_event in unload_chunk_event_reader.read() {
        unload_chunk_events.push(unload_chunk_event.clone());
    }

    for unload_chunk_event in unload_chunk_events {
        let mut chunk_actor_entities = world
            .query::<(Entity, &ChunkActor)>()
            .iter(world)
            .filter(|(_, chunk_actor)| chunk_actor.current_chunk() == unload_chunk_event.0)
            .map(|(entity, _)| entity)
            .collect::<Vec<_>>();

        chunk_actor_entities.retain(|entity| {
            match world.get_entity_mut(*entity) {
                Some(_) => {
                    true
                },
                None => {
                    println!("Tried to unload non-existent entity!");
                    
                    false
                },
            }
        });

        let (_, chunk_registry) = params.get_mut(world);

        let chunk_entity = match chunk_registry.get_loaded_chunk_entity(unload_chunk_event.0) {
            Some(chunk_entity) => chunk_entity,
            None => continue,
        };

        println!("Checking integrity of chunk entity...");
        match world.get_entity(chunk_entity) {
            Some(_) => {
                println!("Chunk Entity '{:?}' exists!", chunk_entity);
            },
            None => {
                panic!("Chunk Entity '{:?}' does not exist!", chunk_entity);
            },
        }

        chunk_actor_entities.push(chunk_entity);
        let all_entities = chunk_actor_entities;
        
        let mut builder = DynamicSceneBuilder::from_world(world);
        
        builder = builder.extract_entities(all_entities.clone().into_iter());

        let dyn_scene = builder.build();
        let type_registry_arc = &world.resource::<AppTypeRegistry>().0;
        let serializer = SceneSerializer::new(&dyn_scene, type_registry_arc);
        let serialized = ron::to_string(&serializer).unwrap();

        for entity in all_entities {
            world.entity_mut(entity).despawn_recursive();
        }

        let (_, mut chunk_registry) = params.get_mut(world);

        chunk_registry.serialize_chunk(unload_chunk_event.0, serialized);

        chunk_registry.unload_chunk(unload_chunk_event.0);

        chunk_registry.stop_unloading_chunk(unload_chunk_event.0);
    }
}