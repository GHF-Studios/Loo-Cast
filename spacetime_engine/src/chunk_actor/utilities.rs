use bevy::prelude::*;
use super::components::*;
use super::structs::*;
use crate::chunk::structs::*;
use crate::chunk::components::*;
use crate::chunk::wrappers::ChunkInstanceRegistry;
use crate::entity::structs::EntityPosition;
use crate::core::singletons::MAIN_TYPE_REGISTRY;
use crate::core::structs::InstanceID;

pub(in crate) fn collect_chunk_actor_updates(
    world: &mut World,
) -> (Vec<UpdateChunkActorInfo>, Vec<DespawnChunkActorInfo>) {
    let mut chunk_actor_query = world.query::<(Entity, &Transform, &mut ChunkActor)>();
    let chunk_actor_query_size = chunk_actor_query.iter(world).count();
    let mut chunk_ids = Vec::new();
    let mut chunk_actor_ids = Vec::new();
    let mut chunk_actor_entities = Vec::new();
    let mut old_chunk_ids = Vec::new();

    let chunk_actor_query_infos = chunk_actor_query.iter(world).map(|(chunk_actor_entity, chunk_actor_transform, chunk_actor)| {
        let chunk_actor_entity_position: EntityPosition = chunk_actor_transform.translation.into();
        let chunk_actor_chunk_position: ChunkPosition = chunk_actor_entity_position.into();
        let chunk_actor_id = chunk_actor.id();
        let chunk_actor_current_chunk_id = chunk_actor.current_chunk();

        (chunk_actor_entity, chunk_actor_chunk_position, chunk_actor_id, chunk_actor_current_chunk_id)
    }).collect::<Vec<_>>();

    for (chunk_actor_entity, chunk_actor_chunk_position, chunk_actor_id, chunk_actor_current_chunk_id) in chunk_actor_query_infos {
        let chunk_id: InstanceID<Chunk> = {
            let mut chunks = world.query::<&Chunk>();

            match chunks.iter(world).find(|chunk| chunk.position() == chunk_actor_chunk_position) {
                Some(chunk) => {
                    chunk.id()
                },
                None => {
                    error!("Chunk '{:?}' not found!", chunk_actor_chunk_position);
                    continue;
                }
            }
        };

        chunk_ids.push(chunk_id);
        chunk_actor_ids.push(chunk_actor_id);
        chunk_actor_entities.push(chunk_actor_entity);
        old_chunk_ids.push(chunk_actor_current_chunk_id);
    }

    let mut updates = Vec::new();
    let mut despawns = Vec::new();

    for i in 0..chunk_actor_query_size {
        let chunk_id = chunk_ids[i];
        let chunk_actor_id = chunk_actor_ids[i];
        let chunk_actor_entity = chunk_actor_entities[i];
        let old_chunk_id = old_chunk_ids[i];

        let main_type_registry = MAIN_TYPE_REGISTRY.lock().unwrap();
        let chunk_instance_registry = match main_type_registry.get_data::<Chunk, ChunkInstanceRegistry>() {
            Some(chunk_instance_registry) => chunk_instance_registry,
            None => {
                error!("Chunk instance registry not found!");
                continue;
            }
        };
        
        if !chunk_instance_registry.is_managed(chunk_id) {
            despawns.push(DespawnChunkActorInfo {
                actor_entity: chunk_actor_entity,
                actor_id: chunk_actor_id,
            });

            error!("Chunk actor '{:?}' despawned because chunk '{:?}' is not managed!", chunk_actor_id, chunk_id);
            continue;
        }
        
        if old_chunk_id == chunk_id {
            continue;
        }

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

        info!("Chunk actor '{:?}' moved from chunk '{:?}' to chunk '{:?}'!", chunk_actor_id, old_chunk_id, chunk_id);
    }

    (updates, despawns)
}

pub(in crate) fn apply_chunk_actor_updates(
    world: &mut World,
    updates: Vec<UpdateChunkActorInfo>,
    despawns: Vec<DespawnChunkActorInfo>,
) {
    let mut chunk_query = world.query::<&mut Chunk>();

    for update in updates {
        let main_type_registry = MAIN_TYPE_REGISTRY.lock().unwrap();
        let chunk_instance_registry = match main_type_registry.get_data::<Chunk, ChunkInstanceRegistry>() {
            Some(chunk_instance_registry) => chunk_instance_registry,
            None => {
                error!("Chunk instance registry not found!");
                continue;
            }
        };

        let old_chunk_entity = chunk_instance_registry.get(update.old_chunk_id).unwrap();
        let mut old_chunk = chunk_query.get_mut(world, *old_chunk_entity).unwrap();
        old_chunk.unregister_chunk_actor(update.actor_id);

        let new_chunk_entity = chunk_instance_registry.get(update.new_chunk_id).unwrap();
        let mut new_chunk = chunk_query.get_mut(world, *new_chunk_entity).unwrap();
        new_chunk.register_chunk_actor(update.actor_id);
    }

    for despawn in despawns {
        world.despawn(despawn.actor_entity);
    }
}