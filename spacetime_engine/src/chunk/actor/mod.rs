pub mod position;
pub mod id;

pub mod components;
pub mod constants;
pub mod events;
pub mod functions;
pub mod resources;
pub mod structs;
pub(in crate) mod systems;

use id::*;
use events::*;
use resources::*;
use systems::*;
use bevy::prelude::*;

pub(in crate) struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CreateChunkActorEntity>()
            .add_event::<DestroyChunkActorEntity>()
            .add_event::<UpgradeToChunkActorEntity>()
            .add_event::<StartChunkActorResult>()
            .add_event::<CreateChunkActorEntityResult>()
            .add_event::<DestroyChunkActorEntityResult>()
            .add_event::<UpgradeToChunkActorEntityResult>()
            .add_plugins(IDPlugin)
            .insert_resource(ChunkActorRegistry::default())
            .add_systems(Update, (start, update))
            .register_type::<components::ChunkActor>();
    }
}