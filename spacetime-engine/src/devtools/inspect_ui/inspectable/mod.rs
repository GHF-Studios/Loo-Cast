//! Visitor-backed rendering/editing of [`Inspect`] implementations.

use std::any::Any;

use super::*;

struct ShowInspectableVisitor<'ui, 'registry> {
    ui: &'ui mut egui::Ui,
    widgets: &'registry InspectorWidgetRegistry,
}

impl InspectFieldVisitor for ShowInspectableVisitor<'_, '_> {
    fn field(&mut self, metadata: &'static InspectFieldMetadata, value: &dyn Any) {
        show_registered_field(self.ui, self.widgets, metadata, value);
    }
}

struct EditInspectableVisitor<'ui, 'registry> {
    ui: &'ui mut egui::Ui,
    widgets: &'registry InspectorWidgetRegistry,
    changed: bool,
}

impl InspectFieldVisitorMut for EditInspectableVisitor<'_, '_> {
    fn read_only(&mut self, metadata: &'static InspectFieldMetadata, value: &dyn Any) {
        show_registered_field(self.ui, self.widgets, metadata, value);
    }

    fn direct(&mut self, metadata: &'static InspectFieldMetadata, value: &mut dyn Any) {
        let context = InspectWidgetContext::from_metadata(metadata);
        match self
            .widgets
            .edit_erased(self.ui, value, &context, metadata.widget)
        {
            Some(changed) => self.changed |= changed,
            None => draw_missing_registered_widget(self.ui, metadata),
        }
        draw_metadata_annotations(self.ui, metadata);
    }
}

/// Renders fields produced by an [`Inspect`] implementation through the external
/// value-widget registry. This is reusable by editor panels and technical game UI.
pub fn show_inspectable<T: Inspect>(
    ui: &mut egui::Ui,
    value: &T,
    widgets: &InspectorWidgetRegistry,
) {
    let mut visitor = ShowInspectableVisitor { ui, widgets };
    value.visit_inspect_fields(&mut visitor);
}

/// Direct-edits only fields that the type metadata marks [`super::InspectAccess::Direct`].
/// Other access modes remain observable and require explicit domain adapters.
pub fn edit_inspectable<T: Inspect>(
    ui: &mut egui::Ui,
    value: &mut T,
    widgets: &InspectorWidgetRegistry,
) -> bool {
    let mut visitor = EditInspectableVisitor {
        ui,
        widgets,
        changed: false,
    };
    value.visit_inspect_fields_mut(&mut visitor);
    visitor.changed
}

/// Type-erased counterpart used when a host discovers a type through
/// [`super::InspectTypeRegistry`].
pub fn show_registered_inspectable(
    ui: &mut egui::Ui,
    registration: InspectTypeRegistration,
    value: &dyn Any,
    widgets: &InspectorWidgetRegistry,
) -> bool {
    let mut visitor = ShowInspectableVisitor { ui, widgets };
    registration.visit(value, &mut visitor)
}

pub fn edit_registered_inspectable(
    ui: &mut egui::Ui,
    registration: InspectTypeRegistration,
    value: &mut dyn Any,
    widgets: &InspectorWidgetRegistry,
) -> Option<bool> {
    let mut visitor = EditInspectableVisitor {
        ui,
        widgets,
        changed: false,
    };
    registration
        .visit_mut(value, &mut visitor)
        .then_some(visitor.changed)
}

fn show_registered_field(
    ui: &mut egui::Ui,
    widgets: &InspectorWidgetRegistry,
    metadata: &'static InspectFieldMetadata,
    value: &dyn Any,
) {
    let context = InspectWidgetContext::from_metadata(metadata);
    if !widgets.show_erased(ui, value, &context, metadata.widget) {
        draw_missing_registered_widget(ui, metadata);
    }
    draw_metadata_annotations(ui, metadata);
}

fn draw_missing_registered_widget(ui: &mut egui::Ui, metadata: &InspectFieldMetadata) {
    ui.horizontal(|ui| {
        ui.label(metadata.label);
        ui.weak(format!(
            "<no inspector widget for {}>",
            metadata.rust_type_name
        ));
    });
}

fn draw_metadata_annotations(ui: &mut egui::Ui, metadata: &InspectFieldMetadata) {
    if metadata.access != super::InspectAccess::ReadOnly
        && metadata.access != super::InspectAccess::Direct
    {
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.weak(format!(
                "authority: {} · requires adapter",
                metadata.access.label()
            ));
        });
    }
    if let Some(hint) = metadata.hint {
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.weak(hint);
        });
    }
}
