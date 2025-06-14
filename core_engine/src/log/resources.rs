use bevy::prelude::*;
use std::collections::BTreeSet;
use std::sync::Arc;

use crate::log::arena::{Arena, NodeIdx};

#[repr(transparent)]
#[derive(Resource, Clone)]
pub struct LogTreeHandle(pub Arc<Arena>);

#[derive(Resource)]
pub struct LogViewerState {
    pub selected:    BTreeSet<NodeIdx>,
}

impl Default for LogViewerState {
    fn default() -> Self {
        Self {
            selected: BTreeSet::new(),
        }
    }
}

#[derive(Resource, Default)]
pub struct UiWindows {
    pub show_log_viewer: bool,
}