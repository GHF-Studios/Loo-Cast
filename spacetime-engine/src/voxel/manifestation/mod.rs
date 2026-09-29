//! One-to-one runtime manifestations derived from store-owned voxel surfaces.
//!
//! Each active 10-native-unit materialization surface owns one disposable
//! runtime manifestation. Rendering and collision share that manifestation's
//! canonical address but have independent lifecycle stages. No cross-chunk
//! render/collision aggregation or semantic LOD is performed here.

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

pub(super) use collision::sync_manifestation_collision_residency;
pub(super) use coverage::sync_capability_realizations;
pub(super) use lifecycle::retire_removed_world_manifestations;
pub use material::VoxelPresentationMaterial;
pub(super) use material::VoxelRenderMaterial;
pub(super) use membership::sync_manifestation_membership;
pub(super) use rebuild::{
    rebuild_dirty_manifestations, sync_manifestation_runtime_transforms,
};

pub(super) fn configure(app: &mut App) {
    frontier::configure(app);
    material::configure(app);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ManifestationKey {
    world: Entity,
    key: VoxelMaterializationKey,
}

/// Root entity for one disposable same-resolution render/collision manifestation.
#[derive(Component)]
pub(super) struct VoxelMaterializationRuntime {
    world: Entity,
    key: VoxelMaterializationKey,
    revision: u64,
    presentation: Entity,
    translucent_presentation: Option<Entity>,
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
}

/// Marks the only `Mesh3d` entity created for one voxel manifestation.
#[derive(Component, Debug, Default, Clone, Copy)]
pub(super) struct VoxelMaterializationPresentation;

/// Incremental one-to-one mapping from canonical materialization surfaces to
/// disposable runtime entities.
#[derive(Resource, Default)]
pub(super) struct VoxelMaterializationRuntimeRegistry {
    revisions: HashMap<ManifestationKey, u64>,
    dirty: HashSet<ManifestationKey>,
    entities: HashMap<ManifestationKey, Entity>,
}
