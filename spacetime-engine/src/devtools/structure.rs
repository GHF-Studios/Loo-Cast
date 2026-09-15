use bevy::prelude::*;

use super::{DeveloperFocus, FocusTarget};

/// Stable semantic identity for one meaningful part of the focused entity.
///
/// Structure IDs are deliberately independent from Inspector section IDs: one
/// semantic thing may contribute multiple inspection sections and one gizmo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructureItemId(pub &'static str);

#[derive(Debug, Clone)]
pub struct StructureItem {
    pub id: StructureItemId,
    pub label: String,
    pub detail: Option<String>,
    pub order: i32,
}

impl StructureItem {
    pub fn new(id: StructureItemId, label: impl Into<String>, order: i32) -> Self {
        Self {
            id,
            label: label.into(),
            detail: None,
            order,
        }
    }

    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

/// One frame of semantic structure for the canonical focus.
///
/// Domains submit only concepts they can currently justify. This is not a
/// reflected-component dump and deliberately does not mirror the ECS archetype.
#[derive(Resource, Debug, Default)]
pub struct StructureFrame {
    items: Vec<StructureItem>,
}

impl StructureFrame {
    pub fn submit(&mut self, item: StructureItem) {
        assert!(
            !self.items.iter().any(|existing| existing.id == item.id),
            "duplicate structure item id {}",
            item.id.0,
        );
        self.items.push(item);
    }

    pub fn sorted_items(&self) -> Vec<&StructureItem> {
        let mut items = self.items.iter().collect::<Vec<_>>();
        items.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.label.cmp(&b.label)));
        items
    }

    pub fn contains(&self, id: StructureItemId) -> bool {
        self.items.iter().any(|item| item.id == id)
    }

    fn clear(&mut self) {
        self.items.clear();
    }
}

/// Semantic refinement inside one canonical entity focus.
///
/// `None` means the whole focused entity. A concrete item means Structure has
/// refined that entity to a meaningful component/module/segment-like concept.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct StructureSelection {
    owner: Option<(Entity, Entity)>,
    item: Option<StructureItemId>,
}

impl StructureSelection {
    pub fn item_for(&self, target: FocusTarget) -> Option<StructureItemId> {
        (self.owner == Some((target.spatial_entity, target.semantic_entity)))
            .then_some(self.item)
            .flatten()
    }

    pub fn select_entity(&mut self, target: FocusTarget) {
        self.owner = Some((target.spatial_entity, target.semantic_entity));
        self.item = None;
    }

    pub fn select_item(&mut self, target: FocusTarget, item: StructureItemId) {
        self.owner = Some((target.spatial_entity, target.semantic_entity));
        self.item = Some(item);
    }

    pub fn clear(&mut self) {
        self.owner = None;
        self.item = None;
    }
}

pub(super) fn clear_structure_frame(mut frame: ResMut<StructureFrame>) {
    frame.clear();
}

pub(super) fn prune_structure_selection(
    focus: Res<DeveloperFocus>,
    frame: Res<StructureFrame>,
    mut selection: ResMut<StructureSelection>,
) {
    let Some(target) = focus.current() else {
        selection.clear();
        return;
    };

    if let Some(item) = selection.item_for(target)
        && !frame.contains(item)
    {
        selection.select_entity(target);
    }
}
