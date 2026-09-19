//! Generic semantic inspection-section UI and request emission.

use super::*;

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
