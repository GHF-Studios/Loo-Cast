//! Asynchronous derivation of disposable CPU geometry caches.

//!
//! Dense voxel atoms live in [`VoxelWorld`]'s compact store. Worker jobs are ECS
//! entities only while work is in flight; finished surface caches return to the
//! store. Rendering and physics consume each materialization cache independently.

use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    reconstructible::{ReconstructibleFrameBudget, ReconstructibleWorkClass},
    spatial::{SpatialScale, UsfPrimaryInteractionSlice, UsfScaleLayer},
};

use super::{
    VoxelBase, VoxelMaterializationKey, VoxelWorld,
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
    mut frame_budget: ResMut<ReconstructibleFrameBudget>,
) {
    let mut published = 0;
    for (task_entity, mut build) in &mut tasks {
        if published >= DERIVED_PUBLISH_BUDGET_PER_FRAME {
            break;
        }
        let Some(work_token) =
            frame_budget.begin(ReconstructibleWorkClass::Publication)
        else {
            break;
        };
        let Some(output) = build.task.try_take() else {
            frame_budget.finish(work_token);
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
        frame_budget.finish(work_token);
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
    interaction: Res<UsfPrimaryInteractionSlice>,
    mut worlds: Query<(
        Entity,
        &mut VoxelWorld,
        &UsfScaleLayer,
        Option<&super::streaming::VoxelStreaming>,
    )>,
    mut telemetry: ResMut<VoxelStreamingTelemetry>,
    mut frame_budget: ResMut<ReconstructibleFrameBudget>,
    mut round_robin_cursor: Local<usize>,
) {
    let task_budget =
        workers.available_slots(VoxelWorkerLane::Derivation);
    let mut started = 0usize;
    let mut empty_published = 0usize;

    let mut world_entities = worlds
        .iter_mut()
        .map(|(entity, _, _, _)| entity)
        .collect::<Vec<_>>();
    if world_entities.is_empty() {
        return;
    }

    let rotate = *round_robin_cursor % world_entities.len();
    world_entities.rotate_left(rotate);
    let mut exhausted = HashSet::<Entity>::new();

    while started < task_budget
        || empty_published < DERIVED_EMPTY_PUBLISH_BUDGET_PER_FRAME
    {
        let mut progressed = false;

        for world_entity in world_entities.iter().copied() {
            if exhausted.contains(&world_entity) {
                continue;
            }
            if started >= task_budget
                && empty_published >= DERIVED_EMPTY_PUBLISH_BUDGET_PER_FRAME
            {
                break;
            }

            let Some(work_token) =
                frame_budget.begin(ReconstructibleWorkClass::Maintenance)
            else {
                *round_robin_cursor =
                    (*round_robin_cursor).wrapping_add(1);
                return;
            };

            let Ok((_, mut world, layer, streaming)) =
                worlds.get_mut(world_entity)
            else {
                exhausted.insert(world_entity);
                frame_budget.finish(work_token);
                continue;
            };

            //
            // Generation completion order is not scheduling authority. Pick the
            // dirty surface whose current streaming rank says it is most useful.
            let key = match streaming {
                Some(streaming) => world
                    .materializations_mut()
                    .pop_dirty_derived_best_by(|a, b| {
                        streaming.compare_work_keys(a, b)
                    }),
                None => world.materializations_mut().pop_dirty_derived(),
            };
            let Some(key) = key else {
                exhausted.insert(world_entity);
                frame_budget.finish(work_token);
                continue;
            };

            if streaming.is_some_and(|streaming| !streaming.surface_required(key)) {
                frame_budget.finish(work_token);
                progressed = true;
                continue;
            }

            let critical = streaming.is_some_and(|streaming| {
                let roles = streaming.effective_roles(key);
                roles.contains(crate::spatial::UsfScaleRoleMask::COLLISION)
                    || roles.contains(crate::spatial::UsfScaleRoleMask::EDITING)
            });

            let Some((revision, snapshot)) =
                world.materializations_mut().begin_surface_build(key)
            else {
                frame_budget.finish(work_token);
                progressed = true;
                continue;
            };

            if !snapshot.has_surface_transition() {
                world.materializations_mut().publish_surface(key, revision, None);
                telemetry.derived_skipped_empty();
                empty_published += 1;
                frame_budget.finish(work_token);
                progressed = true;
                continue;
            }

            if started >= task_budget {
                world.materializations_mut().cancel_surface_build(key, revision);
                frame_budget.finish(work_token);
                continue;
            }

            //
            // Dense Scale-local geometry is diagnostic-colored by USF Scale
            // relative to the current physical interaction slice:
            // current=blue, +1=green, +2=yellow, +3=orange, +4=red,
            // +5=purple, +6=magenta, then repeat.
            //
            // This is deliberately a Scale diagnostic, not presentation LOD.
            let debug_color = debug_scale_band_color(
                layer.scale(),
                interaction.target_scale(),
            );
            let build = move || {
                let surface = mesh::extract_chunk_surface(&snapshot);
                VoxelDerivedOutput {
                    surface,
                    debug_color,
                }
            };
            let task = if critical {
                workers.try_submit_critical(VoxelWorkerLane::Derivation, build)
            } else {
                workers.try_submit(VoxelWorkerLane::Derivation, build)
            };
            let Some(task) = task else {
                world.materializations_mut().cancel_surface_build(key, revision);
                frame_budget.finish(work_token);
                continue;
            };

            telemetry.derived_started();
            commands.spawn((
                Name::new(if critical {
                    "Voxel Critical Surface Derivation"
                } else {
                    "Voxel Surface Derivation"
                }),
                VoxelWorkerTask,
                VoxelDerivedTask {
                    world: world_entity,
                    key,
                    revision,
                    task,
                },
            ));
            started += 1;
            frame_budget.finish(work_token);
            progressed = true;
        }

        if !progressed || exhausted.len() == world_entities.len() {
            break;
        }
    }

    *round_robin_cursor = (*round_robin_cursor).wrapping_add(1);
}


fn debug_scale_band_color(
    scale: SpatialScale,
    interaction_scale: SpatialScale,
) -> [f32; 4] {
    let relative = i16::from(scale.exponent())
        - i16::from(interaction_scale.exponent());
    rainbow_debug_color(relative)
}

//
// This is still a *USF Scale* diagnostic, never presentation LOD. It shares the
// slower sixteen-band visual language with binary LOD diagnostics so either
// hierarchy can be inspected without a harsh seven-color wrap.
const SCALE_DEBUG_HUES: [[f32; 3]; 16] = [
    [0.670, 0.369, 0.820],
    [0.820, 0.369, 0.801],
    [0.820, 0.369, 0.632],
    [0.820, 0.369, 0.463],
    [0.820, 0.444, 0.369],
    [0.820, 0.613, 0.369],
    [0.820, 0.782, 0.369],
    [0.688, 0.820, 0.369],
    [0.519, 0.820, 0.369],
    [0.369, 0.820, 0.388],
    [0.369, 0.820, 0.557],
    [0.369, 0.820, 0.726],
    [0.369, 0.745, 0.820],
    [0.369, 0.576, 0.820],
    [0.369, 0.407, 0.820],
    [0.501, 0.369, 0.820],
];

fn rainbow_debug_color(relative_band: i16) -> [f32; 4] {
    let [r, g, b] = SCALE_DEBUG_HUES[relative_band.rem_euclid(16) as usize];
    [r, g, b, 1.0]
}

#[cfg(test)]
mod scale_band_debug_tests {
    use super::*;

    #[test]
    fn scale_diagnostic_uses_slow_sixteen_band_hue_revolution() {
        let s0 = SpatialScale::ZERO;
        let s1 = SpatialScale::new(1).unwrap();
        let s2 = SpatialScale::new(2).unwrap();

        assert_eq!(debug_scale_band_color(s0, s0), [0.670, 0.369, 0.820, 1.0]);
        assert_eq!(debug_scale_band_color(s1, s0), [0.820, 0.369, 0.801, 1.0]);
        assert_eq!(debug_scale_band_color(s2, s0), [0.820, 0.369, 0.632, 1.0]);
    }
}
