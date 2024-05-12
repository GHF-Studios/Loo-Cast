pub mod actor;
pub mod coordinate;
pub mod id;
pub mod loader;

pub mod components;
pub mod constants;
pub mod events;
pub(in crate) mod functions;
pub mod resources;
pub(in crate) mod systems;

use actor::*;
use coordinate::*;
use id::*;
use loader::*;
use resources::*;
use systems::*;
use bevy::prelude::*;

pub(in crate) struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<events::CreateChunk>()
            .add_event::<events::DestroyChunk>()
            .add_event::<events::LoadChunk>()
            .add_event::<events::UnloadChunk>()
            .add_plugins(ActorPlugin)
            .add_plugins(CoordinatePlugin)
            .add_plugins(IdPlugin)
            .add_plugins(LoaderPlugin)
            .insert_resource(ChunkRegistry::default())
            .add_systems(Update, update)
            .add_systems(Update, change_radius)
            .add_systems(Update, handle_create_chunk_events)
            .add_systems(Update, handle_destroy_chunk_events)
            .add_systems(Update, handle_load_chunk_events)
            .add_systems(Update, handle_unload_chunk_events)
            .register_type::<components::Chunk>()
            .register_type::<Vec<actor::id::structs::ChunkActorID>>();
    }
}