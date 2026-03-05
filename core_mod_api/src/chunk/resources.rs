use crate::bevy::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

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
}
impl Default for ChunkLoadGate {
    fn default() -> Self {
        Self {
            state: ChunkLoadGateState::Open,
            lock_info: None,
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
        changed
    }

    pub fn unlock(&mut self) -> bool {
        let changed = self.is_locked() || self.lock_info.is_some();
        self.state = ChunkLoadGateState::Open;
        self.lock_info = None;
        changed
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ChunkLoadTimeoutSignal {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub timeout_count: usize,
}

static CHUNK_LOAD_TIMEOUT_SIGNAL_SENDER: OnceLock<Sender<ChunkLoadTimeoutSignal>> = OnceLock::new();

pub fn initialize_chunk_load_timeout_signal_channel() -> Receiver<ChunkLoadTimeoutSignal> {
    let (sender, receiver) = unbounded();
    if CHUNK_LOAD_TIMEOUT_SIGNAL_SENDER.set(sender).is_err() {
        unreachable!("Chunk load timeout signal sender already initialized");
    }
    receiver
}

pub fn emit_chunk_load_timeout_signal(signal: ChunkLoadTimeoutSignal) {
    let Some(sender) = CHUNK_LOAD_TIMEOUT_SIGNAL_SENDER.get() else {
        return;
    };
    let _ = sender.send(signal);
}

#[derive(Resource)]
pub struct ChunkLoadTimeoutSignalReceiver(pub Receiver<ChunkLoadTimeoutSignal>);

#[derive(Default, Resource)]
pub struct ChunkActionWorkflowState {
    pub handles: Option<ChunkActionWorkflowHandles>,
}
impl ChunkActionWorkflowState {
    pub fn is_idle(&self) -> bool {
        self.handles.is_none()
    }
}
