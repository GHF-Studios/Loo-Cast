//! Demand-driven residency of voxel materialization caches.
//!
//! Streaming owns *which canonical addresses are active*. Dense voxel data lives
//! in [`super::VoxelWorld`]'s compact materialization store rather than in one ECS entity
//! per address. Generation jobs are transient ECS participants only.

use std::collections::{HashSet, VecDeque};

use bevy::prelude::*;

use super::VoxelMaterializationChunkAddress;
use demand::{DemandedChunk, VoxelDemandPlanKey};

mod demand;
mod generation;

pub(super) use demand::refresh_voxel_residency;
pub(super) use generation::{
    finish_chunk_generation, retire_orphaned_tasks, schedule_voxel_generation,
};

/// Demand-streaming policy for one [`super::VoxelWorld`].
///
/// Presentation material is intentionally separate: residency policy should not
/// own renderer state, and manually resident worlds can use the same realization
/// pipeline without pretending to be streamed.
#[derive(Component, Debug, Clone)]
pub struct VoxelStreaming {
    load_budget_per_frame: usize,
    residency_revision: u64,
    demand_key: Vec<VoxelDemandPlanKey>,
    pending_desired: VecDeque<DemandedChunk>,
    cached_desired_set: HashSet<VoxelMaterializationChunkAddress>,
}

impl VoxelStreaming {
    pub fn new(load_budget_per_frame: usize) -> Self {
        Self {
            load_budget_per_frame: load_budget_per_frame.max(1),
            residency_revision: 0,
            demand_key: Vec::new(),
            pending_desired: VecDeque::new(),
            cached_desired_set: HashSet::new(),
        }
    }

    pub const fn load_budget_per_frame(&self) -> usize {
        self.load_budget_per_frame
    }
}

/// Render material used by disposable voxel manifestations.
#[derive(Component, Debug, Clone)]
pub struct VoxelPresentationMaterial(Handle<StandardMaterial>);

impl VoxelPresentationMaterial {
    pub fn new(material: Handle<StandardMaterial>) -> Self {
        Self(material)
    }

    pub(super) fn handle(&self) -> &Handle<StandardMaterial> {
        &self.0
    }
}

/// Marks a generic [`crate::spatial::SpatialDemandSource`] as requesting voxel
/// materialization.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct VoxelMaterializationDemand;

/// Adds one persistent scale-local materialization scope to a voxel world.
///
/// This is used by the coarsest realization of a celestial body: even when the
/// observer is far away, a small whole-body shell remains materialized using the
/// same voxel/Surface-Nets pipeline as every finer local terrain patch.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct VoxelPinnedDemand {
    center: crate::spatial::UsfPosition,
    half_extent_native: Vec3,
    priority: i32,
    surface_radius_native: Option<f32>,
}

impl VoxelPinnedDemand {
    pub fn cuboid(center: crate::spatial::UsfPosition, half_extent_native: Vec3) -> Self {
        Self {
            center,
            half_extent_native: half_extent_native.abs(),
            priority: 1_000,
            surface_radius_native: None,
        }
    }

    /// Persistent whole-body demand whose generation order starts at the
    /// visible surface instead of wasting the first frames on solid interior.
    pub fn shell(
        center: crate::spatial::UsfPosition,
        radius_native: f32,
        margin_native: f32,
    ) -> Self {
        let radius_native = radius_native.max(0.0);
        let margin_native = margin_native.max(0.0);
        Self {
            center,
            half_extent_native: Vec3::splat(radius_native + margin_native),
            priority: 1_000,
            surface_radius_native: Some(radius_native),
        }
    }

    pub const fn center(self) -> crate::spatial::UsfPosition {
        self.center
    }

    pub const fn half_extent_native(self) -> Vec3 {
        self.half_extent_native
    }

    pub const fn priority(self) -> i32 {
        self.priority
    }

    pub const fn surface_radius_native(self) -> Option<f32> {
        self.surface_radius_native
    }
}

#[cfg(test)]
mod tests;
