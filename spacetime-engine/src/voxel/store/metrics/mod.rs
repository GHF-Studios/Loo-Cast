//! Residency count used by the public voxel-world facade.

use super::*;

impl VoxelMaterializationStore {
    pub(in crate::voxel) fn active_count(&self) -> usize {
        self.entries.values().filter(|entry| entry.active).count()
    }
}
