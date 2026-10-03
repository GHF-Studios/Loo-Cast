//! One-to-one materialization-surface membership tracking.

use bevy::prelude::*;

use crate::reconstructible::{
    ReconstructibleFrameBudget, ReconstructibleWorkClass,
};

use super::{
    ManifestationKey, VoxelMaterializationRuntime,
    VoxelMaterializationRuntimeRegistry,
};
use super::super::{VoxelStreaming, VoxelWorld};

pub(in crate::voxel) fn sync_manifestation_membership(
    mut commands: Commands,
    mut worlds: Query<(Entity, &mut VoxelWorld, Option<&VoxelStreaming>)>,
    runtimes: Query<&VoxelMaterializationRuntime>,
    mut registry: ResMut<VoxelMaterializationRuntimeRegistry>,
    mut frame_budget: ResMut<ReconstructibleFrameBudget>,
) {
    'worlds: for (world_entity, mut world, streaming) in &mut worlds {
        loop {
            let Some(work_token) =
                frame_budget.begin(ReconstructibleWorkClass::Publication)
            else {
                break 'worlds;
            };
            let Some(materialization_key) =
                world.materializations_mut().pop_dirty_render()
            else {
                frame_budget.finish(work_token);
                break;
            };

            let key = ManifestationKey {
                world: world_entity,
                key: materialization_key,
            };
            // demand-owned-render-membership-v1
            let presentation_requested = streaming.is_none_or(|streaming| {
                streaming
                    .effective_roles(materialization_key)
                    .contains(crate::spatial::UsfScaleRoleMask::PRESENTATION)
            });
            let surface_revision = presentation_requested
                .then(|| {
                    world
                        .materializations()
                        .active_surface(materialization_key)
                        .filter(|cache| cache.surface.has_triangles())
                        .map(|cache| cache.revision)
                })
                .flatten();

            if let Some(revision) = surface_revision {
                registry.revisions.insert(key, revision);
                registry.dirty.insert(key);
            } else {
                registry.revisions.remove(&key);
                registry.dirty.remove(&key);
                if let Some(entity) = registry.entities.remove(&key) {
                    match runtimes.get(entity) {
                        Ok(runtime) => {
                            commands.entity(entity).insert(((*runtime).parked(), Visibility::Hidden));
                            if !registry.recycle(entity) {
                                commands.entity(entity).despawn();
                            }
                        }
                        Err(_) => commands.entity(entity).despawn(),
                    }
                }
            }
            frame_budget.finish(work_token);
        }
    }
}
