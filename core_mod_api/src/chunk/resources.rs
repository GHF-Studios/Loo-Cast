use bevy::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::usf::pos::grid::types::GridVec;
use crate::{chunk_loader::types::ChunkLoaderId, gpu::workflows::gpu::generate_chunk_textures::user_items::ChunkRenderExecutor};

use super::intent::{ActionIntent, ActionPriority};

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ActionIntentCommitBuffer {
    pub action_intent: HashMap<GridVec, ActionIntent>,
    pub priority_buckets: BTreeMap<ActionPriority, HashSet<GridVec>>,
}
impl ActionIntentCommitBuffer {
    pub fn commit_intent(&mut self, action_intent: ActionIntent) {
        let coord = action_intent.coord();
        let priority = action_intent.priority();

        self.action_intent.insert(coord.clone(), action_intent);
        self.priority_buckets.entry(priority).or_default().insert(coord);
    }

    pub fn commit_intents(&mut self, action_intents: impl IntoIterator<Item = ActionIntent>) {
        for action_intent in action_intents {
            self.commit_intent(action_intent);
        }
    }

    pub fn remove_intent(&mut self, coord: &GridVec) {
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

    pub fn remove_intents(&mut self, coords: impl IntoIterator<Item = GridVec>) {
        for coord in coords {
            self.remove_intent(&coord);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&GridVec, &ActionIntent)> {
        self.priority_buckets
            .iter()
            .flat_map(|(_, coords)| coords.iter())
            .filter_map(|coord| self.action_intent.get_key_value(coord))
    }

    pub fn get(&self, coord: &GridVec) -> Option<&ActionIntent> {
        self.action_intent.get(coord)
    }
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ActionIntentBuffer {
    pub action_intents: HashMap<GridVec, ActionIntent>,
    pub priority_buckets: BTreeMap<ActionPriority, HashSet<GridVec>>,
}
impl ActionIntentBuffer {
    pub fn buffer_intent(&mut self, action_intent: ActionIntent) {
        let coord = action_intent.coord();
        let priority = action_intent.priority();

        self.action_intents.insert(coord.clone(), action_intent);
        self.priority_buckets.entry(priority).or_default().insert(coord);
    }

    pub fn cancel_intent(&mut self, coord: &GridVec) {
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

    pub fn get(&self, coord: &GridVec) -> Option<&ActionIntent> {
        self.action_intents.get(coord)
    }
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ChunkManager {
    pub loaded_chunks: HashSet<GridVec>,
    pub owned_chunks: HashMap<GridVec, ChunkLoaderId>,
}
impl ChunkManager {
    pub fn get_states(&self, grid_coord: &GridVec) -> (bool, bool) {
        (self.loaded_chunks.contains(grid_coord), self.owned_chunks.contains_key(grid_coord))
    }

    pub fn is_loaded(&self, grid_coord: &GridVec) -> bool {
        self.loaded_chunks.contains(grid_coord)
    }

    pub fn is_owned(&self, grid_coord: &GridVec) -> bool {
        self.owned_chunks.contains_key(grid_coord)
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ChunkRenderHandles {
    pub quad: Handle<Mesh>,
    pub light_material: Handle<ColorMaterial>,
    pub dark_material: Handle<ColorMaterial>,
}

#[derive(Default, Resource)]
pub struct ChunkRenderExecutorRegistry {
    pub executors: HashMap<GridVec, ChunkRenderExecutor>,
}
