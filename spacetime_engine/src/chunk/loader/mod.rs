pub mod components;
pub(in crate) mod systems;

use systems::*;
use bevy::prelude::*;

pub(in crate) struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<components::ChunkLoader>();
    }
}