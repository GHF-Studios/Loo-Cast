use bevy::prelude::*;
use crate::chunk::id::structs::*;
use crate::chunk::coordinate::structs::*;
use crate::chunk::actor::coordinate::structs::*;
use crate::chunk::actor::components::*;
use crate::chunk::actor::events::*;
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

pub(in crate) fn handle_create_events(
    mut commands: Commands,
    mut create_chunk_actor_event_reader: EventReader<CreateChunkActor>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
) {
    // TODO: Implement
}

pub(in crate) fn handle_destroy_events(
    mut commands: Commands,
    mut destroy_chunk_actor_event_reader: EventReader<DestroyChunkActor>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
) {
    // TODO: Implement
}

pub(in crate) fn handle_load_events(
    mut commands: Commands,
    mut load_chunk_actor_event_reader: EventReader<LoadChunkActor>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
) {
    // TODO: Implement
}

pub(in crate) fn handle_unload_events(
    mut commands: Commands,
    mut unload_chunk_actor_event_reader: EventReader<UnloadChunkActor>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
) {
    // TODO: Implement
}

