use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

use crate::chunk::actor::components::ChunkActor;
use crate::chunk::actor::structs::ChunkActorResponse;
use crate::chunk::actor::{ChunkActorRegistry, ChunkActorRequestRegistry, DowngradedFromChunkActor, UpgradedToChunkActor};
use crate::entity::components::SpacetimeEntity;

pub fn on_add_chunk_actor(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let chunk_actor_entity_reference = entity;

    let spacetime_entity_component = match world.get::<SpacetimeEntity>(chunk_actor_entity_reference) {
        Some(spacetime_entity_component) => spacetime_entity_component,
        None => {
            panic!("Failed to get spacetime entity component!");
        }
    };

    let chunk_actor_entity_id = spacetime_entity_component.id;

    let chunk_actor_id = match world.get::<ChunkActor>(chunk_actor_entity_reference) {
        Some(chunk_actor) => chunk_actor.id(),
        None => {
            panic!("Failed to get chunk actor component associated with entity '{:?}'!", chunk_actor_entity_reference);
        }
    };

    let chunk_actor_request_id = {
        let chunk_actor_request_registry = match world.get_resource_mut::<ChunkActorRequestRegistry>() {
            Some(chunk_actor_request_registry) => chunk_actor_request_registry,
            None => {
                panic!("Failed to get chunk actor request registry!");
            }
        };

        match chunk_actor_request_registry
            .loaded_chunk_actor_requests()
            .values()
            .clone()
            .find(|chunk_actor_request| chunk_actor_request.chunk_actor_id == chunk_actor_id)
            .map(|request| {
                request.chunk_actor_request_id
            }) {
            Some(chunk_actor_request_id) => chunk_actor_request_id,
            None => {
                panic!("Failed to get chunk actor request id currently associated with chunk actor entity '{:?}'!", chunk_actor_entity_reference);
            }
        }
    };

    {
        let mut chunk_actor_registry = match world.get_resource_mut::<ChunkActorRegistry>() {
            Some(chunk_actor_registry) => chunk_actor_registry,
            None => {
                panic!("Failed to get chunk actor registry!");
            }
        };

        let is_upgrading_to_chunk_actor = chunk_actor_registry.is_chunk_actor_upgrading_to(chunk_actor_id);
        if !is_upgrading_to_chunk_actor {
            panic!("Chunk actor '{:?}' is not upgrading!", chunk_actor_id);
        }
        
        chunk_actor_registry.load_chunk_actor(chunk_actor_id, chunk_actor_entity_reference);
        chunk_actor_registry.stop_upgrading_to_chunk_actor(chunk_actor_id);

        let mut chunk_actor_request_registry = match world.get_resource_mut::<ChunkActorRequestRegistry>() {
            Some(chunk_actor_request_registry) => chunk_actor_request_registry,
            None => {
                panic!("Failed to get chunk actor request registry!");
            }
        };

        chunk_actor_request_registry.unload_chunk_actor_request(chunk_actor_request_id);

        world.send_event(UpgradedToChunkActor(ChunkActorResponse::Success {
            chunk_actor_request_id,
            chunk_actor_id,
            chunk_actor_entity_id,
        }));
    }
}

pub fn on_remove_chunk_actor(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let chunk_actor_entity_reference = entity;

    let spacetime_entity_component = match world.get::<SpacetimeEntity>(chunk_actor_entity_reference) {
        Some(spacetime_entity_component) => spacetime_entity_component,
        None => {
            panic!("Failed to get spacetime entity component!");
        }
    };

    let chunk_actor_entity_id = spacetime_entity_component.id;

    let chunk_actor_id = match world.get::<ChunkActor>(chunk_actor_entity_reference) {
        Some(chunk_actor) => chunk_actor.id(),
        None => {
            panic!("Failed to get chunk actor component associated with entity '{:?}'!", chunk_actor_entity_reference);
        }
    };

    let chunk_actor_request_id = {
        let chunk_actor_request_registry = match world.get_resource_mut::<ChunkActorRequestRegistry>() {
            Some(chunk_actor_request_registry) => chunk_actor_request_registry,
            None => {
                panic!("Failed to get chunk actor request registry!");
            }
        };

        match chunk_actor_request_registry
            .loaded_chunk_actor_requests()
            .values()
            .clone()
            .find(|chunk_actor_request| chunk_actor_request.chunk_actor_id == chunk_actor_id)
            .map(|request| {
                request.chunk_actor_request_id
            }) {
            Some(chunk_actor_request_id) => chunk_actor_request_id,
            None => {
                panic!("Failed to get chunk actor request id currently associated with chunk actor entity '{:?}'!", chunk_actor_entity_reference);
            }
        }
    };

    {
        let mut chunk_actor_registry = match world.get_resource_mut::<ChunkActorRegistry>() {
            Some(chunk_actor_registry) => chunk_actor_registry,
            None => {
                panic!("Failed to get chunk actor registry!");
            }
        };

        let is_downgrading_to_chunk_actor = chunk_actor_registry.is_chunk_actor_downgrading_from(chunk_actor_id);
        if !is_downgrading_to_chunk_actor {
            panic!("Chunk actor '{:?}' is not downgrading!", chunk_actor_id);
        } 

        chunk_actor_registry.save_chunk_actor(chunk_actor_id);
        chunk_actor_registry.stop_downgrading_from_chunk_actor(chunk_actor_id);

        let mut chunk_actor_request_registry = match world.get_resource_mut::<ChunkActorRequestRegistry>() {
            Some(chunk_actor_request_registry) => chunk_actor_request_registry,
            None => {
                panic!("Failed to get chunk actor request registry!");
            }
        };

        chunk_actor_request_registry.unload_chunk_actor_request(chunk_actor_request_id);

        world.send_event(DowngradedFromChunkActor(ChunkActorResponse::Success {
            chunk_actor_request_id,
            chunk_actor_id,
            chunk_actor_entity_id,
        }));
    }
}