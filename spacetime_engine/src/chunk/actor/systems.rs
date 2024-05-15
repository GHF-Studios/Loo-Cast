use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk;
use crate::chunk::id::structs::*;
use crate::chunk::coordinate::structs::*;
use crate::chunk::actor::coordinate::structs::*;
use crate::chunk::actor::components::*;
use crate::chunk::actor::resources::*;
use crate::chunk::components::*;
use crate::chunk::resources::*;

use super::structs::ChunkActorID;

pub(in crate) fn update(
    world: &mut World,
    query_parameters: &mut SystemState<(
        Query<(Entity, &Transform, &mut Chunk)>,
        Query<(Entity, &Transform, &mut ChunkActor)>
    )>,
    registry_parameters: &mut SystemState<(
        Res<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
) {
    let (mut chunk_query, mut chunk_actor_query) = query_parameters.get_mut(world);

    let chunk_actor_query_size: usize = chunk_actor_query.iter().count(); 
    let mut chunk_actor_entities = Vec::with_capacity(chunk_actor_query_size);
    let mut chunk_actor_world_positions = Vec::with_capacity(chunk_actor_query_size);
    let mut chunk_actor_ids = Vec::with_capacity(chunk_actor_query_size);
    let mut chunk_actors = Vec::with_capacity(chunk_actor_query_size);
    let mut chunk_actor_chunk_id = Vec::with_capacity(chunk_actor_query_size);

    let mut loop_query_iter_mut = chunk_actor_query.iter_mut();
    for i in 0..chunk_actor_query_size {
        let chunk_actor_query_element = loop_query_iter_mut.nth(i).unwrap();

        let chunk_actor_entity = chunk_actor_query_element.0;
        let chunk_actor_world_pos = chunk_actor_query_element.1.translation;
        let chunk_actor_id = chunk_actor_query_element.2.id();
        let chunk_actor = chunk_actor_query_element.2;
        let chunk_id: ChunkID = {
            let chunk_chunk_actor_coordinate: ChunkActorCoordinate = chunk_actor_world_pos.into();
            let chunk_coordinate: ChunkCoordinate = chunk_chunk_actor_coordinate.into();

            chunk_coordinate.into()
        };

        chunk_actor_entities[i] = chunk_actor_entity;
        chunk_actor_world_positions[i] = chunk_actor_world_pos;
        chunk_actor_ids[i] = chunk_actor_id;
        chunk_actors[i] = chunk_actor;
        chunk_actor_chunk_id[i] = chunk_id;
    }

    let chunk_query_size: usize = chunk_query.iter().count();
    let mut chunks = Vec::with_capacity(chunk_query_size);

    let mut loop_query_iter_mut = chunk_query.iter_mut();
    for i in 0..chunk_query_size {
        let chunk_query_element = loop_query_iter_mut.nth(i).unwrap();

        let chunk = chunk_query_element.2;

        chunks[i] = chunk;
    }

    let chunk_registry = registry_parameters.get_mut(world).0;
    for i in 0..chunk_actor_query_size {
        let chunk_actor_entity = chunk_actor_entities[i];
        let chunk_actor_id = chunk_actor_ids[i];
        let chunk_id = chunk_actor_chunk_id[i];


        if !chunk_registry.is_chunk_loaded(chunk_id) {
            let mut chunk_actor_registry = registry_parameters.get_mut(world).1;

            chunk_actor_registry.unload_chunk_actor(chunk_actor_id);

            chunk_actor_registry.unregister_chunk_actor(chunk_actor_id);

            let chunk_actor_entity = match world.get_entity(chunk_actor_entity) {
                Some(chunk_actor_entity) => chunk_actor_entity,
                None => {
                    continue;
                }
            }.id();

            world.despawn(chunk_actor_entity);

            continue;
        }
    }

    for i in 0..chunk_actor_query_size {
        let chunk_actor = chunk_actors[i];
        let chunk_id = chunk_actor_chunk_id[i];
    
        let old_chunk_id = chunk_actor.current_chunk();
        let new_chunk_id = chunk_id;
    
        if new_chunk_id != old_chunk_id {
            let mut chunk_query = query_parameters.get_mut(world).0;
            let chunk_registry = registry_parameters.get_mut(world).0;
    
            let old_chunk_entity = chunk_registry.get_loaded_chunk_entity(old_chunk_id).unwrap();
            let mut old_chunk = chunk_query.get_mut(old_chunk_entity).unwrap().2;
            old_chunk.remove_chunk_actor(chunk_actor.id());
    
            let chunk_actor_current_chunk = chunk_actor.current_chunk_mut();
            *chunk_actor_current_chunk = new_chunk_id;
    
            let new_chunk_entity = chunk_registry.get_loaded_chunk_entity(new_chunk_id).unwrap();
            let mut new_chunk = chunk_query.get_mut(new_chunk_entity).unwrap().2;
            new_chunk.add_chunk_actor(chunk_actor.id());
        }
    }
}
