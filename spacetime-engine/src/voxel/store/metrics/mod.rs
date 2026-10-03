//! Residency count used by the public voxel-world facade.

use super::*;

impl VoxelMaterializationStore {
    pub(in crate::voxel) fn active_count(&self) -> usize {
        self.entries.values().filter(|entry| entry.active).count()
    }

    // warm-store-retirement-metrics-v1
    pub(in crate::voxel) const fn inactive_count(&self) -> usize {
        self.inactive_count
    }

    pub(in crate::voxel) fn total_count(&self) -> usize {
        self.entries.len()
    }
}
