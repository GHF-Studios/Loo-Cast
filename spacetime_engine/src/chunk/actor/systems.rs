use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::id::structs::*;
use crate::chunk::position::structs::*;
use crate::chunk::actor::position::structs::*;
use crate::chunk::actor::components::*;
use crate::chunk::actor::resources::*;
use crate::chunk::actor::structs::*;
use crate::chunk::components::*;
use crate::chunk::resources::*;


pub(in crate) fn update(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
) {
    let (updates, despawns) = collect_actor_updates(world, registry_parameters);

    apply_actor_updates(
        world,
        registry_parameters,
        updates,
        despawns,
    );
}

fn collect_actor_updates(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
) -> (Vec<ActorUpdateInfo>, Vec<ActorDespawnInfo>) {
    let mut chunk_actor_query = world.query::<(Entity, &Transform, &ChunkActor)>();
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
            despawns.push(ActorDespawnInfo {
                actor_entity: chunk_actor_entity,
                actor_id: chunk_actor_id,
            });
        } else if old_chunk_id != chunk_id {
            updates.push(ActorUpdateInfo {
                actor_entity: chunk_actor_entity,
                old_chunk_id,
                new_chunk_id: chunk_id,
                actor_id: chunk_actor_id,
            });
        }
    }

    (updates, despawns)
}

fn apply_actor_updates(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
    updates: Vec<ActorUpdateInfo>,
    despawns: Vec<ActorDespawnInfo>,
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