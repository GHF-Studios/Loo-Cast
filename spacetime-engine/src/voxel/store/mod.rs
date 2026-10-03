//! Compact residency and cache storage for voxel materializations.
//!
//! A canonical materialization address has identity without requiring a Bevy
//! entity. This store owns the lifecycle of dense working data and derived CPU
//! caches. ECS is reserved for transient jobs and runtime manifestations.

use std::collections::{HashMap, HashSet, VecDeque};

use super::{
    VoxelChunk, VoxelEdit, VoxelMaterializationChunkAddress,
    VoxelMaterializationKey, mesh::VoxelSurface,
};

/// Derived surface cache for one independently addressable materialization.
///
/// The cache may remain warm in RAM while the materialization is inactive.
/// Runtime render/physics manifestations are built separately.
#[derive(Debug)]
pub(super) struct VoxelSurfaceCache {
    pub(super) revision: u64,
    pub(super) surface: VoxelSurface,
    pub(super) debug_color: [f32; 4],
}

impl VoxelSurfaceCache {
    pub(super) fn new(revision: u64, surface: VoxelSurface, debug_color: [f32; 4]) -> Self {
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
pub(super) struct VoxelMaterializationStore {
    entries: HashMap<VoxelMaterializationKey, VoxelMaterializationEntry>,
    next_generation_token: u64,
    capability_revision: u64,
    dirty_derived: VecDeque<VoxelMaterializationKey>,
    dirty_derived_set: HashSet<VoxelMaterializationKey>,
    dirty_render: VecDeque<VoxelMaterializationKey>,
    dirty_render_set: HashSet<VoxelMaterializationKey>,
    inactive_lru: VecDeque<VoxelMaterializationKey>,
    /// Dense warm-cache entries currently inactive. Tracked explicitly so
    /// reconciliation never rescans an ever-growing cache just to enforce its bound.
    inactive_count: usize,
}

mod derived;
mod metrics;
mod render;
mod residency;

impl VoxelMaterializationStore {
    fn bump_capability_revision(&mut self) {
        self.capability_revision = self.capability_revision.wrapping_add(1).max(1);
    }

    pub(in crate::voxel) const fn capability_revision(&self) -> u64 {
        self.capability_revision
    }

    fn mark_derived_dirty(&mut self, key: VoxelMaterializationKey) {
        if self.dirty_derived_set.insert(key) {
            self.dirty_derived.push_back(key);
        }
    }

    fn mark_render_dirty(&mut self, key: VoxelMaterializationKey) {
        if self.dirty_render_set.insert(key) {
            self.dirty_render.push_back(key);
        }
    }


    // role-refresh-render-membership-v1
    pub(in crate::voxel) fn refresh_render_membership(
        &mut self,
        key: VoxelMaterializationKey,
    ) {
        if self.entries.get(&key).is_some_and(|entry| entry.active) {
            self.mark_render_dirty(key);
        }
    }
}
