//! Transform-gizmo edit authority and transient interaction model.

use super::*;
use crate::devtools::{
    Inspect, InspectFieldMetadata, InspectFieldVisitor, InspectFieldVisitorMut, InspectTypeMetadata,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EditorTransformSpace {
    #[default]
    World,
    Local,
}

#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct EditorTransformGizmoSettings {
    transform_space: EditorTransformSpace,
}

impl Inspect for EditorTransformGizmoSettings {
    fn inspect_type_metadata() -> &'static InspectTypeMetadata {
        static FIELDS: [InspectFieldMetadata; 1] = [InspectFieldMetadata {
            id: InspectFieldId("transform_space"),
            rust_name: "transform_space",
            rust_type_name: "EditorTransformSpace",
            label: "Space",
            symbol: None,
            access: InspectAccess::Direct,
            unit: None,
            hint: None,
            role: Some("transform_space"),
            widget: Some(InspectWidgetId("editor.transform_space")),
            number_input: None,
        }];
        static METADATA: InspectTypeMetadata = InspectTypeMetadata {
            rust_name: concat!(module_path!(), "::EditorTransformGizmoSettings"),
            label: "Transform gizmo",
            fields: &FIELDS,
        };
        &METADATA
    }

    fn visit_inspect_fields(&self, visitor: &mut dyn InspectFieldVisitor) {
        visitor.field(
            &Self::inspect_type_metadata().fields[0],
            &self.transform_space,
        );
    }

    fn visit_inspect_fields_mut(&mut self, visitor: &mut dyn InspectFieldVisitorMut) {
        visitor.direct(
            &Self::inspect_type_metadata().fields[0],
            &mut self.transform_space,
        );
    }
}

impl EditorTransformGizmoSettings {
    pub fn transform_space(&self) -> EditorTransformSpace {
        self.transform_space
    }

    pub fn set_transform_space(&mut self, space: EditorTransformSpace) {
        self.transform_space = space;
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
pub(super) enum TransformOperation {
    Translate,
    Rotate,
    Scale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TransformAxis {
    X,
    Y,
    Z,
}

impl TransformAxis {
    pub(super) const ALL: [Self; 3] = [Self::X, Self::Y, Self::Z];

    pub(super) fn vector(self) -> Vec3 {
        match self {
            Self::X => Vec3::X,
            Self::Y => Vec3::Y,
            Self::Z => Vec3::Z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct TransformHandle {
    pub(super) operation: TransformOperation,
    pub(super) axis: TransformAxis,
}

#[derive(Debug, Clone)]
pub(super) struct TransformDrag {
    pub(super) entity: Entity,
    pub(super) handle: TransformHandle,
    pub(super) start_transform: Transform,
    pub(super) start_cursor: Vec2,
    pub(super) origin_screen: Vec2,
    pub(super) axis_screen: Vec2,
    pub(super) pixels_per_world: f32,
    pub(super) axis_pixels: f32,
    pub(super) world_axis: Vec3,
    pub(super) space: EditorTransformSpace,
}

#[derive(Resource, Debug, Default)]
pub(super) struct TransformGizmoInteraction {
    pub(super) hovered: Option<TransformHandle>,
    pub(super) drag: Option<TransformDrag>,
}

impl TransformGizmoInteraction {
    pub(super) fn active(&self) -> bool {
        self.drag.is_some()
    }
}
