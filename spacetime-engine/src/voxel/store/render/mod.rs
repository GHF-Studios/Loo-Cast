//! Render-dirty queueing and surface-cache access.

use super::*;

impl VoxelMaterializationStore {
    pub(crate) fn pop_dirty_render(&mut self) -> Option<VoxelMaterializationChunkAddress> {
        while let Some(address) = self.dirty_render.pop_front() {
            if self.dirty_render_set.remove(&address) {
                return Some(address);
            }
        }
        None
    }

    /// Re-enqueues every active surface after manifestation policy changes.
    ///
    /// This is intentionally O(active surfaces), but only runs when runtime
    /// configuration changes grouping topology. Steady state stays change-driven.
    pub(crate) fn mark_all_active_render_dirty(&mut self) {
        let addresses = self
            .entries
            .iter()
            .filter_map(|(&address, entry)| {
                (entry.active && entry.surface.is_some()).then_some(address)
            })
            .collect::<Vec<_>>();

        for address in addresses {
            self.mark_render_dirty(address);
        }
    }

    pub(crate) fn active_surface(
        &self,
        address: VoxelMaterializationChunkAddress,
    ) -> Option<&VoxelSurfaceCache> {
        let entry = self.entries.get(&address)?;
        entry.active.then_some(())?;
        entry.surface.as_ref()
    }

    pub(crate) fn surface(
        &self,
        address: VoxelMaterializationChunkAddress,
    ) -> Option<&VoxelSurfaceCache> {
        self.entries.get(&address)?.surface.as_ref()
    }
}
