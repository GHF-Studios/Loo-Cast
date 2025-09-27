use std::{collections::HashSet, sync::Mutex};

use bevy::prelude::*;
use lazy_static::lazy_static;

use crate::usf::scale::ConstScale;
use crate::{chunk::types::ChunkOwnerId, config::statics::CONFIG, entity::functions::get_reserved_entity};

lazy_static! {
    static ref OWNER_ID_REGISTRY: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkLoader<S: ConstScale> {
    pub radius: u32,
    chunk_owner_id: ChunkOwnerId<S>,
    #[reflect(ignore)]
    phantom_scale: std::marker::PhantomData<S>,
}
impl<S: ConstScale> ChunkLoader<S> {
    pub fn new(owner_id: String) -> Self {
        let owner_id_registry = OWNER_ID_REGISTRY.lock().unwrap();
        if owner_id_registry.contains(&owner_id) {
            unreachable!("ChunkOwnerID '{}' is already in use", owner_id);
        }

        ChunkLoader {
            radius: CONFIG().get::<u32>("chunk_loader/default_radius"),
            chunk_owner_id: ChunkOwnerId::new(owner_id, get_reserved_entity()),
            phantom_scale: std::marker::PhantomData,
        }
    }

    pub fn chunk_owner_id(&self) -> &ChunkOwnerId<S> {
        &self.chunk_owner_id
    }
}
