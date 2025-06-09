use bevy::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::chunk::types::ChunkOwnerId;

use super::intent::{ActionIntent, ActionPriority};

#[derive(Resource, Default)]
pub struct ActionIntentCommitBuffer {
    pub action_intent: HashMap<(i32, i32), ActionIntent>,
    pub priority_buckets: BTreeMap<ActionPriority, HashSet<(i32, i32)>>,
}
impl ActionIntentCommitBuffer {
    pub fn commit_intent(&mut self, action_intent: ActionIntent) {
        let coord = action_intent.coord();
        let priority = action_intent.priority();

        self.action_intent.insert(coord, action_intent);
        self.priority_buckets.entry(priority).or_default().insert(coord);
    }

    pub fn commit_intents(&mut self, action_intents: impl IntoIterator<Item = ActionIntent>) {
        for action_intent in action_intents {
            self.commit_intent(action_intent);
        }
    }

    pub fn remove_intent(&mut self, coord: &(i32, i32)) {
        if let Some(action_intent) = self.action_intent.remove(coord) {
            let priority = action_intent.priority();

            if let Some(bucket) = self.priority_buckets.get_mut(&priority) {
                bucket.remove(coord);
                if bucket.is_empty() {
                    self.priority_buckets.remove(&priority);
                }
            }
        }
    }

    pub fn remove_intents(&mut self, coords: impl IntoIterator<Item = (i32, i32)>) {
        for coord in coords {
            self.remove_intent(&coord);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&(i32, i32), &ActionIntent)> {
        self.priority_buckets
            .iter()
            .flat_map(|(_, coords)| coords.iter())
            .filter_map(|coord| self.action_intent.get_key_value(coord))
    }

    pub fn get(&self, coord: &(i32, i32)) -> Option<&ActionIntent> {
        self.action_intent.get(coord)
    }
}

#[derive(Resource, Default)]
pub struct ActionIntentBuffer {
    pub action_intents: HashMap<(i32, i32), ActionIntent>,
    pub priority_buckets: BTreeMap<ActionPriority, HashSet<(i32, i32)>>,
}
impl ActionIntentBuffer {
    pub fn buffer_intent(&mut self, action_intent: ActionIntent) {
        let coord = action_intent.coord();
        let priority = action_intent.priority();

        self.action_intents.insert(coord, action_intent);
        self.priority_buckets.entry(priority).or_default().insert(coord);
    }

    pub fn cancel_intent(&mut self, coord: &(i32, i32)) {
        if let Some(committed_action_intent) = self.action_intents.remove(coord) {
            let priority = committed_action_intent.priority();

            if let Some(bucket) = self.priority_buckets.get_mut(&priority) {
                bucket.remove(coord);
                if bucket.is_empty() {
                    self.priority_buckets.remove(&priority);
                }
            }
        }
    }

    pub fn get(&self, coord: &(i32, i32)) -> Option<&ActionIntent> {
        self.action_intents.get(coord)
    }
}

#[derive(Resource, Default, Debug)]
pub struct ChunkManager {
    pub loaded_chunks: HashSet<(i32, i32)>,
    pub owned_chunks: HashMap<(i32, i32), ChunkOwnerId>,
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
