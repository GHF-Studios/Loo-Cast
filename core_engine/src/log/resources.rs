use bevy::prelude::*;
use std::collections::BTreeSet;
use std::sync::Arc;

use crate::log::arena::{Arena, Level, NodeIdx, TreeKind};

#[repr(transparent)]
#[derive(Resource, Clone)]
pub struct LogTreeHandle(pub Arc<Arena>);

#[derive(Resource)]
pub struct LogViewerState {
    pub selected: BTreeSet<NodeIdx>,
    pub split_ratio: f32,
    pub threshold: Level,
    pub tree_mode: TreeKind,
}

impl Default for LogViewerState {
    fn default() -> Self {
        Self {
            selected: BTreeSet::new(),
            split_ratio: 0.35,
            threshold: Level::Warn,
            tree_mode: TreeKind::Span,
        }
    }
}

#[derive(Resource, Default)]
pub struct UiWindows {
    pub show_log_viewer: bool,
}
