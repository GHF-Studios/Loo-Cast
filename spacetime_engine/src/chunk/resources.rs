use std::{cmp::Reverse, collections::{BTreeMap, HashMap, HashSet}};
use bevy::prelude::*;

use super::enums::{ChunkAction, ChunkActionPriority};

#[derive(Resource, Default)]
pub(in crate) struct ChunkActionBuffer {
    actions: HashMap<(i32, i32), ChunkAction>,
    priority_buckets: BTreeMap<Reverse<ChunkActionPriority>, HashSet<(i32, i32)>>,
}

impl ChunkActionBuffer {
    pub fn add_action(&mut self, action: ChunkAction) {
        let coord = action.get_coord();
        let priority = Reverse(action.get_priority());

        self.actions.insert(coord, action);

        self.priority_buckets
            .entry(priority)
            .or_default()
            .insert(coord);
    }

    pub fn remove_action(&mut self, coord: &(i32, i32)) {
        if let Some(action) = self.actions.remove(coord) {
            let priority = Reverse(action.get_priority());

            if let Some(bucket) = self.priority_buckets.get_mut(&priority) {
                bucket.remove(coord);
                if bucket.is_empty() {
                    self.priority_buckets.remove(&priority);
                }
            }
        }
    }

    pub fn get(&self, chunk_coord: &(i32, i32)) -> Option<&ChunkAction> {
        self.actions.get(chunk_coord)
    }

    pub fn get_action_states(&self, chunk_coord: &(i32, i32)) -> (bool, bool, bool) {
        match self.get(chunk_coord) {
            Some(action) => match action {
                ChunkAction::Spawn { .. } => (true, false, false),
                ChunkAction::Despawn { .. } => (false, true, false),
                ChunkAction::TransferOwnership { .. } => (false, false, true),
            },
            None => (false, false, false),
        }
    }

    pub fn is_spawning(&self, chunk_coord: &(i32, i32)) -> bool {
        matches!(self.get(chunk_coord), Some(ChunkAction::Spawn { .. }))
    }

    pub fn is_despawning(&self, chunk_coord: &(i32, i32)) -> bool {
        matches!(self.get(chunk_coord), Some(ChunkAction::Despawn { .. }))
    }

    pub fn is_transfering_ownership(&self, chunk_coord: &(i32, i32)) -> bool {
        matches!(self.get(chunk_coord), Some(ChunkAction::TransferOwnership { .. }))
    }

    pub fn has_spawns(&self) -> bool {
        self.actions.values().any(|action| action.is_spawn())
    }

    pub fn has_despawns(&self) -> bool {
        self.actions.values().any(|action| action.is_despawn())
    }

    pub fn has_ownership_transfers(&self) -> bool {
        self.actions
            .values()
            .any(|action| action.is_transfer_ownership())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&(i32, i32), &ChunkAction)> {
        self.priority_buckets
            .iter()
            .flat_map(|(_, coords)| coords.iter())
            .filter_map(|coord| self.actions.get_key_value(coord))
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