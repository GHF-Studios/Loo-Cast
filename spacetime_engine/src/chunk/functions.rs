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
use crate::entity::id::structs::EntityID;
use crate::entity::resources::EntityRegistry;
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

pub fn request_upgrade_to_chunk(
    upgrade_to_chunk_event_writer: &mut EventWriter<UpgradeToChunk>,
    chunk_registry: &mut ChunkRegistry,
    chunk_request_registry: &mut ChunkRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
    entity_id: EntityID,
) -> ChunkRequestID {
    let chunk_request_id = chunk_request_registry.get_unused_chunk_request_id();

    let upgrade_to_chunk_request = ChunkRequest {
        chunk_request_id,
        chunk_id,
        entity_id,
    };

    chunk_request_registry.register_chunk_request(chunk_request_id);

    if !entity_registry.is_entity_registered(entity_id) {
        panic!("Entity '{:?}' is not registered!", entity_id);
    }

    if !entity_registry.is_entity_loaded(entity_id) {
        panic!("Entity '{:?}' is not loaded!", entity_id);
    }

    if chunk_registry.is_chunk_registered(chunk_id) {
        warn!("Chunk '{:?}' is already registered!", chunk_id);

        return chunk_request_id;
    }

    if chunk_registry.is_chunk_loaded(chunk_id) {
        warn!("Chunk '{:?}' is already loaded!", chunk_id);

        return chunk_request_id;
    }

    chunk_registry.start_creating_chunk(chunk_id);
    chunk_request_registry.load_chunk_request(chunk_request_id, upgrade_to_chunk_request);
    upgrade_to_chunk_event_writer.send(UpgradeToChunk(upgrade_to_chunk_request));

    chunk_request_id
}

pub fn request_downgrade_from_chunk(
    downgrade_from_chunk_event_writer: &mut EventWriter<DowngradeFromChunk>,
    chunk_registry: &mut ChunkRegistry,
    chunk_request_registry: &mut ChunkRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
) -> ChunkRequestID {
    let chunk_request_id = chunk_request_registry.get_unused_chunk_request_id();
    let entity_id = {
        let entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => panic!("Chunk entity '{:?}' is not loaded!", chunk_id)
        };

        let entity_id = match entity_registry.get_loaded_entity_id(&entity_reference) {
            Some(entity_id) => entity_id,
            None => panic!("Entity reference '{:?}' is not loaded!", entity_reference)
        };

        entity_id
    };

    let downgrade_from_chunk_request = ChunkRequest {
        chunk_request_id,
        chunk_id,
        entity_id,
    };

    chunk_request_registry.register_chunk_request(chunk_request_id);

    if !entity_registry.is_entity_registered(entity_id) {
        panic!("Entity '{:?}' is not registered!", entity_id);
    }

    if !entity_registry.is_entity_loaded(entity_id) {
        panic!("Entity '{:?}' is not loaded!", entity_id);
    }

    if !chunk_registry.is_chunk_registered(chunk_id) {
        warn!("Chunk '{:?}' is not registered!", chunk_id);

        return chunk_request_id;
    }

    if !chunk_registry.is_chunk_loaded(chunk_id) {
        warn!("Chunk '{:?}' is not loaded!", chunk_id);

        return chunk_request_id;
    }

    chunk_registry.start_unloading_chunk(chunk_id);
    chunk_request_registry.load_chunk_request(chunk_request_id, downgrade_from_chunk_request);
    downgrade_from_chunk_event_writer.send(DowngradeFromChunk(downgrade_from_chunk_request));

    chunk_request_id
}

pub fn request_load_chunk(
    deserialize_chunk_event_writer: &mut EventWriter<LoadChunk>,
    chunk_registry: &mut ChunkRegistry,
    chunk_request_registry: &mut ChunkRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
    entity_id: EntityID,
) -> ChunkRequestID {
    let chunk_request_id = chunk_request_registry.get_unused_chunk_request_id();

    let load_chunk_request = ChunkRequest {
        chunk_request_id,
        chunk_id,
        entity_id,
    };

    chunk_request_registry.register_chunk_request(chunk_request_id);

    if entity_registry.is_entity_registered(entity_id) {
        panic!("Entity '{:?}' is already registered!", entity_id);
    }

    if entity_registry.is_entity_loaded(entity_id) {
        panic!("Entity '{:?}' is already loaded!", entity_id);
    }

    if chunk_registry.is_chunk_registered(chunk_id) {
        warn!("Chunk '{:?}' is already registered!", chunk_id);

        return chunk_request_id;
    }

    if chunk_registry.is_chunk_loaded(chunk_id) {
        warn!("Chunk '{:?}' is already loaded!", chunk_id);

        return chunk_request_id;
    }

    chunk_registry.start_loading_chunk(chunk_id);
    chunk_request_registry.load_chunk_request(chunk_request_id, load_chunk_request);
    deserialize_chunk_event_writer.send(LoadChunk(load_chunk_request));

    chunk_request_id
}

pub fn request_unload_chunk(
    serialize_chunk_event_writer: &mut EventWriter<SaveChunk>,
    chunk_registry: &mut ChunkRegistry,
    chunk_request_registry: &mut ChunkRequestRegistry,
    entity_registry: &mut EntityRegistry,
    chunk_id: ChunkID,
) -> ChunkRequestID {
    let chunk_request_id = chunk_request_registry.get_unused_chunk_request_id();
    let entity_id = {
        let entity_reference = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => panic!("Chunk entity '{:?}' is not loaded!", chunk_id)
        };

        let entity_id = match entity_registry.get_loaded_entity_id(&entity_reference) {
            Some(entity_id) => entity_id,
            None => panic!("Entity reference '{:?}' is not loaded!", entity_reference)
        };

        entity_id
    };

    let unload_chunk_request = ChunkRequest {
        chunk_request_id,
        chunk_id,
        entity_id,
    };

    chunk_request_registry.register_chunk_request(chunk_request_id);

    if !entity_registry.is_entity_registered(entity_id) {
        panic!("Entity '{:?}' is not registered!", entity_id);
    }

    if !entity_registry.is_entity_loaded(entity_id) {
        panic!("Entity '{:?}' is not loaded!", entity_id);
    }

    if !chunk_registry.is_chunk_registered(chunk_id) {
        warn!("Chunk '{:?}' is not registered!", chunk_id);

        return chunk_request_id;
    }

    if !chunk_registry.is_chunk_loaded(chunk_id) {
        warn!("Chunk '{:?}' is not loaded!", chunk_id);

        return chunk_request_id;
    }

    chunk_registry.start_unloading_chunk(chunk_id);
    chunk_request_registry.load_chunk_request(chunk_request_id, unload_chunk_request);
    serialize_chunk_event_writer.send(SaveChunk(unload_chunk_request));

    chunk_request_id
}

fn on_add_chunk(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let entity_reference = entity;

    let world_position = match world.get::<Transform>(entity_reference) {
        Some(transform) => transform.translation.truncate(),
        None => {
            panic!("Failed to get transform component associated with entity '{:?}'!", entity_reference);
        }
    };
    
    let entity_id = {
        let mut entity_registry = match world.get_resource_mut::<EntityRegistry>() {
            Some(entity_registry) => entity_registry,
            None => {
                panic!("Failed to get entity registry!");
            }
        };

        let entity_id = match entity_registry.get_loaded_entity_id(&entity_reference) {
            Some(entity_id) => {
                panic!("Entity '{:?}' is already loaded!", entity_id);
            },
            None => {
                let entity_id = entity_registry.register_entity();
                entity_registry.load_entity(entity_id, entity_reference);

                entity_id
            }
        };

        entity_id
    };

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

        let mut chunk_request_registry = match world.get_resource_mut::<ChunkRequestRegistry>() {
            Some(chunk_request_registry) => chunk_request_registry,
            None => {
                panic!("Failed to get chunk request registry!");
            }
        };

        let is_upgrading_to_chunk = chunk_registry.is_upgrading_to_chunk(chunk_id);
        let is_loading_chunk = chunk_registry.is_loading_chunk(chunk_id);

        if is_upgrading_to_chunk && is_loading_chunk {
            panic!("Chunk '{:?}' is both upgrading and loading!", chunk_id);
        } else if is_upgrading_to_chunk {
            chunk_registry.stop_upgrading_to_chunk(chunk_id);

            chunk_request_registry.unload_chunk_request(chunk_request_id);

            world.send_event(UpgradedToChunk(ChunkResponse::Success {
                chunk_request_id,
                chunk_id,
                chunk_entity_id: entity_id,
                world_position,
            }));
        } else if is_loading_chunk {
            chunk_registry.stop_loading_chunk(chunk_id);

            chunk_request_registry.unload_chunk_request(chunk_request_id);

            world.send_event(LoadedChunk(ChunkResponse::Success {
                chunk_request_id,
                chunk_id,
                chunk_entity_id: entity_id,
                world_position,
            }));
        } else {
            panic!("Chunk '{:?}' is neither upgrading nor loading!", chunk_id);
        }
    }
}

fn on_remove_chunk(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let entity_reference = entity;

    let world_position = match world.get::<Transform>(entity_reference) {
        Some(transform) => transform.translation.truncate(),
        None => {
            panic!("Failed to get transform component associated with entity '{:?}'!", entity_reference);
        }
    };

    let entity_id = {
        let mut entity_registry = match world.get_resource_mut::<EntityRegistry>() {
            Some(entity_registry) => entity_registry,
            None => {
                panic!("Failed to get entity registry!");
            }
        };

        match entity_registry.get_loaded_entity_id(&entity_reference) {
            Some(entity_id) => {
                entity_registry.unload_entity(entity_id);
                entity_registry.unregister_entity(entity_id);

                entity_id
            },
            None => {
                panic!("Failed to get entity id associated with entity '{:?}'!", entity_reference);
            }
        }
    };

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

        let mut chunk_request_registry = match world.get_resource_mut::<ChunkRequestRegistry>() {
            Some(chunk_request_registry) => chunk_request_registry,
            None => {
                panic!("Failed to get chunk request registry!");
            }
        };

        let is_downgrading_from_chunk = chunk_registry.is_downgrading_from_chunk(chunk_id);
        let is_unloading_chunk = chunk_registry.is_unloading_chunk(chunk_id);

        if is_downgrading_from_chunk && is_unloading_chunk {
            panic!("Chunk '{:?}' is both downgrading and unloading!", chunk_id);
        } else if is_downgrading_from_chunk {
            chunk_registry.stop_downgrading_from_chunk(chunk_id);

            chunk_request_registry.unload_chunk_request(chunk_request_id);

            world.send_event(DowngradedFromChunk(ChunkResponse::Success {
                chunk_request_id,
                chunk_id,
                chunk_entity_id: entity_id,
                world_position,
            }));
        } else if is_unloading_chunk {
            chunk_registry.stop_unloading_chunk(chunk_id);

            chunk_request_registry.unload_chunk_request(chunk_request_id);

            world.send_event(UnloadedChunk(ChunkResponse::Success {
                chunk_request_id,
                chunk_id,
                chunk_entity_id: entity_id,
                world_position,
            }));
        } else {
            panic!("Chunk '{:?}' is neither downgrading nor unloading!", chunk_id);
        }
    }
}

pub(in crate) fn new_chunk_entity(world: &mut World, chunk_id: ChunkID) -> Entity {
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