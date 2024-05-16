use bevy::prelude::*;
use crate::chunk::id::structs::*;
use crate::chunk::coordinate::structs::*;
use crate::chunk::actor::coordinate::structs::*;
use crate::chunk::actor::components::*;
use crate::chunk::actor::resources::*;
use crate::chunk::actor::structs::*;
use crate::chunk::components::*;
use crate::chunk::resources::*;


pub(in crate) fn update(
    commands: Commands,
    chunk_query: Query<(Entity, &Transform, &ChunkActor)>,
    mut chunk_component_query: Query<&mut Chunk>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
) {
    let (updates, despawns) = collect_actor_updates(&chunk_query, &mut chunk_registry);

    apply_actor_updates(
        commands,
        &mut chunk_component_query,
        updates,
        despawns,
        &mut chunk_registry,
        &mut chunk_actor_registry,
    );
}

fn collect_actor_updates(
    chunk_actor_query: &Query<(Entity, &Transform, &ChunkActor)>,
    chunk_registry: &mut ResMut<ChunkRegistry>,
) -> (Vec<ActorUpdateInfo>, Vec<ActorDespawnInfo>) {
    let mut updates = Vec::new();
    let mut despawns = Vec::new();

    for (entity, transform, chunk_actor) in chunk_actor_query.iter() {
        let actor_coordinate: ChunkActorCoordinate = transform.translation.into();
        let chunk_coordinate: ChunkCoordinate = actor_coordinate.into();
        let chunk_id: ChunkID = chunk_coordinate.into();

        if !chunk_registry.is_chunk_loaded(chunk_id) {
            despawns.push(ActorDespawnInfo {
                actor_entity: entity,
                actor_id: chunk_actor.id(),
            });
        } else {
            let old_chunk_id = chunk_actor.current_chunk();
            if old_chunk_id != chunk_id {
                updates.push(ActorUpdateInfo {
                    actor_entity: entity,
                    old_chunk_id,
                    new_chunk_id: chunk_id,
                    actor_id: chunk_actor.id(),
                });
            }
        }
    }
    (updates, despawns)
}

fn apply_actor_updates(
    mut commands: Commands,
    chunk_query: &mut Query<&mut Chunk>,
    updates: Vec<ActorUpdateInfo>,
    despawns: Vec<ActorDespawnInfo>,
    chunk_registry: &mut ResMut<ChunkRegistry>,
    chunk_actor_registry: &mut ResMut<ChunkActorRegistry>,
) {
    for update in updates {
        let old_chunk_entity = chunk_registry.get_loaded_chunk_entity(update.old_chunk_id).unwrap();
        let mut old_chunk = chunk_query.get_mut(old_chunk_entity).unwrap();
        old_chunk.remove_chunk_actor(update.actor_id);

        let new_chunk_entity = chunk_registry.get_loaded_chunk_entity(update.new_chunk_id).unwrap();
        let mut new_chunk = chunk_query.get_mut(new_chunk_entity).unwrap();
        new_chunk.add_chunk_actor(update.actor_id);
    }

    for despawn in despawns {
        commands.entity(despawn.actor_entity).despawn_recursive();
        chunk_actor_registry.unload_chunk_actor(despawn.actor_id);
        chunk_actor_registry.unregister_chunk_actor(despawn.actor_id);
    }
}