//! Lifetime cleanup for disposable voxel manifestations.

use std::collections::HashSet;

use bevy::{ecs::lifecycle::RemovedComponents, prelude::*};

use super::VoxelManifestationRegistry;
use super::super::VoxelWorld;

pub(in crate::voxel) fn retire_removed_world_manifestations(
    mut commands: Commands,
    mut removed_worlds: RemovedComponents<VoxelWorld>,
    mut registry: ResMut<VoxelManifestationRegistry>,
) {
    let removed = removed_worlds.read().collect::<HashSet<_>>();
    if removed.is_empty() {
        return;
    }

    let dead_entities = registry
        .entities
        .iter()
        .filter_map(|(key, &entity)| removed.contains(&key.world).then_some((*key, entity)))
        .collect::<Vec<_>>();

    for (key, entity) in dead_entities {
        registry.entities.remove(&key);
        commands.entity(entity).despawn();
    }

    registry
        .revisions
        .retain(|key, _| !removed.contains(&key.world));
    registry.dirty.retain(|key| !removed.contains(&key.world));
}
