use crate::bevy::prelude::*;
use core_mod_macros::component_ctor;
use rhai::Dynamic;

use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::Scale;
use crate::script::ecs::component::internals::traits::InsertComponentFromDynamic;

use super::enums::ZoomState;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Chunk {
    pub(crate) coord: GridVec,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[component_ctor]
pub struct ChunkActor {
    pub coord: GridVec,
}
impl InsertComponentFromDynamic for ChunkActor {
    fn insert_component_from_dynamic(entity: &mut EntityWorldMut, _params: Dynamic) {
        entity.insert(ChunkActor::default());
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[component_ctor]
pub struct ChunkLoader {
    pub(crate) scale: Scale,
    pub(crate) zoom_state: ZoomState,
    pub(crate) coord: GridVec,
    pub(crate) origin_offset: GridVec,
}
impl InsertComponentFromDynamic for ChunkLoader {
    fn insert_component_from_dynamic(entity: &mut EntityWorldMut, _params: Dynamic) {
        entity.insert(ChunkLoader::default());
    }
}
impl ChunkLoader {
    pub fn zoom_in(&mut self, logical_world_pos: Vec2) -> Vec3 {
        if self.zoom_state == ZoomState::None {
            self.scale.zoom_in();
            self.zoom_state = ZoomState::ZoomIn;
            let new_logical_world_pos = self.coord.zoom_in(logical_world_pos);
            self.origin_offset.zoom_in(Vec2::ZERO);
            new_logical_world_pos
        } else {
            logical_world_pos.extend(self.scale.compute_z())
        }
    }

    pub fn zoom_out(&mut self) {
        if self.zoom_state == ZoomState::None {
            self.scale.zoom_out();
            self.zoom_state = ZoomState::ZoomOut;
            self.coord.zoom_out();
            self.origin_offset.zoom_out();
        }
    }
}
