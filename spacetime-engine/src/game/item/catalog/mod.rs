//! Registered semantic item definitions.
//!
//! The catalog is metadata, not behavior. Item behavior is supplied by ordinary
//! Bevy plugins/systems that consume [`super::UseItem`] messages.

use bevy::prelude::*;

/// Stable logical ID used by inventory state and action routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub &'static str);

impl ItemId {
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }
}

/// Presentation metadata for one registered playground item.
#[derive(Debug, Clone, Copy)]
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: &'static str,
    pub description: &'static str,
}

/// Runtime catalog populated by built-in content and mods during startup.
#[derive(Resource, Default)]
pub struct ItemCatalog {
    items: Vec<ItemDefinition>,
}

impl ItemCatalog {
    pub fn register(&mut self, item: ItemDefinition) {
        assert!(
            !self.items.iter().any(|existing| existing.id == item.id),
            "duplicate item id: {}",
            item.id.0,
        );
        self.items.push(item);
    }

    pub fn items(&self) -> &[ItemDefinition] {
        &self.items
    }

    pub fn find(&self, id: ItemId) -> Option<&ItemDefinition> {
        self.items.iter().find(|item| item.id == id)
    }
}
