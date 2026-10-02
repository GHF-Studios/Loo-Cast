//! Shared background worker policy for voxel realization.

use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    mpsc::{self, Receiver, Sender, TryRecvError},
};

use std::time::Instant;

use bevy::{
    prelude::*,
    tasks::{TaskPool, TaskPoolBuilder, available_parallelism},
};

type VoxelWorkerJob = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VoxelWorkerLane {
    Generation,
    Derivation,
    PlanetarySurface,
    PresentationPlanning,
    PresentationResolution,
}
impl VoxelWorkerLane {
    const COUNT: usize = 5;
    const fn index(self) -> usize {
        match self {
            Self::Generation => 0,
            Self::Derivation => 1,
            Self::PlanetarySurface => 2,
            Self::PresentationPlanning => 3,
            Self::PresentationResolution => 4,
        }
    }
}

#[derive(Debug)]
struct VoxelWorkerAdmission {
    outstanding: [AtomicUsize; VoxelWorkerLane::COUNT],
    limits: [usize; VoxelWorkerLane::COUNT],
    average_job_ns: [AtomicU64; VoxelWorkerLane::COUNT],
}
impl VoxelWorkerAdmission {
    fn new(worker_capacity: usize) -> Self {
        let pipeline_depth = worker_capacity.saturating_mul(2).max(1);
        Self {
            outstanding: std::array::from_fn(|_| AtomicUsize::new(0)),
            limits: [pipeline_depth, pipeline_depth, 2, 1, 2],
            average_job_ns: std::array::from_fn(|_| AtomicU64::new(0)),
        }
    }

    fn try_acquire(&self, lane: VoxelWorkerLane) -> bool {
        let index = lane.index();
        let limit = self.limits[index];
        let counter = &self.outstanding[index];
        let mut current = counter.load(Ordering::Acquire);
        loop {
            if current >= limit {
                return false;
            }
            match counter.compare_exchange_weak(
                current, current + 1, Ordering::AcqRel, Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(observed) => current = observed,
            }
        }
    }

    fn release(&self, lane: VoxelWorkerLane) {
        let previous = self.outstanding[lane.index()].fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous > 0, "voxel worker admission underflow");
    }
    fn available(&self, lane: VoxelWorkerLane) -> usize {
        self.limits[lane.index()].saturating_sub(
            self.outstanding[lane.index()].load(Ordering::Acquire),
        )
    }
    fn outstanding(&self, lane: VoxelWorkerLane) -> usize {
        self.outstanding[lane.index()].load(Ordering::Acquire)
    }
    fn total_outstanding(&self) -> usize {
        self.outstanding
            .iter()
            .map(|counter| counter.load(Ordering::Acquire))
            .sum()
    }
    fn record_job_duration(&self, lane: VoxelWorkerLane, elapsed_ns: u64) {
        let average = &self.average_job_ns[lane.index()];
        let mut current = average.load(Ordering::Relaxed);
        loop {
            let next = if current == 0 {
                elapsed_ns.max(1)
            } else {
                current.saturating_mul(7).saturating_add(elapsed_ns.max(1)) / 8
            };
            match average.compare_exchange_weak(
                current, next, Ordering::Relaxed, Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(observed) => current = observed,
            }
        }
    }
    fn average_job_seconds(&self, lane: VoxelWorkerLane) -> Option<f64> {
        let value = self.average_job_ns[lane.index()].load(Ordering::Relaxed);
        (value != 0).then_some(value as f64 * 1.0e-9)
    }
}

pub(super) struct VoxelWorkerTicket<T: Send + 'static> {
    receiver: Mutex<Receiver<T>>,
    cancelled: Arc<AtomicBool>,
    admission: Arc<VoxelWorkerAdmission>,
    lane: VoxelWorkerLane,
}

impl<T: Send + 'static> VoxelWorkerTicket<T> {
    pub(super) fn try_take(&mut self) -> Option<T> {
        let receiver = self
            .receiver
            .get_mut()
            .expect("voxel worker result mutex poisoned");
        match receiver.try_recv() {
            Ok(value) => Some(value),
            Err(TryRecvError::Empty | TryRecvError::Disconnected) => None,
        }
    }
}

impl<T: Send + 'static> Drop for VoxelWorkerTicket<T> {
    fn drop(&mut self) {
        self.cancelled.store(true, Ordering::Release);
        self.admission.release(self.lane);
    }
}

/// Dedicated bounded execution domain for reconstructible voxel work.
///
/// Semantic/canonical authority stays on the simulation side. Fixed workers
/// drain one durable queue; typed tickets preserve versioned publication and
/// cancellation at the existing ECS ownership boundaries.
#[derive(Resource)]
pub(super) struct VoxelWorkerPool {
    // Sender drops before the TaskPool so blocked workers observe queue closure
    // before the executor joins its threads.
    sender: Sender<VoxelWorkerJob>,
    admission: Arc<VoxelWorkerAdmission>,
    capacity: usize,
    _pool: TaskPool,
}

impl Default for VoxelWorkerPool {
    fn default() -> Self {
        let requested = recommended_worker_threads(available_parallelism());
        let pool = TaskPoolBuilder::new()
            .num_threads(requested)
            .thread_name("Voxel Realization Worker".to_string())
            .build();
        let capacity = pool.thread_num().max(1);

        let (sender, receiver) = mpsc::channel::<VoxelWorkerJob>();
        let receiver = Arc::new(Mutex::new(receiver));

        for _ in 0..capacity {
            let receiver = Arc::clone(&receiver);
            pool.spawn(async move {
                loop {
                    let job = {
                        let Ok(receiver) = receiver.lock() else {
                            return;
                        };
                        receiver.recv()
                    };
                    let Ok(job) = job else {
                        return;
                    };
                    job();
                }
            })
            .detach();
        }

        Self {
            sender,
            admission: Arc::new(VoxelWorkerAdmission::new(capacity)),
            capacity,
            _pool: pool,
        }
    }
}

impl VoxelWorkerPool {
    pub(super) const fn capacity(&self) -> usize {
        self.capacity
    }

    pub(super) fn available_slots(&self, lane: VoxelWorkerLane) -> usize {
        self.admission.available(lane)
    }

    pub(super) fn estimated_latency_seconds(&self, lane: VoxelWorkerLane) -> f64 {
        let cost = self.admission.average_job_seconds(lane).unwrap_or(0.008);
        let waves = self.admission.total_outstanding()
            .div_ceil(self.capacity.max(1))
            .saturating_add(1);
        cost * waves as f64
    }

pub(super) fn try_submit<T, F>(
        &self,
        lane: VoxelWorkerLane,
        job: F,
    ) -> Option<VoxelWorkerTicket<T>>
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

        if self.sender.send(Box::new(move || {
            if worker_cancelled.load(Ordering::Acquire) {
                return;
            }

            let started = Instant::now();
            let output = match lane {
                VoxelWorkerLane::Generation => {
                    let _span = bevy::log::info_span!("voxel.worker.generation").entered();
                    job()
                }
                VoxelWorkerLane::Derivation => {
                    let _span = bevy::log::info_span!("voxel.worker.derivation").entered();
                    job()
                }
                VoxelWorkerLane::PlanetarySurface => {
                    let _span = bevy::log::info_span!("voxel.worker.planetary_surface").entered();
                    job()
                }
                VoxelWorkerLane::PresentationPlanning => {
                    let _span = bevy::log::info_span!("voxel.worker.presentation_planning").entered();
                    job()
                }
                VoxelWorkerLane::PresentationResolution => {
                    let _span = bevy::log::info_span!("voxel.worker.presentation_resolution").entered();
                    job()
                }
            };
            let elapsed_ns = started.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
            admission_for_job.record_job_duration(lane, elapsed_ns);
            if !worker_cancelled.load(Ordering::Acquire) {
                let _ = result_sender.send(output);
            }
        })).is_err() {
            self.admission.release(lane);
            return None;
        }

        Some(VoxelWorkerTicket {
            receiver: Mutex::new(result_receiver),
            cancelled,
            admission: Arc::clone(&self.admission),
            lane,
        })
    }

}

#[cfg(feature = "profiling-tracy")]
pub(super) fn emit_worker_pressure(workers: Res<VoxelWorkerPool>) {
    if !tracy_client::Client::is_connected() {
        return;
    }
    let Some(client) = tracy_client::Client::running() else {
        return;
    };

    let generation = workers.admission.outstanding(VoxelWorkerLane::Generation);
    let derivation = workers.admission.outstanding(VoxelWorkerLane::Derivation);
    let planetary = workers.admission.outstanding(VoxelWorkerLane::PlanetarySurface);
    let planning = workers.admission.outstanding(VoxelWorkerLane::PresentationPlanning);
    let presentation = workers.admission.outstanding(VoxelWorkerLane::PresentationResolution);

    client.plot(tracy_client::plot_name!("voxel.worker.capacity"), workers.capacity as f64);
    client.plot(tracy_client::plot_name!("voxel.worker.outstanding.generation"), generation as f64);
    client.plot(tracy_client::plot_name!("voxel.worker.outstanding.derivation"), derivation as f64);
    client.plot(tracy_client::plot_name!("voxel.worker.outstanding.planetary"), planetary as f64);
    client.plot(tracy_client::plot_name!("voxel.worker.outstanding.presentation_planning"), planning as f64);
    client.plot(tracy_client::plot_name!("voxel.worker.outstanding.presentation"), presentation as f64);
    client.plot(
        tracy_client::plot_name!("voxel.worker.outstanding.total"),
        (generation + derivation + planetary + planning + presentation) as f64,
    );
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
pub(super) struct VoxelWorkerTask;

#[cfg(test)]
mod tests {
    use super::{VoxelWorkerAdmission, VoxelWorkerLane, recommended_worker_threads};

    #[test]
    fn dedicated_worker_pool_uses_three_quarters_available_parallelism() {
        assert_eq!(recommended_worker_threads(1), 1);
        assert_eq!(recommended_worker_threads(2), 1);
        assert_eq!(recommended_worker_threads(4), 3);
        assert_eq!(recommended_worker_threads(8), 6);
    }

    #[test]
    fn worker_lanes_have_independent_bounded_admission() {
        let admission = VoxelWorkerAdmission::new(2);

        for _ in 0..4 {
            assert!(admission.try_acquire(VoxelWorkerLane::Generation));
        }
        assert!(!admission.try_acquire(VoxelWorkerLane::Generation));

        assert!(admission.try_acquire(VoxelWorkerLane::Derivation));
        assert!(admission.try_acquire(VoxelWorkerLane::PlanetarySurface));

        admission.release(VoxelWorkerLane::Generation);
        assert!(admission.try_acquire(VoxelWorkerLane::Generation));
    }
}
