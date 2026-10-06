//! Demand ordering and capability-role work rank.

use super::*;

#[derive(Debug, Clone, Copy)]
pub(in crate::voxel::streaming) struct DemandedChunk {
    pub(in crate::voxel::streaming) key: VoxelMaterializationKey,
    pub(in crate::voxel::streaming) priority: i32,
    pub(in crate::voxel::streaming) distance_squared: f32,
    pub(in crate::voxel::streaming) trajectory_distance_squared: f32,
    // Distance from this chunk center to the nearest semantic boundary focus
    // already computed by celestial realization. Infinity means no local
    // boundary focus applies to this scope.
    pub(in crate::voxel::streaming) focus_distance_squared: f32,
    pub(in crate::voxel::streaming) role_priority: u8,
    pub(in crate::voxel::streaming) roles: UsfScaleRoleMask,
}

#[derive(Debug, Clone, Copy)]
pub(in crate::voxel) struct VoxelWorkRank {
    pub(in crate::voxel::streaming) role_priority: u8,
    pub(in crate::voxel::streaming) priority: i32,
    pub(in crate::voxel::streaming) focus_distance_squared: f32,
    pub(in crate::voxel::streaming) trajectory_distance_squared: f32,
    pub(in crate::voxel::streaming) distance_squared: f32,
}

impl DemandedChunk {
    pub(in crate::voxel::streaming) const fn work_rank(self) -> VoxelWorkRank {
        VoxelWorkRank {
            role_priority: self.role_priority,
            priority: self.priority,
            focus_distance_squared: self.focus_distance_squared,
            trajectory_distance_squared: self.trajectory_distance_squared,
            distance_squared: self.distance_squared,
        }
    }
}

pub(in crate::voxel) fn compare_work_ranks(
    a: VoxelWorkRank,
    b: VoxelWorkRank,
) -> std::cmp::Ordering {
    b.role_priority
        .cmp(&a.role_priority)
        .then_with(|| b.priority.cmp(&a.priority))
        .then_with(|| {
            a.focus_distance_squared
                .total_cmp(&b.focus_distance_squared)
        })
        .then_with(|| {
            a.trajectory_distance_squared
                .total_cmp(&b.trajectory_distance_squared)
        })
        .then_with(|| a.distance_squared.total_cmp(&b.distance_squared))
}

pub(super) fn compare_demanded_chunks(a: &DemandedChunk, b: &DemandedChunk) -> std::cmp::Ordering {
    compare_work_ranks(a.work_rank(), b.work_rank())
}
