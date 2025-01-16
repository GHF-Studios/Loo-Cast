use std::collections::{HashMap, HashSet};
use bevy::prelude::*;

use super::enums::{ChunkAction, ChunkActionPriority};

#[derive(Resource, Default)]
pub(in crate) struct ChunkActionBuffer {
    pub actions: HashMap<(i32, i32), (ChunkActionPriority, ChunkAction)>,
}
impl ChunkActionBuffer {
    pub fn add_action(&mut self, action: ChunkAction) {
        let (coord, priority) = match &action {
            ChunkAction::Spawn { coord, priority, .. }
            | ChunkAction::Despawn { coord, priority, .. }
            | ChunkAction::TransferOwnership { coord, priority, .. } => (*coord, *priority),
        };

        self.actions.insert(coord, (priority, action));
    }

    pub fn remove_action(&mut self, coord: &(i32, i32)) {
        self.actions.remove(coord);
    }

    pub fn get(&self, chunk_coord: &(i32, i32)) -> Option<&(ChunkActionPriority, ChunkAction)> {
        self.actions.get(chunk_coord)
    }

    pub fn get_action_states(&self, chunk_coord: &(i32, i32)) -> (bool, bool, bool) {
        match self.get(chunk_coord) {
            Some(action) => {
                match action {
                    (_, ChunkAction::Spawn { .. }) => {
                        (true, false, false)
                    },
                    (_, ChunkAction::Despawn { .. }) => {
                        (false, true, false)
                    },
                    (_, ChunkAction::TransferOwnership { .. }) => {
                        (false, false, true)
                    }
                }
            },
            None => {
                (false, false, false)
            }
        }
    }
    
    pub fn is_spawning(&self, chunk_coord: &(i32, i32)) -> bool {
        if let Some(action) = self.get(chunk_coord) {
            matches!(action, (_, ChunkAction::Spawn { .. }))
        } else {
            false
        }
    }

    pub fn is_despawning(&self, chunk_coord: &(i32, i32)) -> bool {
        if let Some(action) = self.get(chunk_coord) {
            matches!(action, (_, ChunkAction::Despawn { .. }))
        } else {
            false
        }
    }

    pub fn is_transfering_ownership(&self, chunk_coord: &(i32, i32)) -> bool {
        if let Some(action) = self.get(chunk_coord) {
            matches!(action, (_, ChunkAction::TransferOwnership { .. }))
        } else {
            false
        }
    }

    pub fn has_spawns(&self) -> bool {
        self.actions.iter().any(|(_, (_, action))| action.is_spawn())
    }

    pub fn has_despawns(&self) -> bool {
        self.actions.iter().any(|(_, (_, action))| action.is_despawn())
    }

    pub fn has_ownership_transfers(&self) -> bool {
        self.actions.iter().any(|(_, (_, action))| action.is_transfer_ownership())
    }
}

#[derive(Resource, Default, Debug)]
pub(in crate) struct ChunkManager {
    pub loaded_chunks: HashSet<(i32, i32)>,
    pub owned_chunks: HashMap<(i32, i32), Entity>,
}
impl ChunkManager {
    pub fn get_states(&self, chunk_coord: &(i32, i32)) -> (bool, bool) {
        (self.loaded_chunks.contains(chunk_coord), self.owned_chunks.contains_key(chunk_coord))
    }

    pub fn is_loaded(&self, chunk_coord: &(i32, i32)) -> bool {
        self.loaded_chunks.contains(chunk_coord)
    }

    pub fn is_owned(&self, chunk_coord: &(i32, i32)) -> bool {
        self.owned_chunks.contains_key(chunk_coord)
    }
}

#[derive(Resource)]
pub(in crate) struct ChunkRenderHandles {
    pub quad_handle: Handle<Mesh>,
    pub light_material_handle: Handle<ColorMaterial>,
    pub dark_material_handle: Handle<ColorMaterial>,
}