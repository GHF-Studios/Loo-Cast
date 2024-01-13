// Modules

// Local imports

// Internal imports
use crate::system::universe::chunk::*;
use crate::system::universe::entity::pos::*;

// External imports

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[derive(Default)]
pub struct ChunkPos {
    parent_pos: Option<Box<ChunkPos>>,
    absolute_local_pos: AbsoluteLocalChunkPos,
    apparent_local_pos: ApparentLocalChunkPos,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive(Default)]
pub struct AbsoluteLocalChunkPos {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive(Default)]
pub struct ApparentLocalChunkPos {
    pub x: i8,
    pub y: i8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive(Default)]
pub struct ApparentChunkPosShift {
    pub x: i8,
    pub y: i8,
}

// Implementations
impl ChunkPos {
    pub fn from_absolute(parent_pos: Option<Box<ChunkPos>>, absolute_local_pos: AbsoluteLocalChunkPos) -> Self {
        ChunkPos {
            parent_pos,
            absolute_local_pos,
            apparent_local_pos: absolute_local_pos.into(),
        }
    }

    pub fn from_apparent(parent_pos: Option<Box<ChunkPos>>, apparent_local_pos: ApparentLocalChunkPos) -> Self {
        ChunkPos {
            parent_pos,
            absolute_local_pos: apparent_local_pos.into(),
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
            x: x % 10,
            y: y % 10,
        }
    }
}

impl From<AbsoluteLocalChunkPos> for (u8, u8) {
    fn from(val: AbsoluteLocalChunkPos) -> Self {
        (val.x as u8, val.y as u8)
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
            x: ((apparent_local_chunk_pos.x % 10 + 10) % 10) as u8,
            y: ((apparent_local_chunk_pos.y % 10 + 10) % 10) as u8,
        }
    }
}

impl AbsoluteLocalChunkPos {
    pub fn new(x: u8, y: u8) -> Self {
        AbsoluteLocalChunkPos { x, y }
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
            x: absolute_local_chunk_pos.x as i8,
            y: absolute_local_chunk_pos.y as i8,
        }
    }
}

impl From<(AbsoluteLocalChunkPos, ApparentChunkPosShift)> for ApparentLocalChunkPos {
    fn from((absolute_local_chunk_pos, apparent_chunk_pos_shift): (AbsoluteLocalChunkPos, ApparentChunkPosShift)) -> ApparentLocalChunkPos {
        ApparentLocalChunkPos {
            x: absolute_local_chunk_pos.x as i8 + (apparent_chunk_pos_shift.x * 10),
            y: absolute_local_chunk_pos.y as i8 + (apparent_chunk_pos_shift.y * 10),
        }
    }
}

impl ApparentLocalChunkPos {
    pub fn new(x: i8, y: i8) -> Self {
        ApparentLocalChunkPos { x, y }
    }
}

/*
This is the intended logic

if value >= 0 {
    value / 10
} else {
    (value - 9) / 10
}

It has intentionally been optimized into the following logic

(value + 9 * (value >> 7)) / 10


Note: This will only work if value is an i8
*/
impl From<ApparentLocalChunkPos> for ApparentChunkPosShift {
    fn from(apparent_local_chunk_pos: ApparentLocalChunkPos) -> ApparentChunkPosShift {
        ApparentChunkPosShift {
            x: (apparent_local_chunk_pos.x + 9 * (apparent_local_chunk_pos.x >> 7)) / 10,
            y: (apparent_local_chunk_pos.y + 9 * (apparent_local_chunk_pos.y >> 7)) / 10,
        }
    }
}

// Module Functions
