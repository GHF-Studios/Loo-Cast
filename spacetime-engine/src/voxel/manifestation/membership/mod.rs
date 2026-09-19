//! One-to-one materialization-surface membership tracking.

use bevy::prelude::*;

use super::{ManifestationKey, VoxelManifestationRegistry};
use super::super::VoxelWorld;

pub(crate) fn sync_manifestation_membership(
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
                    // Retire them immediately instead of feeding destruction
                    // through the rebuild budget. Delaying retirement lets stale
                    // render/physics manifestations accumulate while the spatial
                    // demand window moves, and the backlog becomes self-amplifying
                    // when frame rate falls.
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
