use std::{collections::HashSet, sync::Mutex};

use bevy::prelude::*;
use lazy_static::lazy_static;

use crate::usf::pos::unit::types::UnitVec;
use crate::{chunk_loader::types::ChunkLoaderId, config::statics::CONFIG, entity::functions::get_reserved_entity};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;

use super::enums::ZoomState;

lazy_static! {
    static ref OWNER_ID_REGISTRY: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkLoader {
    pub radius: u32,
    chunk_owner_id: ChunkLoaderId,
    pub(crate) zoom_state: ZoomState,
    pub(crate) origin_offset: GridVec,
}
impl ChunkLoader {
    pub fn new(owner_id: String, owner_scale: Scale) -> Self {
        let owner_id_registry = OWNER_ID_REGISTRY.lock().unwrap();
        if owner_id_registry.contains(&owner_id) {
            unreachable!("ChunkLoaderID '{}' is already in use", owner_id);
        }

        ChunkLoader {
            radius: CONFIG().get::<u32>("chunk_loader/default_radius"),
            chunk_owner_id: ChunkLoaderId::new(owner_id, get_reserved_entity(), owner_scale),
            zoom_state: ZoomState::default(),
            origin_offset: GridVec::default(),
        }
    }

    pub fn id(&self) -> &ChunkLoaderId {
        &self.chunk_owner_id
    }

    pub fn suggest_zoom_in(&mut self, logical_world_pos: Vec3) -> Vec3 {
        if self.zoom_state == ZoomState::None {
            self.zoom_state = ZoomState::ZoomIn;
            self.chunk_owner_id.scale_mut().zoom_in();
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
            self.chunk_owner_id.scale_mut().zoom_out();
        }
    }
}
