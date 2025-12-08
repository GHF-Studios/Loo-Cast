use bevy::prelude::*;

use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::Scale;

use super::enums::ZoomState;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Chunk {
    pub(crate) coord: GridVec,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ChunkActor {
    pub coord: GridVec,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ChunkLoader {
    pub(crate) scale: Scale,
    pub(crate) zoom_state: ZoomState,
    pub(crate) origin_offset: GridVec,
}
impl ChunkLoader {
    pub fn suggest_zoom_in(&mut self, logical_world_pos: Vec3) -> Vec3 {
        if self.zoom_state == ZoomState::None {
            self.zoom_state = ZoomState::ZoomIn;
            self.scale.zoom_in();
            let mut unit_pos = UnitVec::new(std::mem::take(&mut self.origin_offset), logical_world_pos.truncate());
            unit_pos.zoom_in();
            self.origin_offset = unit_pos.grid_offset;
            unit_pos.unit_offset
        } else {
            logical_world_pos
        }
    }

    pub fn suggest_zoom_out(&mut self) {
        if self.zoom_state == ZoomState::None {
            self.zoom_state = ZoomState::ZoomOut;
            self.scale.zoom_out();
        }
    }
}
