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
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    for (chunk_actor_entity, chunk_actor_transform, mut chunk_actor) in chunk_actor_query.iter_mut() {
        let chunk_chunk_actor_coordinate: ChunkActorCoordinate = chunk_actor_transform.translation.into();
        let chunk_coordinate: ChunkCoordinate = chunk_chunk_actor_coordinate.into();
        let chunk_id: ChunkID = chunk_coordinate.into();

        if !chunk_registry.is_chunk_loaded(chunk_id) {
            // TODO: Local: unload and unregister chunk actor and unload, unregister, and despawn the chunk actor entity via the entity registry and associated lifecycle events
            
            // TODO: Global: Implement entity lifecycle events like I did for chunks, to handle this
            // TODO: Global: Implement chunk actor lifecycle events to handle this
            // TODO: Global: Move chunk actor related stuff from chunk registry to a new chunk actor registry

            // I feel that generally we need to embrace events more in our code, with a temporary focus on chunks/entities and their sub-modules
            // Also I should inspect every location closely where I interact with chunks, it's sub-modules (like actors and loaders), and entities to ensure that I'm not missing anything

            // OLD CODE
            chunk_registry.recycle_chunk_actor_id(chunk_actor.id);
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
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    // TODO: Implement
}

pub(in crate) fn handle_destroy_events(
    mut commands: Commands,
    mut destroy_chunk_actor_event_reader: EventReader<DestroyChunkActor>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    // TODO: Implement
}

pub(in crate) fn handle_load_events(
    mut commands: Commands,
    mut load_chunk_actor_event_reader: EventReader<LoadChunkActor>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    // TODO: Implement
}

pub(in crate) fn handle_unload_events(
    mut commands: Commands,
    mut unload_chunk_actor_event_reader: EventReader<UnloadChunkActor>,
    mut chunk_registry: ResMut<ChunkRegistry>,
) {
    // TODO: Implement
}

