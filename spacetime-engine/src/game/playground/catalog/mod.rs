//! Registered playground item definitions.
//!
//! The catalog is metadata, not behavior. Item behavior is supplied by ordinary
//! Bevy plugins/systems that consume [`super::UsePlaygroundItem`] messages.

use bevy::prelude::*;

/// Stable logical ID used by inventory state and action routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaygroundItemId(pub &'static str);

impl PlaygroundItemId {
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }
}

/// Presentation metadata for one registered playground item.
#[derive(Debug, Clone, Copy)]
pub struct PlaygroundItem {
    pub id: PlaygroundItemId,
    pub name: &'static str,
    pub description: &'static str,
}

/// Runtime catalog populated by built-in content and mods during startup.
#[derive(Resource, Default)]
pub struct PlaygroundCatalog {
    items: Vec<PlaygroundItem>,
}

impl PlaygroundCatalog {
    pub fn register(&mut self, item: PlaygroundItem) {
        assert!(
            !self.items.iter().any(|existing| existing.id == item.id),
            "duplicate playground item id: {}",
            item.id.0,
        );
        self.items.push(item);
    }

    pub fn items(&self) -> &[PlaygroundItem] {
        &self.items
    }

    pub fn find(&self, id: PlaygroundItemId) -> Option<&PlaygroundItem> {
        self.items.iter().find(|item| item.id == id)
    }
}
