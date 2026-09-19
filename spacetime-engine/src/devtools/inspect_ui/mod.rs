//! Reusable egui presentation for the UI-agnostic inspection model.
//!
//! The important split is deliberate:
//!
//! - `inspect` describes values, metadata and authority;
//! - this module knows how to present/edit values with egui;
//! - domains still own validation/commit semantics.
//!
//! Custom widgets implement [`InspectorWidget<T>`]. The trait has separate
//! read-only and editable entry points so presentation is only handed `&mut T`
//! when the host already possesses a real edit capability.

use bevy::prelude::*;
pub use bevy_egui::egui;

use super::{
    FocusTarget, Inspect, InspectAccess, InspectActionRequest, InspectEditRequest,
    InspectField, InspectFieldMetadata, InspectFieldVisitor, InspectFieldVisitorMut,
    InspectNumberFormat, InspectNumberInput, InspectSection, InspectTypeRegistration,
    InspectUnit, InspectValue, InspectWidgetId, InspectionFrame,
    StructureItemId,
};

mod formatting;
mod inspectable;
mod registry;
mod sections;
mod widgets;

pub use formatting::{format_number, format_quantity, format_value};
pub use inspectable::{
    edit_inspectable, edit_registered_inspectable, show_inspectable,
    show_registered_inspectable,
};
pub use registry::{
    AppInspectorWidgetsExt, InspectWidgetContext, InspectorWidget,
    InspectorWidgetRegistry,
};
pub use sections::{
    InspectionUiOutput, draw_contextual_gizmo, draw_section, draw_sections,
};
pub use widgets::{
    BoolWidget, NumberWidget, QuatWidget, StringWidget, TransformWidget, Vec3Widget,
};

pub(super) fn configure(app: &mut App) {
    app.register_inspector_widget::<f32, _>(NumberWidget)
        .register_inspector_widget::<f64, _>(NumberWidget)
        .register_inspector_widget::<bool, _>(BoolWidget)
        .register_inspector_widget::<String, _>(StringWidget)
        .register_inspector_widget::<Vec3, _>(Vec3Widget)
        .register_inspector_widget::<Quat, _>(QuatWidget)
        .register_inspector_widget::<Transform, _>(TransformWidget);
}

#[cfg(test)]
mod tests;
