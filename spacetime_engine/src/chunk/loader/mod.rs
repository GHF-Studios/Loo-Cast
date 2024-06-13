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
            .add_event::<StartChunkLoaderResult>()
            .add_plugins(IDPlugin)
            .add_systems(Update, (start, update))
            .register_type::<components::ChunkLoader>();
    }
}