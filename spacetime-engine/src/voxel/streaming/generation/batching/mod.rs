//! Grouping and reservation policy for dense voxel generation jobs.

use bevy::prelude::*;

use super::super::VoxelStreaming;
use super::super::super::{
    VoxelMaterializationChunkAddress, VoxelWorld,
    aggregate::{VoxelMaterializationAggregateExtent, VoxelMaterializationAggregateScope},
    world::VoxelChunkRecipe,
};

pub(super) struct VoxelGenerationJob {
    pub(super) address: VoxelMaterializationChunkAddress,
    pub(super) token: u64,
    pub(super) recipe: VoxelChunkRecipe,
}

pub(super) struct PendingAggregateGeneration {
    pub(super) scope: VoxelMaterializationAggregateScope,
    pub(super) jobs: Vec<VoxelGenerationJob>,
}

pub(super) fn plan_generation_batches(
    world: &mut VoxelWorld,
    streaming: &mut VoxelStreaming,
    generation_extent: VoxelMaterializationAggregateExtent,
    max_batches: usize,
    max_chunks_per_batch: usize,
) -> Vec<PendingAggregateGeneration> {
    let load_budget = streaming.load_budget_per_frame;
    let mut requested = 0;
    let mut batches = Vec::<PendingAggregateGeneration>::new();

    while requested < load_budget && max_batches > 0 {
        let Some(demanded) = streaming.pending_desired.pop_front() else {
            break;
        };
        let address = demanded.address;

        if world.materializations().is_active(address) {
            continue;
        }

        let Ok(scope) =
            VoxelMaterializationAggregateScope::containing(world, address, generation_extent)
        else {
            error!(
                ?address,
                "voxel aggregate generation scope could not be derived canonically"
            );
            continue;
        };

        if !generation_batch_can_accept(
            &batches,
            scope,
            max_batches,
            max_chunks_per_batch,
        ) {
            streaming.pending_desired.push_front(demanded);
            break;
        }

        let Some(token) = world.materializations_mut().reserve_generation(address) else {
            continue;
        };
        let recipe = world.chunk_recipe(address);
        push_generation_job(
            &mut batches,
            scope,
            max_chunks_per_batch,
            VoxelGenerationJob {
                address,
                token,
                recipe,
            },
        );
        requested += 1;
    }

    batches
}

fn generation_batch_can_accept(
    batches: &[PendingAggregateGeneration],
    scope: VoxelMaterializationAggregateScope,
    max_batches: usize,
    max_chunks_per_batch: usize,
) -> bool {
    batches
        .iter()
        .any(|batch| batch.scope == scope && batch.jobs.len() < max_chunks_per_batch)
        || batches.len() < max_batches
}

fn push_generation_job(
    batches: &mut Vec<PendingAggregateGeneration>,
    scope: VoxelMaterializationAggregateScope,
    max_chunks_per_batch: usize,
    job: VoxelGenerationJob,
) {
    if let Some(batch) = batches
        .iter_mut()
        .find(|batch| batch.scope == scope && batch.jobs.len() < max_chunks_per_batch)
    {
        batch.jobs.push(job);
        return;
    }

    batches.push(PendingAggregateGeneration {
        scope,
        jobs: vec![job],
    });
}
