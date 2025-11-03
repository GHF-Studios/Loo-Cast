use bevy::prelude::*;

use crate::usf::pos::grid::types::GridVec;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkActor {
    pub coord: GridVec
}
impl Default for ChunkActor {
    fn default() -> Self {
        ChunkActor {
            coord: GridVec::default()
        }
    }
}
