//! Lane admission, compute leases, and measured compute pressure.

use super::queue::VoxelWorkLane;
use std::sync::{
    Arc,
    atomic::{AtomicU64, AtomicUsize, Ordering},
};

#[derive(Debug)]
pub(super) struct VoxelWorkAdmission {
    pub(super) outstanding: [AtomicUsize; VoxelWorkLane::COUNT],
    pub(super) running: [AtomicUsize; VoxelWorkLane::COUNT],
    limits: [usize; VoxelWorkLane::COUNT],
    pub(super) average_job_ns: [AtomicU64; VoxelWorkLane::COUNT],
    // Runtime pressure is scheduler state; Tracy only observes it.
}
impl VoxelWorkAdmission {
    pub(super) fn new(worker_capacity: usize) -> Self {
        let pipeline_depth = worker_capacity.saturating_mul(2).max(1);
        Self {
            outstanding: std::array::from_fn(|_| AtomicUsize::new(0)),
            running: std::array::from_fn(|_| AtomicUsize::new(0)),
            // Dense generation/derivation are streaming pipelines.
            // Planning remains deliberately narrow.
            limits: [pipeline_depth, pipeline_depth, 1],
            average_job_ns: std::array::from_fn(|_| AtomicU64::new(0)),
        }
    }

    pub(super) fn try_acquire(&self, lane: VoxelWorkLane) -> bool {
        let index = lane.index();
        let limit = self.limits[index];
        let counter = &self.outstanding[index];
        let mut current = counter.load(Ordering::Acquire);
        loop {
            if current >= limit {
                return false;
            }
            match counter.compare_exchange_weak(
                current,
                current + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(observed) => current = observed,
            }
        }
    }

    pub(super) fn release(&self, lane: VoxelWorkLane) {
        let previous = self.outstanding[lane.index()].fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous > 0, "voxel worker admission underflow");
    }
    pub(super) fn available(&self, lane: VoxelWorkLane) -> usize {
        self.limits[lane.index()]
            .saturating_sub(self.outstanding[lane.index()].load(Ordering::Acquire))
    }
    #[cfg(feature = "profiling-tracy")]
    pub(super) fn outstanding(&self, lane: VoxelWorkLane) -> usize {
        self.outstanding[lane.index()].load(Ordering::Acquire)
    }
    #[cfg(feature = "profiling-tracy")]
    pub(super) fn total_outstanding(&self) -> usize {
        self.outstanding
            .iter()
            .map(|counter| counter.load(Ordering::Acquire))
            .sum()
    }
    #[cfg(feature = "profiling-tracy")]
    pub(super) fn running(&self, lane: VoxelWorkLane) -> usize {
        self.running[lane.index()].load(Ordering::Acquire)
    }
    #[cfg(feature = "profiling-tracy")]
    pub(super) fn total_running(&self) -> usize {
        self.running
            .iter()
            .map(|counter| counter.load(Ordering::Acquire))
            .sum()
    }
    pub(super) fn record_job_duration(&self, lane: VoxelWorkLane, elapsed_ns: u64) {
        let average = &self.average_job_ns[lane.index()];
        let mut current = average.load(Ordering::Relaxed);
        loop {
            let next = if current == 0 {
                elapsed_ns.max(1)
            } else {
                current.saturating_mul(7).saturating_add(elapsed_ns.max(1)) / 8
            };
            match average.compare_exchange_weak(current, next, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(observed) => current = observed,
            }
        }
    }
    pub(super) fn average_job_seconds(&self, lane: VoxelWorkLane) -> Option<f64> {
        let value = self.average_job_ns[lane.index()].load(Ordering::Relaxed);
        (value != 0).then_some(value as f64 * 1.0e-9)
    }
}

/// RAII lease for one queued/running worker computation.
///
/// Admission is compute pressure only. It must end when worker computation
/// ends, not when its result is eventually published on the main thread.
pub(super) struct VoxelWorkComputeLease {
    admission: Option<Arc<VoxelWorkAdmission>>,
    lane: VoxelWorkLane,
}

impl VoxelWorkComputeLease {
    pub(super) fn new(admission: Arc<VoxelWorkAdmission>, lane: VoxelWorkLane) -> Self {
        admission.running[lane.index()].fetch_add(1, Ordering::AcqRel);
        Self {
            admission: Some(admission),
            lane,
        }
    }
}

impl Drop for VoxelWorkComputeLease {
    fn drop(&mut self) {
        if let Some(admission) = self.admission.take() {
            let previous = admission.running[self.lane.index()].fetch_sub(1, Ordering::AcqRel);
            debug_assert!(previous > 0, "voxel worker running counter underflow");
            admission.release(self.lane);
        }
    }
}
