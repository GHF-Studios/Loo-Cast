pub mod actor;
pub mod position;
pub mod loader;

pub mod components;
pub mod constants;
pub mod events;
pub(in crate) mod functions;
pub mod resources;
pub mod structs;
mod systems;

use actor::ActorPlugin;
use position::PositionPlugin;
use loader::LoaderPlugin;
use resources::*;
use systems::*;
use bevy::prelude::*;
use events::*;

pub(in crate) struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UpgradeToChunk>()
            .add_event::<DowngradeFromChunk>()
            .add_event::<LoadChunk>()
            .add_event::<SaveChunk>()
            .add_event::<UpgradedToChunk>()
            .add_event::<DowngradedFromChunk>()
            .add_event::<LoadedChunk>()
            .add_event::<SavedChunk>()
            .add_plugins(ActorPlugin)
            .add_plugins(PositionPlugin)
            .add_plugins(LoaderPlugin)
            .insert_resource(ChunkRegistry::default())
            .insert_resource(ChunkRequestRegistry::default())
            .add_systems(Startup, functions::main::setup)
            .add_systems(Update, (
                request_handlers::handle_upgrade_to_chunk,
                request_handlers::handle_downgrade_from_chunk,
                request_handlers::handle_load_chunk,
                request_handlers::handle_save_chunk,
            ))
            .register_type::<components::Chunk>()
            .register_type::<Vec<actor::id::structs::ChunkActorID>>();
    }
}