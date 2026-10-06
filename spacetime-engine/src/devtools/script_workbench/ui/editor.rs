//! One-frame editor actions, source editing and diagnostics.

use super::super::document::ScriptTarget;
use super::*;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

/// The UI issues one document action per frame; the workbench owns all state transitions.
#[derive(Clone, Copy)]
enum EditorAction {
    Save,
    Reload,
    ResetDefault,
    Compile,
    Commit,
    Revert,
}

pub(super) fn draw_script_editor(ui: &mut egui::Ui, workbench: &mut DeveloperScriptWorkbench) {
    draw_editor_tabs(ui, workbench);
    ui.separator();

    let Some(active) = workbench.active() else {
        ui.weak("No active script.");
        return;
    };
    let target = active.target;
    if let Some(action) = draw_editor_toolbar(ui, workbench) {
        run_editor_action(workbench, action);
    }
    draw_target_contract(ui, workbench, target);
    draw_preview(ui, workbench, target);
    ui.separator();
    draw_source_editor(ui, workbench);
    ui.separator();
    draw_diagnostic(ui, workbench);
}

fn draw_editor_tabs(ui: &mut egui::Ui, workbench: &mut DeveloperScriptWorkbench) {
    let mut requested_tab = None;
    let mut requested_close = None;
    ui.horizontal_wrapped(|ui| {
        for path in &workbench.open_tabs {
            let Some(document) = workbench.documents.get(path) else {
                continue;
            };
            let basename = path.rsplit('/').next().unwrap_or(path);
            let mut label = basename.to_string();
            if document.source_dirty() {
                label.push('*');
            }
            if document.runtime_dirty() {
                label.push('∆');
            }
            if ui
                .selectable_label(workbench.active_path == *path, label)
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
                requested_close = Some(path.clone());
            }
        }
    });
    if let Some(path) = requested_tab {
        workbench.open_document(&path);
    }
    if let Some(path) = requested_close {
        workbench.close_tab(&path);
    }
}

fn draw_editor_toolbar(
    ui: &mut egui::Ui,
    workbench: &DeveloperScriptWorkbench,
) -> Option<EditorAction> {
    let active = workbench.active()?;
    let mut action = None;
    ui.horizontal_wrapped(|ui| {
        ui.monospace(&active.path);
        ui.separator();
        ui.weak(format!("target: {}", active.target.label()));
        ui.separator();
        ui.weak(format!("runtime r{}", active.revision));
        if active.source_dirty() {
            ui.colored_label(egui::Color32::YELLOW, "UNSAVED");
        }
        if active.runtime_dirty() {
            ui.colored_label(egui::Color32::LIGHT_BLUE, "UNCOMMITTED");
        }
        ui.separator();
        if ui.button("Save Live").clicked() {
            action = Some(EditorAction::Save);
        }
        if ui.button("Reload Live").clicked() {
            action = Some(EditorAction::Reload);
        }
        if ui
            .add_enabled(
                workbench.active_has_default(),
                egui::Button::new("Reset Default"),
            )
            .on_hover_text(
                "Replace the LIVE file with the shipped default; repository source is never edited",
            )
            .clicked()
        {
            action = Some(EditorAction::ResetDefault);
        }
        if ui.button("Compile").clicked() {
            action = Some(EditorAction::Compile);
        }
        if ui.button("Commit").clicked() {
            action = Some(EditorAction::Commit);
        }
        if ui.button("Revert Runtime").clicked() {
            action = Some(EditorAction::Revert);
        }
    });
    action
}

fn run_editor_action(workbench: &mut DeveloperScriptWorkbench, action: EditorAction) {
    let result = match action {
        EditorAction::Save => workbench.save_active(),
        EditorAction::Reload => {
            workbench.reload_active_from_saved();
            Ok(())
        }
        EditorAction::ResetDefault => workbench.reset_active_to_default(),
        EditorAction::Compile => workbench.compile_active().map(|_| ()),
        EditorAction::Commit => workbench.commit_active().map(|_| ()),
        EditorAction::Revert => {
            workbench.revert_active_to_committed();
            Ok(())
        }
    };
    if let Err(error) = result
        && let Some(document) = workbench.active_mut()
    {
        document.diagnostic = error;
    }
}

fn draw_target_contract(
    ui: &mut egui::Ui,
    workbench: &mut DeveloperScriptWorkbench,
    target: ScriptTarget,
) {
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
}

fn draw_preview(ui: &mut egui::Ui, workbench: &mut DeveloperScriptWorkbench, target: ScriptTarget) {
    if !target.supports_live() {
        return;
    }
    let mut changed = false;
    if let Some(document) = workbench.active_mut() {
        ui.horizontal(|ui| {
            ui.label("Preview input");
            changed = ui
                .add(egui::DragValue::new(&mut document.preview_input).speed(0.1))
                .changed();
            ui.label(
                document
                    .preview_output
                    .map_or_else(|| "→ —".to_string(), |value| format!("→ {value:.6}")),
            );
        });
    }
    if changed {
        let _ = workbench.compile_active();
    }
}

fn draw_source_editor(ui: &mut egui::Ui, workbench: &mut DeveloperScriptWorkbench) {
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
}

fn draw_diagnostic(ui: &mut egui::Ui, workbench: &DeveloperScriptWorkbench) {
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
