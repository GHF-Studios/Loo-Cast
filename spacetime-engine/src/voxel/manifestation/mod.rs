//! Disposable presentation manifestations derived from store-owned voxel surfaces.
//!
//! Manifestations exist only for materializations with actual presentation
//! geometry. Uniform air/solid realization facts remain store-backed and publish
//! batched capability coverage without per-materialization ECS entities.
//! Collision is independently owned and may aggregate multiple materializations.
//!
//! ## Integration
//!
//! Membership tracks which materialization surfaces need presentation. Rebuild creates recyclable
//! render shells, collision runs independently, and coverage publishes facts about realized output.
//!
//! ## Module map
//!
//! - `collision`: Collision-specific aggregation over realized voxel materializations.
//! - `coverage`: Publishes generic capability readiness from voxel stores and disposable
//!   manifestations.
//! - `frontier`: Cross-resolution presentation frontier derived from realized voxel coverage.
//! - `lifecycle`: Lifetime cleanup for disposable voxel manifestations.
//! - `material`: Voxel presentation materials and coarse/fine refinement clipping.
//! - `membership`: One-to-one materialization-surface membership tracking.
//! - `rebuild`: Per-materialization mesh construction and publication.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

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
    sync_collision_aggregate_runtime_transforms, sync_manifestation_collision_residency,
};
pub(super) use coverage::publish_voxel_capability_realizations;
pub(super) use lifecycle::retire_orphaned_presentation_manifestations;
pub use material::VoxelPresentationMaterial;
pub(super) use material::{VoxelRenderMaterial, create_voxel_render_material};
pub(super) use membership::reconcile_presentation_manifestations;
pub(super) use rebuild::{
    rebuild_dirty_presentation_manifestations, sync_presentation_manifestation_transforms,
};

pub(super) fn configure(app: &mut App) {
    app.init_resource::<collision::VoxelCollisionAggregateRegistry>();

    frontier::configure(app);
    material::configure(app);
}

const MAX_POOLED_MANIFESTATIONS: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ManifestationKey {
    realization: Entity,
    key: VoxelMaterializationKey,
}

/// Stable recyclable shell for one same-resolution presentation manifestation.
///
/// `active=false` means this root is parked: hidden and absent from active
/// lookup maps, while its hierarchy and render handles stay allocated for reuse.
#[derive(Component, Debug, Clone, Copy)]
pub(super) struct VoxelPresentationManifestation {
    realization: Entity,
    key: VoxelMaterializationKey,
    revision: u64,
    presentation: Entity,
    translucent_presentation: Option<Entity>,
    active: bool,
}

impl VoxelPresentationManifestation {
    pub(super) const fn realization(&self) -> Entity {
        self.realization
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

    pub(super) const fn presentation(&self) -> Entity {
        self.presentation
    }
    pub(super) const fn translucent_presentation(&self) -> Option<Entity> {
        self.translucent_presentation
    }

    pub(super) const fn parked(mut self) -> Self {
        self.active = false;
        self
    }

    pub(super) const fn rebound(
        mut self,
        realization: Entity,
        key: VoxelMaterializationKey,
        revision: u64,
    ) -> Self {
        self.realization = realization;
        self.key = key;
        self.revision = revision;
        self.active = true;
        self
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub(super) struct VoxelPresentationGeometry;

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
pub(super) struct VoxelPresentationManifestationRegistry {
    revisions: HashMap<ManifestationKey, u64>,
    dirty: HashSet<ManifestationKey>,
    entities: HashMap<ManifestationKey, Entity>,
    pooled: Vec<Entity>,
}

impl VoxelPresentationManifestationRegistry {
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
}
