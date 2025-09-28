use crate::usf::scale::Scale;

use super::types::ChunkCoord;

pub trait ChunkCoordTupleExt {
    fn scaled(self, scale: Scale) -> ChunkCoord;
}

impl ChunkCoordTupleExt for (i32, i32) {
    fn scaled(self, scale: Scale) -> ChunkCoord {
        ChunkCoord { x: self.0, y: self.1, scale }
    }
}