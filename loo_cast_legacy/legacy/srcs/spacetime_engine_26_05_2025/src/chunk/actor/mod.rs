pub mod position;

pub mod components;
pub mod constants;
pub mod events;
pub mod functions;
pub mod resources;
pub mod structs;
pub(in crate) mod systems;

use events::*;
use resources::*;
use systems::main::*;
use systems::request_handlers::*;
use bevy::prelude::*;

pub(in crate) struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UpgradeToChunkActor>()
            .add_event::<DowngradeFromChunkActor>()
            .add_event::<UpgradedToChunkActor>()
            .add_event::<DowngradedFromChunkActor>()
            .insert_resource(ChunkActorRegistry::default())
            .insert_resource(ChunkActorRequestRegistry::default())
            .add_systems(Startup, (
                setup,
            ))
            .add_systems(Update, (
                update,
                handle_upgrade_to_chunk_actor,
                handle_downgrade_from_chunk_actor,
            ))
            .register_type::<components::ChunkActor>();
    }
}