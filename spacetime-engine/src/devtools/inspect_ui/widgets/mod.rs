//! Built-in inspector widgets for common Bevy/Rust value types.

use super::*;

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
