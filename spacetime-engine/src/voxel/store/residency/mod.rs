//! Dense residency, generation reservation, reactivation and warm-cache eviction.

use super::*;

impl VoxelMaterializationStore {
    pub(in crate::voxel) fn reserve_generation(
        &mut self,
        address: VoxelMaterializationKey,
    ) -> Option<u64> {
        if self.entries.contains_key(&address) {
            return None;
        }

        self.next_generation_token = self.next_generation_token.wrapping_add(1).max(1);
        let token = self.next_generation_token;
        self.entries.insert(
            address,
            VoxelMaterializationEntry {
                active: true,
                state: VoxelMaterializationState::Pending { token },
                derived_revision: None,
                surface: None,
                derived_in_flight: None,
            },
        );
        Some(token)
    }

    /// Reactivates a warm dense entry. Returns whether the address already had
    /// resident data and therefore needs no generation reservation.
    pub(in crate::voxel) fn reactivate(&mut self, address: VoxelMaterializationKey) -> bool {
        let mut render_dirty = false;
        let mut derived_dirty = false;
        let found = match self.entries.get_mut(&address) {
            Some(entry) => {
                if !entry.active {
                    entry.active = true;
                    self.inactive_count = self.inactive_count.saturating_sub(1);
                    render_dirty = true;
                }
                if let Some(chunk) = entry.dense() {
                    derived_dirty = entry.derived_revision != Some(chunk.revision())
                        && entry.derived_in_flight != Some(chunk.revision());
                }
                true
            }
            None => false,
        };

        if render_dirty {
            self.mark_render_dirty(address);
        }
        if derived_dirty {
            self.mark_derived_dirty(address);
        }
        found
    }

    pub(in crate::voxel) fn deactivate(&mut self, address: VoxelMaterializationKey) {
        let mut remove_pending = false;
        let mut became_inactive = false;

        if let Some(entry) = self.entries.get_mut(&address) {
            match &entry.state {
                VoxelMaterializationState::Pending { .. } => {
                    remove_pending = true;
                }
                VoxelMaterializationState::Dense(_) => {
                    if entry.active {
                        entry.active = false;
                        self.inactive_count = self.inactive_count.saturating_add(1);
                        became_inactive = true;
                    }
                }
            }
        }

        if remove_pending {
            self.entries.remove(&address);
            self.dirty_derived_set.remove(&address);
            self.mark_render_dirty(address);
        } else if became_inactive {
            self.inactive_lru.push_back(address);
            self.mark_render_dirty(address);
        }
    }

    /// Inserts already-generated dense data, used by manually resident worlds.
    pub(in crate::voxel) fn insert_dense_active(
        &mut self,
        address: VoxelMaterializationKey,
        chunk: VoxelChunk,
    ) {
        if self.entries.get(&address).is_some_and(|entry| !entry.active) {
            self.inactive_count = self.inactive_count.saturating_sub(1);
        }
        self.entries.insert(
            address,
            VoxelMaterializationEntry {
                active: true,
                state: VoxelMaterializationState::Dense(chunk),
                derived_revision: None,
                surface: None,
                derived_in_flight: None,
            },
        );
        self.mark_derived_dirty(address);
        self.mark_render_dirty(address);
    }

    /// Publishes a worker generation result only if its reservation is still
    /// current. Demand migration therefore cannot resurrect retired work.
    pub(in crate::voxel) fn publish_generated(
        &mut self,
        address: VoxelMaterializationKey,
        token: u64,
        chunk: VoxelChunk,
    ) -> bool {
        let Some(entry) = self.entries.get_mut(&address) else {
            return false;
        };
        if !entry.active {
            return false;
        }
        let VoxelMaterializationState::Pending {
            token: current_token,
        } = &entry.state
        else {
            return false;
        };
        if *current_token != token {
            return false;
        }

        entry.state = VoxelMaterializationState::Dense(chunk);
        entry.derived_revision = None;
        entry.surface = None;
        entry.derived_in_flight = None;
        self.mark_derived_dirty(address);
        true
    }

    pub(in crate::voxel) fn is_active(&self, address: VoxelMaterializationKey) -> bool {
        self.entries.get(&address).is_some_and(|entry| entry.active)
    }

    pub(in crate::voxel) fn active_keys(
        &self,
    ) -> impl Iterator<Item = VoxelMaterializationKey> + '_ {
        self.entries
            .iter()
            .filter_map(|(&address, entry)| entry.active.then_some(address))
    }

    /// Applies only residency membership changes.
    ///
    /// Streaming owns the desired set and computes its delta. The store owns
    /// lifecycle transitions. Quiet resident materializations therefore incur
    /// no scan, hash lookup, or reactivation work.
    pub(in crate::voxel) fn apply_residency_delta(
        &mut self,
        activate: impl IntoIterator<Item = VoxelMaterializationKey>,
        deactivate: impl IntoIterator<Item = VoxelMaterializationKey>,
        maximum_inactive: usize,
    ) {
        for key in deactivate {
            self.deactivate(key);
        }
        for key in activate {
            self.reactivate(key);
        }
        self.trim_inactive(maximum_inactive);
    }

    pub(in crate::voxel) fn active_dense_entries(
        &self,
    ) -> impl Iterator<Item = (VoxelMaterializationKey, &VoxelChunk)> + '_ {
        self.entries.iter().filter_map(|(&address, entry)| {
            entry
                .active
                .then(|| entry.dense().map(|chunk| (address, chunk)))
                .flatten()
        })
    }

    /// Keep inactive dense data warm up to a bounded count. Eviction changes
    /// only disposable cache state; semantic base + modifications stay intact.
    pub(in crate::voxel) fn trim_inactive(&mut self, maximum_inactive: usize) {
        while self.inactive_count > maximum_inactive {
            let Some(address) = self.inactive_lru.pop_front() else {
                debug_assert_eq!(self.inactive_count, 0, "inactive count/LRU drift");
                break;
            };
            if self.entries.get(&address).is_some_and(|entry| !entry.active) {
                self.entries.remove(&address);
                self.dirty_derived_set.remove(&address);
                self.inactive_count = self.inactive_count.saturating_sub(1);
            }
        }
    }
}
