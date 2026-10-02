//! Asynchronous derivation of disposable CPU geometry caches.

//!
//! Dense voxel atoms live in [`VoxelWorld`]'s compact store. Worker jobs are ECS
//! entities only while work is in flight; finished surface caches return to the
//! store. Rendering and physics consume each materialization cache independently.

use std::collections::HashSet;

use bevy::prelude::*;

use crate::spatial::{SPATIAL_SCALE_MAX, SpatialScale, UsfPosition, UsfScaleLayer};

use super::{
    VoxelBase, VoxelMaterializationChunkAddress, VoxelMaterializationKey,
    VoxelWorld,
    streaming::VoxelStreamingTelemetry,
    mesh::{self, VoxelSurface},
    store::VoxelSurfaceCache,
    worker::{VoxelWorkerLane, VoxelWorkerPool, VoxelWorkerTask, VoxelWorkerTicket},
};

const DERIVED_PUBLISH_BUDGET_PER_FRAME: usize = 32;
const DERIVED_EMPTY_PUBLISH_BUDGET_PER_FRAME: usize = 64;

struct VoxelDerivedOutput {
    surface: VoxelSurface,
    debug_color: [f32; 4],
}

/// One in-flight surface extraction for one materialization revision.
#[derive(Component)]
pub(super) struct VoxelDerivedTask {
    world: Entity,
    key: VoxelMaterializationKey,
    revision: u64,
    task: VoxelWorkerTicket<VoxelDerivedOutput>,
}

/// Polls completed worker jobs and returns derived caches to the materialization
/// store. Stale results never overwrite a newer edit revision.
pub(super) fn publish_completed_chunk_builds(
    mut commands: Commands,
    mut worlds: Query<(Option<&Name>, &mut VoxelWorld)>,
    mut tasks: Query<(Entity, &mut VoxelDerivedTask)>,
    mut announced_celestial_surfaces: Local<HashSet<Entity>>,
    mut telemetry: ResMut<VoxelStreamingTelemetry>,
) {
    let mut published = 0;

    for (task_entity, mut build) in &mut tasks {
        if published >= DERIVED_PUBLISH_BUDGET_PER_FRAME {
            break;
        }

        let Some(output) = build.task.try_take() else {
            continue;
        };

        if let Ok((name, mut world)) = worlds.get_mut(build.world) {
            let has_surface =
                !output.surface.positions.is_empty() && !output.surface.indices.is_empty();

            if has_surface
                && matches!(world.base(), VoxelBase::CelestialBody(_))
                && announced_celestial_surfaces.insert(build.world)
            {
                info!(
                    world = %name.map_or("<unnamed celestial voxel world>", Name::as_str),
                    key = ?build.key,
                    vertices = output.surface.positions.len(),
                    triangles = output.surface.indices.len() / 3,
                    "celestial voxel world published its first non-empty terrain surface"
                );
            }

            let cache = has_surface.then(|| {
                VoxelSurfaceCache::new(build.revision, output.surface, output.debug_color)
            });
            world
                .materializations_mut()
                .publish_surface(build.key, build.revision, cache);
        }

        commands.entity(task_entity).despawn();
        telemetry.derived_completed();
        published += 1;
    }
}

/// Cancels Surface-Nets work once its source materialization is no longer active.
pub(super) fn retire_stale_chunk_builds(
    mut commands: Commands,
    mut worlds: Query<&mut VoxelWorld>,
    tasks: Query<(Entity, &VoxelDerivedTask)>,
    mut telemetry: ResMut<VoxelStreamingTelemetry>,
) {
    for (entity, build) in &tasks {
        let stale = match worlds.get_mut(build.world) {
            Ok(mut world) => {
                if world.materializations().is_active(build.key) {
                    false
                } else {
                    world
                        .materializations_mut()
                        .cancel_surface_build(build.key, build.revision);
                    true
                }
            }
            Err(_) => true,
        };
        if stale {
            telemetry.derived_cancelled();
            commands.entity(entity).despawn();
        }
    }
}

/// Starts bounded surface-extraction jobs from an explicit dirty-address queue.
///
/// This is deliberately O(changes), not O(resident materializations). Quiet
/// cached terrain does no per-frame geometry scheduling work.
pub(super) fn queue_dirty_chunk_builds(
    mut commands: Commands,
    workers: Res<VoxelWorkerPool>,
    mut worlds: Query<(Entity, &mut VoxelWorld, &UsfScaleLayer)>,
    mut telemetry: ResMut<VoxelStreamingTelemetry>,
) {
    let task_budget = workers.available_slots(VoxelWorkerLane::Derivation);
    let mut started = 0;
    let mut empty_published = 0;

    for (world_entity, mut world, layer) in &mut worlds {
        while started < task_budget
            || empty_published < DERIVED_EMPTY_PUBLISH_BUDGET_PER_FRAME
        {
            let Some(key) = world.materializations_mut().pop_dirty_derived() else {
                break;
            };
            let Some((revision, snapshot)) =
                world.materializations_mut().begin_surface_build(key)
            else {
                continue;
            };

            // Generation/editing already computed this exact invariant. Known
            // all-solid/all-empty materializations have no isosurface, so mark
            // their derived revision current immediately instead of consuming
            // an async worker slot and running Surface Nets to rediscover
            // emptiness.
            if !snapshot.has_surface_transition() {
                world
                    .materializations_mut()
                    .publish_surface(key, revision, None);
                telemetry.derived_skipped_empty();
                empty_published += 1;
                continue;
            }

            if started >= task_budget {
                // Preserve bounded main-thread work and FIFO ownership. This
                // reservation is returned to the dirty queue for a later frame
                // when an actual async worker slot is available.
                world
                    .materializations_mut()
                    .cancel_surface_build(key, revision);
                break;
            }

            let Ok(address) = world.materialization_address(key) else {
                world.materializations_mut().cancel_surface_build(key, revision);
                continue;
            };

            let debug_color = debug_chunk_color(address, layer.scale());
            let Some(task) = workers.try_submit(VoxelWorkerLane::Derivation, move || {
                let surface = mesh::extract_chunk_surface(&snapshot);
                VoxelDerivedOutput {
                    surface,
                    debug_color,
                }
            }) else {
                world
                    .materializations_mut()
                    .cancel_surface_build(key, revision);
                break;
            };

            telemetry.derived_started();
            commands.spawn((
                Name::new("Voxel Surface Derivation"),
                VoxelWorkerTask,
                VoxelDerivedTask {
                    world: world_entity,
                    key,
                    revision,
                    task,
                },
            ));
            started += 1;
        }

        if started >= task_budget
            && empty_published >= DERIVED_EMPTY_PUBLISH_BUDGET_PER_FRAME
        {
            break;
        }
    }
}

fn debug_chunk_color(address: VoxelMaterializationChunkAddress, scale: SpatialScale) -> [f32; 4] {
    let origin = *address.origin();
    let chart_zero = UsfPosition::zero(scale);
    let mut color = Vec3::splat(0.72);
    let mut initialized = false;

    for raw in (scale.exponent()..=SPATIAL_SCALE_MAX).rev() {
        let level = SpatialScale::new(raw).expect("validated debug color scale");
        let relative = origin
            .relative_at_scale_bounded(&chart_zero, level, 1_000_000.0)
            .unwrap_or(Vec3::ZERO);
        let cell = (relative / 10.0).floor().as_ivec3();
        let hash = debug_hash(cell, level);
        let candidate = Vec3::new(
            0.32 + ((hash & 0xFF) as f32 / 255.0) * 0.62,
            0.32 + (((hash >> 8) & 0xFF) as f32 / 255.0) * 0.62,
            0.32 + (((hash >> 16) & 0xFF) as f32 / 255.0) * 0.62,
        );

        if initialized {
            color = color.lerp(candidate, 0.20);
        } else {
            color = candidate;
            initialized = true;
        }
    }

    [color.x, color.y, color.z, 1.0]
}

fn debug_hash(cell: IVec3, scale: SpatialScale) -> u32 {
    let mut value = (scale.exponent() as i32 as u32).wrapping_mul(0x9E37_79B9);
    for component in [cell.x, cell.y, cell.z] {
        value ^= (component as u32).wrapping_mul(0x85EB_CA6B);
        value ^= value >> 16;
        value = value.wrapping_mul(0x7FEB_352D);
        value ^= value >> 15;
    }
    value
}
