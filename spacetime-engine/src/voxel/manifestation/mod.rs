//! One-to-one runtime manifestations derived from store-owned voxel surfaces.
//!
//! Each active 10-native-unit materialization surface owns one disposable
//! runtime manifestation. Rendering and collision share that manifestation's
//! canonical address but have independent lifecycle stages. No cross-chunk
//! render/collision aggregation or semantic LOD is performed here.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use super::VoxelMaterializationChunkAddress;

mod collision;
mod lifecycle;
mod membership;
mod rebuild;

pub(super) use collision::sync_manifestation_collision_residency;
pub(super) use lifecycle::retire_removed_world_manifestations;
pub(super) use membership::sync_manifestation_membership;
pub(super) use rebuild::{
    initialize_translucent_voxel_material, rebuild_dirty_manifestations,
    sync_manifestation_runtime_transforms,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ManifestationKey {
    world: Entity,
    address: VoxelMaterializationChunkAddress,
}

/// Root entity for one disposable same-resolution render/collision manifestation.
#[derive(Component)]
pub(super) struct VoxelManifestation {
    presentation: Entity,
    translucent_presentation: Option<Entity>,
}

/// Marks the only `Mesh3d` entity created for one voxel manifestation.
#[derive(Component, Debug, Default, Clone, Copy)]
pub(super) struct VoxelManifestationPresentation;

/// Incremental one-to-one mapping from canonical materialization surfaces to
/// disposable runtime entities.
#[derive(Resource, Default)]
pub(super) struct VoxelManifestationRegistry {
    revisions: HashMap<ManifestationKey, u64>,
    dirty: HashSet<ManifestationKey>,
    entities: HashMap<ManifestationKey, Entity>,
}
