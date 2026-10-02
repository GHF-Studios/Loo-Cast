//! One-to-one materialization-surface membership tracking.

use bevy::prelude::*;

use super::{ManifestationKey, VoxelMaterializationRuntimeRegistry};
use super::super::VoxelWorld;

pub(in crate::voxel) fn sync_manifestation_membership(
    mut commands: Commands,
    mut worlds: Query<(Entity, &mut VoxelWorld)>,
    mut registry: ResMut<VoxelMaterializationRuntimeRegistry>,
) {
    for (world_entity, mut world) in &mut worlds {
        while let Some(materialization_key) = world.materializations_mut().pop_dirty_render() {
            let key = ManifestationKey {
                world: world_entity,
                key: materialization_key,
            };

            // surface-only-manifestation-v1
            //
            // Runtime manifestations are presentation geometry, not residency
            // facts. Uniform air/solid and any other derived-current no-surface
            // result publish store-backed capability coverage instead of owning
            // one empty ECS root + child.
            let surface_revision = world
                .materializations()
                .active_surface(materialization_key)
                .filter(|cache| cache.surface.has_triangles())
                .map(|cache| cache.revision);

            if let Some(revision) = surface_revision {
                registry.revisions.insert(key, revision);
                registry.dirty.insert(key);
            } else {
                // Retirement is not rebuild work, so it must never compete with
                // the bounded manifestation rebuild budget.
                registry.revisions.remove(&key);
                registry.dirty.remove(&key);
                if let Some(entity) = registry.entities.remove(&key) {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
