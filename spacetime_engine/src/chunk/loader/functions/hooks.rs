use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use crate::chunk::loader::components::ChunkLoader;
use crate::chunk::loader::structs::ChunkLoaderResponse;
use crate::chunk::loader::{ChunkLoaderRegistry, ChunkLoaderRequestRegistry, DowngradedFromChunkLoader, UpgradedToChunkLoader};
use crate::entity::components::SpacetimeEntity;

pub fn on_add_chunk_loader(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let chunk_loader_entity_reference = entity;

    let spacetime_entity_component = match world.get::<SpacetimeEntity>(chunk_loader_entity_reference) {
        Some(spacetime_entity_component) => spacetime_entity_component,
        None => {
            panic!("Failed to get spacetime entity component!");
        }
    };

    let chunk_loader_entity_id = spacetime_entity_component.id;

    let chunk_loader_id = match world.get::<ChunkLoader>(chunk_loader_entity_reference) {
        Some(chunk_loader) => chunk_loader.id(),
        None => {
            panic!("Failed to get chunk loader component associated with entity '{:?}'!", chunk_loader_entity_reference);
        }
    };

    let chunk_loader_request_id = {
        let chunk_loader_request_registry = match world.get_resource_mut::<ChunkLoaderRequestRegistry>() {
            Some(chunk_loader_request_registry) => chunk_loader_request_registry,
            None => {
                panic!("Failed to get chunk loader request registry!");
            }
        };

        match chunk_loader_request_registry
            .loaded_chunk_loader_requests()
            .values()
            .clone()
            .find(|chunk_loader_request| chunk_loader_request.chunk_loader_id == chunk_loader_id)
            .map(|request| {
                request.chunk_loader_request_id
            }) {
            Some(chunk_loader_request_id) => chunk_loader_request_id,
            None => {
                panic!("Failed to get chunk loader request id currently associated with chunk loader entity '{:?}'!", chunk_loader_entity_reference);
            }
        }
    };

    {
        let mut chunk_loader_registry = match world.get_resource_mut::<ChunkLoaderRegistry>() {
            Some(chunk_loader_registry) => chunk_loader_registry,
            None => {
                panic!("Failed to get chunk loader registry!");
            }
        };

        let is_upgrading_to_chunk_loader = chunk_loader_registry.is_chunk_loader_upgrading_to(chunk_loader_id);

        if !is_upgrading_to_chunk_loader {
            panic!("Chunk loader '{:?}' is not upgrading!", chunk_loader_id);
        }

        chunk_loader_registry.load_chunk_loader(chunk_loader_id, chunk_loader_entity_reference);
        chunk_loader_registry.stop_upgrading_to_chunk_loader(chunk_loader_id);

        let mut chunk_loader_request_registry = match world.get_resource_mut::<ChunkLoaderRequestRegistry>() {
            Some(chunk_loader_request_registry) => chunk_loader_request_registry,
            None => {
                panic!("Failed to get chunk loader request registry!");
            }
        };

        chunk_loader_request_registry.unload_chunk_loader_request(chunk_loader_request_id);

        world.send_event(UpgradedToChunkLoader(ChunkLoaderResponse::Success {
            chunk_loader_request_id,
            chunk_loader_id,
            chunk_loader_entity_id,
        }));
    }
}

pub fn on_remove_chunk_loader(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let chunk_loader_entity_reference = entity;

    let spacetime_entity_component = match world.get::<SpacetimeEntity>(chunk_loader_entity_reference) {
        Some(spacetime_entity_component) => spacetime_entity_component,
        None => {
            panic!("Failed to get spacetime entity component!");
        }
    };

    let chunk_loader_entity_id = spacetime_entity_component.id;

    let chunk_loader_id = match world.get::<ChunkLoader>(chunk_loader_entity_reference) {
        Some(chunk_loader) => chunk_loader.id(),
        None => {
            panic!("Failed to get chunk loader component associated with entity '{:?}'!", chunk_loader_entity_reference);
        }
    };

    let chunk_loader_request_id = {
        let chunk_loader_request_registry = match world.get_resource_mut::<ChunkLoaderRequestRegistry>() {
            Some(chunk_loader_request_registry) => chunk_loader_request_registry,
            None => {
                panic!("Failed to get chunk loader request registry!");
            }
        };

        match chunk_loader_request_registry
            .loaded_chunk_loader_requests()
            .values()
            .clone()
            .find(|chunk_loader_request| chunk_loader_request.chunk_loader_id == chunk_loader_id)
            .map(|request| {
                request.chunk_loader_request_id
            }) {
            Some(chunk_loader_request_id) => chunk_loader_request_id,
            None => {
                panic!("Failed to get chunk loader request id currently associated with chunk loader entity '{:?}'!", chunk_loader_entity_reference);
            }
        }
    };

    {
        let mut chunk_loader_registry = match world.get_resource_mut::<ChunkLoaderRegistry>() {
            Some(chunk_loader_registry) => chunk_loader_registry,
            None => {
                panic!("Failed to get chunk loader registry!");
            }
        };

        let is_downgrading_from_chunk_loader = chunk_loader_registry.is_chunk_loader_downgrading_from(chunk_loader_id);

        if !is_downgrading_from_chunk_loader {
            panic!("Chunk loader '{:?}' is not downgrading!", chunk_loader_id);
        }

        chunk_loader_registry.save_chunk_loader(chunk_loader_id);
        chunk_loader_registry.stop_downgrading_from_chunk_loader(chunk_loader_id);

        let mut chunk_loader_request_registry = match world.get_resource_mut::<ChunkLoaderRequestRegistry>() {
            Some(chunk_loader_request_registry) => chunk_loader_request_registry,
            None => {
                panic!("Failed to get chunk loader request registry!");
            }
        };

        chunk_loader_request_registry.unload_chunk_loader_request(chunk_loader_request_id);

        world.send_event(DowngradedFromChunkLoader(ChunkLoaderResponse::Success {
            chunk_loader_request_id,
            chunk_loader_id,
            chunk_loader_entity_id,
        }));
    }
}