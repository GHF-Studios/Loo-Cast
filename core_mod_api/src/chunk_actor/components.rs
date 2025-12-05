use bevy::prelude::*;

use crate::usf::pos::grid::types::GridVec;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ChunkActor {
    pub coord: GridVec,
}
