//! Shared background worker policy for voxel realization.

use std::{
    collections::VecDeque,
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        mpsc::{self, Receiver, TryRecvError},
    },
    time::Instant,
};

use bevy::{
    prelude::*,
    tasks::{TaskPool, TaskPoolBuilder, available_parallelism},
};

type VoxelWorkerJob = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VoxelWorkerPriority {
    Normal,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VoxelWorkerLane {
    Generation,
    Derivation,
    PresentationPlanning,
}

impl VoxelWorkerLane {
    const COUNT: usize = 3;

    const SERVICE_WHEEL: [Self; 5] = [
        Self::Generation,
        Self::Derivation,
        Self::PresentationPlanning,
        Self::Generation,
        Self::Derivation,
    ];

    const fn index(self) -> usize {
        match self {
            Self::Generation => 0,
            Self::Derivation => 1,
            Self::PresentationPlanning => 2,
        }
    }
}

const MAX_CRITICAL_SERVICE_BURST: usize = 4;

struct VoxelWorkerQueueState {
    normal: [VecDeque<VoxelWorkerJob>; VoxelWorkerLane::COUNT],
    critical: [VecDeque<VoxelWorkerJob>; VoxelWorkerLane::COUNT],
    service_cursor: usize,
    critical_cursor: usize,
    critical_burst: usize,
    closed: bool,
}

impl Default for VoxelWorkerQueueState {
    fn default() -> Self {
        Self {
            normal: std::array::from_fn(|_| VecDeque::new()),
            critical: std::array::from_fn(|_| VecDeque::new()),
            service_cursor: 0,
            critical_cursor: 0,
            critical_burst: 0,
            closed: false,
        }
    }
}

#[derive(Default)]
struct VoxelWorkerQueue {
    state: Mutex<VoxelWorkerQueueState>,
    ready: Condvar,
}

impl VoxelWorkerQueue {
    fn push(
        &self,
        lane: VoxelWorkerLane,
        priority: VoxelWorkerPriority,
        job: VoxelWorkerJob,
    ) -> Result<(), VoxelWorkerJob> {
        let Ok(mut state) = self.state.lock() else {
            return Err(job);
        };
        if state.closed {
            return Err(job);
        }
        match priority {
            VoxelWorkerPriority::Normal => {
                state.normal[lane.index()].push_back(job);
            }
            VoxelWorkerPriority::Critical => {
                state.critical[lane.index()].push_back(job);
            }
        }
        self.ready.notify_one();
        Ok(())
    }

    fn pop(&self) -> Option<VoxelWorkerJob> {
        let mut state = self.state.lock().ok()?;
        loop {
            if state.closed {
                return None;
            }

            let normal_waiting =
                state.normal.iter().any(|lane| !lane.is_empty());
            let critical_waiting =
                state.critical.iter().any(|lane| !lane.is_empty());

            // Physical interaction work remains low-latency, but an endless
            // collision stream must not permanently starve contextual
            // realization/presentation. After a bounded critical burst, serve
            // one normal weighted-fair job if one exists.
            if critical_waiting
                && (state.critical_burst < MAX_CRITICAL_SERVICE_BURST
                    || !normal_waiting)
            {
                for offset in 0..VoxelWorkerLane::COUNT {
                    let lane_index =
                        (state.critical_cursor + offset) % VoxelWorkerLane::COUNT;
                    if let Some(job) = state.critical[lane_index].pop_front() {
                        state.critical_cursor =
                            (lane_index + 1) % VoxelWorkerLane::COUNT;
                        state.critical_burst =
                            state.critical_burst.saturating_add(1);
                        return Some(job);
                    }
                }
            }

            for offset in 0..VoxelWorkerLane::SERVICE_WHEEL.len() {
                let wheel_index =
                    (state.service_cursor + offset) % VoxelWorkerLane::SERVICE_WHEEL.len();
                let lane = VoxelWorkerLane::SERVICE_WHEEL[wheel_index];
                if let Some(job) = state.normal[lane.index()].pop_front() {
                    state.service_cursor =
                        (wheel_index + 1) % VoxelWorkerLane::SERVICE_WHEEL.len();
                    state.critical_burst = 0;
                    return Some(job);
                }
            }

            // If normal work disappeared between the availability check and
            // service scan, do not sleep while critical work is queued.
            if critical_waiting {
                for offset in 0..VoxelWorkerLane::COUNT {
                    let lane_index =
                        (state.critical_cursor + offset) % VoxelWorkerLane::COUNT;
                    if let Some(job) = state.critical[lane_index].pop_front() {
                        state.critical_cursor =
                            (lane_index + 1) % VoxelWorkerLane::COUNT;
                        state.critical_burst =
                            state.critical_burst.saturating_add(1);
                        return Some(job);
                    }
                }
            }

            state = self.ready.wait(state).ok()?;
        }
    }

    fn close(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.closed = true;
            for lane in &mut state.normal {
                lane.clear();
            }
            for lane in &mut state.critical {
                lane.clear();
            }
        }
        self.ready.notify_all();
    }

}

#[derive(Debug)]
struct VoxelWorkerAdmission {
    outstanding: [AtomicUsize; VoxelWorkerLane::COUNT],
    running: [AtomicUsize; VoxelWorkerLane::COUNT],
    limits: [usize; VoxelWorkerLane::COUNT],
    average_job_ns: [AtomicU64; VoxelWorkerLane::COUNT],

    // Runtime pressure is scheduler state; Tracy only observes it.
}
impl VoxelWorkerAdmission {
    fn new(worker_capacity: usize) -> Self {
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
    fn running(&self, lane: VoxelWorkerLane) -> usize {
        self.running[lane.index()].load(Ordering::Acquire)
    }
    fn total_running(&self) -> usize {
        self.running
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

/// RAII lease for one queued/running worker computation.
///
/// Admission is compute pressure only. It must end when worker computation
/// ends, not when its result is eventually published on the main thread.
struct VoxelWorkerComputeAdmission {
    admission: Option<Arc<VoxelWorkerAdmission>>,
    lane: VoxelWorkerLane,
}

impl VoxelWorkerComputeAdmission {
    fn new(
        admission: Arc<VoxelWorkerAdmission>,
        lane: VoxelWorkerLane,
    ) -> Self {
        admission.running[lane.index()].fetch_add(1, Ordering::AcqRel);
        Self {
            admission: Some(admission),
            lane,
        }
    }
}

impl Drop for VoxelWorkerComputeAdmission {
    fn drop(&mut self) {
        if let Some(admission) = self.admission.take() {
            let previous = admission.running[self.lane.index()]
                .fetch_sub(1, Ordering::AcqRel);
            debug_assert!(
                previous > 0,
                "voxel worker running counter underflow"
            );
            admission.release(self.lane);
        }
    }
}

pub(super) struct VoxelWorkerTicket<T: Send + 'static> {
    receiver: Mutex<Receiver<Result<T, VoxelWorkerFailure>>>,
    cancelled: Arc<AtomicBool>,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum VoxelWorkerFailure {
    Panicked,
    Disconnected,
}

impl<T: Send + 'static> VoxelWorkerTicket<T> {
    /// Pending, ready and terminal failure are distinct outcomes. A lost
    /// worker must never leave an ECS task waiting forever.
    pub(super) fn try_take(&mut self) -> Result<Option<T>, VoxelWorkerFailure> {
        let receiver = self
            .receiver
            .get_mut()
            .expect("voxel worker result mutex poisoned");
        match receiver.try_recv() {
            Ok(Ok(value)) => Ok(Some(value)),
            Ok(Err(failure)) => Err(failure),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(VoxelWorkerFailure::Disconnected),
        }
    }
}

impl<T: Send + 'static> Drop for VoxelWorkerTicket<T> {
    fn drop(&mut self) {
        // Result lifetime and compute-admission lifetime are independent.
        // Dropping the ticket prevents publication of stale output, while the
        // queued/running worker closure owns releasing compute admission.
        self.cancelled.store(true, Ordering::Release);
    }
}

/// Dedicated bounded execution domain for reconstructible voxel work.
///
/// Semantic/canonical authority stays on the simulation side. Fixed workers
/// drain one durable queue; typed tickets preserve versioned publication and
/// cancellation at the existing ECS ownership boundaries.
#[derive(Resource)]
pub(super) struct VoxelWorkerPool {
    queue: Arc<VoxelWorkerQueue>,
    admission: Arc<VoxelWorkerAdmission>,
    capacity: usize,
    _pool: TaskPool,
}

impl Drop for VoxelWorkerPool {
    fn drop(&mut self) {
        self.queue.close();
    }
}

impl Default for VoxelWorkerPool {
    fn default() -> Self {
        let requested = recommended_worker_threads(available_parallelism());
        let pool = TaskPoolBuilder::new()
            .num_threads(requested)
            .thread_name("Voxel Realization Worker".to_string())
            .build();
        let capacity = pool.thread_num().max(1);

        let queue = Arc::new(VoxelWorkerQueue::default());

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
        //
        // "total jobs * this lane's average" badly underestimates latency when
        // expensive generation/resolution work shares the pool. Estimate queued
        // compute from each lane's own measured average, then divide by actual
        // parallel capacity and add one local service time.
        const FALLBACK_JOB_SECONDS: f64 = 0.008;

        let queued_compute_seconds = (0..VoxelWorkerLane::COUNT)
            .map(|index| {
                let outstanding =
                    self.admission.outstanding[index].load(Ordering::Acquire);
                let average_ns =
                    self.admission.average_job_ns[index].load(Ordering::Relaxed);
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

pub(super) fn try_submit<T, F>(
        &self,
        lane: VoxelWorkerLane,
        job: F,
    ) -> Option<VoxelWorkerTicket<T>>
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        self.try_submit_with_priority(lane, VoxelWorkerPriority::Normal, job)
    }

    pub(super) fn try_submit_critical<T, F>(
        &self,
        lane: VoxelWorkerLane,
        job: F,
    ) -> Option<VoxelWorkerTicket<T>>
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        self.try_submit_with_priority(lane, VoxelWorkerPriority::Critical, job)
    }

    fn try_submit_with_priority<T, F>(
        &self,
        lane: VoxelWorkerLane,
        priority: VoxelWorkerPriority,
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

        let worker_job: VoxelWorkerJob = Box::new(move || {
            // The compute lease is created inside the durable worker job so it
            // is released on every closure exit path, including cancellation
            // before execution and panic unwind.
            let _compute_admission =
                VoxelWorkerComputeAdmission::new(admission_for_job.clone(), lane);

            if worker_cancelled.load(Ordering::Acquire) {
                return;
            }

            let started = Instant::now();
            let output = catch_unwind(AssertUnwindSafe(|| match lane {
                VoxelWorkerLane::Generation => {
                    let _span = bevy::log::info_span!("voxel.worker.generation").entered();
                    job()
                }
                VoxelWorkerLane::Derivation => {
                    let _span = bevy::log::info_span!("voxel.worker.derivation").entered();
                    job()
                }
                VoxelWorkerLane::PresentationPlanning => {
                    let _span = bevy::log::info_span!("voxel.worker.presentation_planning").entered();
                    job()
                }
            }))
            .map_err(|_| VoxelWorkerFailure::Panicked);
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

        Some(VoxelWorkerTicket {
            receiver: Mutex::new(result_receiver),
            cancelled,
        })
    }

}

#[cfg(feature = "profiling-tracy")]
pub(super) fn emit_worker_pressure(workers: Res<VoxelWorkerPool>) {
    let Some(client) = tracy_client::Client::running() else {
        return;
    };

    let generation_outstanding =
        workers.admission.outstanding(VoxelWorkerLane::Generation);
    let generation_running =
        workers.admission.running(VoxelWorkerLane::Generation);
    let derivation_outstanding =
        workers.admission.outstanding(VoxelWorkerLane::Derivation);
    let derivation_running =
        workers.admission.running(VoxelWorkerLane::Derivation);
    let planning_outstanding =
        workers.admission.outstanding(VoxelWorkerLane::PresentationPlanning);
    let planning_running =
        workers.admission.running(VoxelWorkerLane::PresentationPlanning);

    client.plot(
        tracy_client::plot_name!("Voxel workers/capacity"),
        workers.capacity() as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/running"),
        workers.admission.total_running() as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/outstanding"),
        workers.admission.total_outstanding() as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/queued"),
        workers
            .admission
            .total_outstanding()
            .saturating_sub(workers.admission.total_running()) as f64,
    );

    client.plot(
        tracy_client::plot_name!("Voxel workers/Generation running"),
        generation_running as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/Generation queued"),
        generation_outstanding.saturating_sub(generation_running) as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/Generation avg ms"),
        workers
            .admission
            .average_job_seconds(VoxelWorkerLane::Generation)
            .unwrap_or(0.0)
            * 1_000.0,
    );

    client.plot(
        tracy_client::plot_name!("Voxel workers/Derivation running"),
        derivation_running as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/Derivation queued"),
        derivation_outstanding.saturating_sub(derivation_running) as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/Derivation avg ms"),
        workers
            .admission
            .average_job_seconds(VoxelWorkerLane::Derivation)
            .unwrap_or(0.0)
            * 1_000.0,
    );

    client.plot(
        tracy_client::plot_name!("Voxel workers/PresentationPlanning running"),
        planning_running as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/PresentationPlanning queued"),
        planning_outstanding.saturating_sub(planning_running) as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/PresentationPlanning avg ms"),
        workers
            .admission
            .average_job_seconds(VoxelWorkerLane::PresentationPlanning)
            .unwrap_or(0.0)
            * 1_000.0,
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
