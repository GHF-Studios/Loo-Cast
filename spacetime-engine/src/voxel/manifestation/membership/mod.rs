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

            match world
                .materializations()
                .active_derived_revision(materialization_key)
            {
                Some(revision) => {
                    registry.revisions.insert(key, revision);
                    registry.dirty.insert(key);
                }
                None => {
                    // Only inactive/not-yet-derived materializations retire.
                    // A derived-current empty result deliberately keeps a
                    // meshless runtime so capability coverage can represent
                    // known-empty presentation truth.
                    //
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
