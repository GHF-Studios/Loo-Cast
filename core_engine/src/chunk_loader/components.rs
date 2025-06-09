use std::{collections::HashSet, sync::Mutex};

use crate::{chunk::types::ChunkOwnerId, config::statics::CONFIG, entity::functions::reserve_entity_id};
use bevy::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref OWNER_ID_REGISTRY: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Component)]
pub struct ChunkLoaderComponent {
    pub radius: u32,
    owner_id: ChunkOwnerId,
}
impl ChunkLoaderComponent {
    pub fn new(owner_id: String) -> Self {
        let owner_id_registry = OWNER_ID_REGISTRY.lock().unwrap();
        if owner_id_registry.contains(&owner_id) {
            unreachable!("ChunkOwnerID '{}' is already in use", owner_id);
        }

        ChunkLoaderComponent {
            radius: CONFIG.get::<u32>("chunk_loader/default_radius"),
            owner_id: ChunkOwnerId::new(owner_id, reserve_entity_id()),
        }
    }

    pub fn owner_id(&self) -> &ChunkOwnerId {
        &self.owner_id
    }
}
