//! One-to-one materialization-surface membership tracking.

use bevy::prelude::*;

use super::{ManifestationKey, VoxelManifestationRegistry};
use super::super::VoxelWorld;

pub(in crate::voxel) fn sync_manifestation_membership(
    mut commands: Commands,
    mut worlds: Query<(Entity, &mut VoxelWorld)>,
    mut registry: ResMut<VoxelManifestationRegistry>,
) {
    for (world_entity, mut world) in &mut worlds {
        while let Some(address) = world.materializations_mut().pop_dirty_render() {
            let key = ManifestationKey {
                world: world_entity,
                address,
            };

            match world
                .materializations()
                .active_surface(address)
                .map(|surface| surface.revision)
            {
                Some(revision) => {
                    registry.revisions.insert(key, revision);
                    registry.dirty.insert(key);
                }
                None => {
                    // Inactive manifestations are disposable runtime state.
                    // Retirement is not rebuild work, so it must never compete
                    // with the bounded manifestation rebuild budget.
                    registry.revisions.remove(&key);
                    registry.dirty.remove(&key);
                    if let Some(entity) = registry.entities.remove(&key) {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}
