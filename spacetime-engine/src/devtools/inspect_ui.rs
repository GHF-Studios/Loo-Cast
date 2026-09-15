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
    FocusTarget, InspectActionRequest, InspectEditRequest, InspectField, InspectNumberFormat,
    InspectNumberInput, InspectSection, InspectUnit, InspectValue, InspectionFrame, StructureItemId,
};

pub struct InspectWidgetContext<'a> {
    pub label: &'a str,
    pub unit: Option<InspectUnit>,
    pub hint: Option<&'a str>,
    pub number_input: Option<InspectNumberInput>,
}

impl<'a> InspectWidgetContext<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            unit: None,
            hint: None,
            number_input: None,
        }
    }
}

/// First-class advanced path for bespoke value presentation.
///
/// Implementations never receive a `read_only` flag. A read-only host calls
/// `show`; a host with actual authority may call `edit` and provide mutable data.
pub trait InspectorWidget<T: ?Sized> {
    fn show(&self, ui: &mut egui::Ui, value: &T, context: &InspectWidgetContext<'_>);
    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut T,
        context: &InspectWidgetContext<'_>,
    ) -> bool;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct NumberWidget;

impl InspectorWidget<f32> for NumberWidget {
    fn show(&self, ui: &mut egui::Ui, value: &f32, context: &InspectWidgetContext<'_>) {
        draw_number_text(ui, *value as f64, context);
    }

    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut f32,
        context: &InspectWidgetContext<'_>,
    ) -> bool {
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

    fn edit(
        &self,
        ui: &mut egui::Ui,
        value: &mut f64,
        context: &InspectWidgetContext<'_>,
    ) -> bool {
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
        Vec3Widget.show(ui, &value.translation, &InspectWidgetContext::new("Position"));
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
                number_input: field.number_input,
            },
        ),
        InspectValue::Integer(value) => ui.horizontal(|ui| {
            ui.label(label);
            ui.add(egui::DragValue::new(value)).changed()
        }).inner,
        InspectValue::Number { value, .. } => NumberWidget.edit(
            ui,
            value,
            &InspectWidgetContext {
                label,
                unit: None,
                hint: field.hint.as_deref(),
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
        changed = ui
            .add(egui::DragValue::new(value).speed(input.speed))
            .changed();
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
