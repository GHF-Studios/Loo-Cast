//! Demand-driven residency of voxel materialization caches.
//!
//! Streaming owns *which canonical addresses are active*. Dense voxel data lives
//! in [`super::VoxelWorld`]'s compact materialization store rather than in one ECS entity
//! per address. Generation jobs are transient ECS participants only.

use std::collections::{HashMap, HashSet, VecDeque};

use bevy::prelude::*;

use super::VoxelMaterializationChunkAddress;
use crate::spatial::UsfScaleRoleMask;
use demand::{DemandedChunk, VoxelDemandPlanKey};

mod demand;
mod generation;

pub(super) use demand::refresh_voxel_residency;
pub(super) use generation::{
    finish_chunk_generation, retire_stale_generation_tasks, schedule_voxel_generation,
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
    /// Latest desired address -> capability-role intent.
    cached_desired_roles:
        HashMap<VoxelMaterializationChunkAddress, UsfScaleRoleMask>,
    /// Last address+role target whose replacement transaction committed.
    committed_desired_roles:
        HashMap<VoxelMaterializationChunkAddress, UsfScaleRoleMask>,
}

impl VoxelStreaming {
    pub fn new(load_budget_per_frame: usize) -> Self {
        Self {
            load_budget_per_frame: load_budget_per_frame.max(1),
            residency_revision: 0,
            demand_key: Vec::new(),
            pending_desired: VecDeque::new(),
            cached_desired_roles: HashMap::new(),
            committed_desired_roles: HashMap::new(),
        }
    }

    pub const fn load_budget_per_frame(&self) -> usize {
        self.load_budget_per_frame
    }

    fn stage_desired_roles(
        &mut self,
        desired: HashMap<VoxelMaterializationChunkAddress, UsfScaleRoleMask>,
    ) {
        if self.committed_desired_roles.is_empty() && self.cached_desired_roles.is_empty() {
            self.committed_desired_roles = desired.clone();
        }
        self.cached_desired_roles = desired;
    }

    fn migration_active(&self) -> bool {
        self.cached_desired_roles != self.committed_desired_roles
    }

    fn candidate_addresses(
        &self,
    ) -> impl Iterator<Item = (VoxelMaterializationChunkAddress, UsfScaleRoleMask)> + '_ {
        self.cached_desired_roles
            .iter()
            .map(|(&address, &roles)| (address, roles))
    }

    fn effective_desired_set(&self) -> HashSet<VoxelMaterializationChunkAddress> {
        if !self.migration_active() {
            return self.cached_desired_roles.keys().copied().collect();
        }

        self.committed_desired_roles
            .keys()
            .chain(self.cached_desired_roles.keys())
            .copied()
            .collect()
    }

    fn commit_candidate(&mut self) -> bool {
        if !self.migration_active() {
            return false;
        }
        self.committed_desired_roles = self.cached_desired_roles.clone();
        true
    }

    pub(in crate::voxel) fn retains_committed_role_during_migration(
        &self,
        address: VoxelMaterializationChunkAddress,
        role: UsfScaleRoleMask,
    ) -> bool {
        self.migration_active()
            && self
                .committed_desired_roles
                .get(&address)
                .is_some_and(|roles| roles.contains(role))
    }

    fn next_pending_priority(&self) -> Option<i32> {
        self.pending_desired.front().map(|demand| demand.priority)
    }
}

/// Diagnostic counters only; never used as scheduling authority.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct VoxelStreamingTelemetry {
    generation_in_flight: usize,
    derived_in_flight: usize,
    generation_started_total: u64,
    generation_completed_total: u64,
    generation_cancelled_total: u64,
    generation_chunks_abandoned_total: u64,
    derived_started_total: u64,
    derived_completed_total: u64,
    derived_cancelled_total: u64,
}

impl VoxelStreamingTelemetry {
    pub fn summary(self) -> String {
        format!(
            "workers generation={} derived={} total={} | gen started={} completed={} cancelled={} abandoned_chunks={} | surface started={} completed={} cancelled={}",
            self.generation_in_flight,
            self.derived_in_flight,
            self.generation_in_flight + self.derived_in_flight,
            self.generation_started_total,
            self.generation_completed_total,
            self.generation_cancelled_total,
            self.generation_chunks_abandoned_total,
            self.derived_started_total,
            self.derived_completed_total,
            self.derived_cancelled_total,
        )
    }

    pub(super) fn generation_started(&mut self) {
        self.generation_in_flight += 1;
        self.generation_started_total += 1;
    }
    pub(super) fn generation_completed(&mut self) {
        self.generation_in_flight = self.generation_in_flight.saturating_sub(1);
        self.generation_completed_total += 1;
    }
    pub(super) fn generation_cancelled(&mut self, chunks: usize) {
        self.generation_in_flight = self.generation_in_flight.saturating_sub(1);
        self.generation_cancelled_total += 1;
        self.generation_chunks_abandoned_total += chunks as u64;
    }
    pub(super) fn derived_started(&mut self) {
        self.derived_in_flight += 1;
        self.derived_started_total += 1;
    }
    pub(super) fn derived_completed(&mut self) {
        self.derived_in_flight = self.derived_in_flight.saturating_sub(1);
        self.derived_completed_total += 1;
    }
    pub(super) fn derived_cancelled(&mut self) {
        self.derived_in_flight = self.derived_in_flight.saturating_sub(1);
        self.derived_cancelled_total += 1;
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
