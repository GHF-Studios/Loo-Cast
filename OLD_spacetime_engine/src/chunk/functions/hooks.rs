use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use crate::chunk::components::*;
use crate::chunk::constants::*;
use crate::chunk::position::structs::ChunkPosition as ChunkPosition;
use crate::chunk::actor::position::structs::ChunkActorPosition as ChunkActorPosition;
use crate::chunk::events::*;
use crate::entity::components::SpacetimeEntity;
use crate::chunk::structs::ChunkResponse;
use crate::chunk::{ChunkRequestRegistry, ChunkRegistry};

pub fn on_add_chunk(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let chunk_entity_reference = entity;

    let spacetime_entity_component = match world.get::<SpacetimeEntity>(chunk_entity_reference) {
        Some(spacetime_entity_component) => spacetime_entity_component,
        None => {
            panic!("Failed to get spacetime entity component!");
        }
    };

    let chunk_entity_id = spacetime_entity_component.id;

    let chunk_id = match world.get::<Chunk>(chunk_entity_reference) {
        Some(chunk) => chunk.id(),
        None => {
            panic!("Failed to get chunk component associated with entity '{:?}'!", chunk_entity_reference);
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
                panic!("Failed to get chunk request id currently associated with chunk entity '{:?}'!", chunk_entity_reference);
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
            chunk_registry.load_chunk(chunk_id, chunk_entity_reference);
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
                chunk_entity_id,
            }));
        } else if is_chunk_loading {
            chunk_registry.load_chunk(chunk_id, chunk_entity_reference);
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
                chunk_entity_id,
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

        world.commands().entity(chunk_entity_reference).insert(
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

pub fn on_remove_chunk(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let chunk_entity_reference = entity;

    let spacetime_entity_component = match world.get::<SpacetimeEntity>(chunk_entity_reference) {
        Some(spacetime_entity_component) => spacetime_entity_component,
        None => {
            panic!("Failed to get spacetime entity component!");
        }
    };

    let chunk_entity_id = spacetime_entity_component.id;

    let chunk_id = match world.get::<Chunk>(chunk_entity_reference) {
        Some(chunk) => chunk.id(),
        None => {
            panic!("Failed to get chunk component associated with entity '{:?}'!", chunk_entity_reference);
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
                panic!("Failed to get chunk request id currently associated with chunk entity '{:?}'!", chunk_entity_reference);
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
            chunk_registry.save_chunk(chunk_id);
            chunk_registry.unregister_chunk(chunk_id);
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
                chunk_entity_id,
            }));
        } else if is_saving_chunk {
            chunk_registry.save_chunk(chunk_id);
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
                chunk_entity_id,
            }));
        } else {
            panic!("Chunk '{:?}' is neither downgrading nor saving!", chunk_id);
        }
    }
}