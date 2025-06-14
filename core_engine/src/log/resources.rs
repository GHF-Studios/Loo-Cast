use bevy::prelude::*;
use std::collections::BTreeSet;
use std::sync::Arc;

use crate::log::arena::{Arena, NodeIdx};

#[repr(transparent)]
#[derive(Resource, Clone)]
pub struct LogTreeHandle(pub Arc<Arena>);

#[derive(Resource, Default)]
pub struct LogViewerState {
    pub selected: BTreeSet<NodeIdx>,
    pub autoscroll: bool,
}