//! Shared background worker policy for voxel realization.

use bevy::{
    prelude::*,
    tasks::{TaskPool, TaskPoolBuilder, available_parallelism},
};

/// Dedicated execution domain for long-running reconstructible voxel work.
///
/// Voxel generation and surface derivation used to compete with unrelated
/// latency-sensitive work on Bevy's global `AsyncComputeTaskPool`. On a
/// four-thread machine that pool normally has one worker, which made the voxel
/// pipeline artificially serial regardless of how much independent work was
/// ready.
///
/// This pool is intentionally capability-owned. Semantic/canonical authority
/// remains on the main simulation side; workers only process immutable,
/// reconstructible snapshots whose publication is still version checked.
#[derive(Resource, Debug)]
pub(super) struct VoxelWorkerPool {
    pool: TaskPool,
    capacity: usize,
}

impl Default for VoxelWorkerPool {
    fn default() -> Self {
        let requested = recommended_worker_threads(available_parallelism());
        let pool = TaskPoolBuilder::new()
            .num_threads(requested)
            .thread_name("Voxel Realization Worker".to_string())
            .build();
        let capacity = pool.thread_num().max(1);

        Self { pool, capacity }
    }
}

impl VoxelWorkerPool {
    pub(super) const fn capacity(&self) -> usize {
        self.capacity
    }

    pub(super) const fn available_slots(&self, in_flight: usize) -> usize {
        self.capacity.saturating_sub(in_flight)
    }

    pub(super) const fn pool(&self) -> &TaskPool {
        &self.pool
    }
}

/// Leave approximately half of process parallelism available to latency-critical
/// simulation/render work while giving reconstructible voxel realization a
/// genuinely parallel execution domain of its own.
fn recommended_worker_threads(available: usize) -> usize {
    available.max(1).saturating_add(1) / 2
}

/// Marks any long-running voxel background job, regardless of pipeline stage.
#[derive(Component, Debug, Default, Clone, Copy)]
pub(super) struct VoxelWorkerTask;

#[cfg(test)]
mod tests {
    use super::recommended_worker_threads;

    #[test]
    fn dedicated_worker_pool_uses_about_half_available_parallelism() {
        assert_eq!(recommended_worker_threads(1), 1);
        assert_eq!(recommended_worker_threads(2), 1);
        assert_eq!(recommended_worker_threads(4), 2);
        assert_eq!(recommended_worker_threads(8), 4);
    }
}
