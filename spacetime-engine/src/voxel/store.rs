//! Compact residency and cache storage for voxel materializations.
//!
//! A canonical materialization address has identity without requiring a Bevy
//! entity. This store owns the lifecycle of dense working data and derived CPU
//! caches. ECS is reserved for transient jobs and runtime manifestations.

use std::collections::{HashMap, HashSet, VecDeque};

use super::{VoxelChunk, VoxelEdit, VoxelMaterializationChunkAddress, mesh::VoxelSurface};

/// Derived surface cache for one independently addressable materialization.
///
/// The cache may remain warm in RAM while the materialization is inactive.
/// Runtime render/physics manifestations are built separately.
#[derive(Debug)]
pub(crate) struct VoxelSurfaceCache {
    pub(crate) revision: u64,
    pub(crate) surface: VoxelSurface,
    pub(crate) debug_color: [f32; 4],
}

impl VoxelSurfaceCache {
    pub(crate) fn new(revision: u64, surface: VoxelSurface, debug_color: [f32; 4]) -> Self {
        Self {
            revision,
            surface,
            debug_color,
        }
    }
}

#[derive(Debug)]
enum VoxelMaterializationState {
    Pending { token: u64 },
    Dense(VoxelChunk),
}

#[derive(Debug)]
struct VoxelMaterializationEntry {
    active: bool,
    state: VoxelMaterializationState,
    derived_revision: Option<u64>,
    surface: Option<VoxelSurfaceCache>,
    derived_in_flight: Option<u64>,
}

impl VoxelMaterializationEntry {
    fn dense(&self) -> Option<&VoxelChunk> {
        match &self.state {
            VoxelMaterializationState::Dense(chunk) => Some(chunk),
            VoxelMaterializationState::Pending { .. } => None,
        }
    }

    fn dense_mut(&mut self) -> Option<&mut VoxelChunk> {
        match &mut self.state {
            VoxelMaterializationState::Dense(chunk) => Some(chunk),
            VoxelMaterializationState::Pending { .. } => None,
        }
    }
}

/// Sparse materialization residency + warm-cache store.
///
/// `active` means demanded by the current realization window. Inactive dense
/// entries are warm RAM cache: they cost no ECS/render/physics participation and
/// can be reactivated without regenerating the semantic field.
#[derive(Debug, Default)]
pub(crate) struct VoxelMaterializationStore {
    entries: HashMap<VoxelMaterializationChunkAddress, VoxelMaterializationEntry>,
    next_generation_token: u64,
    dirty_derived: VecDeque<VoxelMaterializationChunkAddress>,
    dirty_derived_set: HashSet<VoxelMaterializationChunkAddress>,
    dirty_render: VecDeque<VoxelMaterializationChunkAddress>,
    dirty_render_set: HashSet<VoxelMaterializationChunkAddress>,
    inactive_lru: VecDeque<VoxelMaterializationChunkAddress>,
}

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

    pub(crate) fn apply_edit(
        &mut self,
        address: VoxelMaterializationChunkAddress,
        edit: VoxelEdit,
    ) -> bool {
        let mut changed = false;
        let mut active = false;
        if let Some(entry) = self.entries.get_mut(&address) {
            active = entry.active;
            if let Some(chunk) = entry.dense_mut() {
                changed = chunk.apply_edit(address, edit).changed();
            }
        }

        if changed && active {
            self.mark_derived_dirty(address);
        }
        changed
    }

    pub(crate) fn pop_dirty_derived(&mut self) -> Option<VoxelMaterializationChunkAddress> {
        while let Some(address) = self.dirty_derived.pop_front() {
            if self.dirty_derived_set.remove(&address) {
                return Some(address);
            }
        }
        None
    }

    pub(crate) fn begin_surface_build(
        &mut self,
        address: VoxelMaterializationChunkAddress,
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
    pub(crate) fn publish_surface(
        &mut self,
        address: VoxelMaterializationChunkAddress,
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
            self.mark_render_dirty(address);
        } else if stale_active {
            self.mark_derived_dirty(address);
        }
        published
    }

    pub(crate) fn pop_dirty_render(&mut self) -> Option<VoxelMaterializationChunkAddress> {
        while let Some(address) = self.dirty_render.pop_front() {
            if self.dirty_render_set.remove(&address) {
                return Some(address);
            }
        }
        None
    }

    pub(crate) fn active_surface(
        &self,
        address: VoxelMaterializationChunkAddress,
    ) -> Option<&VoxelSurfaceCache> {
        let entry = self.entries.get(&address)?;
        entry.active.then_some(())?;
        entry.surface.as_ref()
    }

    pub(crate) fn surface(
        &self,
        address: VoxelMaterializationChunkAddress,
    ) -> Option<&VoxelSurfaceCache> {
        self.entries.get(&address)?.surface.as_ref()
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

    pub(crate) fn resident_count(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn active_count(&self) -> usize {
        self.entries.values().filter(|entry| entry.active).count()
    }

    pub(crate) fn inactive_count(&self) -> usize {
        self.entries.values().filter(|entry| !entry.active).count()
    }

    pub(crate) fn pending_count(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| matches!(&entry.state, VoxelMaterializationState::Pending { .. }))
            .count()
    }

    pub(crate) fn dense_count(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| matches!(&entry.state, VoxelMaterializationState::Dense(_)))
            .count()
    }

    pub(crate) fn active_dense_count(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| {
                entry.active && matches!(&entry.state, VoxelMaterializationState::Dense(_))
            })
            .count()
    }

    pub(crate) fn active_surface_count(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| entry.active && entry.surface.is_some())
            .count()
    }

    pub(crate) fn surface_cache_count(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| entry.surface.is_some())
            .count()
    }

    pub(crate) fn dirty_derived_count(&self) -> usize {
        self.dirty_derived_set.len()
    }

    fn mark_derived_dirty(&mut self, address: VoxelMaterializationChunkAddress) {
        if self.dirty_derived_set.insert(address) {
            self.dirty_derived.push_back(address);
        }
    }

    fn mark_render_dirty(&mut self, address: VoxelMaterializationChunkAddress) {
        if self.dirty_render_set.insert(address) {
            self.dirty_render.push_back(address);
        }
    }
}
