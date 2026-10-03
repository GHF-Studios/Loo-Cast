//! Batching and reservation policy for dense voxel generation jobs.

use bevy::prelude::*;

use super::super::VoxelStreaming;
use super::super::super::{
    VoxelAuthority, VoxelFrameSnapshot, VoxelMaterializationKey, VoxelScaleDomain, VoxelWorld,
    generation_scope::{VoxelGenerationScopeExtent, VoxelGenerationScope},
    world::VoxelChunkRecipe,
};

pub(super) struct VoxelGenerationJob {
    pub(super) key: VoxelMaterializationKey,
    pub(super) token: u64,
    pub(super) recipe: VoxelChunkRecipe,
}

// critical-generation-batches-v1
pub(super) struct PendingGenerationBatch {
    pub(super) scope: VoxelGenerationScope,
    pub(super) critical: bool,
    // latency-sensitive-generation-atoms-v1
    // Prevent PRESENTATION/physical atoms from being merged into a background
    // throughput batch whose result becomes visible only after every atom runs.
    latency_sensitive: bool,
    pub(super) jobs: Vec<VoxelGenerationJob>,
}

pub(super) fn plan_generation_batches(
    world: &mut VoxelWorld,
    streaming: &mut VoxelStreaming,
    authority: Option<(&VoxelAuthority, &VoxelScaleDomain, VoxelFrameSnapshot)>,
    generation_extent: VoxelGenerationScopeExtent,
    max_batches: usize,
    max_chunks_per_batch: usize,
) -> Vec<PendingGenerationBatch> {
    let load_budget = streaming.load_budget_per_frame;
    let mut requested = 0;
    let mut batches = Vec::<PendingGenerationBatch>::new();

    while requested < load_budget && max_batches > 0 {
        let Some(demanded) = streaming.pending_desired.pop_front() else {
            break;
        };
        let key = demanded.key;

        // Incremental residency deliberately leaves stale queue nodes in place:
        // removing an outgoing slab must be O(slab), not O(entire queue).
        // Lazy rejection makes queue cleanup proportional to work actually
        // revisited by the scheduler.
        if !streaming.is_effectively_desired(key)
            || world.materializations().is_active(key)
        {
            continue;
        }

        let Ok(scope) =
            VoxelGenerationScope::containing(key, generation_extent)
        else {
            error!(
                ?key,
                "voxel generation scope could not be derived canonically"
            );
            continue;
        };

        let demanded_critical =
            demanded.roles.contains(crate::spatial::UsfScaleRoleMask::COLLISION)
                || demanded.roles.contains(crate::spatial::UsfScaleRoleMask::EDITING);
        let latency_sensitive =
            demanded_critical
                || demanded
                    .roles
                    .contains(crate::spatial::UsfScaleRoleMask::PRESENTATION);
        let batch_cap = if latency_sensitive {
            1
        } else {
            max_chunks_per_batch
        };
        let can_join_existing = batches.iter().any(|batch| {
            batch.scope == scope
                && batch.critical == demanded_critical
                && batch.latency_sensitive == latency_sensitive
                && batch.jobs.len() < batch_cap
        });
        if !can_join_existing && batches.len() >= max_batches {
            streaming.pending_desired.push_front(demanded);
            break;
        }

        let Ok(address) = world.materialization_address(key) else {
            error!(?key, "voxel materialization key could not be converted canonically");
            continue;
        };
        let Some(token) = world.materializations_mut().reserve_generation(key) else {
            continue;
        };
        let recipe = authority.map_or_else(
            || world.chunk_recipe(address),
            |(authority, domain, frame_snapshot)| {
                world.chunk_recipe_from_authority(address, authority, domain, frame_snapshot)
            },
        );
        let critical = demanded_critical;
        push_generation_job(
            &mut batches,
            scope,
            critical,
            latency_sensitive,
            batch_cap,
            VoxelGenerationJob { key, token, recipe },
        );
        requested += 1;
    }

    batches
}

fn push_generation_job(
    batches: &mut Vec<PendingGenerationBatch>,
    scope: VoxelGenerationScope,
    critical: bool,
    latency_sensitive: bool,
    max_chunks_per_batch: usize,
    job: VoxelGenerationJob,
) {
    if let Some(batch) = batches
        .iter_mut()
        .find(|batch| {
            batch.scope == scope
                && batch.critical == critical
                && batch.latency_sensitive == latency_sensitive
                && batch.jobs.len() < max_chunks_per_batch
        })
    {
        batch.jobs.push(job);
        return;
    }

    batches.push(PendingGenerationBatch {
        scope,
        critical,
        latency_sensitive,
        jobs: vec![job],
    });
}
