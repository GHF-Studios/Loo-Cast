//! Decimal processing scopes for voxel generation batching.
//!
//! These scopes group independently addressable 10-cubed materialization chunks
//! only for background generation work scheduling. Runtime render and collision
//! manifestations are deliberately one-to-one with materialization chunks.

use bevy::prelude::IVec3;

use crate::spatial::UsfPositionError;

use super::{MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationKey};

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
    origin: VoxelMaterializationKey,
}

impl VoxelGenerationScope {
    pub(super) fn containing(
        key: VoxelMaterializationKey,
        extent: VoxelGenerationScopeExtent,
    ) -> Result<Self, UsfPositionError> {
        let [x, y, z] = key.components();
        let edge = i64::from(extent.base_chunks_per_axis);
        let remainder = IVec3::new(
            x.rem_euclid(edge) as i32,
            y.rem_euclid(edge) as i32,
            z.rem_euclid(edge) as i32,
        );
        Ok(Self { origin: key.translated_chunks(-remainder)? })
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{IVec3, Vec3};

    use crate::{
        spatial::UsfPosition,
        voxel::{VoxelBase, VoxelChunkCoord, VoxelWorld},
    };

    use super::*;

    fn extent(base_chunks_per_axis: i32) -> VoxelGenerationScopeExtent {
        VoxelGenerationScopeExtent::from_base_chunks_per_axis(base_chunks_per_axis)
            .expect("test extent must satisfy canonical generation-scope alignment")
    }

    #[test]
    fn generation_extent_accepts_current_alignment_period_divisors() {
        assert_eq!(
            VoxelGenerationScopeExtent::from_base_chunks_per_axis(20)
                .unwrap()
                .base_chunks_per_axis,
            20
        );
        assert!(VoxelGenerationScopeExtent::from_base_chunks_per_axis(3).is_none());
        assert!(VoxelGenerationScopeExtent::from_base_chunks_per_axis(101).is_none());
    }

    #[test]
    fn generation_scope_alignment_is_relative_to_the_voxel_world_grid() {
        let origin = UsfPosition::from_scale0_local(Vec3::new(499.25, -123.5, 17.75)).unwrap();
        let world = VoxelWorld::new_at(VoxelBase::Empty, origin);
        let address = world
            .chunk_address(VoxelChunkCoord::new(IVec3::new(137, -204, 99)))
            .unwrap();

        let hundred_extent = extent(10);
        let key = world.materialization_key(address).unwrap();
        let hundred = VoxelGenerationScope::containing(key, hundred_extent).unwrap();
        let expected = world
            .chunk_address(VoxelChunkCoord::new(IVec3::new(130, -210, 90)))
            .unwrap();
        let expected_key = world.materialization_key(expected).unwrap();

        assert_eq!(hundred.origin, expected_key);
        assert_eq!(
            hundred_extent.base_chunks_per_axis * MATERIALIZATION_CHUNK_SIZE as i32,
            100
        );
    }
}
