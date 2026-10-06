//! Console window and script workspace presentation.

use super::*;

pub(in crate::console) fn draw_console(
    mut contexts: Query<&mut EguiContext, With<PrimaryEguiContext>>,
    mut console: ResMut<ConsoleOverlay>,
    registry: Res<ConsoleCommandRegistry>,
    runtime_variables: Res<RuntimeVariableRegistry>,
    transport: Res<ConsoleTransport>,
    mut script_workbench: Option<ResMut<DeveloperScriptWorkbench>>,
) {
    if !console.open {
        return;
    }
    let Ok(mut context) = contexts.single_mut() else {
        return;
    };
    let ctx = context.get_mut();
    let content_rect = ctx.input(|input| input.content_rect());
    let (size, position) =
        console_window_geometry(content_rect, console.tab, console.scripts_fullscreen);

    egui::Area::new(egui::Id::new("spacetime_developer_console"))
        .order(egui::Order::Foreground)
        .fixed_pos(position)
        .default_size(size)
        .show(ctx, |ui| {
            ui.set_width(size.x);
            ui.set_height(size.y);
            egui::Frame::new()
                .fill(egui::Color32::from_rgba_unmultiplied(10, 12, 14, 248))
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    egui::Color32::from_rgb(72, 82, 92),
                ))
                .show(ui, |ui| {
                    ui.set_width(size.x);
                    ui.set_height(size.y);
                    draw_console_header(ui, &mut console);
                    ui.separator();
                    match console.tab {
                        ConsoleTab::Console => draw_command_console(
                            ui,
                            &mut console,
                            &registry,
                            &runtime_variables,
                            &transport,
                        ),
                        ConsoleTab::Scripts => {
                            if let Some(workbench) = script_workbench.as_mut() {
                                draw_script_workspace(ui, &mut *workbench);
                            } else {
                                ui.colored_label(
                                    egui::Color32::LIGHT_RED,
                                    "Script workspace resource is unavailable.",
                                );
                            }
                        }
                    }
                });
        });
}

fn console_window_geometry(
    content_rect: egui::Rect,
    tab: ConsoleTab,
    scripts_fullscreen: bool,
) -> (egui::Vec2, egui::Pos2) {
    let margin = 12.0_f32;
    let available_width = (content_rect.width() - margin * 2.0).max(320.0);
    let available_height = (content_rect.height() - margin * 2.0).max(220.0);
    match (tab, scripts_fullscreen) {
        (ConsoleTab::Scripts, true) => (content_rect.size(), content_rect.left_top()),
        (ConsoleTab::Scripts, false) => {
            let width = (content_rect.width() * 0.88)
                .clamp(760.0, 1320.0)
                .min(available_width);
            let height = (content_rect.height() * 0.78)
                .clamp(500.0, 900.0)
                .min(available_height);
            (
                egui::vec2(width, height),
                egui::pos2(
                    content_rect.center().x - width * 0.5,
                    content_rect.center().y - height * 0.5,
                ),
            )
        }
        (ConsoleTab::Console, _) => {
            let width = (content_rect.width() * 0.62)
                .clamp(620.0, 900.0)
                .min(available_width);
            let height = (content_rect.height() * 0.36)
                .clamp(240.0, 380.0)
                .min(available_height);
            (
                egui::vec2(width, height),
                egui::pos2(content_rect.left() + margin, content_rect.top() + margin),
            )
        }
    }
}

fn draw_console_header(ui: &mut egui::Ui, console: &mut ConsoleOverlay) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("SPACETIME DEV")
                .monospace()
                .strong()
                .color(egui::Color32::from_rgb(220, 224, 228)),
        );
        ui.separator();

        if ui
            .selectable_label(console.tab == ConsoleTab::Console, "Console")
            .clicked()
        {
            console.tab = ConsoleTab::Console;
            console.scripts_fullscreen = false;
            console.opened_this_frame = true;
        }
        if ui
            .selectable_label(console.tab == ConsoleTab::Scripts, "Scripts")
            .clicked()
        {
            console.tab = ConsoleTab::Scripts;
            console.opened_this_frame = false;
        }

        if console.tab == ConsoleTab::Scripts {
            ui.separator();
            let label = if console.scripts_fullscreen {
                "Windowed"
            } else {
                "Fullscreen"
            };
            if ui.button(label).clicked() {
                console.scripts_fullscreen = !console.scripts_fullscreen;
            }
        }

        ui.separator();
        ui.label(
            egui::RichText::new(if console.tab == ConsoleTab::Console {
                "` toggle   ↑/↓ history   Tab complete"
            } else {
                "host-managed Rhai workspace   Save ≠ Commit"
            })
            .monospace()
            .small()
            .color(egui::Color32::from_rgb(130, 140, 150)),
        );
    });
}
