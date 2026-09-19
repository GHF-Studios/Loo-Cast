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

pub(crate) use collision::sync_manifestation_collision_residency;
pub(crate) use lifecycle::retire_removed_world_manifestations;
pub(crate) use membership::sync_manifestation_membership;
pub(crate) use rebuild::rebuild_dirty_manifestations;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ManifestationKey {
    world: Entity,
    address: VoxelMaterializationChunkAddress,
}

/// Root entity for one disposable same-resolution render/collision manifestation.
#[derive(Component)]
pub(crate) struct VoxelManifestation {
    presentation: Entity,
}

/// Marks the only `Mesh3d` entity created for one voxel manifestation.
#[derive(Component, Debug, Default, Clone, Copy)]
pub(crate) struct VoxelManifestationPresentation;

/// Incremental one-to-one mapping from canonical materialization surfaces to
/// disposable runtime entities.
#[derive(Resource, Default)]
pub(crate) struct VoxelManifestationRegistry {
    revisions: HashMap<ManifestationKey, u64>,
    dirty: HashSet<ManifestationKey>,
    entities: HashMap<ManifestationKey, Entity>,
}
