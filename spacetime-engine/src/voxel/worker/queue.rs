//! Fair, priority-aware service of durable worker closures.

use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

pub(super) type VoxelWorkJob = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VoxelWorkPriority {
    Normal,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::voxel) enum VoxelWorkLane {
    Generation,
    Derivation,
    PresentationPlanning,
}

impl VoxelWorkLane {
    pub(super) const COUNT: usize = 3;

    const SERVICE_WHEEL: [Self; 5] = [
        Self::Generation,
        Self::Derivation,
        Self::PresentationPlanning,
        Self::Generation,
        Self::Derivation,
    ];

    pub(super) const fn index(self) -> usize {
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
pub(super) struct VoxelWorkQueue {
    state: Mutex<VoxelWorkQueueState>,
    ready: Condvar,
}

impl VoxelWorkQueue {
    pub(super) fn push(
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

    pub(super) fn pop(&self) -> Option<VoxelWorkJob> {
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

    pub(super) fn close(&self) {
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
