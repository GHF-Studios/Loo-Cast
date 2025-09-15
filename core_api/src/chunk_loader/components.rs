use std::{collections::HashSet, sync::Mutex};

use crate::{chunk::types::ChunkOwnerId, config::statics::config, entity::functions::get_reserved_entity};
use bevy::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref OWNER_ID_REGISTRY: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkLoader {
    pub radius: u32,
    chunk_owner_id: ChunkOwnerId,
}
impl ChunkLoader {
    pub fn new(owner_id: String) -> Self {
        let owner_id_registry = OWNER_ID_REGISTRY.lock().unwrap();
        if owner_id_registry.contains(&owner_id) {
            unreachable!("ChunkOwnerID '{}' is already in use", owner_id);
        }

        ChunkLoader {
            radius: config().get::<u32>("chunk_loader/default_radius"),
            chunk_owner_id: ChunkOwnerId::new(owner_id, get_reserved_entity()),
        }
    }

    pub fn chunk_owner_id(&self) -> &ChunkOwnerId {
        &self.chunk_owner_id
    }
}
