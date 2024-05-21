use bevy::prelude::*;
use super::super::position::structs::*;
use crate::math::structs::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkID(pub ChunkPosition);

impl From<ChunkPosition> for ChunkID {
    fn from(chunk_position: ChunkPosition) -> Self {
        ChunkID(chunk_position)
    }
}

impl ChunkID {
    fn new(x: i16, y: i16) -> Self {
        ChunkID(ChunkPosition(I16Vec2(x, y)))
    }
}
