//! Shared background worker policy for voxel realization.

use bevy::{
    prelude::*,
    tasks::AsyncComputeTaskPool,
};

/// Marks any long-running voxel background job, regardless of pipeline stage.
#[derive(Component, Debug, Default, Clone, Copy)]
pub(super) struct VoxelWorkerTask;

/// Remaining voxel capacity in the task pool that actually executes voxel work.
///
/// Generation and surface derivation both use [`AsyncComputeTaskPool`]. Budgeting
/// them from machine-wide hardware parallelism can oversubscribe Bevy's much
/// smaller async pool and starve unrelated latency-sensitive async work.
///
/// Voxel realization may use at most half of the async pool concurrently. This
/// preserves the previous "half the available workers" intent, but applies it to
/// the scheduler domain we actually occupy.
pub(super) fn available_slots(in_flight: usize) -> usize {
    let threads = AsyncComputeTaskPool::get().thread_num().max(1);
    let limit = (threads / 2).max(1);
    limit.saturating_sub(in_flight)
}
