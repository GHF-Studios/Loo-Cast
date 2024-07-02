pub mod structs;

use bevy::prelude::*;

pub(in crate) struct IDPlugin;

impl Plugin for IDPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<structs::ChunkLoaderID>()
            .register_type::<structs::ChunkLoaderRequestID>();
    }
}