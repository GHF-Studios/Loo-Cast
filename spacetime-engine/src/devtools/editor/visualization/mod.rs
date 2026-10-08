//! Developer visualization controls.

use super::*;

pub(super) fn draw_visualizations(ui: &mut egui::Ui, world: &mut World) {
    ui.heading("Developer visualizations");
    ui.add_space(4.0);

    let mut tools = world.resource_mut::<DeveloperTools>();
    let mut enabled = tools.enabled();
    if ui
        .checkbox(&mut enabled, "Master developer output")
        .changed()
    {
        tools.set_enabled(enabled);
    }

    ui.separator();

    let mut specs = tools.visualizations().copied().collect::<Vec<_>>();
    specs.sort_by_key(|spec| (spec.order, spec.label));

    for spec in specs {
        let mut selected = tools.visualization_selected(spec.id);
        if ui.checkbox(&mut selected, spec.label).changed() {
            tools.set_visualization_enabled(spec.id, selected);
        }
    }
}
