use bevy::prelude::*;
use std::collections::VecDeque;

use super::enums::ChunkRetryAction;

#[derive(Resource, Default, Debug)]
pub(in crate) struct ChunkRetryQueue {
    pub actions: VecDeque<ChunkRetryAction>,
}