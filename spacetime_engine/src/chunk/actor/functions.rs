use bevy::ecs::system::SystemState;
use bevy::prelude::*;

use crate::chunk::{components::Chunk, id::structs::ChunkID};
use crate::chunk::position::structs::ChunkPosition;
use crate::chunk::ChunkRegistry;
use super::components::ChunkActor;
use super::constants::CHUNK_ACTOR_Z_INDEX;
use super::{id::structs::ChunkActorID, position::structs::ChunkActorPosition, structs::{DespawnChunkActorInfo, UpdateChunkActorInfo}, ChunkActorRegistry};

pub(super) fn new_chunk_actor_entity(
    world: &mut World,
    chunk_actor_id: ChunkActorID,
    chunk_id: ChunkID,
    world_position: Vec2,
) -> Entity {
    let new_chunk_actor_entity = world
    .spawn((
        Transform::from_translation(world_position.extend(CHUNK_ACTOR_Z_INDEX)),
        ChunkActor::new(chunk_actor_id, chunk_id)
    ))
    .id();

    new_chunk_actor_entity
}

pub(super) fn upgrade_to_chunk_actor_entity(
    world: &mut World,
    chunk_actor_id: ChunkActorID,
    chunk_id: ChunkID,
    target_entity_reference: Entity,
    ineligible_entity_query_0: &mut Query<Entity, Without<Transform>>,
    ineligible_entity_query_1: &mut Query<Entity, With<ChunkActor>>,
    eligible_entity_query: &mut Query<Entity, (With<Transform>, Without<ChunkActor>)>,
) -> Result<Entity, Entity> {
    if let Ok(_) = ineligible_entity_query_0.get(target_entity_reference) {
        error!("Entity '{:?}' does not have a Transform component!", target_entity_reference);

        return Err(target_entity_reference);
    };

    if let Ok(_) = ineligible_entity_query_1.get(target_entity_reference) {
        error!("Entity '{:?}' already has a ChunkActor component!", target_entity_reference);

        return Err(target_entity_reference);
    };

    if let Ok(eligible_entity) = eligible_entity_query.get_mut(target_entity_reference) {
        return Ok(world.entity_mut(eligible_entity).insert(ChunkActor::new(chunk_actor_id, chunk_id)).id());
    } else {
        error!("Entity does not exist or does not have a Transform component.");

        return Err(target_entity_reference);
    };
}

pub(super) fn collect_chunk_actor_updates(
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

pub(super) fn apply_chunk_actor_updates(
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
        let (_, mut chunk_actor_registry) = registry_parameters.get_mut(world);
        chunk_actor_registry.unload_chunk_actor(despawn.actor_id);
        chunk_actor_registry.unregister_chunk_actor(despawn.actor_id);
    }
}
