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
        app
            .add_event::<events::CreateChunkEntity>()
            .add_event::<events::CreatedChunkEntity>()
            .add_event::<events::DestroyChunkEntity>()
            .add_event::<events::DestroyedChunkEntity>()
            .add_event::<events::LoadChunkEntity>()
            .add_event::<events::LoadedChunkEntity>()
            .add_event::<events::UnloadChunkEntity>()
            .add_event::<events::UnloadedChunkEntity>()
            .add_event::<events::CreateChunkEntityInternal>()
            .add_event::<events::CreatedChunkEntityInternal>()
            .add_event::<events::DestroyChunkEntityInternal>()
            .add_event::<events::DestroyedChunkEntityInternal>()
            .add_event::<events::LoadChunkEntityInternal>()
            .add_event::<events::LoadedChunkEntityInternal>()
            .add_event::<events::UnloadChunkEntityInternal>()
            .add_event::<events::UnloadedChunkEntityInternal>()
            .add_plugins(ActorPlugin)
            .add_plugins(PositionPlugin)
            .add_plugins(IdPlugin)
            .add_plugins(LoaderPlugin)
            .insert_resource(ChunkRegistry::default())
            .insert_resource(ChunkEventRegistry::default())
            .configure_sets(Update, (
                StartExternalOperationSystems.before(StartInternalOperationSystems),
                StartInternalOperationSystems.before(FinishedInternalOperationSystems),
            ))
            .add_systems(Update, (
                handle_create_chunk_entity_events, 
                handle_destroy_chunk_entity_events, 
                handle_load_chunk_entity_events, 
                handle_unload_chunk_entity_events
            ).in_set(StartExternalOperationSystems))
            .add_systems(Update, (
                handle_create_chunk_entity_internal_events,
                handle_destroy_chunk_entity_internal_events,
                handle_load_chunk_entity_internal_events,
                handle_unload_chunk_entity_internal_events
            ).in_set(StartInternalOperationSystems))
            .add_systems(Update, (
                handle_created_chunk_entity_internal_events,
                handle_destroyed_chunk_entity_internal_events,
                handle_loaded_chunk_entity_internal_events,
                handle_unloaded_chunk_entity_internal_events
            ).in_set(FinishedInternalOperationSystems))
            .register_type::<components::Chunk>()
            .register_type::<Vec<actor::id::structs::ChunkActorID>>();
    }
}