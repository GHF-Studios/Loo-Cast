//! Asynchronous derivation of disposable CPU geometry caches.
//!
//! Dense voxel atoms live in [`VoxelWorld`]'s compact store. Worker jobs are ECS
//! entities only while work is in flight; finished surface caches return to the
//! store. Rendering and physics consume each materialization cache independently.

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures::check_ready},
};

use crate::spatial::{SPATIAL_SCALE_MAX, SpatialScale, UsfPosition, UsfScaleLayer};

use super::{
    VoxelMaterializationChunkAddress, VoxelWorld,
    mesh::{self, VoxelSurface},
    store::VoxelSurfaceCache,
    worker::{VoxelWorkerTask, available_slots},
};

const DERIVED_TASK_START_BUDGET_PER_FRAME: usize = 8;
const DERIVED_PUBLISH_BUDGET_PER_FRAME: usize = 8;

struct VoxelDerivedOutput {
    surface: VoxelSurface,
    debug_color: [f32; 4],
}

/// One in-flight surface extraction for one materialization revision.
#[derive(Component)]
pub(super) struct VoxelDerivedTask {
    world: Entity,
    address: VoxelMaterializationChunkAddress,
    revision: u64,
    task: Task<VoxelDerivedOutput>,
}

/// Polls completed worker jobs and returns derived caches to the materialization
/// store. Stale results never overwrite a newer edit revision.
pub(super) fn publish_completed_chunk_builds(
    mut commands: Commands,
    mut worlds: Query<&mut VoxelWorld>,
    mut tasks: Query<(Entity, &mut VoxelDerivedTask)>,
) {
    let mut published = 0;

    for (task_entity, mut build) in &mut tasks {
        if published >= DERIVED_PUBLISH_BUDGET_PER_FRAME {
            break;
        }

        let Some(output) = check_ready(&mut build.task) else {
            continue;
        };

        if let Ok(mut world) = worlds.get_mut(build.world) {
            let has_surface =
                !output.surface.positions.is_empty() && !output.surface.indices.is_empty();
            let cache = has_surface.then(|| {
                VoxelSurfaceCache::new(build.revision, output.surface, output.debug_color)
            });
            world
                .materializations_mut()
                .publish_surface(build.address, build.revision, cache);
        }

        commands.entity(task_entity).despawn();
        published += 1;
    }
}

/// Starts bounded surface-extraction jobs from an explicit dirty-address queue.
///
/// This is deliberately O(changes), not O(resident materializations). Quiet
/// cached terrain does no per-frame geometry scheduling work.
pub(super) fn queue_dirty_chunk_builds(
    mut commands: Commands,
    mut worlds: Query<(Entity, &mut VoxelWorld, &UsfScaleLayer)>,
    worker_tasks: Query<(), With<VoxelWorkerTask>>,
) {
    let pool = AsyncComputeTaskPool::get();
    let available = available_slots(worker_tasks.iter().count());
    if available == 0 {
        return;
    }

    let budget = DERIVED_TASK_START_BUDGET_PER_FRAME.min(available);
    let mut started = 0;

    for (world_entity, mut world, layer) in &mut worlds {
        while started < budget {
            let Some(address) = world.materializations_mut().pop_dirty_derived() else {
                break;
            };
            let Some((revision, snapshot)) =
                world.materializations_mut().begin_surface_build(address)
            else {
                continue;
            };

            let debug_color = debug_chunk_color(address, layer.scale());
            let task = pool.spawn(async move {
                let surface = mesh::extract_chunk_surface(&snapshot);
                VoxelDerivedOutput {
                    surface,
                    debug_color,
                }
            });

            commands.spawn((
                Name::new("Voxel Surface Derivation"),
                VoxelWorkerTask,
                VoxelDerivedTask {
                    world: world_entity,
                    address,
                    revision,
                    task,
                },
            ));
            started += 1;
        }

        if started >= budget {
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
