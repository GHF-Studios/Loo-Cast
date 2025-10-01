use bevy::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{chunk::types::ChunkOwnerId, gpu::workflows::gpu::generate_chunk_textures::user_items::ChunkRenderExecutor};
use crate::chunk::types::GridCoord;
use crate::utils::types::I128Vec2;

use super::intent::{ActionIntent, ActionPriority};

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ActionIntentCommitBuffer {
    pub action_intent: HashMap<GridCoord, ActionIntent>,
    pub priority_buckets: BTreeMap<ActionPriority, HashSet<GridCoord>>,
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

    pub fn remove_intent(&mut self, coord: &GridCoord) {
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

    pub fn remove_intents(&mut self, coords: impl IntoIterator<Item = GridCoord>) {
        for coord in coords {
            self.remove_intent(&coord);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&GridCoord, &ActionIntent)> {
        self.priority_buckets
            .iter()
            .flat_map(|(_, coords)| coords.iter())
            .filter_map(|coord| self.action_intent.get_key_value(coord))
    }

    pub fn get(&self, coord: &GridCoord) -> Option<&ActionIntent> {
        self.action_intent.get(coord)
    }
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ActionIntentBuffer {
    pub action_intents: HashMap<GridCoord, ActionIntent>,
    pub priority_buckets: BTreeMap<ActionPriority, HashSet<GridCoord>>,
}
impl ActionIntentBuffer {
    pub fn buffer_intent(&mut self, action_intent: ActionIntent) {
        let coord = action_intent.coord();
        let priority = action_intent.priority();

        self.action_intents.insert(coord, action_intent);
        self.priority_buckets.entry(priority).or_default().insert(coord);
    }

    pub fn cancel_intent(&mut self, coord: &GridCoord) {
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

    pub fn get(&self, coord: &GridCoord) -> Option<&ActionIntent> {
        self.action_intents.get(coord)
    }
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct ChunkManager {
    pub loaded_chunks: HashSet<GridCoord>,
    pub owned_chunks: HashMap<GridCoord, ChunkOwnerId>,
}
impl ChunkManager {
    pub fn get_states(&self, grid_coord: &GridCoord) -> (bool, bool) {
        (self.loaded_chunks.contains(grid_coord), self.owned_chunks.contains_key(grid_coord))
    }

    pub fn is_loaded(&self, grid_coord: &GridCoord) -> bool {
        self.loaded_chunks.contains(grid_coord)
    }

    pub fn is_owned(&self, grid_coord: &GridCoord) -> bool {
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
    pub executors: HashMap<GridCoord, ChunkRenderExecutor>,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GridOriginOffset(pub I128Vec2);