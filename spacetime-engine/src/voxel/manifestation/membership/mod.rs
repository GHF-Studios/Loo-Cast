//! One-to-one materialization-surface membership tracking.

use bevy::prelude::*;

use super::{ManifestationKey, VoxelManifestationRegistry};
use super::super::VoxelWorld;

pub(crate) fn sync_manifestation_membership(
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
                    registry.revisions.remove(&key);
                    if registry.entities.contains_key(&key) {
                        registry.dirty.insert(key);
                    } else {
                        registry.dirty.remove(&key);
                    }
                }
            }
        }
    }
}
