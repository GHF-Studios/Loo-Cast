// Modules

// Local imports

// Internal imports
use crate::engine::kernel::universe::chunk::*;
use crate::engine::kernel::universe::entity::pos::*;

// External imports

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ChunkPos {
    parent_pos: Option<Box<ChunkPos>>,
    absolute_local_pos: AbsoluteLocalChunkPos,
    apparent_local_pos: ApparentLocalChunkPos,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AbsoluteLocalChunkPos {
    pub x: i8,
    pub y: i8,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ApparentLocalChunkPos {
    pub x: i8,
    pub y: i8,
}

// Implementations
impl Default for ChunkPos {
    fn default() -> Self {
        ChunkPos {
            parent_pos: None,
            absolute_local_pos: AbsoluteLocalChunkPos::default(),
            apparent_local_pos: ApparentLocalChunkPos::default(),
        }
    }
}

impl ChunkPos {
    pub fn from_absolute(parent_pos: Option<Box<ChunkPos>>, absolute_local_pos: AbsoluteLocalChunkPos) -> Self {
        ChunkPos {
            parent_pos,
            absolute_local_pos: absolute_local_pos.clone(),
            apparent_local_pos: absolute_local_pos.into(),
        }
    }

    pub fn from_apparent(parent_pos: Option<Box<ChunkPos>>, apparent_local_pos: ApparentLocalChunkPos) -> Self {
        ChunkPos {
            parent_pos,
            absolute_local_pos: apparent_local_pos.clone().into(),
            apparent_local_pos,
        }
    }

    pub fn get_parent_pos(&self) -> &Option<Box<ChunkPos>> {
        &self.parent_pos
    }

    pub fn get_absolute_local_pos(&self) -> &AbsoluteLocalChunkPos {
        &self.absolute_local_pos
    }
    pub fn get_apparent_local_pos(&self) -> &ApparentLocalChunkPos {
        &self.apparent_local_pos
    }
}

impl From<(u8, u8)> for AbsoluteLocalChunkPos {
    fn from((x, y): (u8, u8)) -> AbsoluteLocalChunkPos {
        AbsoluteLocalChunkPos {
            x: x as i8 % 10,
            y: y as i8 % 10,
        }
    }
}

impl Into<(u8, u8)> for AbsoluteLocalChunkPos {
    fn into(self) -> (u8, u8) {
        (self.x as u8, self.y as u8)
    }
}

impl From<LocalEntityPos> for AbsoluteLocalChunkPos {
    fn from(local_entity_pos: LocalEntityPos) -> AbsoluteLocalChunkPos {
        let apparent_local_chunk_pos = ApparentLocalChunkPos::from(local_entity_pos);
        apparent_local_chunk_pos.into()
    }
}

impl From<ApparentLocalChunkPos> for AbsoluteLocalChunkPos {
    fn from(apparent_local_chunk_pos: ApparentLocalChunkPos) -> AbsoluteLocalChunkPos {
        AbsoluteLocalChunkPos {
            x: (apparent_local_chunk_pos.x % 10 + 10) % 10,
            y: (apparent_local_chunk_pos.y % 10 + 10) % 10,
        }
    }
}

impl Default for AbsoluteLocalChunkPos {
    fn default() -> Self {
        AbsoluteLocalChunkPos { x: 0, y: 0 }
    }
}

impl AbsoluteLocalChunkPos {
    pub fn new(x: i8, y: i8) -> Self {
        AbsoluteLocalChunkPos { x, y }
    }
}

impl From<(u8, u8)> for ApparentLocalChunkPos {
    fn from((x, y): (u8, u8)) -> ApparentLocalChunkPos {
        ApparentLocalChunkPos {
            x: x as i8,
            y: y as i8,
        }
    }
}

impl Into<(u8, u8)> for ApparentLocalChunkPos {
    fn into(self) -> (u8, u8) {
        (self.x as u8, self.y as u8)
    }
}

impl From<LocalEntityPos> for ApparentLocalChunkPos {
    fn from(local_entity_pos: LocalEntityPos) -> ApparentLocalChunkPos {
        let half_chunk = (CHUNK_SIZE as f32) / 2.0;
        ApparentLocalChunkPos::new(
            ((local_entity_pos.x + half_chunk) / CHUNK_SIZE as f32).floor() as i8,
            ((local_entity_pos.y + half_chunk) / CHUNK_SIZE as f32).floor() as i8,
        )
    }
}

impl From<AbsoluteLocalChunkPos> for ApparentLocalChunkPos {
    fn from(absolute_local_chunk_pos: AbsoluteLocalChunkPos) -> ApparentLocalChunkPos {
        ApparentLocalChunkPos {
            x: absolute_local_chunk_pos.x,
            y: absolute_local_chunk_pos.y,
        }
    }
}

impl Default for ApparentLocalChunkPos {
    fn default() -> Self {
        ApparentLocalChunkPos { x: 0, y: 0 }
    }
}

impl ApparentLocalChunkPos {
    pub fn new(x: i8, y: i8) -> Self {
        ApparentLocalChunkPos { x, y }
    }
}

// Module Functions
