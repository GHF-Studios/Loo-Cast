//! Inspector control for gizmo transform-space selection.

use super::*;

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct EditorTransformSpaceWidget;

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
