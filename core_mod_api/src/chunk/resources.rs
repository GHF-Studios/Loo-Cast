use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

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
