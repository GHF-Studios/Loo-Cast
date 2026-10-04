//! One-to-one materialization-surface membership tracking.

use bevy::prelude::*;

use crate::reconstructible::{
    ReconstructibleFrameBudget, ReconstructibleWorkClass,
};

use super::{
    ManifestationKey, VoxelMaterializationRuntime,
    VoxelMaterializationRuntimeRegistry,
    VoxelPresentationFallbackRetireReady,
};
use super::super::{VoxelStreaming, VoxelWorld};

pub(in crate::voxel) fn sync_manifestation_membership(
    mut commands: Commands,
    mut worlds: Query<(Entity, &mut VoxelWorld, Option<&VoxelStreaming>)>,
    runtimes: Query<&VoxelMaterializationRuntime>,
    retire_ready: Query<Entity, With<VoxelPresentationFallbackRetireReady>>,
    mut registry: ResMut<VoxelMaterializationRuntimeRegistry>,
    mut frame_budget: ResMut<ReconstructibleFrameBudget>,
) {
    // Dense presentation fallbacks are retired by an explicit compositor
    // handshake, not by waiting for another store-dirty event.
    for entity in &retire_ready {
        let Ok(runtime) = runtimes.get(entity) else {
            commands
                .entity(entity)
                .remove::<VoxelPresentationFallbackRetireReady>();
            continue;
        };
        let key = ManifestationKey {
            world: runtime.world(),
            key: runtime.key(),
        };
        if registry.entities.get(&key).copied() != Some(entity) {
            commands
                .entity(entity)
                .remove::<VoxelPresentationFallbackRetireReady>();
            continue;
        }

        // If demand returned before the retirement handshake was consumed,
        // resurrection wins. Do not churn a now-active manifestation through
        // the pool just because last frame's compositor marked it retireable.
        if let Ok((_, _, streaming)) = worlds.get_mut(runtime.world())
            && streaming.is_none_or(|streaming| {
                streaming
                    .effective_roles(runtime.key())
                    .contains(crate::spatial::UsfScaleRoleMask::PRESENTATION)
            })
        {
            commands
                .entity(entity)
                .remove::<VoxelPresentationFallbackRetireReady>();
            continue;
        }

        registry.entities.remove(&key);
        registry.revisions.remove(&key);
        registry.dirty.remove(&key);
        commands
            .entity(entity)
            .insert(((*runtime).parked(), Visibility::Hidden))
            .remove::<VoxelPresentationFallbackRetireReady>();
        if !registry.recycle(entity) {
            commands.entity(entity).despawn();
        }
    }

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
                if let Some(&entity) = registry.entities.get(&key) {
                    commands
                        .entity(entity)
                        .remove::<VoxelPresentationFallbackRetireReady>();
                }
            } else {
                //
                // Physical/store active residency may retire before the binary
                // hierarchy has a visible replacement. If an already-built
                // dense renderer exists and the warm store still owns its
                // surface, keep that renderer shell alive. It is NOT capability
                // coverage: active_dense_revision() is already false.
                let fallback_revision = (!presentation_requested)
                    .then(|| {
                        world
                            .materializations()
                            .surface(materialization_key)
                            .filter(|cache| cache.surface.has_triangles())
                            .map(|cache| cache.revision)
                    })
                    .flatten();
                let existing = registry.entities.get(&key).copied();
                let hold_fallback = existing
                    .zip(fallback_revision)
                    .is_some_and(|(entity, _)| retire_ready.get(entity).is_err());

                if hold_fallback {
                    let (_, revision) = existing
                        .zip(fallback_revision)
                        .expect("fallback tuple was proven present");
                    registry.revisions.insert(key, revision);
                    registry.dirty.remove(&key);
                    frame_budget.finish(work_token);
                    continue;
                }

                registry.revisions.remove(&key);
                registry.dirty.remove(&key);
                if let Some(entity) = registry.entities.remove(&key) {
                    match runtimes.get(entity) {
                        Ok(runtime) => {
                            commands
                                .entity(entity)
                                .insert(((*runtime).parked(), Visibility::Hidden))
                                .remove::<VoxelPresentationFallbackRetireReady>();
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
