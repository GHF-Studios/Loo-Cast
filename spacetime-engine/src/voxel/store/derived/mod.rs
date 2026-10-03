//! Dense-edit invalidation and derived surface-build lifecycle.

use super::*;

impl VoxelMaterializationStore {
    pub(in crate::voxel) fn apply_edit(
        &mut self,
        key: VoxelMaterializationKey,
        address: VoxelMaterializationChunkAddress,
        edit: VoxelEdit,
    ) -> bool {
        let mut changed = false;
        let mut active = false;
        if let Some(entry) = self.entries.get_mut(&key) {
            active = entry.active;
            if let Some(chunk) = entry.dense_mut() {
                changed = chunk.apply_edit(address, edit).changed();
            }
        }

        if changed && active {
            self.bump_capability_revision();
            self.mark_derived_dirty(key);
        }
        changed
    }

    pub(in crate::voxel) fn pop_dirty_derived(&mut self) -> Option<VoxelMaterializationKey> {
        while let Some(address) = self.dirty_derived.pop_front() {
            if self.dirty_derived_set.remove(&address) {
                return Some(address);
            }
        }
        None
    }

    // role-aware-derived-queue-v1
    // persistent-work-rank-derived-v1
    /// Pops the best still-dirty address according to a caller-owned scheduling
    /// comparator while preserving the relative order of every other live item.
    ///
    /// Dirty queues are local active working sets, so this O(n) selection is
    /// deliberately paid only when a derivation slot is available. It prevents
    /// async completion order from destroying semantic demand priority.
    pub(in crate::voxel) fn pop_dirty_derived_best_by(
        &mut self,
        mut compare: impl FnMut(
            VoxelMaterializationKey,
            VoxelMaterializationKey,
        ) -> std::cmp::Ordering,
    ) -> Option<VoxelMaterializationKey> {
        let scan = self.dirty_derived.len();
        let mut best = None::<VoxelMaterializationKey>;

        for _ in 0..scan {
            let Some(address) = self.dirty_derived.pop_front() else {
                break;
            };
            if !self.dirty_derived_set.contains(&address) {
                continue;
            }

            if best.is_none_or(|current| {
                compare(address, current) == std::cmp::Ordering::Less
            }) {
                best = Some(address);
            }
            self.dirty_derived.push_back(address);
        }

        let target = best?;
        let scan = self.dirty_derived.len();
        for _ in 0..scan {
            let Some(address) = self.dirty_derived.pop_front() else {
                break;
            };
            if !self.dirty_derived_set.contains(&address) {
                continue;
            }
            if address == target {
                self.dirty_derived_set.remove(&address);
                return Some(address);
            }
            self.dirty_derived.push_back(address);
        }

        None
    }

    /// Pops the first still-dirty address matching `predicate`, preserving the
    /// relative order of unmatched entries.
    pub(in crate::voxel) fn pop_dirty_derived_matching(
        &mut self,
        mut predicate: impl FnMut(VoxelMaterializationKey) -> bool,
    ) -> Option<VoxelMaterializationKey> {
        let remaining = self.dirty_derived.len();
        for _ in 0..remaining {
            let Some(address) = self.dirty_derived.pop_front() else {
                break;
            };
            if !self.dirty_derived_set.contains(&address) {
                continue;
            }
            if predicate(address) {
                self.dirty_derived_set.remove(&address);
                return Some(address);
            }
            self.dirty_derived.push_back(address);
        }
        None
    }

    /// Re-arm surface derivation from already resident dense truth after a
    /// capability-role upgrade. Current/in-flight revisions remain no-ops.
    pub(in crate::voxel) fn ensure_derived_dirty(
        &mut self,
        address: VoxelMaterializationKey,
    ) {
        let should_mark = self.entries.get(&address).is_some_and(|entry| {
            if !entry.active {
                return false;
            }
            let Some(chunk) = entry.dense() else {
                return false;
            };
            let revision = chunk.revision();
            entry.derived_revision != Some(revision)
                && entry.derived_in_flight != Some(revision)
        });
        if should_mark {
            self.mark_derived_dirty(address);
        }
    }

    /// Whether an active materialization completed surface derivation for its
    /// current dense revision. Processed-empty chunks are current despite
    /// having no surface cache.
    pub(in crate::voxel) fn is_derived_current(
        &self,
        address: VoxelMaterializationKey,
    ) -> bool {
        self.active_derived_revision(address).is_some()
    }

    /// Current dense revision of one active materialization whose derived
    /// representation has completed, regardless of whether that result owns
    /// any triangles.
    ///
    /// `None` surface is a valid derived result for known-empty space. Runtime
    /// capability lifetime must therefore follow this revision rather than
    /// `VoxelSurfaceCache` existence.
    pub(in crate::voxel) fn active_derived_revision(
        &self,
        address: VoxelMaterializationKey,
    ) -> Option<u64> {
        let entry = self.entries.get(&address)?;
        if !entry.active {
            return None;
        }
        let chunk = entry.dense()?;
        let revision = chunk.revision();
        (entry.derived_revision == Some(revision)).then_some(revision)
    }

    pub(in crate::voxel) fn begin_surface_build(
        &mut self,
        address: VoxelMaterializationKey,
    ) -> Option<(u64, VoxelChunk)> {
        let entry = self.entries.get_mut(&address)?;
        if !entry.active {
            return None;
        }
        let chunk = entry.dense()?;
        let revision = chunk.revision();
        if entry.derived_revision == Some(revision) || entry.derived_in_flight == Some(revision) {
            return None;
        }

        let snapshot = chunk.clone();
        entry.derived_in_flight = Some(revision);
        Some((revision, snapshot))
    }

    /// Publishes one derived surface cache. Empty surfaces are represented by
    /// `None` while `derived_revision` records that the revision was processed.
    /// Releases one abandoned derivation reservation so later reactivation can retry it.
    pub(in crate::voxel) fn cancel_surface_build(
        &mut self,
        address: VoxelMaterializationKey,
        revision: u64,
    ) {
        let mut reactivate_dirty = false;
        if let Some(entry) = self.entries.get_mut(&address)
            && entry.derived_in_flight == Some(revision)
        {
            entry.derived_in_flight = None;
            reactivate_dirty = entry.active;
        }
        if reactivate_dirty {
            self.mark_derived_dirty(address);
        }
    }

    pub(in crate::voxel) fn publish_surface(
        &mut self,
        address: VoxelMaterializationKey,
        revision: u64,
        surface: Option<VoxelSurfaceCache>,
    ) -> bool {
        let mut stale_active = false;
        let mut published = false;

        if let Some(entry) = self.entries.get_mut(&address) {
            let current_revision = entry.dense().map(VoxelChunk::revision);
            if current_revision == Some(revision) {
                entry.surface = surface;
                entry.derived_revision = Some(revision);
                entry.derived_in_flight = None;
                if let Some(chunk) = entry.dense_mut() {
                    chunk.mark_meshed();
                }
                published = true;
            } else {
                if entry.derived_in_flight == Some(revision) {
                    entry.derived_in_flight = None;
                }
                stale_active = entry.active && current_revision.is_some();
            }
        }

        if published {
            self.bump_capability_revision();
            self.mark_render_dirty(address);
        } else if stale_active {
            self.mark_derived_dirty(address);
        }
        published
    }
}
