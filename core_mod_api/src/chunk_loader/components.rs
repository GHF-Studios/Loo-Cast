use std::{collections::HashSet, sync::Mutex};

use bevy::prelude::*;
use lazy_static::lazy_static;

use crate::{chunk::types::ChunkOwnerId, config::statics::CONFIG, entity::functions::get_reserved_entity};
use crate::usf::scale::Scale;

use super::enums::ZoomState;

lazy_static! {
    static ref OWNER_ID_REGISTRY: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkLoader {
    pub radius: u32,
    chunk_owner_id: ChunkOwnerId,
    pub(crate) zoom_state: ZoomState,
}
impl ChunkLoader {
    pub fn new(owner_id: String, owner_scale: Scale) -> Self {
        let owner_id_registry = OWNER_ID_REGISTRY.lock().unwrap();
        if owner_id_registry.contains(&owner_id) {
            unreachable!("ChunkOwnerID '{}' is already in use", owner_id);
        }

        ChunkLoader {
            radius: CONFIG().get::<u32>("chunk_loader/default_radius"),
            chunk_owner_id: ChunkOwnerId::new(owner_id, get_reserved_entity(), owner_scale),
            zoom_state: ZoomState::default(),
        }
    }

    pub fn id(&self) -> &ChunkOwnerId {
        &self.chunk_owner_id
    }

    pub(crate) fn id_mut(&mut self) -> &mut ChunkOwnerId {
        &mut self.chunk_owner_id
    }

    pub fn suggest_zoom_out(&mut self) {
        if self.zoom_state == ZoomState::None {
            self.zoom_state = ZoomState::ZoomOut;
            println!("Suggesting zoom OUT, deferring scale change to workflow finalization");
        }
    }
    
    pub fn suggest_zoom_in(&mut self) {
        if self.zoom_state == ZoomState::None {
            self.zoom_state = ZoomState::ZoomIn;
            println!("Suggesting zoom IN, deferring scale change to workflow finalization");
        }
    }
}
