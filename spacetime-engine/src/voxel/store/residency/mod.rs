//! Dense residency, generation reservation, reactivation and warm-cache eviction.

use super::*;

impl VoxelMaterializationStore {
    pub(crate) fn reserve_generation(
        &mut self,
        address: VoxelMaterializationChunkAddress,
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
    pub(crate) fn reactivate(&mut self, address: VoxelMaterializationChunkAddress) -> bool {
        let mut render_dirty = false;
        let mut derived_dirty = false;
        let found = match self.entries.get_mut(&address) {
            Some(entry) => {
                if !entry.active {
                    entry.active = true;
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

    pub(crate) fn deactivate(&mut self, address: VoxelMaterializationChunkAddress) {
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
    pub(crate) fn insert_dense_active(
        &mut self,
        address: VoxelMaterializationChunkAddress,
        chunk: VoxelChunk,
    ) {
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
    pub(crate) fn publish_generated(
        &mut self,
        address: VoxelMaterializationChunkAddress,
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

    pub(crate) fn is_active(&self, address: VoxelMaterializationChunkAddress) -> bool {
        self.entries.get(&address).is_some_and(|entry| entry.active)
    }

    pub(crate) fn active_addresses(
        &self,
    ) -> impl Iterator<Item = VoxelMaterializationChunkAddress> + '_ {
        self.entries
            .iter()
            .filter_map(|(&address, entry)| entry.active.then_some(address))
    }

    pub(crate) fn active_dense_entries(
        &self,
    ) -> impl Iterator<Item = (VoxelMaterializationChunkAddress, &VoxelChunk)> + '_ {
        self.entries.iter().filter_map(|(&address, entry)| {
            entry
                .active
                .then(|| entry.dense().map(|chunk| (address, chunk)))
                .flatten()
        })
    }

    /// Keep inactive dense data warm up to a bounded count. Eviction changes
    /// only disposable cache state; semantic base + modifications stay intact.
    pub(crate) fn trim_inactive(&mut self, maximum_inactive: usize) {
        let mut inactive = self.entries.values().filter(|entry| !entry.active).count();
        while inactive > maximum_inactive {
            let Some(address) = self.inactive_lru.pop_front() else {
                break;
            };
            if self
                .entries
                .get(&address)
                .is_some_and(|entry| !entry.active)
            {
                self.entries.remove(&address);
                self.dirty_derived_set.remove(&address);
                inactive -= 1;
            }
        }
    }
}
