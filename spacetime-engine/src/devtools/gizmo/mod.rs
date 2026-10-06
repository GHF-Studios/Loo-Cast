//! Contextual editor gizmos.
//!
//! A gizmo is broader than a viewport handle: it may expose rich state, drawing,
//! interaction and actions. This module contains the first concrete gizmo: a
//! unified Transform gizmo whose translate, rotate and scale affordances are all
//! visible at once. Visibility and edit authority are deliberately separate.

use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::{
    input_focus::{InputFocus, InputFocusSet},
    view::{PrimaryGameView, PrimaryViewPresentation, ViewportSpace},
};

use super::{
    AppInspectExt, AppInspectorWidgetsExt, DeveloperFocus, DeveloperSet, DrawDepth, InspectAccess,
    InspectEditRequest, InspectField, InspectFieldId, InspectNumberInput, InspectSection,
    InspectSectionId, InspectValue, InspectWidgetId, InspectionFrame, StructureFrame,
    StructureItem, StructureItemId, StructureSelection, WorldDrawBatch, WorldDrawFrame,
    inspect_ui::{InspectWidgetContext, InspectorWidget, egui},
};

const INPUT_FOCUS_OWNER: &str = "editor_gizmo";
const TRANSFORM_SPACE_WIDGET: InspectWidgetId = InspectWidgetId("editor.transform_space");
pub(in crate::devtools) const TRANSFORM_SECTION: InspectSectionId = InspectSectionId("transform");
pub(in crate::devtools) const TRANSFORM_STRUCTURE: StructureItemId =
    StructureItemId("core.transform");
pub(in crate::devtools) const TRANSLATION_FIELD: InspectFieldId =
    InspectFieldId("transform.translation");
pub(in crate::devtools) const ROTATION_FIELD: InspectFieldId = InspectFieldId("transform.rotation");
pub(in crate::devtools) const SCALE_FIELD: InspectFieldId = InspectFieldId("transform.scale");
const HANDLE_PICK_PIXELS: f32 = 9.0;
const RING_SEGMENTS: usize = 40;

mod model;
mod widget;

pub use model::{EditorTransformGizmoSettings, EditorTransformSpace, EditorTransformWritable};
use model::{
    TransformAxis, TransformDrag, TransformGizmoInteraction, TransformHandle, TransformOperation,
};
use widget::EditorTransformSpaceWidget;

mod geometry;
mod inspection;
mod interaction;
mod presentation;

use geometry::{
    AxisHandleGeometry, gizmo_world_size, handle_color, handle_world_axis, hit_test, project,
    transform_context_visible,
};
use inspection::{
    apply_transform_inspection_edits, collect_transform_inspection, collect_transform_structure,
};
use interaction::{claim_gizmo_input, select_from_primary_view, update_transform_gizmo};
use presentation::collect_transform_gizmo;

pub(super) fn configure(app: &mut App) {
    app.register_inspectable::<EditorTransformGizmoSettings>()
        .register_named_inspector_widget::<EditorTransformSpace, _>(
            TRANSFORM_SPACE_WIDGET,
            EditorTransformSpaceWidget,
        )
        .init_resource::<InputFocus>()
        .init_resource::<PrimaryViewPresentation>()
        .init_resource::<EditorTransformGizmoSettings>()
        .init_resource::<TransformGizmoInteraction>()
        .add_systems(PreUpdate, claim_gizmo_input.before(InputFocusSet::Resolve))
        // Inspection UI writes proposals after gameplay update; commit them in
        // the next PreUpdate, still safely before Bevy's normal PostUpdate
        // transform propagation. Viewport dragging remains immediate in Update.
        .add_systems(PreUpdate, apply_transform_inspection_edits)
        .add_systems(Update, update_transform_gizmo)
        .add_systems(
            PostUpdate,
            select_from_primary_view.in_set(DeveloperSet::Interact),
        )
        .add_systems(
            PostUpdate,
            collect_transform_structure.in_set(DeveloperSet::CollectStructure),
        )
        .add_systems(
            PostUpdate,
            collect_transform_inspection.in_set(DeveloperSet::CollectInspection),
        )
        .add_systems(
            PostUpdate,
            collect_transform_gizmo.in_set(DeveloperSet::CollectWorldDraw),
        );
}
