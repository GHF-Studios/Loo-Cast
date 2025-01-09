use std::collections::HashMap;

use bevy::prelude::*;

use crate::chunk::enums::ChunkAction;

#[derive(Resource)]
pub(in super) struct ChunkLoaderActionBuffer(pub HashMap<(i32, i32), ChunkAction>);
impl Default for ChunkLoaderActionBuffer {
    fn default() -> Self {
        ChunkLoaderActionBuffer(HashMap::new())
    }
}