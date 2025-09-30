use bevy::math::{Vec2, IVec2};

use crate::usf::scale::Scale;

use super::types::{WorldCoord, ChunkCoord};

pub trait Vec2Ext {
    fn scaled(self, scale: Scale) -> WorldCoord;
    fn world_coord_to_chunk_coord(self) -> IVec2;
}

impl Vec2Ext for Vec2 {
    fn scaled(self, scale: Scale) -> WorldCoord {
        WorldCoord { xy: Vec2::new(self.x, self.y), scale }
    }

    fn world_coord_to_chunk_coord(self) -> IVec2 {
        let chunk_size = 1000.0;
        let chunk_x = ((self.x + chunk_size / 2.0) / chunk_size).floor() as i32;
        let chunk_y = ((self.y + chunk_size / 2.0) / chunk_size).floor() as i32;
        IVec2::new(chunk_x, chunk_y)
    }
}

pub trait IVec2Ext {
    fn scaled(self, scale: Scale) -> ChunkCoord;
    fn chunk_coord_to_world_coord(self) -> Vec2;
}

impl IVec2Ext for IVec2 {
    fn scaled(self, scale: Scale) -> ChunkCoord {
        ChunkCoord { xy: IVec2::new(self.x, self.y), scale }
    }

    fn chunk_coord_to_world_coord(self) -> Vec2 {
        let chunk_size = 1000.0;
        let chunk_x = self.x as f32 * chunk_size;
        let chunk_y = self.y as f32 * chunk_size;
        Vec2::new(chunk_x, chunk_y)
    }
}