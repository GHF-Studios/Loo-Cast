//! Spatial-demand interpretation and voxel residency reconciliation.
//!
//! ## Module map
//!
//! - `motion`: Quantized predictive motion as work priority, not authority.
//! - `plan`: Bounded chunk plan, region culling and predictive tube selection.
//! - `rank`: Demand ordering and capability-role work rank.
//! - `systems`: ECS residency reconciliation and hot/warm transitions.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use bevy::math::DVec3;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    reconstructible::{ReconstructibleFrameBudget, ReconstructibleWorkClass},
    spatial::{
        SpatialDemandMotionSnapshot, SpatialDemandScope, SpatialRealizationGranularityRequest,
        SpatialScale, UsfCapabilityRealization, UsfChunkAddress, UsfContextResidency, UsfPosition,
        UsfPositionError, UsfScaleLayer, UsfScaleRoleMask, UsfViewDemandSnapshot,
    },
};

use super::VoxelMaterializationResidency;

use super::super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelCollisionDisabled, VoxelEditingDisabled,
    VoxelMaterializationKey, VoxelQueryPosition, VoxelRealizationDemandSnapshot,
    VoxelRealizationScope, VoxelRegionSpan, VoxelScaleRealization,
    manifestation::VoxelPresentationManifestation,
    worker::{VoxelWorkExecutor, VoxelWorkLane},
};

mod motion;
mod plan;
mod rank;
mod systems;

use motion::{
    MOVING_PLAN_CENTER_HOLD_CHUNKS, VoxelDemandMotion, VoxelMotionPriorityKey, quantized_motion_key,
};
use plan::*;
pub(super) use rank::DemandedChunk;
use rank::compare_demanded_chunks;
pub(in crate::voxel) use rank::{VoxelWorkRank, compare_work_ranks};
pub(in crate::voxel) use systems::reconcile_voxel_materialization_residency;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VoxelDemandPlanKey {
    source: Entity,
    center_key: VoxelMaterializationKey,
    minimum: IVec3,
    maximum: IVec3,
    priority: i32,
    roles: u16,
    view_revision: u64,
    motion: VoxelMotionPriorityKey,
    validity_chunks: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VoxelMaterializationBox {
    minimum: [i64; 3],
    maximum: [i64; 3],
}

impl VoxelMaterializationBox {
    fn from_plan(plan: VoxelDemandPlanKey) -> Option<Self> {
        let center = plan.center_key.components();
        Some(Self {
            minimum: [
                center[0].checked_add(i64::from(plan.minimum.x))?,
                center[1].checked_add(i64::from(plan.minimum.y))?,
                center[2].checked_add(i64::from(plan.minimum.z))?,
            ],
            maximum: [
                center[0].checked_add(i64::from(plan.maximum.x))?,
                center[1].checked_add(i64::from(plan.maximum.y))?,
                center[2].checked_add(i64::from(plan.maximum.z))?,
            ],
        })
    }

    fn intersection(self, other: Self) -> Option<Self> {
        let minimum = [
            self.minimum[0].max(other.minimum[0]),
            self.minimum[1].max(other.minimum[1]),
            self.minimum[2].max(other.minimum[2]),
        ];
        let maximum = [
            self.maximum[0].min(other.maximum[0]),
            self.maximum[1].min(other.maximum[1]),
            self.maximum[2].min(other.maximum[2]),
        ];
        (minimum[0] <= maximum[0] && minimum[1] <= maximum[1] && minimum[2] <= maximum[2])
            .then_some(Self { minimum, maximum })
    }

    fn for_each(self, mut visit: impl FnMut(VoxelMaterializationKey)) {
        for z in self.minimum[2]..=self.maximum[2] {
            for y in self.minimum[1]..=self.maximum[1] {
                for x in self.minimum[0]..=self.maximum[0] {
                    visit(VoxelMaterializationKey::new([x, y, z]));
                }
            }
        }
    }

    fn for_each_difference(self, subtract: Self, mut visit: impl FnMut(VoxelMaterializationKey)) {
        let Some(intersection) = self.intersection(subtract) else {
            self.for_each(visit);
            return;
        };

        let slab = |minimum: [i64; 3],
                    maximum: [i64; 3],
                    visit: &mut dyn FnMut(VoxelMaterializationKey)| {
            if minimum[0] > maximum[0] || minimum[1] > maximum[1] || minimum[2] > maximum[2] {
                return;
            }
            VoxelMaterializationBox { minimum, maximum }.for_each(visit);
        };

        slab(
            self.minimum,
            [
                intersection.minimum[0] - 1,
                self.maximum[1],
                self.maximum[2],
            ],
            &mut visit,
        );
        slab(
            [
                intersection.maximum[0] + 1,
                self.minimum[1],
                self.minimum[2],
            ],
            self.maximum,
            &mut visit,
        );

        let middle_x = [intersection.minimum[0], intersection.maximum[0]];
        slab(
            [middle_x[0], self.minimum[1], self.minimum[2]],
            [middle_x[1], intersection.minimum[1] - 1, self.maximum[2]],
            &mut visit,
        );
        slab(
            [middle_x[0], intersection.maximum[1] + 1, self.minimum[2]],
            [middle_x[1], self.maximum[1], self.maximum[2]],
            &mut visit,
        );

        let middle_y = [intersection.minimum[1], intersection.maximum[1]];
        slab(
            [middle_x[0], middle_y[0], self.minimum[2]],
            [middle_x[1], middle_y[1], intersection.minimum[2] - 1],
            &mut visit,
        );
        slab(
            [middle_x[0], middle_y[0], intersection.maximum[2] + 1],
            [middle_x[1], middle_y[1], self.maximum[2]],
            &mut visit,
        );
    }
}
