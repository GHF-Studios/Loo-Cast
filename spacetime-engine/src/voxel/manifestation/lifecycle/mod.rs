//! Lifetime cleanup for disposable voxel manifestations.

use std::collections::HashSet;

use bevy::{ecs::lifecycle::RemovedComponents, prelude::*};

use super::super::VoxelScaleRealization;
use super::{VoxelPresentationManifestation, VoxelPresentationManifestationRegistry};

pub(in crate::voxel) fn retire_orphaned_presentation_manifestations(
    mut commands: Commands,
    mut removed_worlds: RemovedComponents<VoxelScaleRealization>,
    runtimes: Query<&VoxelPresentationManifestation>,
    mut registry: ResMut<VoxelPresentationManifestationRegistry>,
) {
    let removed = removed_worlds.read().collect::<HashSet<_>>();
    if removed.is_empty() {
        return;
    }

    let dead_entities = registry
        .entities
        .iter()
        .filter_map(|(key, &entity)| removed.contains(&key.realization).then_some((*key, entity)))
        .collect::<Vec<_>>();

    for (key, entity) in dead_entities {
        registry.entities.remove(&key);
        match runtimes.get(entity) {
            Ok(runtime) => {
                commands
                    .entity(entity)
                    .insert(((*runtime).parked(), Visibility::Hidden));
                if !registry.recycle(entity) {
                    commands.entity(entity).despawn();
                }
            }
            Err(_) => commands.entity(entity).despawn(),
        }
    }

    registry
        .revisions
        .retain(|key, _| !removed.contains(&key.realization));
    registry
        .dirty
        .retain(|key| !removed.contains(&key.realization));
}
