pub mod components;
pub mod events;
pub mod functions;
pub(in crate) mod systems;

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
            .add_event::<UpdatedChunkLoader>()
            .add_plugins(IDPlugin)
            .add_systems(Startup, startup)
            .add_systems(Update, update)
            .register_type::<components::ChunkLoader>();
    }
}