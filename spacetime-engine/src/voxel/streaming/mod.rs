//! Demand-driven residency of voxel materialization caches.
//!
//! Streaming owns *which canonical addresses are active*. Dense voxel data lives
//! in [`super::VoxelWorld`]'s compact materialization store rather than in one ECS entity
//! per address. Generation jobs are transient ECS participants only.

use std::collections::{HashMap, HashSet, VecDeque};

use bevy::prelude::*;

use super::VoxelMaterializationKey;
use crate::spatial::UsfScaleRoleMask;
use demand::{
    DemandedChunk, VoxelDemandPlanKey, VoxelWorkRank, compare_work_ranks,
};

mod demand;
mod generation;

fn roles_require_surface(roles: UsfScaleRoleMask) -> bool {
    roles.contains(UsfScaleRoleMask::PRESENTATION)
        || roles.contains(UsfScaleRoleMask::COLLISION)
}

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
    policy_revision: u64,
    demand_key: Vec<VoxelDemandPlanKey>,
    pending_desired: VecDeque<DemandedChunk>,
    /// Latest desired address -> capability-role intent.
    cached_desired_roles:
        HashMap<VoxelMaterializationKey, UsfScaleRoleMask>,
    // persistent-work-rank-v1
    // Scheduling rank survives beyond the pending generation queue so surface
    // derivation can honor the same useful-work ordering.
    desired_work_ranks:
        HashMap<VoxelMaterializationKey, VoxelWorkRank>,
    /// Sparse original committed state for keys whose current candidate
    /// differs during make-before-break migration.
    ///
    /// `None` means the key did not exist in the committed target; `Some(roles)`
    /// records the committed role set. Stable state is empty, so migration cost
    /// scales with the changed boundary rather than the whole desired volume.
    migration_original_roles:
        HashMap<VoxelMaterializationKey, Option<UsfScaleRoleMask>>,
    effective_desired: HashSet<VoxelMaterializationKey>,
    residency_activate: HashSet<VoxelMaterializationKey>,
    residency_deactivate: HashSet<VoxelMaterializationKey>,
    /// Addresses whose capability-role change may require surface derivation
    /// and/or renderer membership to be reconsidered without regeneration.
    role_refresh: HashSet<VoxelMaterializationKey>,
}

impl VoxelStreaming {
    pub fn new(load_budget_per_frame: usize) -> Self {
        Self {
            load_budget_per_frame: load_budget_per_frame.max(1),
            residency_revision: 0,
            policy_revision: 0,
            demand_key: Vec::new(),
            pending_desired: VecDeque::new(),
            cached_desired_roles: HashMap::new(),
            desired_work_ranks: HashMap::new(),
            migration_original_roles: HashMap::new(),
            effective_desired: HashSet::new(),
            residency_activate: HashSet::new(),
            residency_deactivate: HashSet::new(),
            role_refresh: HashSet::new(),
        }
    }

    pub const fn load_budget_per_frame(&self) -> usize {
        self.load_budget_per_frame
    }

    fn pending_desired_len(&self) -> usize {
        self.pending_desired.len()
    }



    fn retire_all_desired(&mut self) -> bool {
        let changed = !self.cached_desired_roles.is_empty()
            || !self.effective_desired.is_empty()
            || !self.pending_desired.is_empty()
            || !self.migration_original_roles.is_empty()
            || !self.demand_key.is_empty();

        self.pending_desired.clear();
        self.cached_desired_roles.clear();
        self.desired_work_ranks.clear();
        self.migration_original_roles.clear();
        self.role_refresh.clear();
        self.demand_key.clear();

        let retiring = self.effective_desired.drain().collect::<Vec<_>>();
        for key in retiring {
            self.residency_activate.remove(&key);
            self.residency_deactivate.insert(key);
        }

        if changed {
            self.policy_revision = self.policy_revision.wrapping_add(1).max(1);
        }
        changed
    }

    fn remember_committed_state(&mut self, key: VoxelMaterializationKey) {
        if !self.migration_original_roles.contains_key(&key) {
            self.migration_original_roles
                .insert(key, self.cached_desired_roles.get(&key).copied());
        }
    }

    fn reconcile_changed_key(&mut self, key: VoxelMaterializationKey) {
        if let Some(original) = self.migration_original_roles.get(&key).copied()
            && self.cached_desired_roles.get(&key).copied() == original
        {
            self.migration_original_roles.remove(&key);
        }

        let retained_committed = self
            .migration_original_roles
            .get(&key)
            .copied()
            .flatten()
            .is_some();
        let should_be_effective =
            self.cached_desired_roles.contains_key(&key) || retained_committed;

        if should_be_effective {
            if self.effective_desired.insert(key) {
                self.residency_deactivate.remove(&key);
                self.residency_activate.insert(key);
            }
        } else if self.effective_desired.remove(&key) {
            self.residency_activate.remove(&key);
            self.residency_deactivate.insert(key);
        }
    }

    fn stage_desired_roles(
        &mut self,
        desired: HashMap<VoxelMaterializationKey, UsfScaleRoleMask>,
    ) {
        if self.cached_desired_roles == desired {
            return;
        }

        if self.cached_desired_roles.is_empty()
            && self.effective_desired.is_empty()
            && self.migration_original_roles.is_empty()
        {
            for &key in desired.keys() {
                self.effective_desired.insert(key);
                self.residency_activate.insert(key);
            }
            self.cached_desired_roles = desired;
            self.policy_revision = self.policy_revision.wrapping_add(1).max(1);
            return;
        }

        let mut changed = HashSet::<VoxelMaterializationKey>::new();
        for (&key, &roles) in &self.cached_desired_roles {
            if desired.get(&key).copied() != Some(roles) {
                changed.insert(key);
            }
        }
        for (&key, &roles) in &desired {
            if self.cached_desired_roles.get(&key).copied() != Some(roles) {
                changed.insert(key);
            }
        }

        // aggressive-departure-retirement-v1
        //
        // Make-before-break protects changed capability for addresses that
        // remain desired. It must never retain space that has actually left
        // demand. New addresses also have no old representation to preserve.
        for &key in &changed {
            let old = self.cached_desired_roles.get(&key).copied();
            let next = desired.get(&key).copied();
            match (old, next) {
                (Some(old_roles), Some(next_roles)) if old_roles != next_roles => {
                    self.remember_committed_state(key);
                }
                (Some(_), None) => {
                    self.migration_original_roles.remove(&key);
                }
                _ => {}
            }
        }
        self.cached_desired_roles = desired;
        for key in changed {
            self.role_refresh.insert(key);
            self.reconcile_changed_key(key);
        }

        self.policy_revision = self.policy_revision.wrapping_add(1).max(1);
    }

    pub(super) fn replace_desired_work_ranks(
        &mut self,
        ranks: HashMap<VoxelMaterializationKey, VoxelWorkRank>,
    ) {
        self.desired_work_ranks = ranks;
    }

    fn stage_incremental_desired(
        &mut self,
        leaving: impl IntoIterator<Item = VoxelMaterializationKey>,
        entering: impl IntoIterator<Item = DemandedChunk>,
    ) {
        let mut changed = false;

        for key in leaving {
            if !self.cached_desired_roles.contains_key(&key) {
                continue;
            }
            // Pure movement is not a migration transaction: the trailing slab
            // has left demand and should retire immediately.
            self.migration_original_roles.remove(&key);
            self.cached_desired_roles.remove(&key);
            self.desired_work_ranks.remove(&key);
            self.role_refresh.insert(key);
            self.reconcile_changed_key(key);
            changed = true;
        }

        for demanded in entering {
            let key = demanded.key;
            let roles = demanded.roles;
            self.desired_work_ranks.insert(key, demanded.work_rank());
            if self.cached_desired_roles.get(&key).copied() == Some(roles) {
                continue;
            }
            // A newly entered address has no committed representation to keep.
            self.cached_desired_roles.insert(key, roles);
            self.role_refresh.insert(key);
            self.reconcile_changed_key(key);
            self.pending_desired.push_back(demanded);
            changed = true;
        }

        if changed {
            self.prune_stale_pending();
            self.policy_revision = self.policy_revision.wrapping_add(1).max(1);
        }
    }

    fn migration_active(&self) -> bool {
        !self.migration_original_roles.is_empty()
    }

    fn is_effectively_desired(&self, key: VoxelMaterializationKey) -> bool {
        self.effective_desired.contains(&key)
    }

    fn prune_stale_pending(&mut self) {
        let desired = &self.effective_desired;
        self.pending_desired
            .retain(|demanded| desired.contains(&demanded.key));
    }

    fn migration_candidate_addresses(
        &self,
    ) -> impl Iterator<Item = (VoxelMaterializationKey, UsfScaleRoleMask)> + '_ {
        self.migration_original_roles
            .keys()
            .filter_map(|&key| {
                self.cached_desired_roles
                    .get(&key)
                    .copied()
                    .map(|roles| (key, roles))
            })
    }

    #[cfg(test)]
    fn effective_desired_set(&self) -> HashSet<VoxelMaterializationKey> {
        self.effective_desired.clone()
    }

    fn take_residency_delta(
        &mut self,
    ) -> (Vec<VoxelMaterializationKey>, Vec<VoxelMaterializationKey>) {
        (
            self.residency_activate.drain().collect(),
            self.residency_deactivate.drain().collect(),
        )
    }

    fn commit_candidate(&mut self) -> bool {
        if self.migration_original_roles.is_empty() {
            return false;
        }

        let previous =
            std::mem::take(&mut self.migration_original_roles);
        for (key, original) in previous {
            self.role_refresh.insert(key);
            if original.is_some()
                && !self.cached_desired_roles.contains_key(&key)
                && self.effective_desired.remove(&key)
            {
                self.residency_activate.remove(&key);
                self.residency_deactivate.insert(key);
            }
        }

        self.policy_revision = self.policy_revision.wrapping_add(1).max(1);
        true
    }

    pub(in crate::voxel) const fn collision_policy_revision(&self) -> u64 {
        self.policy_revision
    }

    // role-aware-surface-work-v1
    pub(in crate::voxel) fn effective_roles(
        &self,
        key: VoxelMaterializationKey,
    ) -> UsfScaleRoleMask {
        let mut roles = self
            .cached_desired_roles
            .get(&key)
            .copied()
            .unwrap_or(UsfScaleRoleMask::NONE);
        if let Some(committed) = self
            .migration_original_roles
            .get(&key)
            .copied()
            .flatten()
        {
            roles = roles.union(committed);
        }
        roles
    }

    pub(in crate::voxel) fn surface_required(
        &self,
        key: VoxelMaterializationKey,
    ) -> bool {
        roles_require_surface(self.effective_roles(key))
    }

    fn take_role_refresh(&mut self) -> Vec<VoxelMaterializationKey> {
        self.role_refresh.drain().collect()
    }

    pub(in crate::voxel) fn retains_committed_role_during_migration(
        &self,
        key: VoxelMaterializationKey,
        role: UsfScaleRoleMask,
    ) -> bool {
        self.migration_original_roles
            .get(&key)
            .copied()
            .flatten()
            .is_some_and(|roles| roles.contains(role))
    }

    pub(in crate::voxel) fn compare_work_keys(
        &self,
        a: VoxelMaterializationKey,
        b: VoxelMaterializationKey,
    ) -> std::cmp::Ordering {
        match (
            self.desired_work_ranks.get(&a).copied(),
            self.desired_work_ranks.get(&b).copied(),
        ) {
            (Some(a), Some(b)) => compare_work_ranks(a, b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    }

    fn next_pending_work_rank(&self) -> Option<VoxelWorkRank> {
        self.pending_desired.front().map(|demand| demand.work_rank())
    }
}

/// Diagnostic counters only; never used as scheduling authority.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct VoxelStreamingTelemetry {
    worker_capacity: usize,
    generation_in_flight: usize,
    derived_in_flight: usize,
    generation_started_total: u64,
    generation_completed_total: u64,
    generation_cancelled_total: u64,
    generation_chunks_abandoned_total: u64,
    derived_started_total: u64,
    derived_completed_total: u64,
    derived_cancelled_total: u64,
    derived_skipped_empty_total: u64,
}

impl VoxelStreamingTelemetry {
    pub(super) fn worker_capacity(&mut self, capacity: usize) {
        self.worker_capacity = capacity;
    }

    pub fn summary(self) -> String {
        format!(
            "workers capacity={} generation={} derived={} total={} | gen started={} completed={} cancelled={} abandoned_chunks={} | surface started={} completed={} cancelled={} skipped_empty={}",
            self.worker_capacity,
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
            self.derived_skipped_empty_total,
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
    pub(super) fn derived_skipped_empty(&mut self) {
        self.derived_skipped_empty_total += 1;
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
