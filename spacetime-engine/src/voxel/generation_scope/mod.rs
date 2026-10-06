//! Decimal processing scopes for voxel generation batching.
//!
//! These scopes group independently addressable 10-cubed materialization chunks
//! only for background generation work scheduling. Runtime render and collision
//! manifestations are deliberately one-to-one with materialization chunks.

use crate::spatial::UsfPositionError;

use super::{MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationKey, VoxelRegionSpan};

/// Decimal edge length for one generation processing scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct VoxelGenerationScopeExtent {
    base_chunks_per_axis: i32,
}

impl VoxelGenerationScopeExtent {
    pub(super) fn from_base_chunks_per_axis(base_chunks_per_axis: i32) -> Option<Self> {
        let chunks_per_usf_digit = 1000 / MATERIALIZATION_CHUNK_SIZE as i32;
        (base_chunks_per_axis > 0
            && base_chunks_per_axis <= chunks_per_usf_digit
            && chunks_per_usf_digit % base_chunks_per_axis == 0)
            .then_some(Self {
                base_chunks_per_axis,
            })
    }
}

/// One aligned generation-processing scope over canonical materialization
/// addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct VoxelGenerationScope {
    region: VoxelRegionSpan,
}

impl VoxelGenerationScope {
    pub(super) fn containing(
        key: VoxelMaterializationKey,
        extent: VoxelGenerationScopeExtent,
    ) -> Result<Self, UsfPositionError> {
        Ok(Self {
            region: VoxelRegionSpan::aligned_containing(key, extent.base_chunks_per_axis)?,
        })
    }
}
