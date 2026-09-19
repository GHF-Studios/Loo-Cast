//! Immutable dense-generation recipe captured from semantic voxel state.

use super::*;

/// Immutable background-generation recipe for one dense local chunk.
///
/// The recipe contains canonical semantic address + semantic edits only. No
/// runtime frame coordinate or giant flat voxel lattice survives into worker
/// generation.
#[derive(Debug, Clone)]
pub(in crate::voxel) struct VoxelChunkRecipe {
    pub(super) address: VoxelMaterializationChunkAddress,
    pub(super) world_origin: VoxelQueryPosition,
    pub(super) base: VoxelBase,
    pub(super) edits: Vec<VoxelEdit>,
    pub(super) applied_edit_count: usize,
}

impl VoxelChunkRecipe {
    pub(in crate::voxel) const fn applied_edit_count(&self) -> usize {
        self.applied_edit_count
    }

    pub(in crate::voxel) fn materialize(self) -> VoxelChunk {
        let Self {
            address,
            world_origin,
            base,
            edits,
            ..
        } = self;
        let anchor = address.query_origin();
        let sampler = base.prepare_chunk_sampler(world_origin, anchor);
        let extra_extent = MATERIALIZATION_CHUNK_SIZE as f32 + SAMPLE_PADDING as f32;
        let local_edits = edits
            .into_iter()
            .filter_map(|edit| edit.localized(anchor, extra_extent))
            .collect::<Vec<_>>();

        VoxelChunk::generate(move |local_point| {
            let mut sample = sampler.sample(local_point);
            for edit in &local_edits {
                sample = edit.apply_to_sample(local_point, sample);
            }
            sample
        })
    }
}
