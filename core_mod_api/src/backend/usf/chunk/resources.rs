use crate::bevy::prelude::*;
use std::collections::HashSet;

use crate::config::statics::CONFIG;
use crate::usf::chunk::types::ChunkActionWorkflowHandles;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::Scale;

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct ChunkManager {
    pub chunks: HashSet<GridVec>,
    pub load_radius: u32,
    pub active_scale: Scale,
    pub loader_origin_grid: GridVec,
    pub loader_origin_unit: UnitVec,
}
impl Default for ChunkManager {
    fn default() -> Self {
        ChunkManager {
            chunks: HashSet::new(),
            load_radius: CONFIG().get::<u32>("chunk_loader/load_radius"),
            active_scale: Scale::MAX,
            loader_origin_grid: GridVec::default(),
            loader_origin_unit: UnitVec::default(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChunkBatchTargets {
    pub spawn: HashSet<GridVec>,
    pub despawn: HashSet<GridVec>,
}
impl ChunkBatchTargets {
    pub fn from_refs(spawn: &HashSet<GridVec>, despawn: &HashSet<GridVec>) -> Self {
        Self {
            spawn: spawn.clone(),
            despawn: despawn.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.spawn.is_empty() && self.despawn.is_empty()
    }

    pub fn contains_chunk(&self, grid_coord: &GridVec) -> bool {
        self.spawn.contains(grid_coord) || self.despawn.contains(grid_coord)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChunkBatchSnapshot {
    pub id: u64,
    pub targets: ChunkBatchTargets,
}
impl ChunkBatchSnapshot {
    pub fn spawn_count(&self) -> usize {
        self.targets.spawn.len()
    }

    pub fn despawn_count(&self) -> usize {
        self.targets.despawn.len()
    }

    pub fn contains_chunk(&self, grid_coord: &GridVec) -> bool {
        self.targets.contains_chunk(grid_coord)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkBatchCancellationReason {
    NoBoundaryRequest,
    Replanned,
}
impl ChunkBatchCancellationReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChunkBatchCancellationReason::NoBoundaryRequest => "NoBoundaryRequest",
            ChunkBatchCancellationReason::Replanned => "Replanned",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChunkBatchPlanResult {
    Unchanged,
    Planned(ChunkBatchSnapshot),
    Replanned {
        previous: ChunkBatchSnapshot,
        planned: ChunkBatchSnapshot,
    },
    Cleared {
        previous: ChunkBatchSnapshot,
        reason: ChunkBatchCancellationReason,
    },
}

#[derive(Resource, Debug)]
pub struct ChunkBatchTracker {
    next_batch_id: u64,
    planned: Option<ChunkBatchSnapshot>,
    running: Option<ChunkBatchSnapshot>,
}
impl Default for ChunkBatchTracker {
    fn default() -> Self {
        Self {
            next_batch_id: 1,
            planned: None,
            running: None,
        }
    }
}
impl ChunkBatchTracker {
    pub fn is_batch_planned(&self) -> bool {
        self.planned.is_some()
    }

    pub fn is_batch_running(&self) -> bool {
        self.running.is_some()
    }

    pub fn planned_batch(&self) -> Option<&ChunkBatchSnapshot> {
        self.planned.as_ref()
    }

    pub fn running_batch(&self) -> Option<&ChunkBatchSnapshot> {
        self.running.as_ref()
    }

    pub fn is_chunk_in_planned_batch(&self, grid_coord: &GridVec) -> bool {
        self.planned.as_ref().is_some_and(|batch| batch.contains_chunk(grid_coord))
    }

    pub fn is_chunk_in_running_batch(&self, grid_coord: &GridVec) -> bool {
        self.running.as_ref().is_some_and(|batch| batch.contains_chunk(grid_coord))
    }

    pub fn sync_plan(&mut self, spawn_targets: &HashSet<GridVec>, despawn_targets: &HashSet<GridVec>) -> ChunkBatchPlanResult {
        let targets = ChunkBatchTargets::from_refs(spawn_targets, despawn_targets);
        if targets.is_empty() {
            if let Some(previous) = self.planned.take() {
                return ChunkBatchPlanResult::Cleared {
                    previous,
                    reason: ChunkBatchCancellationReason::NoBoundaryRequest,
                };
            }
            return ChunkBatchPlanResult::Unchanged;
        }

        if self.running.as_ref().is_some_and(|running| running.targets == targets) {
            return ChunkBatchPlanResult::Unchanged;
        }

        if self.planned.as_ref().is_some_and(|planned| planned.targets == targets) {
            return ChunkBatchPlanResult::Unchanged;
        }

        let planned = self.new_snapshot(targets);
        if let Some(previous) = self.planned.replace(planned.clone()) {
            return ChunkBatchPlanResult::Replanned { previous, planned };
        }

        ChunkBatchPlanResult::Planned(planned)
    }

    pub fn start_planned_or_direct(&mut self, spawn_targets: &HashSet<GridVec>, despawn_targets: &HashSet<GridVec>) -> Option<ChunkBatchSnapshot> {
        let targets = ChunkBatchTargets::from_refs(spawn_targets, despawn_targets);
        if targets.is_empty() {
            return None;
        }

        if self.running.as_ref().is_some_and(|running| running.targets == targets) {
            return None;
        }

        let batch = if self.planned.as_ref().is_some_and(|planned| planned.targets == targets) {
            self.planned.take().expect("ChunkBatchTracker invariant: planned batch vanished before start")
        } else {
            self.new_snapshot(targets)
        };

        self.running = Some(batch.clone());
        Some(batch)
    }

    pub fn finish_running(&mut self) -> Option<ChunkBatchSnapshot> {
        self.running.take()
    }

    fn new_snapshot(&mut self, targets: ChunkBatchTargets) -> ChunkBatchSnapshot {
        let id = self.next_batch_id;
        self.next_batch_id += 1;
        ChunkBatchSnapshot { id, targets }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum ChunkLoadGateState {
    Open,
    LockedByTimeout,
    LockedByInFlightBoundary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub struct ChunkLoadGateLockInfo {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub timeout_count: usize,
}

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct ChunkLoadGate {
    pub state: ChunkLoadGateState,
    pub lock_info: Option<ChunkLoadGateLockInfo>,
    pub timeout_lock_count: u64,
    pub boundary_overlap_lock_count: u64,
    pub unlock_count: u64,
    /// Consecutive idle/no-boundary-request frames while locked by in-flight overlap.
    /// Used to avoid one-frame lock/unlock chatter in overload border visuals.
    pub boundary_quiet_frames: u8,
}
impl Default for ChunkLoadGate {
    fn default() -> Self {
        Self {
            state: ChunkLoadGateState::Open,
            lock_info: None,
            timeout_lock_count: 0,
            boundary_overlap_lock_count: 0,
            unlock_count: 0,
            boundary_quiet_frames: 0,
        }
    }
}
impl ChunkLoadGate {
    pub fn is_locked(&self) -> bool {
        self.state != ChunkLoadGateState::Open
    }

    pub fn lock_by_timeout(&mut self, module_name: &'static str, workflow_name: &'static str, timeout_count: usize) -> bool {
        self.state = ChunkLoadGateState::LockedByTimeout;
        let next_info = Some(ChunkLoadGateLockInfo {
            module_name,
            workflow_name,
            timeout_count,
        });
        let changed = self.lock_info != next_info;
        self.lock_info = next_info;
        if changed {
            self.timeout_lock_count += 1;
        }
        self.boundary_quiet_frames = 0;
        changed
    }

    pub fn lock_by_in_flight_boundary(&mut self) -> bool {
        let changed = self.state != ChunkLoadGateState::LockedByInFlightBoundary || self.lock_info.is_some();
        self.state = ChunkLoadGateState::LockedByInFlightBoundary;
        self.lock_info = None;
        if changed {
            self.boundary_overlap_lock_count += 1;
        }
        self.boundary_quiet_frames = 0;
        changed
    }

    pub fn unlock(&mut self) -> bool {
        let changed = self.is_locked() || self.lock_info.is_some();
        self.state = ChunkLoadGateState::Open;
        self.lock_info = None;
        if changed {
            self.unlock_count += 1;
        }
        self.boundary_quiet_frames = 0;
        changed
    }
}

#[derive(Default, Resource)]
pub struct ChunkActionWorkflowState {
    pub handles: Option<ChunkActionWorkflowHandles>,
    pub in_flight_spawn_targets: HashSet<GridVec>,
    pub in_flight_despawn_targets: HashSet<GridVec>,
}
impl ChunkActionWorkflowState {
    pub fn is_idle(&self) -> bool {
        self.handles.is_none()
    }

    pub fn set_in_flight_targets(&mut self, spawn_targets: HashSet<GridVec>, despawn_targets: HashSet<GridVec>) {
        self.in_flight_spawn_targets = spawn_targets;
        self.in_flight_despawn_targets = despawn_targets;
    }

    pub fn clear_in_flight_targets(&mut self) {
        self.in_flight_spawn_targets.clear();
        self.in_flight_despawn_targets.clear();
    }

    pub fn has_new_boundary_request(&self, spawn_targets: &HashSet<GridVec>, despawn_targets: &HashSet<GridVec>) -> bool {
        !spawn_targets.is_subset(&self.in_flight_spawn_targets) || !despawn_targets.is_subset(&self.in_flight_despawn_targets)
    }
}

#[cfg(test)]
mod tests {
    use super::{ChunkBatchPlanResult, ChunkBatchTracker};
    use crate::usf::pos::grid::types::GridVec;
    use crate::usf::pos::types::GridXyz;
    use std::collections::HashSet;

    fn set(coords: &[GridVec]) -> HashSet<GridVec> {
        coords.iter().cloned().collect()
    }

    #[test]
    fn chunk_batch_tracker_plan_start_finish_and_query_flow() {
        let mut tracker = ChunkBatchTracker::default();
        let coord_a = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let coord_b = GridVec::new_root(GridXyz::new_local(1, 0, 0));

        let planned = match tracker.sync_plan(&set(&[coord_a.clone()]), &set(&[coord_b.clone()])) {
            ChunkBatchPlanResult::Planned(batch) => batch,
            other => panic!("expected Planned, got {other:?}"),
        };
        assert!(tracker.is_batch_planned());
        assert!(tracker.is_chunk_in_planned_batch(&coord_a));
        assert!(tracker.is_chunk_in_planned_batch(&coord_b));

        let started = tracker
            .start_planned_or_direct(&set(&[coord_a.clone()]), &set(&[coord_b.clone()]))
            .expect("planned batch should start");
        assert_eq!(started.id, planned.id);
        assert!(tracker.is_batch_running());
        assert!(tracker.is_chunk_in_running_batch(&coord_a));
        assert!(tracker.is_chunk_in_running_batch(&coord_b));

        let finished = tracker.finish_running().expect("running batch should finish");
        assert_eq!(finished.id, started.id);
        assert!(!tracker.is_batch_running());
    }

    #[test]
    fn chunk_batch_tracker_replans_and_clears_when_targets_change() {
        let mut tracker = ChunkBatchTracker::default();
        let coord_a = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let coord_b = GridVec::new_root(GridXyz::new_local(1, 0, 0));

        let first_plan = match tracker.sync_plan(&set(&[coord_a.clone()]), &HashSet::new()) {
            ChunkBatchPlanResult::Planned(batch) => batch,
            other => panic!("expected first Planned, got {other:?}"),
        };

        let second_plan = match tracker.sync_plan(&set(&[coord_b.clone()]), &HashSet::new()) {
            ChunkBatchPlanResult::Replanned { previous, planned } => {
                assert_eq!(previous.id, first_plan.id);
                planned
            }
            other => panic!("expected Replanned, got {other:?}"),
        };
        assert_ne!(first_plan.id, second_plan.id);

        match tracker.sync_plan(&HashSet::new(), &HashSet::new()) {
            ChunkBatchPlanResult::Cleared { previous, .. } => {
                assert_eq!(previous.id, second_plan.id);
            }
            other => panic!("expected Cleared, got {other:?}"),
        }
        assert!(!tracker.is_batch_planned());
    }

    #[test]
    fn chunk_batch_tracker_does_not_plan_duplicate_running_targets() {
        let mut tracker = ChunkBatchTracker::default();
        let coord = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let spawn = set(&[coord.clone()]);
        let despawn = HashSet::new();

        let started = tracker.start_planned_or_direct(&spawn, &despawn).expect("batch should start directly");
        assert!(tracker.is_batch_running());

        let result = tracker.sync_plan(&spawn, &despawn);
        assert!(matches!(result, ChunkBatchPlanResult::Unchanged));
        assert!(!tracker.is_batch_planned());

        let finished = tracker.finish_running().expect("running batch should finish");
        assert_eq!(finished.id, started.id);
    }
}
