pub mod structs;

use bevy::prelude::*;

pub(in crate) struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<structs::ChunkActorPosition>();
    }
}