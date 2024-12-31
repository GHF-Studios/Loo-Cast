use bevy::prelude::*;
use std::collections::VecDeque;
use std::collections::HashMap;

use super::enums::ChunkRetryAction;
use super::enums::ChunkState;

#[derive(Resource, Default, Debug)]
pub(in crate) struct ChunkRetryQueue {
    pub actions: VecDeque<ChunkRetryAction>,
}

#[derive(Resource, Default)]
pub(in crate) struct ChunkStateManager {
    states: HashMap<(i32, i32), ChunkState>,
}

impl ChunkStateManager {
    fn new() -> Self {
        ChunkStateManager {
            states: HashMap::new(),
        }
    }

    fn get_state(&self, chunk: (i32, i32)) -> Option<ChunkState> {
        self.states.get(&chunk).copied()
    }

    fn set_state(&mut self, chunk: (i32, i32), state: ChunkState) {
        self.states.insert(chunk, state);
    }

    fn remove_state(&mut self, chunk: (i32, i32)) {
        self.states.remove(&chunk);
    }
}