pub mod components;
pub mod events;
pub mod functions;
pub(in crate) mod systems;
pub mod resources;
pub mod structs;

pub mod id;

use events::*;
use systems::*;
use id::*;
use bevy::prelude::*;

pub(in crate) struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<StartedChunkLoader>()
            .add_plugins(IDPlugin)
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