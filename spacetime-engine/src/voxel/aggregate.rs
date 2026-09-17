//! Decimal processing scopes over canonical voxel materialization addresses.
//!
//! These scopes are deliberately representation-local. They group already
//! requested `10³` base materializations into useful work/cache neighborhoods;
//! they do not create another semantic chunk hierarchy and do not imply that the
//! whole scope is allocated.

use bevy::prelude::IVec3;

use crate::spatial::UsfPositionError;

use super::{MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationChunkAddress, VoxelWorld};

/// Decimal edge length for one materialization processing scope.
///
/// The stored value is a count of `10³` base chunks per axis. Consequently `10`
/// means a `100³` native-unit scope and `100` means a `1000³` scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct VoxelMaterializationAggregateExtent {
    base_chunks_per_axis: i32,
}

impl VoxelMaterializationAggregateExtent {
    pub(crate) const HUNDRED: Self = Self {
        base_chunks_per_axis: 10,
    };

    #[allow(dead_code)]
    pub(crate) const THOUSAND: Self = Self {
        base_chunks_per_axis: 100,
    };

    pub(crate) const fn base_chunks_per_axis(self) -> i32 {
        self.base_chunks_per_axis
    }

    pub(crate) const fn native_units_per_axis(self) -> u32 {
        self.base_chunks_per_axis as u32 * MATERIALIZATION_CHUNK_SIZE
    }
}

/// One aligned aggregate processing scope over canonical base materializations.
///
/// The origin is still an ordinary base-chunk address. No registry entry or
/// dense allocation is created merely because this value exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct VoxelMaterializationAggregateScope {
    origin: VoxelMaterializationChunkAddress,
    extent: VoxelMaterializationAggregateExtent,
}

impl VoxelMaterializationAggregateScope {
    /// Finds the decimal aggregate containing `address`, aligned to this voxel
    /// world's materialization grid rather than to a universe-wide integer
    /// lattice.
    pub(crate) fn containing(
        world: &VoxelWorld,
        address: VoxelMaterializationChunkAddress,
        extent: VoxelMaterializationAggregateExtent,
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
        let edge = extent.base_chunks_per_axis();
        let remainder = IVec3::new(
            phase.x.rem_euclid(edge),
            phase.y.rem_euclid(edge),
            phase.z.rem_euclid(edge),
        );

        Ok(Self {
            origin: address.translated_chunks(-remainder)?,
            extent,
        })
    }

    pub(crate) const fn extent(self) -> VoxelMaterializationAggregateExtent {
        self.extent
    }
}

fn phase_chunks(delta_native: f32, chunk_size: f32) -> i32 {
    // Every canonical base address is separated from its VoxelWorld origin by
    // whole base chunks. Normalization may move multiples of 1000 native units
    // into USF digits, but both supported aggregate extents divide 1000 exactly;
    // therefore the bounded leaf-offset difference contains all alignment phase
    // information needed here.
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

    #[test]
    fn decimal_aggregate_alignment_is_relative_to_the_voxel_world_grid() {
        let origin = UsfPosition::from_scale0_local(Vec3::new(499.25, -123.5, 17.75)).unwrap();
        let world = VoxelWorld::new_at(VoxelBase::Empty, origin);
        let address = world
            .chunk_address(VoxelChunkCoord::new(IVec3::new(137, -204, 99)))
            .unwrap();

        let hundred = VoxelMaterializationAggregateScope::containing(
            &world,
            address,
            VoxelMaterializationAggregateExtent::HUNDRED,
        )
        .unwrap();
        let thousand = VoxelMaterializationAggregateScope::containing(
            &world,
            address,
            VoxelMaterializationAggregateExtent::THOUSAND,
        )
        .unwrap();

        assert_eq!(
            hundred.origin,
            world
                .chunk_address(VoxelChunkCoord::new(IVec3::new(130, -210, 90)))
                .unwrap()
        );
        assert_eq!(
            thousand.origin,
            world
                .chunk_address(VoxelChunkCoord::new(IVec3::new(100, -300, 0)))
                .unwrap()
        );
    }

    #[test]
    fn hundred_scopes_compose_inside_thousand_scopes() {
        let world = VoxelWorld::new(VoxelBase::Empty);
        let address = world
            .chunk_address(VoxelChunkCoord::new(IVec3::new(-137, 204, 99)))
            .unwrap();
        let hundred = VoxelMaterializationAggregateScope::containing(
            &world,
            address,
            VoxelMaterializationAggregateExtent::HUNDRED,
        )
        .unwrap();
        let thousand = VoxelMaterializationAggregateScope::containing(
            &world,
            address,
            VoxelMaterializationAggregateExtent::THOUSAND,
        )
        .unwrap();
        let hundred_parent = VoxelMaterializationAggregateScope::containing(
            &world,
            hundred.origin,
            VoxelMaterializationAggregateExtent::THOUSAND,
        )
        .unwrap();

        assert_eq!(hundred_parent, thousand);
        assert_eq!(hundred.extent().native_units_per_axis(), 100);
        assert_eq!(thousand.extent().native_units_per_axis(), 1000);
    }
}
