//! One-to-one materialization-surface membership tracking.

use bevy::prelude::*;

use super::{
    ManifestationKey, VoxelMaterializationRuntime,
    VoxelMaterializationRuntimeRegistry,
};
use super::super::VoxelWorld;

pub(in crate::voxel) fn sync_manifestation_membership(
    mut commands: Commands,
    mut worlds: Query<(Entity, &mut VoxelWorld)>,
    runtimes: Query<&VoxelMaterializationRuntime>,
    mut registry: ResMut<VoxelMaterializationRuntimeRegistry>,
) {
    for (world_entity, mut world) in &mut worlds {
        while let Some(materialization_key) = world.materializations_mut().pop_dirty_render() {
            let key = ManifestationKey {
                world: world_entity,
                key: materialization_key,
            };

            let surface_revision = world
                .materializations()
                .active_surface(materialization_key)
                .filter(|cache| cache.surface.has_triangles())
                .map(|cache| cache.revision);

            if let Some(revision) = surface_revision {
                registry.revisions.insert(key, revision);
                registry.dirty.insert(key);
            } else {
                registry.revisions.remove(&key);
                registry.dirty.remove(&key);
                if let Some(entity) = registry.entities.remove(&key) {
                    match runtimes.get(entity) {
                        Ok(runtime) => {
                            commands.entity(entity).insert((
                                (*runtime).parked(),
                                Visibility::Hidden,
                            ));
                            if !registry.recycle(entity) {
                                commands.entity(entity).despawn();
                            }
                        }
                        Err(_) => commands.entity(entity).despawn(),
                    }
                }
            }
        }
    }
}
