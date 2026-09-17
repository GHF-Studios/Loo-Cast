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

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

use bevy::prelude::*;
pub use bevy_egui::egui;

use super::{
    FocusTarget, Inspect, InspectActionRequest, InspectEditRequest, InspectField,
    InspectFieldMetadata, InspectFieldVisitor, InspectFieldVisitorMut, InspectNumberFormat,
    InspectNumberInput, InspectSection, InspectTypeRegistration, InspectUnit, InspectValue,
    InspectWidgetId, InspectionFrame, StructureItemId,
};

pub struct InspectWidgetContext<'a> {
    pub label: &'a str,
    pub unit: Option<InspectUnit>,
    pub hint: Option<&'a str>,
    /// Semantic role for context-sensitive presentation of otherwise identical
    /// Rust types, e.g. `Vec3` as position vs velocity vs scale.
    pub role: Option<&'a str>,
    pub number_input: Option<InspectNumberInput>,
}

impl<'a> InspectWidgetContext<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            unit: None,
            hint: None,
            role: None,
            number_input: None,
        }
    }

    pub fn from_metadata(metadata: &'a InspectFieldMetadata) -> Self {
        Self {
            label: metadata.label,
            unit: metadata.unit,
            hint: metadata.hint,
            role: metadata.role,
            number_input: metadata.number_input,
        }
    }
}

/// First-class advanced path for bespoke value presentation.
///
/// Implementations never receive a `read_only` flag. A read-only host calls
/// `show`; a host with actual authority may call `edit` and provide mutable data.
pub trait InspectorWidget<T: ?Sized> {
    fn show(&self, ui: &mut egui::Ui, value: &T, context: &InspectWidgetContext<'_>);
    fn edit(&self, ui: &mut egui::Ui, value: &mut T, context: &InspectWidgetContext<'_>) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct InspectorWidgetKey {
    type_id: TypeId,
    widget: Option<InspectWidgetId>,
}

trait ErasedInspectorWidget: Send + Sync {
    fn show(&self, ui: &mut egui::Ui, value: &dyn Any, context: &InspectWidgetContext<'_>) -> bool;
    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut dyn Any,
        context: &InspectWidgetContext<'_>,
    ) -> Option<bool>;
}

struct TypedInspectorWidget<T, W> {
    widget: W,
    marker: PhantomData<fn() -> T>,
}

impl<T, W> ErasedInspectorWidget for TypedInspectorWidget<T, W>
where
    T: 'static,
    W: InspectorWidget<T> + Send + Sync + 'static,
{
    fn show(&self, ui: &mut egui::Ui, value: &dyn Any, context: &InspectWidgetContext<'_>) -> bool {
        let Some(value) = value.downcast_ref::<T>() else {
            return false;
        };
        self.widget.show(ui, value, context);
        true
    }

    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut dyn Any,
        context: &InspectWidgetContext<'_>,
    ) -> Option<bool> {
        let value = value.downcast_mut::<T>()?;
        Some(self.widget.edit(ui, value, context))
    }
}

/// Reusable registry for value widgets supplied by the engine, a game, a mod, or
/// another package. Widget registration is separate from [`Inspect`] metadata:
/// types describe meaning; UI hosts decide how concrete Rust values are rendered.
#[derive(Resource, Default)]
pub struct InspectorWidgetRegistry {
    widgets: HashMap<InspectorWidgetKey, Box<dyn ErasedInspectorWidget>>,
}

impl InspectorWidgetRegistry {
    pub fn register<T, W>(&mut self, widget: W)
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static,
    {
        self.insert::<T, W>(None, widget);
    }

    pub fn register_named<T, W>(&mut self, id: InspectWidgetId, widget: W)
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static,
    {
        self.insert::<T, W>(Some(id), widget);
    }

    fn insert<T, W>(&mut self, id: Option<InspectWidgetId>, widget: W)
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static,
    {
        let key = InspectorWidgetKey {
            type_id: TypeId::of::<T>(),
            widget: id,
        };
        // Registration order is the explicit override policy: later registrations
        // replace the same `(Rust type, optional widget ID)` key. A named widget
        // remains independent from the default widget for that Rust type.
        self.widgets.insert(
            key,
            Box::new(TypedInspectorWidget::<T, W> {
                widget,
                marker: PhantomData,
            }),
        );
    }

    fn resolve(
        &self,
        type_id: TypeId,
        requested: Option<InspectWidgetId>,
    ) -> Option<&dyn ErasedInspectorWidget> {
        requested
            .and_then(|widget| {
                self.widgets
                    .get(&InspectorWidgetKey {
                        type_id,
                        widget: Some(widget),
                    })
                    .map(Box::as_ref)
            })
            .or_else(|| {
                self.widgets
                    .get(&InspectorWidgetKey {
                        type_id,
                        widget: None,
                    })
                    .map(Box::as_ref)
            })
    }

    pub fn show<T: 'static>(
        &self,
        ui: &mut egui::Ui,
        value: &T,
        context: &InspectWidgetContext<'_>,
        widget: Option<InspectWidgetId>,
    ) -> bool {
        self.show_erased(ui, value, context, widget)
    }

    pub fn edit<T: 'static>(
        &self,
        ui: &mut egui::Ui,
        value: &mut T,
        context: &InspectWidgetContext<'_>,
        widget: Option<InspectWidgetId>,
    ) -> Option<bool> {
        self.edit_erased(ui, value, context, widget)
    }

    pub fn show_erased(
        &self,
        ui: &mut egui::Ui,
        value: &dyn Any,
        context: &InspectWidgetContext<'_>,
        widget: Option<InspectWidgetId>,
    ) -> bool {
        self.resolve(value.type_id(), widget)
            .is_some_and(|renderer| renderer.show(ui, value, context))
    }

    pub fn edit_erased(
        &self,
        ui: &mut egui::Ui,
        value: &mut dyn Any,
        context: &InspectWidgetContext<'_>,
        widget: Option<InspectWidgetId>,
    ) -> Option<bool> {
        self.resolve((&*value).type_id(), widget)
            .and_then(|renderer| renderer.edit(ui, value, context))
    }
}

pub trait AppInspectorWidgetsExt {
    fn register_inspector_widget<T, W>(&mut self, widget: W) -> &mut Self
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static;

    fn register_named_inspector_widget<T, W>(
        &mut self,
        id: InspectWidgetId,
        widget: W,
    ) -> &mut Self
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static;
}

impl AppInspectorWidgetsExt for App {
    fn register_inspector_widget<T, W>(&mut self, widget: W) -> &mut Self
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static,
    {
        self.init_resource::<InspectorWidgetRegistry>();
        self.world_mut()
            .resource_mut::<InspectorWidgetRegistry>()
            .register::<T, W>(widget);
        self
    }

    fn register_named_inspector_widget<T, W>(&mut self, id: InspectWidgetId, widget: W) -> &mut Self
    where
        T: 'static,
        W: InspectorWidget<T> + Send + Sync + 'static,
    {
        self.init_resource::<InspectorWidgetRegistry>();
        self.world_mut()
            .resource_mut::<InspectorWidgetRegistry>()
            .register_named::<T, W>(id, widget);
        self
    }
}

pub(super) fn configure(app: &mut App) {
    app.register_inspector_widget::<f32, _>(NumberWidget)
        .register_inspector_widget::<f64, _>(NumberWidget)
        .register_inspector_widget::<bool, _>(BoolWidget)
        .register_inspector_widget::<String, _>(StringWidget)
        .register_inspector_widget::<Vec3, _>(Vec3Widget)
        .register_inspector_widget::<Quat, _>(QuatWidget)
        .register_inspector_widget::<Transform, _>(TransformWidget);
}

#[derive(Debug, Default, Clone, Copy)]
pub struct NumberWidget;

impl InspectorWidget<f32> for NumberWidget {
    fn show(&self, ui: &mut egui::Ui, value: &f32, context: &InspectWidgetContext<'_>) {
        draw_number_text(ui, *value as f64, context);
    }

    fn edit(&self, ui: &mut egui::Ui, value: &mut f32, context: &InspectWidgetContext<'_>) -> bool {
        let mut value64 = *value as f64;
        let changed = edit_f64(ui, &mut value64, context);
        if changed && value64.is_finite() {
            *value = value64 as f32;
        }
        changed
    }
}

impl InspectorWidget<f64> for NumberWidget {
    fn show(&self, ui: &mut egui::Ui, value: &f64, context: &InspectWidgetContext<'_>) {
        draw_number_text(ui, *value, context);
    }

    fn edit(&self, ui: &mut egui::Ui, value: &mut f64, context: &InspectWidgetContext<'_>) -> bool {
        edit_f64(ui, value, context)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BoolWidget;

impl InspectorWidget<bool> for BoolWidget {
    fn show(&self, ui: &mut egui::Ui, value: &bool, context: &InspectWidgetContext<'_>) {
        ui.horizontal(|ui| {
            ui.label(context.label);
            ui.monospace(if *value { "yes" } else { "no" });
        });
    }

    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut bool,
        context: &InspectWidgetContext<'_>,
    ) -> bool {
        ui.checkbox(value, context.label).changed()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StringWidget;

impl InspectorWidget<String> for StringWidget {
    fn show(&self, ui: &mut egui::Ui, value: &String, context: &InspectWidgetContext<'_>) {
        ui.horizontal(|ui| {
            ui.label(context.label);
            ui.monospace(value.as_str());
        });
    }

    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut String,
        context: &InspectWidgetContext<'_>,
    ) -> bool {
        ui.horizontal(|ui| {
            ui.label(context.label);
            ui.text_edit_singleline(value).changed()
        })
        .inner
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3Widget;

impl InspectorWidget<Vec3> for Vec3Widget {
    fn show(&self, ui: &mut egui::Ui, value: &Vec3, context: &InspectWidgetContext<'_>) {
        ui.horizontal(|ui| {
            ui.label(context.label);
            ui.monospace(format!("{:.4}  {:.4}  {:.4}", value.x, value.y, value.z));
            if let Some(unit) = context.unit {
                ui.weak(unit.0);
            }
        });
    }

    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut Vec3,
        context: &InspectWidgetContext<'_>,
    ) -> bool {
        let speed = context.number_input.unwrap_or_default().speed;
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label(context.label);
            for (axis, component) in [
                ("X", &mut value.x),
                ("Y", &mut value.y),
                ("Z", &mut value.z),
            ] {
                ui.weak(axis);
                changed |= ui
                    .add(egui::DragValue::new(component).speed(speed))
                    .changed();
            }
            if let Some(unit) = context.unit {
                ui.weak(unit.0);
            }
        });
        changed
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct QuatWidget;

impl InspectorWidget<Quat> for QuatWidget {
    fn show(&self, ui: &mut egui::Ui, value: &Quat, context: &InspectWidgetContext<'_>) {
        let (x, y, z) = value.to_euler(EulerRot::XYZ);
        Vec3Widget.show(
            ui,
            &Vec3::new(x.to_degrees(), y.to_degrees(), z.to_degrees()),
            &InspectWidgetContext {
                label: context.label,
                unit: context.unit,
                hint: context.hint,
                role: context.role,
                number_input: context.number_input,
            },
        );
    }

    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut Quat,
        context: &InspectWidgetContext<'_>,
    ) -> bool {
        let (x, y, z) = value.to_euler(EulerRot::XYZ);
        let mut degrees = Vec3::new(x.to_degrees(), y.to_degrees(), z.to_degrees());
        if !Vec3Widget.edit(ui, &mut degrees, context) {
            return false;
        }
        *value = Quat::from_euler(
            EulerRot::XYZ,
            degrees.x.to_radians(),
            degrees.y.to_radians(),
            degrees.z.to_radians(),
        );
        true
    }
}

/// Reusable rich Transform editor used by the Transform gizmo UI. It deliberately
/// performs no authority check itself; the host chooses `show` vs `edit`.
#[derive(Debug, Default, Clone, Copy)]
pub struct TransformWidget;

impl InspectorWidget<Transform> for TransformWidget {
    fn show(&self, ui: &mut egui::Ui, value: &Transform, _context: &InspectWidgetContext<'_>) {
        Vec3Widget.show(
            ui,
            &value.translation,
            &InspectWidgetContext::new("Position"),
        );
        QuatWidget.show(
            ui,
            &value.rotation,
            &InspectWidgetContext::new("Rotation °"),
        );
        Vec3Widget.show(ui, &value.scale, &InspectWidgetContext::new("Scale"));
    }

    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut Transform,
        _context: &InspectWidgetContext<'_>,
    ) -> bool {
        let mut changed = false;

        let translation_context = InspectWidgetContext {
            number_input: Some(InspectNumberInput::speed(0.02)),
            ..InspectWidgetContext::new("Position")
        };
        changed |= Vec3Widget.edit(ui, &mut value.translation, &translation_context);

        let rotation_context = InspectWidgetContext {
            number_input: Some(InspectNumberInput::speed(0.2)),
            ..InspectWidgetContext::new("Rotation °")
        };
        changed |= QuatWidget.edit(ui, &mut value.rotation, &rotation_context);

        let scale_context = InspectWidgetContext {
            number_input: Some(InspectNumberInput::speed(0.01)),
            ..InspectWidgetContext::new("Scale")
        };
        // Direct really means direct: no hidden positivity clamp. A domain that
        // forbids mirrored/negative scale must expose validated authority.
        changed |= Vec3Widget.edit(ui, &mut value.scale, &scale_context);

        changed
    }
}

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

#[derive(Debug, Default)]
pub struct InspectionUiOutput {
    pub edits: Vec<InspectEditRequest>,
    pub actions: Vec<InspectActionRequest>,
}

/// Draw semantic inspection sections using reusable value widgets.
///
/// Editable fields edit a local proposal and emit a request. The UI never writes
/// domain state directly; the owning domain validates and commits the proposal.
pub fn draw_sections(
    ui: &mut egui::Ui,
    frame: &InspectionFrame,
    target: FocusTarget,
    structure_item: Option<StructureItemId>,
    show_actions: bool,
) -> InspectionUiOutput {
    let mut output = InspectionUiOutput::default();
    let sections = frame.sorted_sections_for(structure_item);

    if sections.is_empty() {
        ui.weak("No semantic inspection data for this Structure item.");
        return output;
    }

    for (index, section) in sections.into_iter().enumerate() {
        if index != 0 {
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);
        }
        draw_section(ui, section, target, show_actions, &mut output);
    }

    output
}

pub fn draw_contextual_gizmo(
    ui: &mut egui::Ui,
    frame: &InspectionFrame,
    target: FocusTarget,
    structure_item: StructureItemId,
) -> InspectionUiOutput {
    let mut output = InspectionUiOutput::default();
    let sections = frame
        .sorted_sections_for(Some(structure_item))
        .into_iter()
        .filter(|section| section.contextual_gizmo)
        .collect::<Vec<_>>();

    if sections.is_empty() {
        ui.weak("No contextual gizmo is registered for this Structure item.");
        return output;
    }

    for (index, section) in sections.into_iter().enumerate() {
        if index != 0 {
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);
        }
        draw_section(ui, section, target, true, &mut output);
    }

    output
}

pub fn draw_section(
    ui: &mut egui::Ui,
    section: &InspectSection,
    target: FocusTarget,
    show_actions: bool,
    output: &mut InspectionUiOutput,
) {
    ui.heading(&section.title);

    for field in &section.fields {
        let proposed = draw_field(ui, field);
        if let (Some(field_id), Some(value)) = (field.id, proposed) {
            output.edits.push(InspectEditRequest {
                target,
                section: section.id,
                field: field_id,
                value,
            });
        }
    }

    if show_actions && !section.actions.is_empty() {
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            for action in &section.actions {
                let mut response = ui.button(&action.label);
                if let Some(hint) = &action.hint {
                    response = response.on_hover_text(hint);
                }
                if response.clicked() {
                    output.actions.push(InspectActionRequest {
                        target,
                        section: section.id,
                        action: action.id,
                    });
                }
            }
        });
    }
}

fn draw_field(ui: &mut egui::Ui, field: &InspectField) -> Option<InspectValue> {
    let label = field
        .symbol
        .map(|symbol| format!("{} ({symbol})", field.label))
        .unwrap_or_else(|| field.label.clone());

    let editable = field.access.editable() && field.id.is_some();
    let mut proposed = field.value.clone();
    let changed = if editable {
        edit_inspect_value(ui, &label, &mut proposed, field)
    } else {
        show_inspect_value(ui, &label, &field.value, field);
        false
    };

    if field.access != super::InspectAccess::ReadOnly {
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.weak(format!("authority: {}", field.access.label()));
            if field.id.is_none() {
                ui.weak("(no generic edit binding)");
            }
        });
    }
    if let Some(hint) = &field.hint {
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.weak(hint);
        });
    }

    changed.then_some(proposed)
}

fn show_inspect_value(ui: &mut egui::Ui, label: &str, value: &InspectValue, field: &InspectField) {
    match value {
        InspectValue::Vec3(value) => Vec3Widget.show(
            ui,
            value,
            &InspectWidgetContext {
                label,
                unit: None,
                hint: field.hint.as_deref(),
                role: None,
                number_input: field.number_input,
            },
        ),
        _ => {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.monospace(format_value(value));
            });
        }
    }
}

fn edit_inspect_value(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut InspectValue,
    field: &InspectField,
) -> bool {
    match value {
        InspectValue::Text(value) => StringWidget.edit(
            ui,
            value,
            &InspectWidgetContext {
                label,
                unit: None,
                hint: field.hint.as_deref(),
                role: None,
                number_input: field.number_input,
            },
        ),
        InspectValue::Bool(value) => BoolWidget.edit(
            ui,
            value,
            &InspectWidgetContext {
                label,
                unit: None,
                hint: field.hint.as_deref(),
                role: None,
                number_input: field.number_input,
            },
        ),
        InspectValue::Integer(value) => {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(egui::DragValue::new(value)).changed()
            })
            .inner
        }
        InspectValue::Number { value, .. } => NumberWidget.edit(
            ui,
            value,
            &InspectWidgetContext {
                label,
                unit: None,
                hint: field.hint.as_deref(),
                role: None,
                number_input: field.number_input,
            },
        ),
        InspectValue::Quantity { value, unit, .. } => NumberWidget.edit(
            ui,
            value,
            &InspectWidgetContext {
                label,
                unit: Some(*unit),
                hint: field.hint.as_deref(),
                role: None,
                number_input: field.number_input,
            },
        ),
        InspectValue::Vec3(value) => Vec3Widget.edit(
            ui,
            value,
            &InspectWidgetContext {
                label,
                unit: None,
                hint: field.hint.as_deref(),
                role: None,
                number_input: field.number_input,
            },
        ),
        // Entity references and derived ranges need explicit semantic widgets.
        InspectValue::Range { .. } | InspectValue::Entity(_) => {
            show_inspect_value(ui, label, value, field);
            false
        }
    }
}

fn edit_f64(ui: &mut egui::Ui, value: &mut f64, context: &InspectWidgetContext<'_>) -> bool {
    let input = context.number_input.unwrap_or_default();
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(context.label);
        changed = if input.slider {
            match (input.minimum, input.maximum) {
                (Some(minimum), Some(maximum)) => ui
                    .add(egui::Slider::new(value, minimum..=maximum))
                    .changed(),
                _ => ui
                    .add(egui::DragValue::new(value).speed(input.speed))
                    .changed(),
            }
        } else {
            ui.add(egui::DragValue::new(value).speed(input.speed))
                .changed()
        };
        if let Some(unit) = context.unit {
            ui.weak(unit.0);
        }
    });

    if changed {
        if let Some(minimum) = input.minimum {
            *value = (*value).max(minimum);
        }
        if let Some(maximum) = input.maximum {
            *value = (*value).min(maximum);
        }
    }
    changed
}

fn draw_number_text(ui: &mut egui::Ui, value: f64, context: &InspectWidgetContext<'_>) {
    ui.horizontal(|ui| {
        ui.label(context.label);
        let value = format_number(value, InspectNumberFormat::default());
        ui.monospace(value.as_str());
        if let Some(unit) = context.unit {
            ui.weak(unit.0);
        }
    });
}

pub fn format_value(value: &InspectValue) -> String {
    match value {
        InspectValue::Text(value) => value.clone(),
        InspectValue::Bool(value) => (if *value { "yes" } else { "no" }).to_owned(),
        InspectValue::Integer(value) => value.to_string(),
        InspectValue::Number { value, format } => format_number(*value, *format),
        InspectValue::Quantity {
            value,
            unit,
            format,
        } => format_quantity(*value, unit.0, *format),
        InspectValue::Range {
            minimum,
            maximum,
            unit,
            format,
        } => format!(
            "{} … {}",
            format_quantity(*minimum, unit.0, *format),
            format_quantity(*maximum, unit.0, *format),
        ),
        InspectValue::Entity(entity) => format!("{entity:?}"),
        InspectValue::Vec3(value) => format!("({:.3}, {:.3}, {:.3})", value.x, value.y, value.z),
    }
}

pub fn format_quantity(value: f64, unit: &str, format: InspectNumberFormat) -> String {
    format!("{} {unit}", format_number(value, format))
}

pub fn format_number(value: f64, format: InspectNumberFormat) -> String {
    if !value.is_finite() {
        return "—".to_owned();
    }
    if value == 0.0 {
        return "0".to_owned();
    }

    let significant_digits = usize::from(format.significant_digits.clamp(1, 12));
    let magnitude = value.abs();
    let exponent = magnitude.log10().floor() as i32;

    if exponent >= 6 || exponent <= -4 {
        let engineering_exponent = exponent.div_euclid(3) * 3;
        let scaled = value / 10.0_f64.powi(engineering_exponent);
        let scaled_exponent = scaled.abs().log10().floor() as i32;
        let decimals = (significant_digits as i32 - 1 - scaled_exponent).clamp(0, 10) as usize;
        return format!(
            "{} × 10{}",
            trim_decimal_zeros(format!("{scaled:.decimals$}")),
            superscript_integer(engineering_exponent),
        );
    }

    let decimals = (significant_digits as i32 - 1 - exponent).clamp(0, 10) as usize;
    trim_decimal_zeros(format!("{value:.decimals$}"))
}

fn trim_decimal_zeros(mut value: String) -> String {
    if !value.contains('.') {
        return value;
    }
    while value.ends_with('0') {
        value.pop();
    }
    if value.ends_with('.') {
        value.pop();
    }
    value
}

fn superscript_integer(value: i32) -> String {
    let mut result = String::new();
    if value < 0 {
        result.push('⁻');
    }
    for character in value.unsigned_abs().to_string().chars() {
        result.push(match character {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => unreachable!(),
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantities_use_compact_engineering_notation() {
        assert_eq!(
            format_quantity(
                0.000_012_3,
                "m²/s",
                InspectNumberFormat::significant_digits(3),
            ),
            "12.3 × 10⁻⁶ m²/s",
        );
    }
}
