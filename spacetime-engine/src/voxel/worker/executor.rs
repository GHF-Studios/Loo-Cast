//! Fixed worker pool and typed work submission.

use super::{
    admission::{VoxelWorkAdmission, VoxelWorkComputeLease},
    queue::{VoxelWorkJob, VoxelWorkLane, VoxelWorkPriority, VoxelWorkQueue},
    ticket::{VoxelWorkFailure, VoxelWorkTicket},
};
use bevy::{
    prelude::*,
    tasks::{TaskPool, TaskPoolBuilder, available_parallelism},
};
use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Instant,
};

/// Dedicated bounded executor for reconstructible voxel work.
///
/// Semantic/canonical authority stays on the simulation side. Fixed workers
/// drain one durable queue; typed tickets preserve versioned publication and
/// cancellation at the existing ECS ownership boundaries.
#[derive(Resource)]
pub(in crate::voxel) struct VoxelWorkExecutor {
    queue: Arc<VoxelWorkQueue>,
    pub(super) admission: Arc<VoxelWorkAdmission>,
    capacity: usize,
    _pool: TaskPool,
}

impl Drop for VoxelWorkExecutor {
    fn drop(&mut self) {
        self.queue.close();
    }
}

impl Default for VoxelWorkExecutor {
    fn default() -> Self {
        let requested = recommended_worker_threads(available_parallelism());
        let pool = TaskPoolBuilder::new()
            .num_threads(requested)
            .thread_name("Voxel Work".to_string())
            .build();
        let capacity = pool.thread_num().max(1);

        let queue = Arc::new(VoxelWorkQueue::default());

        for _ in 0..capacity {
            let queue = Arc::clone(&queue);
            pool.spawn(async move {
                while let Some(job) = queue.pop() {
                    job();
                }
            })
            .detach();
        }

        Self {
            queue,
            admission: Arc::new(VoxelWorkAdmission::new(capacity)),
            capacity,
            _pool: pool,
        }
    }
}

impl VoxelWorkExecutor {
    pub(in crate::voxel) const fn capacity(&self) -> usize {
        self.capacity
    }

    pub(in crate::voxel) fn available_slots(&self, lane: VoxelWorkLane) -> usize {
        self.admission.available(lane)
    }

    pub(in crate::voxel) fn estimated_latency_seconds(&self, lane: VoxelWorkLane) -> f64 {
        //
        // "total jobs * this lane's average" badly underestimates latency when
        // expensive generation/resolution work shares the pool. Estimate queued
        // compute from each lane's own measured average, then divide by actual
        // parallel capacity and add one local service time.
        const FALLBACK_JOB_SECONDS: f64 = 0.008;

        let queued_compute_seconds = (0..VoxelWorkLane::COUNT)
            .map(|index| {
                let outstanding = self.admission.outstanding[index].load(Ordering::Acquire);
                let average_ns = self.admission.average_job_ns[index].load(Ordering::Relaxed);
                let average_seconds = if average_ns == 0 {
                    FALLBACK_JOB_SECONDS
                } else {
                    average_ns as f64 * 1.0e-9
                };
                outstanding as f64 * average_seconds
            })
            .sum::<f64>();

        queued_compute_seconds / self.capacity.max(1) as f64
            + self
                .admission
                .average_job_seconds(lane)
                .unwrap_or(FALLBACK_JOB_SECONDS)
    }

    pub(in crate::voxel) fn try_submit<T, F>(
        &self,
        lane: VoxelWorkLane,
        job: F,
    ) -> Option<VoxelWorkTicket<T>>
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        self.try_submit_with_priority(lane, VoxelWorkPriority::Normal, job)
    }

    pub(in crate::voxel) fn try_submit_critical<T, F>(
        &self,
        lane: VoxelWorkLane,
        job: F,
    ) -> Option<VoxelWorkTicket<T>>
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        self.try_submit_with_priority(lane, VoxelWorkPriority::Critical, job)
    }

    fn try_submit_with_priority<T, F>(
        &self,
        lane: VoxelWorkLane,
        priority: VoxelWorkPriority,
        job: F,
    ) -> Option<VoxelWorkTicket<T>>
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        if !self.admission.try_acquire(lane) {
            return None;
        }

        let (result_sender, result_receiver) = mpsc::sync_channel(1);
        let cancelled = Arc::new(AtomicBool::new(false));
        let worker_cancelled = Arc::clone(&cancelled);
        let admission_for_job = Arc::clone(&self.admission);

        let worker_job: VoxelWorkJob = Box::new(move || {
            // The compute lease is created inside the durable worker job so it
            // is released on every closure exit path, including cancellation
            // before execution and panic unwind.
            let _compute_admission = VoxelWorkComputeLease::new(admission_for_job.clone(), lane);

            if worker_cancelled.load(Ordering::Acquire) {
                return;
            }

            let started = Instant::now();
            let output = catch_unwind(AssertUnwindSafe(|| match lane {
                VoxelWorkLane::Generation => {
                    let _span = bevy::log::info_span!("voxel.worker.generation").entered();
                    job()
                }
                VoxelWorkLane::Derivation => {
                    let _span = bevy::log::info_span!("voxel.worker.derivation").entered();
                    job()
                }
                VoxelWorkLane::PresentationPlanning => {
                    let _span =
                        bevy::log::info_span!("voxel.worker.presentation_planning").entered();
                    job()
                }
            }))
            .map_err(|_| VoxelWorkFailure::Panicked);
            let elapsed_ns = started.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
            admission_for_job.record_job_duration(lane, elapsed_ns);
            if !worker_cancelled.load(Ordering::Acquire) {
                let _ = result_sender.send(output);
            }
        });

        if self.queue.push(lane, priority, worker_job).is_err() {
            self.admission.release(lane);
            return None;
        }

        Some(VoxelWorkTicket::new(result_receiver, cancelled))
    }
}

fn recommended_worker_threads(available: usize) -> usize {
    let available = available.max(1);
    if available <= 2 {
        1
    } else {
        available.saturating_mul(3).div_ceil(4).max(1)
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub(in crate::voxel) struct VoxelWorkTask;
