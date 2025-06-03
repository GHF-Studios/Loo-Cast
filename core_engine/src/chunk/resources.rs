use bevy::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};

use super::intent::{ActionIntent, ActionPriority};

#[derive(Resource, Default)]
pub struct ChunkActionBuffer {
    pub actions: HashMap<(i32, i32), ActionIntent>,
    pub priority_buckets: BTreeMap<ActionPriority, HashSet<(i32, i32)>>,
}

impl ChunkActionBuffer {
    pub fn add_action(&mut self, action_intent: ActionIntent) {
        let coord = action_intent.coord();
        let priority = action_intent.priority();

        self.actions.insert(coord, action_intent);

        self.priority_buckets.entry(priority).or_default().insert(coord);
    }

    pub fn add_actions<I>(&mut self, actions: I)
    where
        I: IntoIterator<Item = ActionIntent>,
    {
        for action_intent in actions {
            let coord = action_intent.coord();
            let priority = action_intent.priority();

            // Add to the actions map
            self.actions.insert(coord, action_intent);

            // Add to the priority bucket
            self.priority_buckets.entry(priority).or_default().insert(coord);
        }
    }

    pub fn remove_action(&mut self, coord: &(i32, i32)) {
        if let Some(action_intent) = self.actions.remove(coord) {
            let priority = action_intent.priority();

            if let Some(bucket) = self.priority_buckets.get_mut(&priority) {
                bucket.remove(coord);
                if bucket.is_empty() {
                    self.priority_buckets.remove(&priority);
                }
            }
        }
    }

    pub fn remove_actions<I>(&mut self, coords: I)
    where
        I: IntoIterator<Item = (i32, i32)>,
    {
        for coord in coords {
            // Remove from the actions map
            if let Some(action_intent) = self.actions.remove(&coord) {
                let priority = action_intent.priority();

                // Remove from the priority bucket
                if let Some(bucket) = self.priority_buckets.get_mut(&priority) {
                    bucket.remove(&coord);

                    // Clean up empty buckets
                    if bucket.is_empty() {
                        self.priority_buckets.remove(&priority);
                    }
                }
            }
        }
    }

    pub fn get(&self, chunk_coord: &(i32, i32)) -> Option<&ActionIntent> {
        self.actions.get(chunk_coord)
    }

    pub fn get_action_states(&self, chunk_coord: &(i32, i32)) -> (bool, bool, bool) {
        match self.get(chunk_coord) {
            Some(action_intent) => match action_intent {
                ActionIntent::Spawn { .. } => (true, false, false),
                ActionIntent::Despawn { .. } => (false, true, false),
                ActionIntent::TransferOwnership { .. } => (false, false, true),
            },
            None => (false, false, false),
        }
    }

    pub fn is_spawning(&self, chunk_coord: &(i32, i32)) -> bool {
        matches!(self.get(chunk_coord), Some(ActionIntent::Spawn { .. }))
    }

    pub fn is_despawning(&self, chunk_coord: &(i32, i32)) -> bool {
        matches!(self.get(chunk_coord), Some(ActionIntent::Despawn { .. }))
    }

    pub fn is_transfering_ownership(&self, chunk_coord: &(i32, i32)) -> bool {
        matches!(self.get(chunk_coord), Some(ActionIntent::TransferOwnership { .. }))
    }

    pub fn has_spawns(&self) -> bool {
        self.actions.values().any(|action_intent| action_intent.is_spawn())
    }

    pub fn has_despawns(&self) -> bool {
        self.actions.values().any(|action_intent| action_intent.is_despawn())
    }

    pub fn has_ownership_transfers(&self) -> bool {
        self.actions.values().any(|action_intent| action_intent.is_transfer_ownership())
    }

    pub fn has_any_actions(&self) -> bool {
        !self.actions.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&(i32, i32), &ActionIntent)> {
        self.priority_buckets
            .iter()
            .flat_map(|(_, coords)| coords.iter())
            .filter_map(|coord| self.actions.get_key_value(coord))
    }
}

#[derive(Resource, Default, Debug)]
pub struct ChunkManager {
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
pub struct ChunkRenderHandles {
    pub quad: Handle<Mesh>,
    pub light_material: Handle<ColorMaterial>,
    pub dark_material: Handle<ColorMaterial>,
}
