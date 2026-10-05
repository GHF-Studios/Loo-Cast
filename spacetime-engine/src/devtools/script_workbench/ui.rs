//! Script explorer and editor surface embedded in the console.

use super::DeveloperScriptWorkbench;
use std::collections::BTreeMap;
use bevy_egui::egui;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

/// Starfall/E2-style script workspace embedded by the developer-console shell.
pub(crate) fn draw_script_workspace(ui: &mut egui::Ui, workbench: &mut DeveloperScriptWorkbench) {
    let available_height = ui.available_height().max(240.0);
    let explorer_width = (ui.available_width() * 0.22).clamp(170.0, 260.0);

    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(explorer_width, available_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| draw_script_explorer(ui, workbench),
        );
        ui.separator();
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), available_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| draw_script_editor(ui, workbench),
        );
    });
}

fn draw_script_explorer(ui: &mut egui::Ui, workbench: &mut DeveloperScriptWorkbench) {
    ui.horizontal(|ui| {
        ui.strong("Scripts");
        if ui
            .small_button("+")
            .on_hover_text("New scratch script")
            .clicked()
        {
            workbench.new_scratch();
        }
    });

    ui.small(
        egui::RichText::new(format!("LIVE · {}", workbench.roots.live_root.display()))
            .monospace()
            .weak(),
    );
    ui.small(
        egui::RichText::new(format!(
            "DEFAULTS · {}",
            workbench.roots.defaults_root.display()
        ))
        .monospace()
        .weak(),
    );
    if let Some(warning) = workbench.bootstrap_warning.as_ref() {
        ui.colored_label(egui::Color32::LIGHT_RED, warning);
    }
    ui.separator();

    let mut grouped = BTreeMap::<String, Vec<String>>::new();
    for path in workbench.documents.keys() {
        let (folder, _) = path.split_once('/').unwrap_or((".", path.as_str()));
        grouped
            .entry(folder.to_string())
            .or_default()
            .push(path.clone());
    }

    let mut requested_open = None::<String>;
    egui::ScrollArea::vertical()
        .id_salt("developer_script_explorer")
        .show(ui, |ui| {
            for (folder, paths) in grouped {
                egui::CollapsingHeader::new(folder)
                    .default_open(true)
                    .show(ui, |ui| {
                        for path in paths {
                            let document = &workbench.documents[&path];
                            let basename = path.rsplit('/').next().unwrap_or(&path);
                            let mut label = basename.to_string();
                            if document.source_dirty() {
                                label.push('*');
                            }
                            if document.runtime_dirty() {
                                label.push('∆');
                            }
                            let selected = workbench.active_path == path;
                            let response = ui.selectable_label(selected, label);
                            if response.clicked() {
                                requested_open = Some(path.clone());
                            }
                            response.on_hover_text(format!(
                                "{}\n{}\n{}",
                                path,
                                document.target.label(),
                                document.target.contract(),
                            ));
                        }
                    });
            }
        });

    if let Some(path) = requested_open {
        workbench.open_document(&path);
    }
}

fn draw_script_editor(ui: &mut egui::Ui, workbench: &mut DeveloperScriptWorkbench) {
    let tabs = workbench.open_tabs.clone();
    let mut requested_tab = None::<String>;
    let mut requested_close = None::<String>;

    ui.horizontal_wrapped(|ui| {
        for path in tabs {
            let Some(document) = workbench.documents.get(&path) else {
                continue;
            };
            let basename = path.rsplit('/').next().unwrap_or(&path);
            let mut label = basename.to_string();
            if document.source_dirty() {
                label.push('*');
            }
            if document.runtime_dirty() {
                label.push('∆');
            }
            if ui
                .selectable_label(workbench.active_path == path, label)
                .clicked()
            {
                requested_tab = Some(path.clone());
            }
            if workbench.open_tabs.len() > 1
                && ui
                    .small_button("×")
                    .on_hover_text(format!("Close {path}"))
                    .clicked()
            {
                requested_close = Some(path);
            }
        }
    });

    if let Some(path) = requested_tab {
        workbench.open_document(&path);
    }
    if let Some(path) = requested_close {
        workbench.close_tab(&path);
    }

    ui.separator();

    let Some(active) = workbench.active() else {
        ui.weak("No active script.");
        return;
    };
    let target = active.target;
    let active_path = active.path.clone();
    let source_dirty = active.source_dirty();
    let runtime_dirty = active.runtime_dirty();
    let revision = active.revision;

    let mut do_save = false;
    let mut do_reload_saved = false;
    let mut do_reset_default = false;
    let mut do_compile = false;
    let mut do_commit = false;
    let mut do_revert = false;

    ui.horizontal_wrapped(|ui| {
        ui.monospace(&active_path);
        ui.separator();
        ui.weak(format!("target: {}", target.label()));
        ui.separator();
        ui.weak(format!("runtime r{revision}"));
        if source_dirty {
            ui.colored_label(egui::Color32::YELLOW, "UNSAVED");
        }
        if runtime_dirty {
            ui.colored_label(egui::Color32::LIGHT_BLUE, "UNCOMMITTED");
        }

        ui.separator();
        do_save = ui.button("Save Live").clicked();
        do_reload_saved = ui.button("Reload Live").clicked();
        do_reset_default = ui
            .add_enabled(
                workbench.active_has_default(),
                egui::Button::new("Reset Default"),
            )
            .on_hover_text(
                "Replace the LIVE file with the shipped default; repository source is never edited",
            )
            .clicked();
        do_compile = ui.button("Compile").clicked();
        do_commit = ui.button("Commit").clicked();
        do_revert = ui.button("Revert Runtime").clicked();
    });

    if target.supports_live() {
        if let Some(document) = workbench.active_mut() {
            ui.horizontal(|ui| {
                ui.checkbox(&mut document.live_enabled, "Live");
                ui.weak(document.target.contract());
            });
        }
    } else {
        ui.weak(target.contract());
    }

    egui::CollapsingHeader::new("Script API")
        .default_open(false)
        .show(ui, |ui| {
            ui.monospace(target.api_help());
        });

    if do_save {
        if let Err(error) = workbench.save_active()
            && let Some(document) = workbench.active_mut()
        {
            document.diagnostic = error;
        }
    }
    if do_reload_saved {
        workbench.reload_active_from_saved();
    }
    if do_reset_default {
        if let Err(error) = workbench.reset_active_to_default()
            && let Some(document) = workbench.active_mut()
        {
            document.diagnostic = error;
        }
    }
    if do_compile {
        let _ = workbench.compile_active();
    }
    if do_commit {
        let _ = workbench.commit_active();
    }
    if do_revert {
        workbench.revert_active_to_committed();
    }

    if target.supports_live() {
        let mut preview_changed = false;
        if let Some(document) = workbench.active_mut() {
            ui.horizontal(|ui| {
                ui.label("Preview input");
                preview_changed = ui
                    .add(egui::DragValue::new(&mut document.preview_input).speed(0.1))
                    .changed();
                ui.label(
                    document
                        .preview_output
                        .map_or_else(|| "→ —".to_string(), |value| format!("→ {value:.6}")),
                );
            });
        }
        if preview_changed {
            let _ = workbench.compile_active();
        }
    }

    ui.separator();

    let rows = ((ui.available_height() - 86.0) / 17.0)
        .floor()
        .clamp(8.0, 64.0) as usize;
    if let Some(document) = workbench.active_mut() {
        CodeEditor::default()
            .id_source(format!("developer_script_editor:{}", document.path))
            .with_rows(rows)
            .with_fontsize(14.0)
            .with_theme(ColorTheme::GRUVBOX)
            .with_syntax(Syntax::rust())
            .with_numlines(true)
            .vscroll(true)
            .show(ui, &mut document.source);
    }

    ui.separator();
    if let Some(document) = workbench.active() {
        let diagnostic = &document.diagnostic;
        let lower = diagnostic.to_ascii_lowercase();
        let status_color = if lower.contains("error") || lower.contains("failed") {
            egui::Color32::LIGHT_RED
        } else {
            egui::Color32::LIGHT_GREEN
        };
        ui.horizontal(|ui| {
            ui.strong("Diagnostics");
            ui.colored_label(status_color, diagnostic);
        });
    }
}
