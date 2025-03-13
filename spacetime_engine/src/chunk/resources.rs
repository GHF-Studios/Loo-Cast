use bevy::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};

use super::enums::{ChunkWorkflow, ChunkWorkflowPriority};

#[derive(Resource, Default)]
pub(crate) struct ChunkWorkflowBuffer {
    pub workflows: HashMap<(i32, i32), ChunkWorkflow>,
    pub priority_buckets: BTreeMap<ChunkWorkflowPriority, HashSet<(i32, i32)>>,
}

impl ChunkWorkflowBuffer {
    pub fn add_workflow(&mut self, workflow: ChunkWorkflow) {
        let coord = workflow.get_coord();
        let priority = workflow.get_priority();

        self.workflows.insert(coord, workflow);

        self.priority_buckets
            .entry(priority)
            .or_default()
            .insert(coord);
    }

    pub fn add_workflows<I>(&mut self, workflows: I)
    where
        I: IntoIterator<Item = ChunkWorkflow>,
    {
        for workflow in workflows {
            let coord = workflow.get_coord();
            let priority = workflow.get_priority();

            // Add to the workflows map
            self.workflows.insert(coord, workflow);

            // Add to the priority bucket
            self.priority_buckets
                .entry(priority)
                .or_default()
                .insert(coord);
        }
    }

    pub fn remove_workflow(&mut self, coord: &(i32, i32)) {
        if let Some(workflow) = self.workflows.remove(coord) {
            let priority = workflow.get_priority();

            if let Some(bucket) = self.priority_buckets.get_mut(&priority) {
                bucket.remove(coord);
                if bucket.is_empty() {
                    self.priority_buckets.remove(&priority);
                }
            }
        }
    }

    pub fn remove_workflows<I>(&mut self, coords: I)
    where
        I: IntoIterator<Item = (i32, i32)>,
    {
        for coord in coords {
            // Remove from the workflows map
            if let Some(workflow) = self.workflows.remove(&coord) {
                let priority = workflow.get_priority();

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

    pub fn get(&self, chunk_coord: &(i32, i32)) -> Option<&ChunkWorkflow> {
        self.workflows.get(chunk_coord)
    }

    pub fn get_workflow_states(&self, chunk_coord: &(i32, i32)) -> (bool, bool, bool) {
        match self.get(chunk_coord) {
            Some(workflow) => match workflow {
                ChunkWorkflow::Spawn { .. } => (true, false, false),
                ChunkWorkflow::Despawn { .. } => (false, true, false),
                ChunkWorkflow::TransferOwnership { .. } => (false, false, true),
            },
            None => (false, false, false),
        }
    }

    pub fn is_spawning(&self, chunk_coord: &(i32, i32)) -> bool {
        matches!(self.get(chunk_coord), Some(ChunkWorkflow::Spawn { .. }))
    }

    pub fn is_despawning(&self, chunk_coord: &(i32, i32)) -> bool {
        matches!(self.get(chunk_coord), Some(ChunkWorkflow::Despawn { .. }))
    }

    pub fn is_transfering_ownership(&self, chunk_coord: &(i32, i32)) -> bool {
        matches!(
            self.get(chunk_coord),
            Some(ChunkWorkflow::TransferOwnership { .. })
        )
    }

    pub fn has_spawns(&self) -> bool {
        self.workflows.values().any(|workflow| workflow.is_spawn())
    }

    pub fn has_despawns(&self) -> bool {
        self.workflows
            .values()
            .any(|workflow| workflow.is_despawn())
    }

    pub fn has_ownership_transfers(&self) -> bool {
        self.workflows
            .values()
            .any(|workflow| workflow.is_transfer_ownership())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&(i32, i32), &ChunkWorkflow)> {
        self.priority_buckets
            .iter()
            .flat_map(|(_, coords)| coords.iter())
            .filter_map(|coord| self.workflows.get_key_value(coord))
    }
}

#[derive(Resource, Default, Debug)]
pub(crate) struct ChunkManager {
    pub loaded_chunks: HashSet<(i32, i32)>,
    pub owned_chunks: HashMap<(i32, i32), Entity>,
}
impl ChunkManager {
    pub fn get_states(&self, chunk_coord: &(i32, i32)) -> (bool, bool) {
        (
            self.loaded_chunks.contains(chunk_coord),
            self.owned_chunks.contains_key(chunk_coord),
        )
    }

    pub fn is_loaded(&self, chunk_coord: &(i32, i32)) -> bool {
        self.loaded_chunks.contains(chunk_coord)
    }

    pub fn is_owned(&self, chunk_coord: &(i32, i32)) -> bool {
        self.owned_chunks.contains_key(chunk_coord)
    }
}

#[derive(Resource)]
pub(crate) struct ChunkRenderHandles {
    pub quad: Handle<Mesh>,
    pub light_material: Handle<ColorMaterial>,
    pub dark_material: Handle<ColorMaterial>,
}
