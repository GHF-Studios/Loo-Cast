//! Dense-edit invalidation and derived surface-build lifecycle.

use super::*;

impl VoxelMaterializationStore {
    pub(in crate::voxel) fn apply_edit(
        &mut self,
        address: VoxelMaterializationChunkAddress,
        edit: VoxelEdit,
    ) -> bool {
        let mut changed = false;
        let mut active = false;
        if let Some(entry) = self.entries.get_mut(&address) {
            active = entry.active;
            if let Some(chunk) = entry.dense_mut() {
                changed = chunk.apply_edit(address, edit).changed();
            }
        }

        if changed && active {
            self.mark_derived_dirty(address);
        }
        changed
    }

    pub(in crate::voxel) fn pop_dirty_derived(&mut self) -> Option<VoxelMaterializationChunkAddress> {
        while let Some(address) = self.dirty_derived.pop_front() {
            if self.dirty_derived_set.remove(&address) {
                return Some(address);
            }
        }
        None
    }

    pub(in crate::voxel) fn begin_surface_build(
        &mut self,
        address: VoxelMaterializationChunkAddress,
    ) -> Option<(u64, VoxelChunk)> {
        let entry = self.entries.get_mut(&address)?;
        if !entry.active {
            return None;
        }
        let chunk = entry.dense()?;
        let revision = chunk.revision();
        if entry.derived_revision == Some(revision) || entry.derived_in_flight == Some(revision) {
            return None;
        }

        let snapshot = chunk.clone();
        entry.derived_in_flight = Some(revision);
        Some((revision, snapshot))
    }

    /// Publishes one derived surface cache. Empty surfaces are represented by
    /// `None` while `derived_revision` records that the revision was processed.
    pub(in crate::voxel) fn publish_surface(
        &mut self,
        address: VoxelMaterializationChunkAddress,
        revision: u64,
        surface: Option<VoxelSurfaceCache>,
    ) -> bool {
        let mut stale_active = false;
        let mut published = false;

        if let Some(entry) = self.entries.get_mut(&address) {
            let current_revision = entry.dense().map(VoxelChunk::revision);
            if current_revision == Some(revision) {
                entry.surface = surface;
                entry.derived_revision = Some(revision);
                entry.derived_in_flight = None;
                if let Some(chunk) = entry.dense_mut() {
                    chunk.mark_meshed();
                }
                published = true;
            } else {
                if entry.derived_in_flight == Some(revision) {
                    entry.derived_in_flight = None;
                }
                stale_active = entry.active && current_revision.is_some();
            }
        }

        if published {
            self.mark_render_dirty(address);
        } else if stale_active {
            self.mark_derived_dirty(address);
        }
        published
    }
}
