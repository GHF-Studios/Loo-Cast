pub mod components;
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
            .add_plugins(IDPlugin)
            .insert_resource(ChunkLoaderRegistry::default())
            .insert_resource(ChunkLoaderEventRegistry::default())
            .add_systems(Update, (
                start, 
                update,
                handle_create_chunk_loader_entity_events,
                handle_destroy_chunk_loader_entity_events,
                handle_upgrade_to_chunk_loader_entity_events
            ))
            .register_type::<components::ChunkLoader>();
    }
}