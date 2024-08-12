use bevy::ecs::system::SystemState;
use bevy::prelude::*;

use crate::chunk::actor::components::ChunkActor;
use crate::chunk::actor::id::structs::ChunkActorID;
use crate::chunk::actor::position::structs::ChunkActorPosition;
use crate::chunk::actor::structs::{DespawnChunkActorInfo, UpdateChunkActorInfo};
use crate::chunk::actor::ChunkActorRegistry;
use crate::chunk::components::Chunk;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::position::structs::ChunkPosition;
use crate::chunk::ChunkRegistry;

pub(in crate) fn upgrade_to_chunk_actor(
    world: &mut World,
    start_chunk_id: ChunkID,
    chunk_actor_id: ChunkActorID,
    entity_reference: Entity,
) {
    world.entity_mut(entity_reference).insert(ChunkActor::new(chunk_actor_id, start_chunk_id));
}

pub(in crate) fn downgrade_from_chunk_actor(
    world: &mut World,
    entity_reference: Entity,
) {
    world.entity_mut(entity_reference).remove::<ChunkActor>();
}

pub(in crate) fn collect_chunk_actor_updates(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
) -> (Vec<UpdateChunkActorInfo>, Vec<DespawnChunkActorInfo>) {
    let mut chunk_actor_query = world.query::<(Entity, &Transform, &mut ChunkActor)>();
    let chunk_actor_query_size = chunk_actor_query.iter(world).count();
    let mut chunk_ids = Vec::new();
    let mut chunk_actor_ids = Vec::new();
    let mut chunk_actor_entities = Vec::new();
    let mut old_chunk_ids = Vec::new();

    for (chunk_actor_entity, chunk_actor_transform, chunk_actor) in chunk_actor_query.iter(world) {
        let actor_position: ChunkActorPosition = chunk_actor_transform.translation.into();
        let chunk_position: ChunkPosition = actor_position.into();
        let chunk_id: ChunkID = chunk_position.into();
        let chunk_actor_id = chunk_actor.id();
        let old_chunk_id = chunk_actor.current_chunk();

        chunk_ids.push(chunk_id);
        chunk_actor_ids.push(chunk_actor_id);
        chunk_actor_entities.push(chunk_actor_entity);
        old_chunk_ids.push(old_chunk_id);
    }

    let mut updates = Vec::new();
    let mut despawns = Vec::new();

    for i in 0..chunk_actor_query_size {
        let chunk_id = chunk_ids[i];
        let chunk_actor_id = chunk_actor_ids[i];
        let chunk_actor_entity = chunk_actor_entities[i];
        let old_chunk_id = old_chunk_ids[i];

        let (chunk_registry, _) = registry_parameters.get_mut(world);
        
        if !chunk_registry.is_chunk_loaded(chunk_id) {
            warn!("Chunk actor '{:?}' despawned because chunk '{:?}' is not loaded!", chunk_actor_id, chunk_id);

            despawns.push(DespawnChunkActorInfo {
                actor_entity: chunk_actor_entity,
                actor_id: chunk_actor_id,
            });
        } else if old_chunk_id != chunk_id {
            info!("Chunk actor '{:?}' moved from chunk '{:?}' to chunk '{:?}'!", chunk_actor_id, old_chunk_id, chunk_id);

            let mut chunk_actor = match chunk_actor_query.iter_mut(world).find(|(_, _, chunk_actor)| chunk_actor.id() == chunk_actor_id) {
                Some((_, _, chunk_actor)) => chunk_actor,
                None => {
                    error!("Chunk actor '{:?}' not found in query!", chunk_actor_id);
                    continue;
                }
            };

            *chunk_actor.current_chunk_mut() = chunk_id;

            updates.push(UpdateChunkActorInfo {
                actor_entity: chunk_actor_entity,
                old_chunk_id,
                new_chunk_id: chunk_id,
                actor_id: chunk_actor_id,
            });
        }
    }

    (updates, despawns)
}

pub(in crate) fn apply_chunk_actor_updates(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
    updates: Vec<UpdateChunkActorInfo>,
    despawns: Vec<DespawnChunkActorInfo>,
) {
    let mut chunk_query = world.query::<&mut Chunk>();

    for update in updates {
        let (chunk_registry, _) = registry_parameters.get_mut(world);
        let old_chunk_entity = chunk_registry.get_loaded_chunk_entity(update.old_chunk_id).unwrap();
        let mut old_chunk = chunk_query.get_mut(world, old_chunk_entity).unwrap();
        old_chunk.remove_chunk_actor(update.actor_id);

        let (chunk_registry, _) = registry_parameters.get_mut(world);
        let new_chunk_entity = chunk_registry.get_loaded_chunk_entity(update.new_chunk_id).unwrap();
        let mut new_chunk = chunk_query.get_mut(world, new_chunk_entity).unwrap();
        new_chunk.add_chunk_actor(update.actor_id);
    }

    for despawn in despawns {
        world.despawn(despawn.actor_entity);
    }
}