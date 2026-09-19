//! Compact residency and cache storage for voxel materializations.
//!
//! A canonical materialization address has identity without requiring a Bevy
//! entity. This store owns the lifecycle of dense working data and derived CPU
//! caches. ECS is reserved for transient jobs and runtime manifestations.

use std::collections::{HashMap, HashSet, VecDeque};

use super::{VoxelChunk, VoxelEdit, VoxelMaterializationChunkAddress, mesh::VoxelSurface};

/// Derived surface cache for one independently addressable materialization.
///
/// The cache may remain warm in RAM while the materialization is inactive.
/// Runtime render/physics manifestations are built separately.
#[derive(Debug)]
pub(crate) struct VoxelSurfaceCache {
    pub(crate) revision: u64,
    pub(crate) surface: VoxelSurface,
    pub(crate) debug_color: [f32; 4],
}

impl VoxelSurfaceCache {
    pub(crate) fn new(revision: u64, surface: VoxelSurface, debug_color: [f32; 4]) -> Self {
        Self {
            revision,
            surface,
            debug_color,
        }
    }
}

#[derive(Debug)]
enum VoxelMaterializationState {
    Pending { token: u64 },
    Dense(VoxelChunk),
}

#[derive(Debug)]
struct VoxelMaterializationEntry {
    active: bool,
    state: VoxelMaterializationState,
    derived_revision: Option<u64>,
    surface: Option<VoxelSurfaceCache>,
    derived_in_flight: Option<u64>,
}

impl VoxelMaterializationEntry {
    fn dense(&self) -> Option<&VoxelChunk> {
        match &self.state {
            VoxelMaterializationState::Dense(chunk) => Some(chunk),
            VoxelMaterializationState::Pending { .. } => None,
        }
    }

    fn dense_mut(&mut self) -> Option<&mut VoxelChunk> {
        match &mut self.state {
            VoxelMaterializationState::Dense(chunk) => Some(chunk),
            VoxelMaterializationState::Pending { .. } => None,
        }
    }
}

/// Sparse materialization residency + warm-cache store.
///
/// `active` means demanded by the current realization window. Inactive dense
/// entries are warm RAM cache: they cost no ECS/render/physics participation and
/// can be reactivated without regenerating the semantic field.
#[derive(Debug, Default)]
pub(crate) struct VoxelMaterializationStore {
    entries: HashMap<VoxelMaterializationChunkAddress, VoxelMaterializationEntry>,
    next_generation_token: u64,
    dirty_derived: VecDeque<VoxelMaterializationChunkAddress>,
    dirty_derived_set: HashSet<VoxelMaterializationChunkAddress>,
    dirty_render: VecDeque<VoxelMaterializationChunkAddress>,
    dirty_render_set: HashSet<VoxelMaterializationChunkAddress>,
    inactive_lru: VecDeque<VoxelMaterializationChunkAddress>,
}

mod derived;
mod metrics;
mod render;
mod residency;

impl VoxelMaterializationStore {
    fn mark_derived_dirty(&mut self, address: VoxelMaterializationChunkAddress) {
        if self.dirty_derived_set.insert(address) {
            self.dirty_derived.push_back(address);
        }
    }

    fn mark_render_dirty(&mut self, address: VoxelMaterializationChunkAddress) {
        if self.dirty_render_set.insert(address) {
            self.dirty_render.push_back(address);
        }
    }
}
