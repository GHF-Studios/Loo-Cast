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
