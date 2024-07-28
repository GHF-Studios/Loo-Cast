pub mod actor;
pub mod position;
pub mod id;
pub mod loader;

pub mod components;
pub mod constants;
pub mod events;
pub(in crate) mod functions;
pub mod resources;
pub mod structs;
pub(in crate) mod systems;

use actor::ActorPlugin;
use position::PositionPlugin;
use id::IdPlugin;
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
            .add_plugins(IdPlugin)
            .add_plugins(LoaderPlugin)
            .insert_resource(ChunkRegistry::default())
            .insert_resource(ChunkRequestRegistry::default())
            .add_systems(Update, (
                handle_upgrade_to_chunk,
                handle_downgrade_from_chunk,
                handle_load_chunk,
                handle_save_chunk,
                handle_created_entity,
                handle_upgraded_to_chunk
            ))
            .register_type::<components::Chunk>()
            .register_type::<Vec<actor::id::structs::ChunkActorID>>();
    }
}