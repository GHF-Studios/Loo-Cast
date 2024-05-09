use bevy::prelude::*;
use crate::chunk::id::structs::*;
use crate::chunk::coordinate::structs::*;
use crate::chunk::actor::coordinate::structs::*;
use crate::chunk::actor::components::*;
use crate::chunk::actor::events::*;
use crate::chunk::resources::*;

pub(in crate) fn update(
    mut commands: Commands,
    mut chunk_actor_query: Query<(Entity, &Transform, &mut ChunkActor)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for (chunk_actor_entity, chunk_actor_transform, mut chunk_actor) in chunk_actor_query.iter_mut() {
        let chunk_chunk_actor_coordinate: ChunkActorCoordinate = chunk_actor_transform.translation.into();
        let chunk_coordinate: ChunkCoordinate = chunk_chunk_actor_coordinate.into();
        let chunk_id: ChunkID = chunk_coordinate.into();

        if !chunk_manager.loaded_chunks.contains_key(&chunk_id) {
            chunk_manager.recycle_chunk_actor_id(chunk_actor.id);
            commands.entity(chunk_actor_entity).despawn_recursive();
            continue;
        }

        if chunk_id != chunk_actor.current_chunk {
            chunk_actor.current_chunk = chunk_id;
        }
    }
}

pub(in crate) fn handle_create_events(
    mut commands: Commands,
    mut create_chunk_actor_event_reader: EventReader<CreateChunkActor>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    // TODO: Implement
}

pub(in crate) fn handle_destroy_events(
    mut commands: Commands,
    mut destroy_chunk_actor_event_reader: EventReader<DestroyChunkActor>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    // TODO: Implement
}

pub(in crate) fn handle_load_events(
    mut commands: Commands,
    mut load_chunk_actor_event_reader: EventReader<LoadChunkActor>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    // TODO: Implement
}

pub(in crate) fn handle_unload_events(
    mut commands: Commands,
    mut unload_chunk_actor_event_reader: EventReader<UnloadChunkActor>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    // TODO: Implement
}

