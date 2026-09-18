//! Manifestation grouping policy and incremental aggregate membership.

use bevy::prelude::*;

use crate::config::EngineConfig;

use super::{AggregateKey, VoxelManifestationGroupingPolicy, VoxelRenderAggregateRegistry};
use super::super::{
    VoxelMaterializationChunkAddress, VoxelWorld,
    aggregate::VoxelMaterializationAggregateScope,
};

/// Applies manifestation-grouping policy changes.
///
/// Regrouping invalidates only disposable manifestation state. Store-owned dense
/// materializations and semantic voxel state remain untouched.
pub(crate) fn sync_manifestation_grouping_policy(
    config: Res<EngineConfig>,
    mut commands: Commands,
    mut worlds: Query<&mut VoxelWorld>,
    mut registry: ResMut<VoxelRenderAggregateRegistry>,
) {
    let grouping_policy =
        VoxelManifestationGroupingPolicy::from_config(config.voxel.manifestation.grouping)
            .expect("validated engine config must produce a manifestation grouping policy");

    if registry.grouping_policy == Some(grouping_policy) {
        return;
    }

    for (_, entity) in registry.aggregate_entities.drain() {
        commands.entity(entity).despawn();
    }
    registry.groups.clear();
    registry.address_keys.clear();
    registry.dirty.clear();
    registry.grouping_policy = Some(grouping_policy);

    for mut world in &mut worlds {
        world.materializations_mut().mark_all_active_render_dirty();
    }

    info!(
        base_chunks_per_axis = grouping_policy.extent.base_chunks_per_axis(),
        native_units_per_axis = grouping_policy.extent.native_units_per_axis(),
        "reconfigured voxel physical manifestation grouping"
    );
}

/// Consumes store-side surface changes into manifestation membership.
///
/// This stage is proportional to changed surface caches rather than resident
/// world size.
pub(crate) fn sync_manifestation_membership(
    mut worlds: Query<(Entity, &mut VoxelWorld)>,
    mut registry: ResMut<VoxelRenderAggregateRegistry>,
) {
    let Some(grouping_policy) = registry.grouping_policy else {
        return;
    };

    for (world_entity, mut world) in &mut worlds {
        while let Some(address) = world.materializations_mut().pop_dirty_render() {
            let current = world
                .materializations()
                .active_surface(address)
                .map(|surface| surface.revision);
            let previous = registry.address_keys.get(&(world_entity, address)).copied();

            match current {
                Some(revision) => {
                    let Ok(scope) = VoxelMaterializationAggregateScope::containing(
                        &world,
                        address,
                        grouping_policy.extent,
                    ) else {
                        warn!(
                            ?address,
                            "voxel manifestation scope could not be derived canonically"
                        );
                        continue;
                    };
                    let key = AggregateKey {
                        world: world_entity,
                        scope,
                    };

                    if let Some(previous) = previous
                        && previous != key
                    {
                        remove_member(&mut registry, previous, address);
                    }

                    registry.address_keys.insert((world_entity, address), key);
                    registry
                        .groups
                        .entry(key)
                        .or_default()
                        .insert(address, revision);
                    registry.dirty.insert(key);
                }
                None => {
                    if let Some(previous) = registry.address_keys.remove(&(world_entity, address)) {
                        remove_member(&mut registry, previous, address);
                    }
                }
            }
        }
    }
}

fn remove_member(
    registry: &mut VoxelRenderAggregateRegistry,
    key: AggregateKey,
    address: VoxelMaterializationChunkAddress,
) {
    if let Some(group) = registry.groups.get_mut(&key) {
        group.remove(&address);
    }
    registry.dirty.insert(key);
}
