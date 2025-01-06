use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use bevy::prelude::*;

use lazy_static::lazy_static;

use super::enums::{ChunkRequest, ChunkState};

lazy_static!{
    static ref REQUESTED_CHUNK_ADDITIONS: Arc<Mutex<HashSet<(i32, i32)>>> = Arc::new(Mutex::new(HashSet::new()));
    static ref REQUESTED_CHUNK_REMOVALS: Arc<Mutex<HashSet<(i32, i32)>>> = Arc::new(Mutex::new(HashSet::new()));
    static ref CHUNK_OWNERSHIP: Arc<Mutex<HashMap<(i32, i32), Entity>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref LOADED_CHUNKS: Arc<Mutex<HashSet<(i32, i32)>>> = Arc::new(Mutex::new(HashSet::new()));
    static ref CHUNK_STATES: Arc<Mutex<HashMap<(i32, i32), ChunkState>>> = Arc::new(Mutex::new(HashMap::new()));
}

lazy_static!{
    pub static ref CHUNK_REQUESTS_BUFFER: Arc<Mutex<HashMap<(i32, i32), ChunkRequest>>> = Arc::new(Mutex::new(HashMap::new()));
}