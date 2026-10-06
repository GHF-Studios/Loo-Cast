//! Bounded execution facility for reconstructible voxel work.

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

type VoxelWorkJob = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VoxelWorkPriority {
    Normal,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VoxelWorkLane {
    Generation,
    Derivation,
    PresentationPlanning,
}

impl VoxelWorkLane {
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

struct VoxelWorkQueueState {
    normal: [VecDeque<VoxelWorkJob>; VoxelWorkLane::COUNT],
    critical: [VecDeque<VoxelWorkJob>; VoxelWorkLane::COUNT],
    service_cursor: usize,
    critical_cursor: usize,
    critical_burst: usize,
    closed: bool,
}

impl Default for VoxelWorkQueueState {
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
struct VoxelWorkQueue {
    state: Mutex<VoxelWorkQueueState>,
    ready: Condvar,
}

impl VoxelWorkQueue {
    fn push(
        &self,
        lane: VoxelWorkLane,
        priority: VoxelWorkPriority,
        job: VoxelWorkJob,
    ) -> Result<(), VoxelWorkJob> {
        let Ok(mut state) = self.state.lock() else {
            return Err(job);
        };
        if state.closed {
            return Err(job);
        }
        match priority {
            VoxelWorkPriority::Normal => {
                state.normal[lane.index()].push_back(job);
            }
            VoxelWorkPriority::Critical => {
                state.critical[lane.index()].push_back(job);
            }
        }
        self.ready.notify_one();
        Ok(())
    }

    fn pop(&self) -> Option<VoxelWorkJob> {
        let mut state = self.state.lock().ok()?;
        loop {
            if state.closed {
                return None;
            }

            let normal_waiting = state.normal.iter().any(|lane| !lane.is_empty());
            let critical_waiting = state.critical.iter().any(|lane| !lane.is_empty());

            // Physical interaction work remains low-latency, but an endless
            // collision stream must not permanently starve contextual
            // realization/presentation. After a bounded critical burst, serve
            // one normal weighted-fair job if one exists.
            if critical_waiting
                && (state.critical_burst < MAX_CRITICAL_SERVICE_BURST || !normal_waiting)
            {
                for offset in 0..VoxelWorkLane::COUNT {
                    let lane_index = (state.critical_cursor + offset) % VoxelWorkLane::COUNT;
                    if let Some(job) = state.critical[lane_index].pop_front() {
                        state.critical_cursor = (lane_index + 1) % VoxelWorkLane::COUNT;
                        state.critical_burst = state.critical_burst.saturating_add(1);
                        return Some(job);
                    }
                }
            }

            for offset in 0..VoxelWorkLane::SERVICE_WHEEL.len() {
                let wheel_index =
                    (state.service_cursor + offset) % VoxelWorkLane::SERVICE_WHEEL.len();
                let lane = VoxelWorkLane::SERVICE_WHEEL[wheel_index];
                if let Some(job) = state.normal[lane.index()].pop_front() {
                    state.service_cursor = (wheel_index + 1) % VoxelWorkLane::SERVICE_WHEEL.len();
                    state.critical_burst = 0;
                    return Some(job);
                }
            }

            // If normal work disappeared between the availability check and
            // service scan, do not sleep while critical work is queued.
            if critical_waiting {
                for offset in 0..VoxelWorkLane::COUNT {
                    let lane_index = (state.critical_cursor + offset) % VoxelWorkLane::COUNT;
                    if let Some(job) = state.critical[lane_index].pop_front() {
                        state.critical_cursor = (lane_index + 1) % VoxelWorkLane::COUNT;
                        state.critical_burst = state.critical_burst.saturating_add(1);
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
struct VoxelWorkAdmission {
    outstanding: [AtomicUsize; VoxelWorkLane::COUNT],
    running: [AtomicUsize; VoxelWorkLane::COUNT],
    limits: [usize; VoxelWorkLane::COUNT],
    average_job_ns: [AtomicU64; VoxelWorkLane::COUNT],
    // Runtime pressure is scheduler state; Tracy only observes it.
}
impl VoxelWorkAdmission {
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

    fn try_acquire(&self, lane: VoxelWorkLane) -> bool {
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

    fn release(&self, lane: VoxelWorkLane) {
        let previous = self.outstanding[lane.index()].fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous > 0, "voxel worker admission underflow");
    }
    fn available(&self, lane: VoxelWorkLane) -> usize {
        self.limits[lane.index()]
            .saturating_sub(self.outstanding[lane.index()].load(Ordering::Acquire))
    }
    fn outstanding(&self, lane: VoxelWorkLane) -> usize {
        self.outstanding[lane.index()].load(Ordering::Acquire)
    }
    fn total_outstanding(&self) -> usize {
        self.outstanding
            .iter()
            .map(|counter| counter.load(Ordering::Acquire))
            .sum()
    }
    fn running(&self, lane: VoxelWorkLane) -> usize {
        self.running[lane.index()].load(Ordering::Acquire)
    }
    fn total_running(&self) -> usize {
        self.running
            .iter()
            .map(|counter| counter.load(Ordering::Acquire))
            .sum()
    }
    fn record_job_duration(&self, lane: VoxelWorkLane, elapsed_ns: u64) {
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
    fn average_job_seconds(&self, lane: VoxelWorkLane) -> Option<f64> {
        let value = self.average_job_ns[lane.index()].load(Ordering::Relaxed);
        (value != 0).then_some(value as f64 * 1.0e-9)
    }
}

/// RAII lease for one queued/running worker computation.
///
/// Admission is compute pressure only. It must end when worker computation
/// ends, not when its result is eventually published on the main thread.
struct VoxelWorkComputeLease {
    admission: Option<Arc<VoxelWorkAdmission>>,
    lane: VoxelWorkLane,
}

impl VoxelWorkComputeLease {
    fn new(admission: Arc<VoxelWorkAdmission>, lane: VoxelWorkLane) -> Self {
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

pub(super) struct VoxelWorkTicket<T: Send + 'static> {
    receiver: Mutex<Receiver<Result<T, VoxelWorkFailure>>>,
    cancelled: Arc<AtomicBool>,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum VoxelWorkFailure {
    Panicked,
    Disconnected,
}

impl<T: Send + 'static> VoxelWorkTicket<T> {
    /// Pending, ready and terminal failure are distinct outcomes. A lost
    /// worker must never leave an ECS task waiting forever.
    pub(super) fn try_take(&mut self) -> Result<Option<T>, VoxelWorkFailure> {
        let receiver = self
            .receiver
            .get_mut()
            .expect("voxel worker result mutex poisoned");
        match receiver.try_recv() {
            Ok(Ok(value)) => Ok(Some(value)),
            Ok(Err(failure)) => Err(failure),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(VoxelWorkFailure::Disconnected),
        }
    }
}

impl<T: Send + 'static> Drop for VoxelWorkTicket<T> {
    fn drop(&mut self) {
        // Result lifetime and compute-admission lifetime are independent.
        // Dropping the ticket prevents publication of stale output, while the
        // queued/running worker closure owns releasing compute admission.
        self.cancelled.store(true, Ordering::Release);
    }
}

/// Dedicated bounded executor for reconstructible voxel work.
///
/// Semantic/canonical authority stays on the simulation side. Fixed workers
/// drain one durable queue; typed tickets preserve versioned publication and
/// cancellation at the existing ECS ownership boundaries.
#[derive(Resource)]
pub(super) struct VoxelWorkExecutor {
    queue: Arc<VoxelWorkQueue>,
    admission: Arc<VoxelWorkAdmission>,
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
    pub(super) const fn capacity(&self) -> usize {
        self.capacity
    }

    pub(super) fn available_slots(&self, lane: VoxelWorkLane) -> usize {
        self.admission.available(lane)
    }

    pub(super) fn estimated_latency_seconds(&self, lane: VoxelWorkLane) -> f64 {
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

    pub(super) fn try_submit<T, F>(&self, lane: VoxelWorkLane, job: F) -> Option<VoxelWorkTicket<T>>
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        self.try_submit_with_priority(lane, VoxelWorkPriority::Normal, job)
    }

    pub(super) fn try_submit_critical<T, F>(
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

        Some(VoxelWorkTicket {
            receiver: Mutex::new(result_receiver),
            cancelled,
        })
    }
}

#[cfg(feature = "profiling-tracy")]
pub(super) fn emit_worker_pressure(workers: Res<VoxelWorkExecutor>) {
    let Some(client) = tracy_client::Client::running() else {
        return;
    };

    let generation_outstanding = workers.admission.outstanding(VoxelWorkLane::Generation);
    let generation_running = workers.admission.running(VoxelWorkLane::Generation);
    let derivation_outstanding = workers.admission.outstanding(VoxelWorkLane::Derivation);
    let derivation_running = workers.admission.running(VoxelWorkLane::Derivation);
    let planning_outstanding = workers
        .admission
        .outstanding(VoxelWorkLane::PresentationPlanning);
    let planning_running = workers
        .admission
        .running(VoxelWorkLane::PresentationPlanning);

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
            .average_job_seconds(VoxelWorkLane::Generation)
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
            .average_job_seconds(VoxelWorkLane::Derivation)
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
            .average_job_seconds(VoxelWorkLane::PresentationPlanning)
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
pub(super) struct VoxelWorkTask;
