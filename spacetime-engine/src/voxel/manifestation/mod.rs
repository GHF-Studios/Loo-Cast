//! Runtime manifestations derived from store-owned voxel surface caches.
//!
//! The 10-native-unit materialization atom remains independently addressable in
//! the [`VoxelWorld`] store. This module groups cached atom surfaces into a much
//! smaller number of Bevy/Avian manifestations. Rendering and collision are
//! consumers of the same grouping membership, but remain separate lifecycle
//! stages. No semantic LOD is introduced.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use crate::config::{VoxelGroupingStrategy, VoxelManifestationGroupingConfig};

use super::{
    VoxelMaterializationChunkAddress,
    aggregate::{VoxelMaterializationAggregateExtent, VoxelMaterializationAggregateScope},
};

mod collision;
mod grouping;
mod lifecycle;
mod rebuild;

pub(crate) use collision::{
    aggregate_collider_proximity_squared, sync_manifestation_collision_residency,
};
pub(crate) use grouping::{
    sync_manifestation_grouping_policy, sync_manifestation_membership,
};
pub(crate) use lifecycle::retire_removed_world_manifestations;
pub(crate) use rebuild::rebuild_dirty_manifestations;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AggregateKey {
    world: Entity,
    scope: VoxelMaterializationAggregateScope,
}

/// Maps virtual atom surfaces to physical mesh manifestations.
///
/// Semantic voxel identity never depends on this policy. Future adaptive/cost
/// partitioners can replace `AlignedRegions` behind this boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VoxelManifestationGroupingPolicy {
    extent: VoxelMaterializationAggregateExtent,
}

impl VoxelManifestationGroupingPolicy {
    fn from_config(config: VoxelManifestationGroupingConfig) -> Option<Self> {
        match config.strategy {
            VoxelGroupingStrategy::AlignedRegions => {
                VoxelMaterializationAggregateExtent::from_base_chunks_per_axis(
                    config.base_chunks_per_axis,
                )
                .map(|extent| Self { extent })
            }
        }
    }
}

/// Root entity for one disposable same-resolution render/collision manifestation.
#[derive(Component)]
pub(crate) struct VoxelRenderAggregate {
    presentation: Entity,
    member_count: usize,
    scope: VoxelMaterializationAggregateScope,
}

impl VoxelRenderAggregate {
    pub(crate) const fn member_count(&self) -> usize {
        self.member_count
    }

    pub(crate) const fn scope(&self) -> VoxelMaterializationAggregateScope {
        self.scope
    }
}

/// Marks the only `Mesh3d` entity created for one render aggregate.
#[derive(Component, Debug, Default, Clone, Copy)]
pub(crate) struct VoxelRenderAggregatePresentation;

/// Incremental aggregate membership registry keyed by canonical atom addresses.
///
/// Quiet resident atoms never participate in this system. Store-side dirty
/// queues update only aggregates whose membership or cached surface changed.
#[derive(Resource, Default)]
pub(crate) struct VoxelRenderAggregateRegistry {
    grouping_policy: Option<VoxelManifestationGroupingPolicy>,
    groups: HashMap<AggregateKey, HashMap<VoxelMaterializationChunkAddress, u64>>,
    address_keys: HashMap<(Entity, VoxelMaterializationChunkAddress), AggregateKey>,
    dirty: HashSet<AggregateKey>,
    aggregate_entities: HashMap<AggregateKey, Entity>,
}
