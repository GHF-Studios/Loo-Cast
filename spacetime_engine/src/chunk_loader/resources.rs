use std::collections::{HashMap, HashSet};
use bevy::prelude::*;

#[derive(Resource, Default, Clone)]
pub struct ChunkOwnership {
    // Maps chunk coordinates to the loader entity that owns them
    pub ownership: HashMap<(i32, i32), Entity>,
    // Tracks all loaded chunk coordinates
    pub loaded_chunks: HashSet<(i32, i32)>,
}
