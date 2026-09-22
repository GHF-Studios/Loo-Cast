//! Async dense-materialization generation lifecycle.

use std::collections::VecDeque;

use bevy::{
    ecs::lifecycle::RemovedComponents,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures::check_ready},
};

use crate::{
    ecs::UsfManifestationOf,config::EngineConfig};

use super::VoxelStreaming;
use super::super::{
    VoxelAuthority, VoxelChunk, VoxelMaterializationChunkAddress, VoxelScaleDomain, VoxelWorld,
    generation_scope::VoxelGenerationScopeExtent,
    worker::{VoxelWorkerTask, available_slots},
};

mod batching;

use batching::{VoxelGenerationJob, plan_generation_batches};

struct VoxelGeneratedChunk {
    address: VoxelMaterializationChunkAddress,
    token: u64,
    applied_edit_count: usize,
    chunk: VoxelChunk,
}

/// One asynchronous work item over several independently addressable atoms.
#[derive(Component)]
pub(in crate::voxel) struct VoxelGenerationTask {
    world: Entity,
    task: Option<Task<Vec<VoxelGeneratedChunk>>>,
    ready: VecDeque<VoxelGeneratedChunk>,
}

impl VoxelGenerationTask {
    fn spawn(world: Entity, jobs: Vec<VoxelGenerationJob>) -> Self {
        debug_assert!(!jobs.is_empty());
        let task = AsyncComputeTaskPool::get().spawn(async move {
            jobs.into_iter()
                .map(|job| {
                    let applied_edit_count = job.recipe.applied_edit_count();
                    let chunk = job.recipe.materialize();
                    VoxelGeneratedChunk {
                        address: job.address,
                        token: job.token,
                        applied_edit_count,
                        chunk,
                    }
                })
                .collect()
        });
        Self {
            world,
            task: Some(task),
            ready: VecDeque::new(),
        }
    }
}

pub(in crate::voxel) fn finish_chunk_generation(
    config: Res<EngineConfig>,
    mut commands: Commands,
    mut worlds: Query<(&mut VoxelWorld, Option<&UsfManifestationOf>)>,
    authorities: Query<(&VoxelAuthority, &VoxelScaleDomain)>,
    mut tasks: Query<(Entity, &mut VoxelGenerationTask)>,
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

        let Ok((mut world, realization)) = worlds.get_mut(generation.world) else {
            generation.ready.clear();
            commands.entity(task_entity).despawn();
            continue;
        };
        let authority = realization
            .and_then(|realization| authorities.get(realization.0).ok());

        while published < publish_budget {
            let Some(mut output) = generation.ready.pop_front() else {
                break;
            };

            catch_up_generated_chunk_with_authority(
                &world,
                authority,
                output.address,
                output.applied_edit_count,
                &mut output.chunk,
            );

            if world.materializations_mut().publish_generated(
                output.address,
                output.token,
                output.chunk,
            ) {
                published += 1;
            }
        }

        if generation.task.is_none() && generation.ready.is_empty() {
            commands.entity(task_entity).despawn();
        }
    }
}

/// Generation jobs are the only per-materialization ECS objects left in this
/// stage. If their semantic world disappears, retire them immediately.
pub(in crate::voxel) fn retire_orphaned_tasks(
    mut commands: Commands,
    mut removed_worlds: RemovedComponents<VoxelWorld>,
    generation_tasks: Query<(Entity, &VoxelGenerationTask)>,
    mut removed: Local<Vec<Entity>>,
) {
    removed.clear();
    removed.extend(removed_worlds.read());
    if removed.is_empty() {
        return;
    }

    for (entity, task) in &generation_tasks {
        if removed.contains(&task.world) {
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
    mut commands: Commands,
    mut worlds: Query<(
        Entity,
        &mut VoxelWorld,
        &mut VoxelStreaming,
        Option<&UsfManifestationOf>,
    )>,
    authorities: Query<(&VoxelAuthority, &VoxelScaleDomain)>,
    worker_tasks: Query<(), With<VoxelWorkerTask>>,
    mut round_robin_cursor: Local<usize>,
) {
    let streaming_config = config.voxel.streaming;
    let generation_scope_extent = VoxelGenerationScopeExtent::from_base_chunks_per_axis(
        streaming_config.generation_group_base_chunks_per_axis,
    )
    .expect("validated engine config must produce a generation grouping extent");

    let mut generation_slots = available_slots(worker_tasks.iter().count());
    if generation_slots == 0 {
        return;
    }

    // One busy world must never consume every global worker slot forever.
    // Rotate the first world every frame and admit at most one generation batch
    // per world per pass.
    let world_entities = worlds
        .iter_mut()
        .map(|(entity, _, _, _)| entity)
        .collect::<Vec<_>>();
    if world_entities.is_empty() {
        return;
    }

    let start = *round_robin_cursor % world_entities.len();
    for offset in 0..world_entities.len() {
        if generation_slots == 0 {
            break;
        }

        let entity = world_entities[(start + offset) % world_entities.len()];
        let Ok((world_entity, mut world, mut streaming, realization)) = worlds.get_mut(entity) else {
            continue;
        };

        let authority = realization
            .and_then(|realization| authorities.get(realization.0).ok());
        let batches = plan_generation_batches(
            &mut world,
            &mut streaming,
            authority,
            generation_scope_extent,
            1,
            streaming_config.max_chunks_per_generation_task,
        );
        let scheduled = batches.len();

        for batch in batches {
            commands.spawn((
                Name::new("Voxel Generation Task"),
                VoxelWorkerTask,
                VoxelGenerationTask::spawn(world_entity, batch.jobs),
            ));
        }

        generation_slots = generation_slots.saturating_sub(scheduled);
    }

    *round_robin_cursor = (start + 1) % world_entities.len();
}

/// Applies edits appended after a generation task took its immutable snapshot.
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
    authority: Option<(&VoxelAuthority, &VoxelScaleDomain)>,
    address: VoxelMaterializationChunkAddress,
    applied_edit_count: usize,
    chunk: &mut VoxelChunk,
) {
    if let Some((authority, domain)) = authority {
        if domain.editable(world.origin().leaf_scale()) {
            for edit in authority.edits_since(applied_edit_count) {
                chunk.apply_edit(address, edit);
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
