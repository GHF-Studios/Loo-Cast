pub mod structs;

use bevy::prelude::*;

pub(in crate) struct CoordinatePlugin;

impl Plugin for CoordinatePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<structs::ChunkCoordinate>();
    }
}