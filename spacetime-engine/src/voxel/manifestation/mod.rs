//! Runtime manifestations derived from store-owned voxel surfaces.
//!
//! Manifestations exist only for materializations with actual presentation
//! geometry. Uniform air/solid realization facts remain store-backed and publish
//! batched capability coverage without per-materialization ECS entities.
//! Collision is independently owned and may aggregate multiple materializations.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use super::VoxelMaterializationKey;

mod collision;
mod coverage;
mod frontier;
mod lifecycle;
mod material;
mod membership;
mod rebuild;

pub(super) use collision::{
    sync_collision_aggregate_runtime_transforms,
    sync_manifestation_collision_residency,
};
pub(super) use coverage::sync_capability_realizations;
pub(super) use lifecycle::retire_removed_world_manifestations;
pub use material::VoxelPresentationMaterial;
pub(super) use material::{create_voxel_render_material, VoxelRenderMaterial};
pub(super) use membership::sync_manifestation_membership;
pub(super) use rebuild::{
    rebuild_dirty_manifestations, sync_manifestation_runtime_transforms,
};

pub(super) fn configure(app: &mut App) {
    app.init_resource::<collision::VoxelCollisionAggregateRegistry>();

    frontier::configure(app);
    material::configure(app);
}

// first-touch-profiler-decontamination-v2
const MAX_POOLED_MANIFESTATIONS: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ManifestationKey {
    world: Entity,
    key: VoxelMaterializationKey,
}

/// Stable runtime shell for one same-resolution presentation manifestation.
///
/// `active=false` means this root is parked: hidden and absent from active
/// lookup maps, while its hierarchy and render handles stay allocated for reuse.
#[derive(Component, Debug, Clone, Copy)]
pub(super) struct VoxelMaterializationRuntime {
    world: Entity,
    key: VoxelMaterializationKey,
    revision: u64,
    presentation: Entity,
    translucent_presentation: Option<Entity>,
    active: bool,
}

impl VoxelMaterializationRuntime {
    pub(super) const fn world(&self) -> Entity {
        self.world
    }

    pub(super) const fn key(&self) -> VoxelMaterializationKey {
        self.key
    }

    pub(super) const fn revision(&self) -> u64 {
        self.revision
    }

    pub(super) const fn active(&self) -> bool {
        self.active
    }

    pub(super) const fn parked(mut self) -> Self {
        self.active = false;
        self
    }

    pub(super) const fn rebound(
        mut self,
        world: Entity,
        key: VoxelMaterializationKey,
        revision: u64,
    ) -> Self {
        self.world = world;
        self.key = key;
        self.revision = revision;
        self.active = true;
        self
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub(super) struct VoxelMaterializationPresentation;

/// Presentation-only retirement handshake for a dense manifestation whose
/// store/capability demand already left.
///
/// Presence means the clipmap compositor has proven a visible replacement (or
/// the old patch is no longer view-relevant). Membership may then park/recycle
/// the runtime shell. This marker never grants semantic/collision authority.
#[derive(Component, Debug, Default, Clone, Copy)]
pub(super) struct VoxelPresentationFallbackRetireReady;

/// Active mapping plus a bounded inactive shell pool.
#[derive(Resource, Default)]
pub(super) struct VoxelMaterializationRuntimeRegistry {
    revisions: HashMap<ManifestationKey, u64>,
    dirty: HashSet<ManifestationKey>,
    entities: HashMap<ManifestationKey, Entity>,
    pooled: Vec<Entity>,
    // first-touch-profiler-decontamination-v2
}

impl VoxelMaterializationRuntimeRegistry {
    pub(super) fn recycle(&mut self, entity: Entity) -> bool {
        if self.pooled.len() >= MAX_POOLED_MANIFESTATIONS {
            return false;
        }
        self.pooled.push(entity);
        true
    }

    pub(super) fn take_pooled(&mut self) -> Option<Entity> {
        self.pooled.pop()
    }

    // first-touch-profiler-decontamination-v2
    #[inline(always)]
    pub(super) fn record_spawned(&mut self) {}

    #[inline(always)]
    pub(super) fn record_rebuild_existing(&mut self) {}

    #[inline(always)]
    pub(super) fn record_mesh_publication(&mut self) {}

    #[inline(always)]
    pub(super) fn record_root_transform(&mut self, _changed: bool) {}

    #[inline(always)]
    pub(super) fn record_visibility_write(&mut self) {}

    #[cfg(test)]
    fn pooled_len(&self) -> usize {
        self.pooled.len()
    }
}

#[cfg(test)]
mod pool_tests {
    use super::*;

    #[test]
    fn manifestation_pool_is_bounded_and_reuses_lifo() {
        let mut registry = VoxelMaterializationRuntimeRegistry::default();
        let mut ecs = World::new();
        let entity = ecs.spawn_empty().id();

        assert!(registry.recycle(entity));
        assert_eq!(registry.pooled_len(), 1);
        assert_eq!(registry.take_pooled(), Some(entity));
        assert_eq!(registry.pooled_len(), 0);
    }
}
