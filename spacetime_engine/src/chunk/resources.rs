use std::collections::{hash_map::Keys, HashMap, HashSet};

use bevy::prelude::*;

use super::enums::ChunkAction;

#[derive(Resource, Default, Debug)]
pub(in crate) struct ChunkManager {
    loaded_chunks: HashSet<(i32, i32)>,
    chunk_owners: HashMap<(i32, i32), Entity>,
    chunk_actions: HashMap<(i32, i32), ChunkAction>
}
impl ChunkManager {
    pub fn get_loaded_chunks(&self) -> &HashSet<(i32, i32)> {
        &self.loaded_chunks
    }

    pub fn get_owned_chunks(&self) -> &HashMap<(i32, i32), Entity> {
        &self.chunk_owners
    }

    pub fn is_loaded(&self, chunk_coord: &(i32, i32)) -> bool {
        self.loaded_chunks.contains(&chunk_coord)
    }

    pub fn is_owned(&self, chunk_coord: &(i32, i32)) -> bool {
        if !self.is_loaded(chunk_coord) {
            return false;
        }
        
        self.chunk_owners.get(chunk_coord).is_some()
    }

    pub fn is_spawning(&self, chunk_coord: &(i32, i32)) -> bool {
        if !self.is_loaded(chunk_coord) {
            return false;
        }

        if let Some(action) = self.chunk_actions.get(chunk_coord) {
            match action {
                ChunkAction::SpawnChunk { .. } => true,
                _ => false
            }
        } else {
            false
        }
    }

    pub fn is_despawning(&self, chunk_coord: &(i32, i32)) -> bool {
        if !self.is_loaded(chunk_coord) {
            return false;
        }

        if let Some(action) = self.chunk_actions.get(chunk_coord) {
            match action {
                ChunkAction::DespawnChunk { .. } => true,
                _ => false
            }
        } else {
            false
        }
    }

    pub fn is_transfering_ownership(&self, chunk_coord: &(i32, i32)) -> bool {
        if !self.is_loaded(chunk_coord) {
            return false;
        }

        if let Some(action) = self.chunk_actions.get(chunk_coord) {
            match action {
                ChunkAction::TransferChunkOwnership { .. } => true,
                _ => false
            }
        } else {
            false
        }
    }
}