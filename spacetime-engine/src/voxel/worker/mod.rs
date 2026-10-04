//! Shared background worker policy for voxel realization.

use std::{
    collections::VecDeque,
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

// critical-interaction-worker-priority-v1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VoxelWorkerPriority {
    Normal,
    Critical,
}

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

    // presentation-resolution-service-weight-v2
    //
    // Normal reconstructible presentation used to receive only 2/8 weighted
    // service slots. Critical interaction work still preempts this wheel, so
    // increasing presentation service improves visual latency without allowing
    // it to outrank collision-critical jobs.
    const SERVICE_WHEEL: [Self; 10] = [
        Self::Generation,
        Self::PresentationResolution,
        Self::Derivation,
        Self::PresentationResolution,
        Self::PresentationPlanning,
        Self::PresentationResolution,
        Self::Generation,
        Self::Derivation,
        Self::PresentationResolution,
        Self::PlanetarySurface,
    ];

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

// bounded-critical-worker-burst-v1
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

    fn depths(
        &self,
    ) -> (
        [usize; VoxelWorkerLane::COUNT],
        [usize; VoxelWorkerLane::COUNT],
    ) {
        let Ok(state) = self.state.lock() else {
            return ([0; VoxelWorkerLane::COUNT], [0; VoxelWorkerLane::COUNT]);
        };
        (
            std::array::from_fn(|index| state.normal[index].len()),
            std::array::from_fn(|index| state.critical[index].len()),
        )
    }
}

#[derive(Debug)]
struct VoxelWorkerAdmission {
    outstanding: [AtomicUsize; VoxelWorkerLane::COUNT],
    limits: [usize; VoxelWorkerLane::COUNT],
    average_job_ns: [AtomicU64; VoxelWorkerLane::COUNT],

    // worker-instrumentation-compact-noise-megapass-v1
    active: [AtomicUsize; VoxelWorkerLane::COUNT],
    average_queue_wait_ns: [AtomicU64; VoxelWorkerLane::COUNT],
}
impl VoxelWorkerAdmission {
    fn new(worker_capacity: usize) -> Self {
        let pipeline_depth = worker_capacity.saturating_mul(2).max(1);
        Self {
            outstanding: std::array::from_fn(|_| AtomicUsize::new(0)),
            // Dense generation/derivation and binary presentation resolution
            // are streaming pipelines. Keep enough queued/running work to feed
            // the shared pool; planning and legacy planetary surface remain
            // deliberately narrow.
            limits: [pipeline_depth, pipeline_depth, 1, 1, pipeline_depth],
            average_job_ns: std::array::from_fn(|_| AtomicU64::new(0)),
            active: std::array::from_fn(|_| AtomicUsize::new(0)),
            average_queue_wait_ns:
                std::array::from_fn(|_| AtomicU64::new(0)),
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

    fn record_queue_wait(&self, lane: VoxelWorkerLane, elapsed_ns: u64) {
        let average = &self.average_queue_wait_ns[lane.index()];
        let mut current = average.load(Ordering::Relaxed);
        loop {
            let next = if current == 0 {
                elapsed_ns.max(1)
            } else {
                current
                    .saturating_mul(7)
                    .saturating_add(elapsed_ns.max(1))
                    / 8
            };
            match average.compare_exchange_weak(
                current,
                next,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(observed) => current = observed,
            }
        }
    }

    fn average_queue_wait_seconds(
        &self,
        lane: VoxelWorkerLane,
    ) -> Option<f64> {
        let value =
            self.average_queue_wait_ns[lane.index()]
                .load(Ordering::Relaxed);
        (value != 0).then_some(value as f64 * 1.0e-9)
    }

    fn active(&self, lane: VoxelWorkerLane) -> usize {
        self.active[lane.index()].load(Ordering::Acquire)
    }

}

// worker-instrumentation-compact-noise-megapass-v1
struct VoxelWorkerActiveJob {
    admission: Arc<VoxelWorkerAdmission>,
    lane: VoxelWorkerLane,
}

impl VoxelWorkerActiveJob {
    fn new(
        admission: Arc<VoxelWorkerAdmission>,
        lane: VoxelWorkerLane,
    ) -> Self {
        admission.active[lane.index()].fetch_add(1, Ordering::AcqRel);
        Self { admission, lane }
    }
}

impl Drop for VoxelWorkerActiveJob {
    fn drop(&mut self) {
        let previous = self.admission.active[self.lane.index()]
            .fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous > 0, "voxel worker active-job underflow");
    }
}

// voxel-compute-admission-lifetime-v1
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
        Self {
            admission: Some(admission),
            lane,
        }
    }
}

impl Drop for VoxelWorkerComputeAdmission {
    fn drop(&mut self) {
        if let Some(admission) = self.admission.take() {
            admission.release(self.lane);
        }
    }
}

pub(super) struct VoxelWorkerTicket<T: Send + 'static> {
    receiver: Mutex<Receiver<T>>,
    cancelled: Arc<AtomicBool>,
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
        // weighted-worker-latency-estimate-v2
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

        let submitted_at = Instant::now();
        let worker_job: VoxelWorkerJob = Box::new(move || {
            let dequeued_at = Instant::now();
            admission_for_job.record_queue_wait(
                lane,
                dequeued_at
                    .duration_since(submitted_at)
                    .as_nanos()
                    .min(u128::from(u64::MAX)) as u64,
            );

            // The compute lease is created inside the durable worker job so it
            // is released on every closure exit path, including cancellation
            // before execution and panic unwind.
            let _compute_admission =
                VoxelWorkerComputeAdmission::new(admission_for_job.clone(), lane);

            if worker_cancelled.load(Ordering::Acquire) {
                return;
            }

            let _active_job =
                VoxelWorkerActiveJob::new(admission_for_job.clone(), lane);
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

// worker-profiler-signal-prune-v1
//
// Keep only signals that answer a distinct scheduling question:
// - active.total: are the durable workers actually saturated?
// - queued.*.total: is normal/critical work waiting for service?
// - per-lane queue_wait_seconds: which lane is suffering scheduler latency?
// - per-lane compute_seconds: which lane's jobs are intrinsically expensive?
//
// Per-lane outstanding/active/queue-depth mirrors and monotonically increasing
// completion counters were visual noise and duplicated the same pressure state.
#[cfg(feature = "profiling-tracy")]
pub(super) fn emit_worker_pressure(workers: Res<VoxelWorkerPool>) {
    if !tracy_client::Client::is_connected() {
        return;
    }
    let Some(client) = tracy_client::Client::running() else {
        return;
    };

    let lanes = [
        VoxelWorkerLane::Generation,
        VoxelWorkerLane::Derivation,
        VoxelWorkerLane::PlanetarySurface,
        VoxelWorkerLane::PresentationPlanning,
        VoxelWorkerLane::PresentationResolution,
    ];
    let (queued_normal, queued_critical) = workers.queue.depths();

    client.plot(
        tracy_client::plot_name!("voxel.worker.active.total"),
        lanes.iter()
            .map(|&lane| workers.admission.active(lane))
            .sum::<usize>() as f64,
    );
    client.plot(
        tracy_client::plot_name!("voxel.worker.queued.normal.total"),
        queued_normal.iter().sum::<usize>() as f64,
    );
    client.plot(
        tracy_client::plot_name!("voxel.worker.queued.critical.total"),
        queued_critical.iter().sum::<usize>() as f64,
    );

    client.plot(
        tracy_client::plot_name!("voxel.worker.queue_wait_seconds.generation"),
        workers.admission.average_queue_wait_seconds(lanes[0]).unwrap_or(0.0),
    );
    client.plot(
        tracy_client::plot_name!("voxel.worker.queue_wait_seconds.derivation"),
        workers.admission.average_queue_wait_seconds(lanes[1]).unwrap_or(0.0),
    );
    client.plot(
        tracy_client::plot_name!("voxel.worker.queue_wait_seconds.planetary"),
        workers.admission.average_queue_wait_seconds(lanes[2]).unwrap_or(0.0),
    );
    client.plot(
        tracy_client::plot_name!("voxel.worker.queue_wait_seconds.presentation_planning"),
        workers.admission.average_queue_wait_seconds(lanes[3]).unwrap_or(0.0),
    );
    client.plot(
        tracy_client::plot_name!("voxel.worker.queue_wait_seconds.presentation"),
        workers.admission.average_queue_wait_seconds(lanes[4]).unwrap_or(0.0),
    );

    client.plot(
        tracy_client::plot_name!("voxel.worker.compute_seconds.generation"),
        workers.admission.average_job_seconds(lanes[0]).unwrap_or(0.0),
    );
    client.plot(
        tracy_client::plot_name!("voxel.worker.compute_seconds.derivation"),
        workers.admission.average_job_seconds(lanes[1]).unwrap_or(0.0),
    );
    client.plot(
        tracy_client::plot_name!("voxel.worker.compute_seconds.planetary"),
        workers.admission.average_job_seconds(lanes[2]).unwrap_or(0.0),
    );
    client.plot(
        tracy_client::plot_name!("voxel.worker.compute_seconds.presentation_planning"),
        workers.admission.average_job_seconds(lanes[3]).unwrap_or(0.0),
    );
    client.plot(
        tracy_client::plot_name!("voxel.worker.compute_seconds.presentation"),
        workers.admission.average_job_seconds(lanes[4]).unwrap_or(0.0),
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
    use std::sync::{Arc, Mutex};

    use std::sync::Arc;

    use super::{
        VoxelWorkerAdmission, VoxelWorkerComputeAdmission, VoxelWorkerLane,
        recommended_worker_threads,
    };

    #[test]
    fn dedicated_worker_pool_uses_three_quarters_available_parallelism() {
        assert_eq!(recommended_worker_threads(1), 1);
        assert_eq!(recommended_worker_threads(2), 1);
        assert_eq!(recommended_worker_threads(4), 3);
        assert_eq!(recommended_worker_threads(8), 6);
    }

    #[test]
    fn critical_burst_eventually_services_normal_work() {
        let queue = VoxelWorkerQueue::default();
        let order = Arc::new(Mutex::new(Vec::<u8>::new()));

        for _ in 0..(MAX_CRITICAL_SERVICE_BURST + 2) {
            let order = Arc::clone(&order);
            queue
                .push(
                    VoxelWorkerLane::Generation,
                    VoxelWorkerPriority::Critical,
                    Box::new(move || order.lock().unwrap().push(1)),
                )
                .unwrap();
        }

        let normal_order = Arc::clone(&order);
        queue
            .push(
                VoxelWorkerLane::PresentationResolution,
                VoxelWorkerPriority::Normal,
                Box::new(move || normal_order.lock().unwrap().push(2)),
            )
            .unwrap();

        for _ in 0..=MAX_CRITICAL_SERVICE_BURST {
            queue.pop().unwrap()();
        }

        let observed = order.lock().unwrap();
        assert_eq!(
            observed[MAX_CRITICAL_SERVICE_BURST],
            2,
            "normal work must get service after the bounded critical burst",
        );
    }

    #[test]
    fn critical_worker_queue_preempts_normal_backlog() {
        let queue = VoxelWorkerQueue::default();
        let order = Arc::new(Mutex::new(Vec::<u8>::new()));

        let normal_order = Arc::clone(&order);
        queue
            .push(
                VoxelWorkerLane::Generation,
                VoxelWorkerPriority::Normal,
                Box::new(move || normal_order.lock().unwrap().push(1)),
            )
            .unwrap();

        let critical_order = Arc::clone(&order);
        queue
            .push(
                VoxelWorkerLane::Generation,
                VoxelWorkerPriority::Critical,
                Box::new(move || critical_order.lock().unwrap().push(2)),
            )
            .unwrap();

        queue.pop().unwrap()();
        queue.pop().unwrap()();

        assert_eq!(*order.lock().unwrap(), vec![2, 1]);
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

    #[test]
    fn compute_admission_ends_with_compute_not_result_ticket_lifetime() {
        let admission = Arc::new(VoxelWorkerAdmission::new(1));
        let lane = VoxelWorkerLane::Generation;
        let full_capacity = admission.available(lane);

        assert!(admission.try_acquire(lane));
        assert_eq!(admission.available(lane), full_capacity - 1);

        {
            let _compute =
                VoxelWorkerComputeAdmission::new(Arc::clone(&admission), lane);
            // The actual submission path acquires before constructing this
            // lease. Dropping the compute lease models the worker closure
            // finishing while a separate result ticket may remain alive.
        }

        assert_eq!(admission.available(lane), full_capacity);
    }

    #[test]
    fn cancelled_compute_still_releases_its_admission_lease() {
        let admission = Arc::new(VoxelWorkerAdmission::new(1));
        let lane = VoxelWorkerLane::Derivation;
        let full_capacity = admission.available(lane);

        assert!(admission.try_acquire(lane));
        let compute =
            VoxelWorkerComputeAdmission::new(Arc::clone(&admission), lane);
        drop(compute);

        assert_eq!(admission.available(lane), full_capacity);
    }
}
