pub mod actor;
pub mod position;
pub mod id;
pub mod loader;

pub mod components;
pub mod constants;
pub mod events;
pub(in crate) mod functions;
pub mod resources;
pub(in crate) mod systems;

use actor::*;
use position::*;
use id::*;
use loader::*;
use resources::*;
use systems::*;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct StartExternalOperationSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct StartInternalOperationSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct FinishedInternalOperationSystems;

pub(in crate) struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (
            StartExternalOperationSystems.before(StartInternalOperationSystems),
            StartInternalOperationSystems.before(FinishedInternalOperationSystems),
        ));

        app
            .add_event::<events::CreateChunk>()
            .add_event::<events::CreatedChunk>()
            .add_event::<events::DestroyChunk>()
            .add_event::<events::DestroyedChunk>()
            .add_event::<events::LoadChunk>()
            .add_event::<events::LoadedChunk>()
            .add_event::<events::UnloadChunk>()
            .add_event::<events::UnloadedChunk>()
            .add_event::<events::CreateChunkInternal>()
            .add_event::<events::CreatedChunkInternal>()
            .add_event::<events::DestroyChunkInternal>()
            .add_event::<events::DestroyedChunkInternal>()
            .add_event::<events::LoadChunkInternal>()
            .add_event::<events::LoadedChunkInternal>()
            .add_event::<events::UnloadChunkInternal>()
            .add_event::<events::UnloadedChunkInternal>()
            .add_plugins(ActorPlugin)
            .add_plugins(PositionPlugin)
            .add_plugins(IdPlugin)
            .add_plugins(LoaderPlugin)
            .insert_resource(ChunkRegistry::default())
            .add_systems(Update, (
                handle_create_chunk_events, 
                handle_destroy_chunk_events, 
                handle_load_chunk_events, 
                handle_unload_chunk_events
            ).in_set(StartExternalOperationSystems))
            .add_systems(Update, (
                handle_create_chunk_internal_events,
                handle_destroy_chunk_internal_events,
                handle_load_chunk_internal_events,
                handle_unload_chunk_internal_events
            ).in_set(StartInternalOperationSystems))
            .add_systems(Update, (
                handle_created_chunk_internal_events,
                handle_destroyed_chunk_internal_events,
                handle_loaded_chunk_internal_events,
                handle_unloaded_chunk_internal_events
            ).in_set(FinishedInternalOperationSystems))
            .register_type::<components::Chunk>()
            .register_type::<Vec<actor::id::structs::ChunkActorID>>();
    }
}