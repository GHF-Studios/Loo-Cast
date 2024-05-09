pub mod structs;

use bevy::prelude::*;

pub(in crate) struct IdPlugin;

impl Plugin for IdPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<structs::ChunkID>();
    }
}