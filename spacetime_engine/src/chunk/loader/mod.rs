pub mod components;
pub mod constants;
pub mod events;
pub mod functions;
pub(in crate) mod systems;
pub mod resources;
pub mod structs;

pub mod id;

use events::*;
use resources::*;
use systems::*;
use id::*;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct StartExternalOperationSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct StartInternalOperationSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct FinishedInternalOperationSystems;

pub(in crate) struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CreateChunkLoaderEntity>()
            .add_event::<DestroyChunkLoaderEntity>()
            .add_event::<UpgradeToChunkLoaderEntity>()
            .add_event::<StartedChunkLoader>()
            .add_event::<CreatedChunkLoaderEntity>()
            .add_event::<DestroyedChunkLoaderEntity>()
            .add_event::<UpgradedToChunkLoaderEntity>()
            .add_event::<CreateChunkLoaderEntityInternal>()
            .add_event::<DestroyChunkLoaderEntityInternal>()
            .add_event::<UpgradeToChunkLoaderEntityInternal>()
            .add_event::<CreatedChunkLoaderEntityInternal>()
            .add_event::<DestroyedChunkLoaderEntityInternal>()
            .add_event::<UpgradedToChunkLoaderEntityInternal>()
            .add_plugins(IDPlugin)
            .insert_resource(ChunkLoaderRegistry::default())
            .insert_resource(ChunkLoaderRequestRegistry::default())
            .configure_sets(Update, (
                StartExternalOperationSystems.before(StartInternalOperationSystems),
                StartInternalOperationSystems.before(FinishedInternalOperationSystems),
            ))
            .add_systems(Update, (
                start.before(update), 
                update.before(handle_updated_chunks), 
                handle_updated_chunks
            ))
            .add_systems(Update, (
                handle_create_chunk_loader_entity_events, 
                handle_destroy_chunk_loader_entity_events, 
                handle_upgrade_to_chunk_loader_entity_events,
            ).in_set(StartExternalOperationSystems))
            .add_systems(Update, (
                handle_create_chunk_loader_entity_internal_events,
                handle_destroy_chunk_loader_entity_internal_events,
                handle_upgrade_to_chunk_loader_entity_internal_events,
            ).in_set(StartInternalOperationSystems))
            .add_systems(Update, (
                handle_created_chunk_loader_entity_internal_events,
                handle_destroyed_chunk_loader_entity_internal_events,
                handle_upgraded_to_chunk_loader_entity_internal_events,
            ).in_set(FinishedInternalOperationSystems))
            .register_type::<components::ChunkLoader>();
    }
}