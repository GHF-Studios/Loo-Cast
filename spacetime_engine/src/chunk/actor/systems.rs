use bevy::prelude::*;
use crate::chunk::id::structs::*;
use crate::chunk::coordinate::structs::*;
use crate::chunk::actor::coordinate::structs::*;
use crate::chunk::actor::components::*;
use crate::chunk::actor::resources::*;
use crate::chunk::components::*;
use crate::chunk::resources::*;

pub(in crate) fn update(
    mut commands: Commands,
    mut chunk_query: Query<(Entity, &Transform, &mut Chunk)>,
    mut chunk_actor_query: Query<(Entity, &Transform, &mut ChunkActor)>,
    chunk_registry: Res<ChunkRegistry>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
) {
    for (chunk_actor_entity, chunk_actor_transform, mut chunk_actor) in chunk_actor_query.iter_mut() {
        let chunk_chunk_actor_coordinate: ChunkActorCoordinate = chunk_actor_transform.translation.into();
        let chunk_coordinate: ChunkCoordinate = chunk_chunk_actor_coordinate.into();
        let chunk_id: ChunkID = chunk_coordinate.into();

        if !chunk_registry.is_chunk_loaded(chunk_id) {
            chunk_actor_registry.unload_chunk_actor(chunk_actor.id());

            chunk_actor_registry.unregister_chunk_actor(chunk_actor.id());

            if let Some(chunk_actor_entity) = commands.get_entity(chunk_actor_entity) {
                chunk_actor_entity.despawn_recursive();
            }

            continue;
        }

        let old_chunk_id = chunk_actor.current_chunk();
        let new_chunk_id = chunk_id;

        if new_chunk_id != old_chunk_id {
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
