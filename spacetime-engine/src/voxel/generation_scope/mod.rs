//! Decimal processing scopes for voxel generation batching.
//!
//! These scopes group independently addressable 10-cubed materialization chunks
//! only for background generation work scheduling. Runtime render and collision
//! manifestations are deliberately one-to-one with materialization chunks.

use bevy::prelude::IVec3;

use crate::spatial::UsfPositionError;

use super::{MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationChunkAddress, VoxelWorld};

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
    origin: VoxelMaterializationChunkAddress,
}

impl VoxelGenerationScope {
    pub(super) fn containing(
        world: &VoxelWorld,
        address: VoxelMaterializationChunkAddress,
        extent: VoxelGenerationScopeExtent,
    ) -> Result<Self, UsfPositionError> {
        if address.origin().leaf_scale() != world.origin().leaf_scale() {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }

        let chunk_size = MATERIALIZATION_CHUNK_SIZE as f32;
        let offset_delta = address.origin().offset() - world.origin().offset();
        let phase = IVec3::new(
            phase_chunks(offset_delta.x, chunk_size),
            phase_chunks(offset_delta.y, chunk_size),
            phase_chunks(offset_delta.z, chunk_size),
        );
        let edge = extent.base_chunks_per_axis;
        let remainder = IVec3::new(
            phase.x.rem_euclid(edge),
            phase.y.rem_euclid(edge),
            phase.z.rem_euclid(edge),
        );

        Ok(Self {
            origin: address.translated_chunks(-remainder)?,
        })
    }

}

fn phase_chunks(delta_native: f32, chunk_size: f32) -> i32 {
    let chunks = delta_native / chunk_size;
    debug_assert!((chunks - chunks.round()).abs() < 0.001);
    chunks.round() as i32
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{IVec3, Vec3};

    use crate::{
        spatial::UsfPosition,
        voxel::{VoxelBase, VoxelChunkCoord},
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
        let hundred = VoxelGenerationScope::containing(
            &world,
            address,
            hundred_extent,
        )
        .unwrap();

        assert_eq!(
            hundred.origin,
            world
                .chunk_address(VoxelChunkCoord::new(IVec3::new(130, -210, 90)))
                .unwrap()
        );
        assert_eq!(
            hundred_extent.base_chunks_per_axis * MATERIALIZATION_CHUNK_SIZE as i32,
            100
        );
    }
}
