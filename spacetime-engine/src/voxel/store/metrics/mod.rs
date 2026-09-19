//! Residency/cache counts exposed to voxel diagnostics.

use super::*;

impl VoxelMaterializationStore {
    pub(crate) fn resident_count(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn active_count(&self) -> usize {
        self.entries.values().filter(|entry| entry.active).count()
    }

    pub(crate) fn inactive_count(&self) -> usize {
        self.entries.values().filter(|entry| !entry.active).count()
    }

    pub(crate) fn pending_count(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| matches!(&entry.state, VoxelMaterializationState::Pending { .. }))
            .count()
    }

    pub(crate) fn dense_count(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| matches!(&entry.state, VoxelMaterializationState::Dense(_)))
            .count()
    }

    pub(crate) fn active_dense_count(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| {
                entry.active && matches!(&entry.state, VoxelMaterializationState::Dense(_))
            })
            .count()
    }

    pub(crate) fn active_surface_count(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| entry.active && entry.surface.is_some())
            .count()
    }

    pub(crate) fn surface_cache_count(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| entry.surface.is_some())
            .count()
    }

    pub(crate) fn dirty_derived_count(&self) -> usize {
        self.dirty_derived_set.len()
    }
}
