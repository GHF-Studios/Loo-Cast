use bevy::math::{Vec2, IVec2};

use crate::usf::scale::Scale;

use super::types::{WorldCoord, ChunkCoord};

pub trait Vec2Ext {
    fn scaled(self, scale: Scale) -> WorldCoord;
}

impl Vec2Ext for Vec2 {
    fn scaled(self, scale: Scale) -> WorldCoord {
        WorldCoord { xy: Vec2::new(self.x, self.y), scale }
    }
}

pub trait IVec2Ext {
    fn scaled(self, scale: Scale) -> ChunkCoord;
}

impl IVec2Ext for IVec2 {
    fn scaled(self, scale: Scale) -> ChunkCoord {
        ChunkCoord { xy: IVec2::new(self.x, self.y), scale }
    }
}