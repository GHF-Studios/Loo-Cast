use std::collections::{HashMap, HashSet};
use bevy::prelude::*;

use super::enums::ChunkAction;

#[derive(Resource)]
pub(in crate) struct ChunkActionBuffer(pub HashMap<(i32, i32), ChunkAction>);
impl Default for ChunkActionBuffer {
    fn default() -> Self {
        ChunkActionBuffer(HashMap::new())
    }
}
impl ChunkActionBuffer {
    pub fn is_spawning(&self, chunk_coord: &(i32, i32)) -> bool {
        if let Some(action) = self.0.get(chunk_coord) {
            match action {
                ChunkAction::Spawn { .. } => true,
                _ => false
            }
        } else {
            false
        }
    }

    pub fn is_despawning(&self, chunk_coord: &(i32, i32)) -> bool {
        if let Some(action) = self.0.get(chunk_coord) {
            match action {
                ChunkAction::Despawn { .. } => true,
                _ => false
            }
        } else {
            false
        }
    }

    pub fn is_transfering_ownership(&self, chunk_coord: &(i32, i32)) -> bool {
        if let Some(action) = self.0.get(chunk_coord) {
            match action {
                ChunkAction::TransferOwnership { .. } => true,
                _ => false
            }
        } else {
            false
        }
    }

    pub fn get(&self, chunk_coord: &(i32, i32)) -> Option<&ChunkAction> {
        self.0.get(chunk_coord)
    }

    pub fn get_action_states(&self, chunk_coord: &(i32, i32)) -> (bool, bool, bool) {
        match self.get(&chunk_coord) {
            Some(action) => {
                match action {
                    ChunkAction::Spawn { .. } => {
                        (true, false, false)
                    },
                    ChunkAction::Despawn { .. } => {
                        (false, true, false)
                    },
                    ChunkAction::TransferOwnership { .. } => {
                        (false, false, true)
                    }
                }
            },
            None => {
                (false, false, false)
            }
        }
    }
}

#[derive(Resource, Default, Debug)]
pub(in crate) struct ChunkManager {
    pub loaded_chunks: HashSet<(i32, i32)>,
    pub owned_chunks: HashMap<(i32, i32), Entity>,
}
impl ChunkManager {
    pub fn get_states(&self, chunk_coord: &(i32, i32)) -> (bool, bool) {
        (self.loaded_chunks.contains(&chunk_coord), self.owned_chunks.contains_key(&chunk_coord))
    }

    pub fn is_loaded(&self, chunk_coord: &(i32, i32)) -> bool {
        self.loaded_chunks.contains(&chunk_coord)
    }

    pub fn is_owned(&self, chunk_coord: &(i32, i32)) -> bool {
        self.owned_chunks.contains_key(chunk_coord)
    }
}