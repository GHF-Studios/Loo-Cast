//! Lifetime cleanup for disposable voxel manifestations.

use std::collections::HashSet;

use bevy::{ecs::lifecycle::RemovedComponents, prelude::*};

use super::VoxelRenderAggregateRegistry;
use super::super::VoxelWorld;

/// Retires all disposable manifestations owned by voxel worlds that disappeared.
///
/// This stage owns world-lifetime cleanup only. It does not derive membership,
/// build meshes, or make collision-residency decisions.
pub(crate) fn retire_removed_world_manifestations(
    mut commands: Commands,
    mut removed_worlds: RemovedComponents<VoxelWorld>,
    mut registry: ResMut<VoxelRenderAggregateRegistry>,
) {
    let removed = removed_worlds.read().collect::<HashSet<_>>();
    if removed.is_empty() {
        return;
    }

    let dead_keys = registry
        .aggregate_entities
        .keys()
        .copied()
        .filter(|key| removed.contains(&key.world))
        .collect::<Vec<_>>();

    for key in dead_keys {
        if let Some(entity) = registry.aggregate_entities.remove(&key) {
            commands.entity(entity).despawn();
        }
        registry.groups.remove(&key);
        registry.dirty.remove(&key);
    }

    registry
        .address_keys
        .retain(|(world, _), _| !removed.contains(world));
}
