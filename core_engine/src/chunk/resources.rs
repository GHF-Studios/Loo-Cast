use bevy::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};

use super::intent::{ActionIntent, ActionPriority};

#[derive(Resource, Default)]
pub(in crate) struct ActionIntentCommitBuffer {
    pub committed_action_intents: HashMap<(i32, i32), ActionIntent>,
    pub priority_buckets: BTreeMap<ActionPriority, HashSet<(i32, i32)>>,
}
impl ActionIntentCommitBuffer {
    pub fn commit_intent(&mut self, action_intent: ActionIntent) {
        let coord = action_intent.coord();
        let priority = action_intent.priority();

        self.committed_action_intents.insert(coord, action_intent);

        self.priority_buckets.entry(priority).or_default().insert(coord);
    }

    pub fn commit_intents<I>(&mut self, committed_action_intents: I)
    where
        I: IntoIterator<Item = ActionIntent>,
    {
        for committed_action_intent in committed_action_intents {
            let coord = committed_action_intent.coord();
            let priority = committed_action_intent.priority();

            self.committed_action_intents.insert(coord, committed_action_intent);

            self.priority_buckets.entry(priority).or_default().insert(coord);
        }
    }

    pub fn remove_intent(&mut self, coord: &(i32, i32)) {
        if let Some(committed_action_intent) = self.committed_action_intents.remove(coord) {
            let priority = committed_action_intent.priority();

            if let Some(bucket) = self.priority_buckets.get_mut(&priority) {
                bucket.remove(coord);
                if bucket.is_empty() {
                    self.priority_buckets.remove(&priority);
                }
            }
        }
    }

    pub fn remove_intents<I>(&mut self, coords: I)
    where
        I: IntoIterator<Item = (i32, i32)>,
    {
        for coord in coords {
            if let Some(committed_action_intent) = self.committed_action_intents.remove(&coord) {
                let priority = committed_action_intent.priority();

                if let Some(bucket) = self.priority_buckets.get_mut(&priority) {
                    bucket.remove(&coord);

                    if bucket.is_empty() {
                        self.priority_buckets.remove(&priority);
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&(i32, i32), &ActionIntent)> {
        self.priority_buckets
            .iter()
            .flat_map(|(_, coords)| coords.iter())
            .filter_map(|coord| self.committed_action_intents.get_key_value(coord))
    }
}

#[derive(Resource, Default)]
pub(in crate) struct ActionIntentBuffer {
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

    pub fn clear_buffer(&mut self) {
        self.action_intents.clear();
        self.priority_buckets.clear();
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
