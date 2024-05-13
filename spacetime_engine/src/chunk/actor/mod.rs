pub mod coordinate;
pub mod id;

pub mod components;
pub mod events;
pub mod resources;
pub(in crate) mod systems;

use id::*;
use resources::*;
use systems::*;
use bevy::prelude::*;

pub(in crate) struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<events::CreateChunkActor>()
            .add_event::<events::DestroyChunkActor>()
            .add_event::<events::LoadChunkActor>()
            .add_event::<events::UnloadChunkActor>()
            .add_plugins(IDPlugin)
            .insert_resource(ChunkActorRegistry::default())
            .add_systems(Update, update)
            .add_systems(Update, handle_create_events)
            .add_systems(Update, handle_destroy_events)
            .add_systems(Update, handle_load_events)
            .add_systems(Update, handle_unload_events)
            .register_type::<components::ChunkActor>();
    }
}