//! Demand-driven residency of voxel materialization caches.
//!
//! Streaming owns *which canonical addresses are active*. Dense voxel data lives
//! in [`super::VoxelWorld`]'s compact materialization store rather than in one ECS entity
//! per address. Generation jobs are transient ECS participants only.

use std::collections::{HashSet, VecDeque};

use bevy::prelude::*;

use super::VoxelMaterializationChunkAddress;
use demand::{DemandedChunk, VoxelDemandPlanKey};

mod demand;
mod generation;

pub(super) use demand::refresh_voxel_residency;
pub(super) use generation::{
    finish_chunk_generation, retire_orphaned_tasks, schedule_voxel_generation,
};

/// Demand-streaming policy for one [`super::VoxelWorld`].
///
/// Presentation material is intentionally separate: residency policy should not
/// own renderer state, and manually resident worlds can use the same realization
/// pipeline without pretending to be streamed.
#[derive(Component, Debug, Clone)]
pub struct VoxelStreaming {
    load_budget_per_frame: usize,
    demand_key: Vec<VoxelDemandPlanKey>,
    pending_desired: VecDeque<DemandedChunk>,
    cached_desired_set: HashSet<VoxelMaterializationChunkAddress>,
}

impl VoxelStreaming {
    pub fn new(load_budget_per_frame: usize) -> Self {
        Self {
            load_budget_per_frame: load_budget_per_frame.max(1),
            demand_key: Vec::new(),
            pending_desired: VecDeque::new(),
            cached_desired_set: HashSet::new(),
        }
    }

    pub const fn load_budget_per_frame(&self) -> usize {
        self.load_budget_per_frame
    }
}

/// Render material used by disposable voxel manifestations.
#[derive(Component, Debug, Clone)]
pub struct VoxelPresentationMaterial(Handle<StandardMaterial>);

impl VoxelPresentationMaterial {
    pub fn new(material: Handle<StandardMaterial>) -> Self {
        Self(material)
    }

    pub(super) fn handle(&self) -> &Handle<StandardMaterial> {
        &self.0
    }
}

/// Marks a generic [`crate::spatial::SpatialDemandSource`] as requesting voxel
/// materialization.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct VoxelMaterializationDemand;

#[cfg(test)]
mod tests;
