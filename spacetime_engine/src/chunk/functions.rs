use bevy::ecs::component::ComponentId;
use bevy::ecs::entity::EntityHashMap;
use bevy::ecs::system::SystemState;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::scene::ron;
use bevy::scene::serde::{SceneSerializer, SceneDeserializer};
use crate::chunk::components::*;
use crate::chunk::constants::*;
use crate::chunk::id::structs::ChunkID as ChunkID;
use crate::chunk::position::structs::ChunkPosition as ChunkPosition;
use crate::chunk::actor::position::structs::ChunkActorPosition as ChunkActorPosition;
use crate::chunk::events::*;
use crate::entity;
use crate::entity::events::*;
use crate::entity::id::structs::EntityID;
use crate::entity::resources::{EntityRegistry, EntityRequestRegistry};
use crate::entity::structs::EntityResponse;
use crate::math::structs::I16Vec2;
use serde::de::DeserializeSeed;
use super::actor::components::ChunkActor;
use super::id::structs::ChunkRequestID;
use super::loader::components::ChunkLoader;
use super::structs::{ChunkRequest, ChunkResponse};
use super::{ChunkRequestRegistry, ChunkRegistry};


pub(super) fn setup(world: &mut World) {
    world
        .register_component_hooks::<Chunk>()
        .on_add(on_add_chunk)
        .on_remove(on_remove_chunk);

}

pub fn can_request_upgrade_to_chunk(
    chunk_registry: &mut ChunkRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
    entity_id: EntityID,
) -> bool {
    let mut result = true;

    if !entity_registry.is_entity_registered(entity_id) { result = false; }
    if !entity_registry.is_entity_loaded(entity_id) { result = false; }

    if entity_registry.is_entity_creating(entity_id) { result = false; }
    if entity_registry.is_entity_destroying(entity_id) { result = false; }
    if entity_registry.is_entity_loading(entity_id) { result = false; }
    if entity_registry.is_entity_saving(entity_id) { result = false; }

    if chunk_registry.is_chunk_registered(chunk_id) { result = false; }
    if chunk_registry.is_chunk_loaded(chunk_id) { result = false; }

    if chunk_registry.is_chunk_upgrading_to(chunk_id) { result = false; }
    if chunk_registry.is_chunk_downgrading_from(chunk_id) { result = false; }
    if chunk_registry.is_chunk_loading(chunk_id) { result = false; }
    if chunk_registry.is_chunk_saving(chunk_id) { result = false; }

    result
}

pub fn can_request_downgrade_from_chunk(
    chunk_registry: &mut ChunkRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
) -> bool {
    let mut result = true;

    let entity_id = {
        let entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => panic!("Chunk entity '{:?}' is not loaded!", chunk_id)
        };

        match entity_registry.get_loaded_entity_id(&entity_reference) {
            Some(entity_id) => entity_id,
            None => panic!("Entity reference '{:?}' is not loaded!", entity_reference)
        }
    };

    if !entity_registry.is_entity_registered(entity_id) { result = false; }
    if !entity_registry.is_entity_loaded(entity_id) { result = false; }

    if entity_registry.is_entity_creating(entity_id) { result = false; }
    if entity_registry.is_entity_destroying(entity_id) { result = false; }
    if entity_registry.is_entity_loading(entity_id) { result = false; }
    if entity_registry.is_entity_saving(entity_id) { result = false; }

    if !chunk_registry.is_chunk_registered(chunk_id) { result = false; }
    if !chunk_registry.is_chunk_loaded(chunk_id) { result = false; }

    if chunk_registry.is_chunk_upgrading_to(chunk_id) { result = false; }
    if chunk_registry.is_chunk_downgrading_from(chunk_id) { result = false; }
    if chunk_registry.is_chunk_loading(chunk_id) { result = false; }
    if chunk_registry.is_chunk_saving(chunk_id) { result = false; }

    result
}

pub fn can_request_load_chunk(
    chunk_registry: &mut ChunkRegistry,
    chunk_id: ChunkID,
) -> bool {
    let mut result = true;

    if chunk_registry.is_chunk_registered(chunk_id) { result = false; }
    if chunk_registry.is_chunk_loaded(chunk_id) { result = false; }

    if chunk_registry.is_chunk_upgrading_to(chunk_id) { result = false; }
    if chunk_registry.is_chunk_downgrading_from(chunk_id) { result = false; }
    if chunk_registry.is_chunk_loading(chunk_id) { result = false; }
    if chunk_registry.is_chunk_saving(chunk_id) { result = false; }

    result
}

pub fn can_request_save_chunk(
    chunk_registry: &mut ChunkRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
) -> bool {
    let mut result = true;

    let entity_id = {
        let entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => panic!("Chunk entity '{:?}' is not loaded!", chunk_id)
        };

        match entity_registry.get_loaded_entity_id(&entity_reference) {
            Some(entity_id) => entity_id,
            None => panic!("Entity reference '{:?}' is not loaded!", entity_reference)
        }
    };

    if !entity_registry.is_entity_registered(entity_id) { result = false; }
    if !entity_registry.is_entity_loaded(entity_id) { result = false; }

    if entity_registry.is_entity_creating(entity_id) { result = false; }
    if entity_registry.is_entity_destroying(entity_id) { result = false; }
    if entity_registry.is_entity_loading(entity_id) { result = false; }
    if entity_registry.is_entity_saving(entity_id) { result = false; }

    if !chunk_registry.is_chunk_registered(chunk_id) { result = false; }
    if !chunk_registry.is_chunk_loaded(chunk_id) { result = false; }

    if chunk_registry.is_chunk_upgrading_to(chunk_id) { result = false; }
    if chunk_registry.is_chunk_downgrading_from(chunk_id) { result = false; }
    if chunk_registry.is_chunk_loading(chunk_id) { result = false; }
    if chunk_registry.is_chunk_saving(chunk_id) { result = false; }

    result
}

pub fn request_upgrade_to_chunk(
    upgrade_to_chunk_event_writer: &mut EventWriter<UpgradeToChunk>,
    chunk_registry: &mut ChunkRegistry,
    chunk_request_registry: &mut ChunkRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
    entity_id: EntityID,
) -> Option<ChunkRequestID> {
    let chunk_request_id = chunk_request_registry.get_unused_chunk_request_id();

    if !can_request_upgrade_to_chunk(chunk_registry, entity_registry, chunk_id, entity_id) {
        return None;
    }

    let upgrade_to_chunk_request = ChunkRequest {
        chunk_request_id,
        chunk_id,
    };

    chunk_registry.start_upgrading_to_chunk(chunk_id);
    chunk_request_registry.register_chunk_request(chunk_request_id);
    chunk_request_registry.load_chunk_request(chunk_request_id, upgrade_to_chunk_request);
    upgrade_to_chunk_event_writer.send(UpgradeToChunk(upgrade_to_chunk_request));

    Some(chunk_request_id)
}

pub fn request_downgrade_from_chunk(
    downgrade_from_chunk_event_writer: &mut EventWriter<DowngradeFromChunk>,
    chunk_registry: &mut ChunkRegistry,
    chunk_request_registry: &mut ChunkRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
) -> Option<ChunkRequestID> {
    let chunk_request_id = chunk_request_registry.get_unused_chunk_request_id();

    if !can_request_downgrade_from_chunk(chunk_registry, entity_registry, chunk_id) {
        return None;
    }

    let downgrade_from_chunk_request = ChunkRequest {
        chunk_request_id,
        chunk_id,
    };

    chunk_registry.start_downgrading_from_chunk(chunk_id);
    chunk_request_registry.register_chunk_request(chunk_request_id);
    chunk_request_registry.load_chunk_request(chunk_request_id, downgrade_from_chunk_request);
    downgrade_from_chunk_event_writer.send(DowngradeFromChunk(downgrade_from_chunk_request));

    Some(chunk_request_id)
}

pub fn request_load_chunk(
    load_chunk_event_writer: &mut EventWriter<LoadChunk>,
    chunk_registry: &mut ChunkRegistry,
    chunk_request_registry: &mut ChunkRequestRegistry,
    chunk_id: ChunkID,
) -> Option<ChunkRequestID> {
    let chunk_request_id = chunk_request_registry.get_unused_chunk_request_id();

    if !can_request_load_chunk(chunk_registry, chunk_id) {
        return None;
    }

    let load_chunk_request = ChunkRequest {
        chunk_request_id,
        chunk_id,
    };

    chunk_registry.start_loading_chunk(chunk_id);
    chunk_request_registry.register_chunk_request(chunk_request_id);
    chunk_request_registry.load_chunk_request(chunk_request_id, load_chunk_request);
    load_chunk_event_writer.send(LoadChunk(load_chunk_request));

    Some(chunk_request_id)
}

pub fn request_save_chunk(
    save_chunk_event_writer: &mut EventWriter<SaveChunk>,
    chunk_registry: &mut ChunkRegistry,
    chunk_request_registry: &mut ChunkRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
) -> Option<ChunkRequestID> {
    let chunk_request_id = chunk_request_registry.get_unused_chunk_request_id();

    if !can_request_save_chunk(chunk_registry, entity_registry, chunk_id) {
        return None;
    }

    let unload_chunk_request = ChunkRequest {
        chunk_request_id,
        chunk_id,
    };

    chunk_registry.start_saving_chunk(chunk_id);
    chunk_request_registry.register_chunk_request(chunk_request_id);
    chunk_request_registry.load_chunk_request(chunk_request_id, unload_chunk_request);
    save_chunk_event_writer.send(SaveChunk(unload_chunk_request));

    Some(chunk_request_id)
}

fn on_add_chunk(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let entity_reference = entity;

    let chunk_id = match world.get::<Chunk>(entity_reference) {
        Some(chunk) => chunk.id(),
        None => {
            panic!("Failed to get chunk component associated with entity '{:?}'!", entity_reference);
        }
    };

    let chunk_request_id = {
        let chunk_request_registry = match world.get_resource_mut::<ChunkRequestRegistry>() {
            Some(chunk_request_registry) => chunk_request_registry,
            None => {
                panic!("Failed to get chunk request registry!");
            }
        };

        match chunk_request_registry
            .loaded_chunk_requests()
            .values()
            .clone()
            .find(|chunk_request| chunk_request.chunk_id == chunk_id)
            .map(|request| {
                request.chunk_request_id
            }) {
            Some(chunk_request_id) => chunk_request_id,
            None => {
                panic!("Failed to get chunk request id currently associated with chunk entity '{:?}'!", entity_reference);
            }
        }
    };

    {
        let mut chunk_registry = match world.get_resource_mut::<ChunkRegistry>() {
            Some(chunk_registry) => chunk_registry,
            None => {
                panic!("Failed to get chunk registry!");
            }
        };

        let is_upgrading_to_chunk = chunk_registry.is_chunk_upgrading_to(chunk_id);
        let is_chunk_loading = chunk_registry.is_chunk_loading(chunk_id);

        if is_upgrading_to_chunk && is_chunk_loading {
            panic!("Chunk '{:?}' is both upgrading and loading!", chunk_id);
        } else if is_upgrading_to_chunk {
            chunk_registry.stop_upgrading_to_chunk(chunk_id);

            let mut chunk_request_registry = match world.get_resource_mut::<ChunkRequestRegistry>() {
                Some(chunk_request_registry) => chunk_request_registry,
                None => {
                    panic!("Failed to get chunk request registry!");
                }
            };

            chunk_request_registry.unload_chunk_request(chunk_request_id);

            world.send_event(UpgradedToChunk(ChunkResponse::Success {
                chunk_request_id,
                chunk_id,
            }));
        } else if is_chunk_loading {
            chunk_registry.stop_loading_chunk(chunk_id);

            let mut chunk_request_registry = match world.get_resource_mut::<ChunkRequestRegistry>() {
                Some(chunk_request_registry) => chunk_request_registry,
                None => {
                    panic!("Failed to get chunk request registry!");
                }
            };

            chunk_request_registry.unload_chunk_request(chunk_request_id);

            world.send_event(LoadedChunk(ChunkResponse::Success {
                chunk_request_id,
                chunk_id,
            }));
        } else {
            panic!("Chunk '{:?}' is neither upgrading nor loading!", chunk_id);
        }
    }

    {
        let chunk_position: ChunkPosition = chunk_id.into();
        let chunk_chunk_actor_position: ChunkActorPosition = chunk_position.into();
        let chunk_position = chunk_position.0;
        let world_position = chunk_chunk_actor_position.0;
        let world_position = Vec3::new(world_position.x, world_position.y, CHUNK_Z_INDEX);
    
        let chunk_color = if (chunk_position.0 + chunk_position.1) % 2 == 0 {
            Color::srgb(0.25, 0.25, 0.25)
        } else {
            Color::srgb(0.75, 0.75, 0.75)
        };

        world.commands().entity(entity_reference).insert(
            SpriteBundle {
                sprite: Sprite {
                    color: chunk_color,
                    custom_size: Some(Vec2::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32)),
                    ..default()
                },
                transform: Transform::from_translation(world_position),
                ..default()
            },
        );
    }
}

fn on_remove_chunk(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let entity_reference = entity;

    let chunk_id = match world.get::<Chunk>(entity_reference) {
        Some(chunk) => chunk.id(),
        None => {
            panic!("Failed to get chunk component associated with entity '{:?}'!", entity_reference);
        }
    };

    let chunk_request_id = {
        let chunk_request_registry = match world.get_resource_mut::<ChunkRequestRegistry>() {
            Some(chunk_request_registry) => chunk_request_registry,
            None => {
                panic!("Failed to get chunk request registry!");
            }
        };

        match chunk_request_registry
            .loaded_chunk_requests()
            .values()
            .clone()
            .find(|chunk_request| chunk_request.chunk_id == chunk_id)
            .map(|request| {
                request.chunk_request_id
            }) {
            Some(chunk_request_id) => chunk_request_id,
            None => {
                panic!("Failed to get chunk request id currently associated with chunk entity '{:?}'!", entity_reference);
            }
        }
    };

    {
        let mut chunk_registry = match world.get_resource_mut::<ChunkRegistry>() {
            Some(chunk_registry) => chunk_registry,
            None => {
                panic!("Failed to get chunk registry!");
            }
        };

        let is_downgrading_from_chunk = chunk_registry.is_chunk_downgrading_from(chunk_id);
        let is_saving_chunk = chunk_registry.is_chunk_saving(chunk_id);

        if is_downgrading_from_chunk && is_saving_chunk {
            panic!("Chunk '{:?}' is both downgrading and saving!", chunk_id);
        } else if is_downgrading_from_chunk {
            chunk_registry.stop_downgrading_from_chunk(chunk_id);

            let mut chunk_request_registry = match world.get_resource_mut::<ChunkRequestRegistry>() {
                Some(chunk_request_registry) => chunk_request_registry,
                None => {
                    panic!("Failed to get chunk request registry!");
                }
            };

            chunk_request_registry.unload_chunk_request(chunk_request_id);

            world.send_event(DowngradedFromChunk(ChunkResponse::Success {
                chunk_request_id,
                chunk_id,
            }));
        } else if is_saving_chunk {
            chunk_registry.stop_saving_chunk(chunk_id);

            let mut chunk_request_registry = match world.get_resource_mut::<ChunkRequestRegistry>() {
                Some(chunk_request_registry) => chunk_request_registry,
                None => {
                    panic!("Failed to get chunk request registry!");
                }
            };

            chunk_request_registry.unload_chunk_request(chunk_request_id);

            world.send_event(SavedChunk(ChunkResponse::Success {
                chunk_request_id,
                chunk_id,
            }));
        } else {
            panic!("Chunk '{:?}' is neither downgrading nor saving!", chunk_id);
        }
    }
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
    registry_parameter: &mut SystemState<
        ResMut<ChunkRegistry>,
    >,
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

    let chunk_registry = registry_parameter.get_mut(world);

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

    let type_registry = type_registry_arc.read();

    let serializer = SceneSerializer::new(&dyn_scene, &*type_registry);

    let serialized_chunk = match ron::to_string(&serializer) {
        Ok(serialized_chunk) => serialized_chunk.clone(),
        Err(error) => {
            panic!("Failed to serialize chunk '{:?}'! Error: {:?}", chunk_id, error);
        },
    };

    drop(type_registry);

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

    (old_chunks, unchanged_chunks, new_chunks)
}
