use bevy::prelude::*;
use std::collections::{BTreeSet, HashSet};
use std::sync::Arc;

use crate::log::arena::FilterTreeMode;
use crate::log::types::{LocationPathSegment, LogPath};
use crate::log::{arena::{Level, NodeIdx, TreeKind}, location_tree::LocationTree, span_tree::SpanTree, storage::LogStorage};

#[repr(transparent)]
#[derive(Resource, Clone)]
pub struct LogStorageHandle(pub Arc<LogStorage>);

#[repr(transparent)]
#[derive(Resource, Clone)]
pub struct SpanTreeHandle(pub Arc<SpanTree>);

#[repr(transparent)]
#[derive(Resource, Clone)]
pub struct LocationTreeHandle(pub Arc<LocationTree>);

#[derive(Default)]
pub struct SpanTreeSelection {
    pub selected: HashSet<Vec<String>>,
}

impl SpanTreeSelection {
    pub fn select(&mut self, path: &Vec<String>) {
        self.selected.insert(path.clone());
    }

    pub fn deselect(&mut self, path: &Vec<String>) {
        self.selected.remove(path);
    }

    pub fn toggle(&mut self, path: &Vec<String>) {
        if !self.selected.remove(path) {
            self.selected.insert(path.clone());
        }
    }

    pub fn is_selected(&self, path: &Vec<String>) -> bool {
        self.selected.contains(path)
    }

    pub fn clear(&mut self) {
        self.selected.clear();
    }

    pub fn all(&self) -> impl Iterator<Item = &Vec<String>> {
        self.selected.iter()
    }
}

#[derive(Default)]
pub struct LocationTreeSelection {
    pub selected: HashSet<Vec<LocationPathSegment>>,
}

impl LocationTreeSelection {
    pub fn select(&mut self, path: &Vec<LocationPathSegment>) {
        self.selected.insert(path.clone());
    }

    pub fn deselect(&mut self, path: &Vec<LocationPathSegment>) {
        self.selected.remove(path);
    }

    pub fn toggle(&mut self, path: &Vec<LocationPathSegment>) {
        if !self.selected.remove(path) {
            self.selected.insert(path.clone());
        }
    }

    pub fn is_selected(&self, path: &Vec<LocationPathSegment>) -> bool {
        self.selected.contains(path)
    }

    pub fn clear(&mut self) {
        self.selected.clear();
    }

    pub fn all(&self) -> impl Iterator<Item = &Vec<LocationPathSegment>> {
        self.selected.iter()
    }
}

#[derive(Resource)]
pub struct LogViewerState {
    pub selected_spans: SpanTreeSelection,
    pub selected_locations: LocationTreeSelection,
    pub split_ratio: f32,
    pub threshold: Level,
    pub tree_mode: FilterTreeMode,
}

impl Default for LogViewerState {
    fn default() -> Self {
        Self {
            selected_spans: SpanTreeSelection::default(),
            selected_locations: LocationTreeSelection::default(),
            split_ratio: 0.35,
            threshold: Level::Warn,
            tree_mode: FilterTreeMode::Span,
        }
    }
}

#[derive(Resource, Default)]
pub struct UiWindows {
    pub show_log_viewer: bool,
}
