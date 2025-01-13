use std::collections::{HashMap, HashSet};
use bevy::prelude::*;

use super::enums::{ChunkAction, ChunkActionPriority};

#[derive(Resource, Default)]
pub(in crate) struct ChunkActionBuffer {
    pub actions: Vec<(ChunkActionPriority, (i32, i32), ChunkAction)>, // (Priority, Chunk Coord, Action)
}
impl ChunkActionBuffer {
    pub fn add_action(&mut self, action: ChunkAction) {
        let priority = match &action {
            ChunkAction::Spawn { priority, .. }
            | ChunkAction::Despawn { priority, .. }
            | ChunkAction::TransferOwnership { priority, .. } => *priority,
        };
        let coord = match &action {
            ChunkAction::Spawn { coord, .. } => *coord,
            ChunkAction::Despawn { coord, .. } => *coord,
            ChunkAction::TransferOwnership { coord, .. } => *coord,
        };

        self.actions.push((priority, coord, action));
    }

    /// Removes actions for a specific chunk coordinate.
    pub fn cancel_action(&mut self, coord: (i32, i32)) {
        self.actions.retain(|(_, action_coord, _)| *action_coord != coord);
    }

    /// Sorts actions by priority, highest to lowest.
    pub fn sort_by_priority(&mut self) {
        self.actions.sort_by(|(p1, _, _), (p2, _, _)| p1.cmp(p2));
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

    pub fn get(&self, chunk_coord: &(i32, i32)) -> Option<(&ChunkActionPriority, &ChunkAction)> {
        self.actions
            .iter()
            .find(|(_, coord, _)| chunk_coord == coord )
            .map(|(priority, _, action)| (priority, action))
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