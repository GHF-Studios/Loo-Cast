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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EditorTransformSpace {
    #[default]
    World,
    Local,
}

#[derive(Resource, Debug, Default, Clone, Copy, spacetime_engine_macros::Inspect)]
#[inspect(label = "Transform gizmo")]
pub struct EditorTransformGizmoSettings {
    #[inspect(
        label = "Space",
        direct,
        role = "transform_space",
        widget = "editor.transform_space"
    )]
    transform_space: EditorTransformSpace,
}

impl EditorTransformGizmoSettings {
    pub fn transform_space(&self) -> EditorTransformSpace {
        self.transform_space
    }

    pub fn set_transform_space(&mut self, space: EditorTransformSpace) {
        self.transform_space = space;
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct EditorTransformSpaceWidget;

impl InspectorWidget<EditorTransformSpace> for EditorTransformSpaceWidget {
    fn show(
        &self,
        ui: &mut egui::Ui,
        value: &EditorTransformSpace,
        context: &InspectWidgetContext<'_>,
    ) {
        ui.horizontal(|ui| {
            ui.label(context.label);
            ui.monospace(match value {
                EditorTransformSpace::World => "World",
                EditorTransformSpace::Local => "Local",
            });
        });
    }

    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut EditorTransformSpace,
        context: &InspectWidgetContext<'_>,
    ) -> bool {
        let before = *value;
        ui.horizontal(|ui| {
            ui.label(context.label);
            for (candidate, label) in [
                (EditorTransformSpace::World, "World"),
                (EditorTransformSpace::Local, "Local"),
            ] {
                if ui.selectable_label(*value == candidate, label).clicked() {
                    *value = candidate;
                }
            }
        });
        *value != before
    }
}

/// Explicit permission for the generic gizmo to mutate this runtime Transform.
///
/// Mere presence of [`Transform`] grants observation, never authority. Generated,
/// simulated, asset-authored or otherwise derived transforms should instead gain
/// domain-specific authoring adapters that commit to their real source of truth.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct EditorTransformWritable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransformOperation {
    Translate,
    Rotate,
    Scale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransformAxis {
    X,
    Y,
    Z,
}

impl TransformAxis {
    const ALL: [Self; 3] = [Self::X, Self::Y, Self::Z];

    fn vector(self) -> Vec3 {
        match self {
            Self::X => Vec3::X,
            Self::Y => Vec3::Y,
            Self::Z => Vec3::Z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TransformHandle {
    operation: TransformOperation,
    axis: TransformAxis,
}

#[derive(Debug, Clone)]
struct TransformDrag {
    entity: Entity,
    handle: TransformHandle,
    start_transform: Transform,
    start_cursor: Vec2,
    origin_screen: Vec2,
    axis_screen: Vec2,
    pixels_per_world: f32,
    axis_pixels: f32,
    world_axis: Vec3,
    space: EditorTransformSpace,
}

#[derive(Resource, Debug, Default)]
struct TransformGizmoInteraction {
    hovered: Option<TransformHandle>,
    drag: Option<TransformDrag>,
}

impl TransformGizmoInteraction {
    fn active(&self) -> bool {
        self.drag.is_some()
    }
}

mod geometry;
mod inspection;
mod interaction;
mod presentation;

use geometry::{
    gizmo_world_size, handle_color, handle_world_axis, hit_test, project, ring_basis,
    transform_context_visible,
};
use inspection::{
    apply_transform_inspection_edits, collect_transform_inspection,
    collect_transform_structure,
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
