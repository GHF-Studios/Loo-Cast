//! Editor-shell toolbar and Transform-gizmo controls.

use super::*;

#[expect(
    deprecated,
    reason = "egui 0.34 deprecated root Context panel entry points before bevy_egui exposes a root Ui"
)]
pub(super) fn draw_toolbar(ctx: &egui::Context, world: &mut World) {
    egui::Panel::top("spacetime_editor_toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.strong("SPACETIME");
            ui.separator();
            ui.label("F2  return to game");

            ui.separator();
            let mut enabled = world.resource::<DeveloperTools>().enabled();
            if ui.checkbox(&mut enabled, "Developer output").changed() {
                world.resource_mut::<DeveloperTools>().set_enabled(enabled);
            }

            ui.separator();
            draw_transform_space_control(ui, world);

            ui.separator();
            ui.weak("Unified Transform gizmo: translate + rotate + scale. Esc cancels a drag or toggles gameplay capture.");
        });
    });
}

pub(in crate::devtools::editor) fn draw_transform_space_control(ui: &mut egui::Ui, world: &mut World) {
    ui.label("Transform");
    world.resource_scope(|world, mut settings: Mut<EditorTransformGizmoSettings>| {
        let registration = world
            .resource::<InspectTypeRegistry>()
            .get::<EditorTransformGizmoSettings>()
            .expect("Transform gizmo settings must be registered for inspection");
        let widgets = world.resource::<InspectorWidgetRegistry>();
        let _ = inspect_ui::edit_registered_inspectable(ui, registration, &mut *settings, widgets);
    });
}
