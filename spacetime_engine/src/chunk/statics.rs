use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use bevy::prelude::*;

use lazy_static::lazy_static;

use super::enums::ChunkState;

lazy_static!{
    pub static ref REQUESTED_CHUNK_ADDITIONS: Arc<Mutex<HashSet<(i32, i32)>>> = Arc::new(Mutex::new(HashSet::new()));
    pub static ref REQUESTED_CHUNK_REMOVALS: Arc<Mutex<HashSet<(i32, i32)>>> = Arc::new(Mutex::new(HashSet::new()));
    pub static ref CHUNK_OWNERSHIP: Arc<Mutex<HashMap<(i32, i32), Entity>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref LOADED_CHUNKS: Arc<Mutex<HashSet<(i32, i32)>>> = Arc::new(Mutex::new(HashSet::new()));
    pub static ref CHUNK_STATES: Arc<Mutex<HashMap<(i32, i32), ChunkState>>> = Arc::new(Mutex::new(HashMap::new()));
}