//! Async dense-materialization generation lifecycle.

use std::collections::VecDeque;

use bevy::{
    prelude::*,
    tasks::{Task, futures::check_ready},
};

use crate::{
    config::EngineConfig,
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    spatial::{UsfPosition, UsfPrimaryInteractionSlice, UsfScaleLayer, UsfSemanticFrame},
};

use super::{VoxelStreaming, VoxelStreamingTelemetry};
use super::super::{
    VoxelAuthority, VoxelChunk, VoxelFrameSnapshot, VoxelMaterializationChunkAddress,
    VoxelMaterializationKey, VoxelScaleDomain, VoxelWorld,
    generation_scope::VoxelGenerationScopeExtent,
    worker::{VoxelWorkerPool, VoxelWorkerTask},
};

mod batching;

use batching::{VoxelGenerationJob, plan_generation_batches};

struct VoxelGeneratedChunk {
    key: VoxelMaterializationKey,
    token: u64,
    applied_edit_count: usize,
    chunk: VoxelChunk,
}

/// One asynchronous work item over several independently addressable atoms.
#[derive(Component)]
pub(in crate::voxel) struct VoxelGenerationTask {
    world: Entity,
    /// Unpublished addresses represented by this worker batch.
    keys: Vec<VoxelMaterializationKey>,
    task: Option<Task<Vec<VoxelGeneratedChunk>>>,
    ready: VecDeque<VoxelGeneratedChunk>,
}

impl VoxelGenerationTask {
    fn spawn(
        workers: &VoxelWorkerPool,
        world: Entity,
        jobs: Vec<VoxelGenerationJob>,
    ) -> Self {
        debug_assert!(!jobs.is_empty());
        let keys = jobs.iter().map(|job| job.key).collect();
        let task = workers.pool().spawn(async move {
            jobs.into_iter()
                .map(|job| {
                    let applied_edit_count = job.recipe.applied_edit_count();
                    let chunk = job.recipe.materialize();
                    VoxelGeneratedChunk {
                        key: job.key,
                        token: job.token,
                        applied_edit_count,
                        chunk,
                    }
                })
                .collect()
        });
        Self {
            world,
            keys,
            task: Some(task),
            ready: VecDeque::new(),
        }
    }
}

pub(in crate::voxel) fn finish_chunk_generation(
    config: Res<EngineConfig>,
    mut commands: Commands,
    mut worlds: Query<(&mut VoxelWorld, Option<&UsfLogicalRealizationOf>)>,
    authority_partitions: Query<&UsfAuthorityPartitionOf>,
    authorities: Query<(&UsfPosition, &UsfSemanticFrame, &VoxelAuthority, &VoxelScaleDomain)>,
    mut tasks: Query<(Entity, &mut VoxelGenerationTask)>,
    mut telemetry: ResMut<VoxelStreamingTelemetry>,
) {
    let publish_budget = config.voxel.streaming.generation_publish_budget_per_frame;
    let mut published = 0;

    for (task_entity, mut generation) in &mut tasks {
        if published >= publish_budget {
            break;
        }

        if generation.ready.is_empty() {
            let completed = generation.task.as_mut().and_then(|task| check_ready(task));
            let Some(completed) = completed else {
                continue;
            };
            generation.task = None;
            generation.ready = completed.into();
        }

        let Ok((mut world, logical_realization)) =
            worlds.get_mut(generation.world)
        else {
            generation.ready.clear();
            commands.entity(task_entity).despawn();
            continue;
        };
        let authority = logical_realization
            .and_then(|logical| authority_partitions.get(logical.0).ok())
            .and_then(|partition| authorities.get(partition.0).ok())
            .map(|(origin, frame, authority, domain)| {
                (
                    authority,
                    domain,
                    VoxelFrameSnapshot::new(*origin, *frame, world.origin().leaf_scale()),
                )
            });

        while published < publish_budget {
            let Some(mut output) = generation.ready.pop_front() else {
                break;
            };

            let Ok(address) = world.materialization_address(output.key) else {
                continue;
            };
            catch_up_generated_chunk_with_authority(
                &world,
                authority,
                address,
                output.applied_edit_count,
                &mut output.chunk,
            );

            generation.keys.retain(|key| *key != output.key);
            if world.materializations_mut().publish_generated(
                output.key,
                output.token,
                output.chunk,
            ) {
                published += 1;
            }
        }

        if generation.task.is_none() && generation.ready.is_empty() {
            commands.entity(task_entity).despawn();
            telemetry.generation_completed();
        }
    }
}

/// Cancels a batch when none of its unpublished addresses remain active after
/// the latest residency reconciliation. Dropping Bevy's Task handle cancels it.
pub(in crate::voxel) fn retire_stale_generation_tasks(
    mut commands: Commands,
    worlds: Query<&VoxelWorld>,
    generation_tasks: Query<(Entity, &VoxelGenerationTask)>,
    mut telemetry: ResMut<VoxelStreamingTelemetry>,
) {
    for (entity, task) in &generation_tasks {
        let useful = worlds.get(task.world).is_ok_and(|world| {
            task.keys
                .iter()
                .copied()
                .any(|key| world.materializations().is_active(key))
        });
        if !useful {
            telemetry.generation_cancelled(task.keys.len());
            commands.entity(entity).despawn();
        }
    }
}

/// Schedules asynchronous generation for demanded addresses that are not already
/// backed by warm resident data.
///
/// Global worker-slot accounting remains shared across voxel worlds so one world
/// cannot independently saturate the compute pool.
pub(in crate::voxel) fn schedule_voxel_generation(
    config: Res<EngineConfig>,
    workers: Res<VoxelWorkerPool>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    mut commands: Commands,
    mut worlds: Query<(
        Entity,
        &mut VoxelWorld,
        &mut VoxelStreaming,
        &UsfScaleLayer,
        Option<&UsfLogicalRealizationOf>,
    )>,
    authority_partitions: Query<&UsfAuthorityPartitionOf>,
    authorities: Query<(&UsfPosition, &UsfSemanticFrame, &VoxelAuthority, &VoxelScaleDomain)>,
    worker_tasks: Query<(), With<VoxelWorkerTask>>,
    mut round_robin_cursor: Local<usize>,
    mut telemetry: ResMut<VoxelStreamingTelemetry>,
) {
    let streaming_config = config.voxel.streaming;
    let generation_scope_extent = VoxelGenerationScopeExtent::from_base_chunks_per_axis(
        streaming_config.generation_group_base_chunks_per_axis,
    )
    .expect("validated engine config must produce a generation grouping extent");

    telemetry.worker_capacity(workers.capacity());
    let mut generation_slots = workers.available_slots(worker_tasks.iter().count());
    if generation_slots == 0 {
        return;
    }

    // Rotate equal-priority ties for fairness, then rank globally so current
    // physical and make-before-break replacement work beats arbitrary ECS order.
    let mut world_entities = worlds
        .iter_mut()
        .filter_map(|(entity, _, streaming, layer, _)| {
            let pending_priority = streaming.next_pending_priority()?;
            let scale_distance = (
                i16::from(layer.scale().exponent())
                    - i16::from(interaction.scale().exponent())
            )
            .unsigned_abs();
            Some((
                entity,
                layer.scale() == interaction.scale(),
                streaming.migration_active(),
                pending_priority,
                scale_distance,
            ))
        })
        .collect::<Vec<_>>();
    if world_entities.is_empty() {
        return;
    }

    let rotate = *round_robin_cursor % world_entities.len();
    world_entities.rotate_left(rotate);
    world_entities.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then_with(|| b.2.cmp(&a.2))
            .then_with(|| b.3.cmp(&a.3))
            .then_with(|| a.4.cmp(&b.4))
    });

    for (entity, _, _, _, _) in world_entities.iter().copied() {
        if generation_slots == 0 {
            break;
        }

        let Ok((world_entity, mut world, mut streaming, _layer, logical_realization)) =
            worlds.get_mut(entity)
        else {
            continue;
        };

        let authority = logical_realization
            .and_then(|logical| authority_partitions.get(logical.0).ok())
            .and_then(|partition| authorities.get(partition.0).ok())
            .map(|(origin, frame, authority, domain)| {
                (
                    authority,
                    domain,
                    VoxelFrameSnapshot::new(*origin, *frame, world.origin().leaf_scale()),
                )
            });
        let batches = plan_generation_batches(
            &mut world,
            &mut streaming,
            authority,
            generation_scope_extent,
            generation_slots,
            streaming_config.max_chunks_per_generation_task,
        );
        let scheduled = batches.len();

        for batch in batches {
            telemetry.generation_started();
            commands.spawn((
                Name::new("Voxel Generation Task"),
                VoxelWorkerTask,
                VoxelGenerationTask::spawn(&workers, world_entity, batch.jobs),
            ));
        }

        generation_slots = generation_slots.saturating_sub(scheduled);
    }

    *round_robin_cursor = (*round_robin_cursor).wrapping_add(1);
}

/// Applies edits appended after a generation task took its immutable snapshot.
///
/// This direct no-authority path exists only to regression-test edit replay.
#[cfg(test)]
pub(super) fn catch_up_generated_chunk(
    world: &VoxelWorld,
    address: VoxelMaterializationChunkAddress,
    applied_edit_count: usize,
    chunk: &mut VoxelChunk,
) {
    catch_up_generated_chunk_with_authority(
        world,
        None,
        address,
        applied_edit_count,
        chunk,
    );
}

fn catch_up_generated_chunk_with_authority(
    world: &VoxelWorld,
    authority: Option<(&VoxelAuthority, &VoxelScaleDomain, VoxelFrameSnapshot)>,
    address: VoxelMaterializationChunkAddress,
    applied_edit_count: usize,
    chunk: &mut VoxelChunk,
) {
    if let Some((authority, domain, frame_snapshot)) = authority {
        if domain.editable(world.origin().leaf_scale()) {
            for edit in authority.edits_since(applied_edit_count) {
                if let Ok(edit) = edit.projected_world(frame_snapshot) {
                    chunk.apply_edit(address, edit);
                }
            }
        }
        return;
    }

    for edit in world
        .modifications()
        .for_chunk_since(address, applied_edit_count)
    {
        chunk.apply_edit(address, edit);
    }
}
