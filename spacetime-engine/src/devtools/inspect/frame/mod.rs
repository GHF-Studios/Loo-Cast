//! Per-frame inspection snapshot and edit/action request protocol.

use bevy::prelude::{Message, ResMut, Resource};

use super::{
    metadata::{InspectActionId, InspectFieldId, InspectSectionId},
    model::{InspectSection, InspectValue},
};
use crate::devtools::{FocusTarget, StructureItemId};

/// Proposed semantic field edit emitted by an inspection UI host.
///
/// This is a request, never authority. The owning domain must re-check target,
/// access, invariants and persistence semantics before applying it.
#[derive(Message, Debug, Clone)]
pub struct InspectEditRequest {
    pub target: FocusTarget,
    pub section: InspectSectionId,
    pub field: InspectFieldId,
    pub value: InspectValue,
}

/// Contextual operation emitted by an inspection/gizmo UI host.
#[derive(Message, Debug, Clone, Copy)]
pub struct InspectActionRequest {
    pub target: FocusTarget,
    pub section: InspectSectionId,
    pub action: InspectActionId,
}

/// One frame of structured semantic inspection data for the canonical focus.
#[derive(Resource, Debug, Default)]
pub struct InspectionFrame {
    sections: Vec<InspectSection>,
}

impl InspectionFrame {
    pub fn submit(&mut self, section: InspectSection) {
        assert!(
            !self
                .sections
                .iter()
                .any(|existing| existing.id == section.id),
            "duplicate inspection section id {}",
            section.id.0,
        );
        self.sections.push(section);
    }

    pub fn sections(&self) -> &[InspectSection] {
        &self.sections
    }

    pub fn section(&self, id: InspectSectionId) -> Option<&InspectSection> {
        self.sections.iter().find(|section| section.id == id)
    }

    pub fn sorted_sections(&self) -> Vec<&InspectSection> {
        self.sorted_sections_for(None)
    }

    /// `None` means whole-entity inspection and therefore returns all sections.
    pub fn sorted_sections_for(
        &self,
        structure_item: Option<StructureItemId>,
    ) -> Vec<&InspectSection> {
        let mut sections = self
            .sections
            .iter()
            .filter(|section| structure_item.is_none() || section.structure_item == structure_item)
            .collect::<Vec<_>>();
        sections.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.title.cmp(&b.title)));
        sections
    }

    fn clear(&mut self) {
        self.sections.clear();
    }
}

pub(super) fn clear_inspection_frame(mut frame: ResMut<InspectionFrame>) {
    frame.clear();
}
