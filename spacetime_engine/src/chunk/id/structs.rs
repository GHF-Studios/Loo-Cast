use bevy::prelude::*;
use super::super::coordinate::structs::*;
use crate::math::structs::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkID(pub ChunkCoordinate);

impl From<ChunkCoordinate> for ChunkID {
    fn from(chunk_coordinate: ChunkCoordinate) -> Self {
        ChunkID(chunk_coordinate)
    }
}

impl ChunkID {
    fn new(x: i16, y: i16) -> Self {
        ChunkID(ChunkCoordinate(I16Vec2(x, y)))
    }
}
