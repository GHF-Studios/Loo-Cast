use crate::bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::chunk::types::ChunkActionWorkflowHandles;
use crate::config::statics::CONFIG;
use crate::gpu::workflows::gpu::generate_chunk_textures::user_items::ChunkRenderExecutor;
use crate::usf::pos::grid::types::GridVec;

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct ChunkManager {
    pub chunks: HashSet<GridVec>,
    pub load_radius: u32,
}
impl Default for ChunkManager {
    fn default() -> Self {
        ChunkManager {
            chunks: HashSet::new(),
            load_radius: CONFIG().get::<u32>("chunk_loader/load_radius"),
        }
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ChunkRenderHandles {
    pub quad: Handle<Mesh>,
    pub light_material: Handle<ColorMaterial>,
    pub dark_material: Handle<ColorMaterial>,
}

#[derive(Default, Resource)]
pub struct ChunkRenderExecutorRegistry {
    pub executors: HashMap<GridVec, ChunkRenderExecutor>,
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
}
impl Default for ChunkLoadGate {
    fn default() -> Self {
        Self {
            state: ChunkLoadGateState::Open,
            lock_info: None,
            timeout_lock_count: 0,
            boundary_overlap_lock_count: 0,
            unlock_count: 0,
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
        changed
    }

    pub fn lock_by_in_flight_boundary(&mut self) -> bool {
        let changed = self.state != ChunkLoadGateState::LockedByInFlightBoundary || self.lock_info.is_some();
        self.state = ChunkLoadGateState::LockedByInFlightBoundary;
        self.lock_info = None;
        if changed {
            self.boundary_overlap_lock_count += 1;
        }
        changed
    }

    pub fn unlock(&mut self) -> bool {
        let changed = self.is_locked() || self.lock_info.is_some();
        self.state = ChunkLoadGateState::Open;
        self.lock_info = None;
        if changed {
            self.unlock_count += 1;
        }
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
