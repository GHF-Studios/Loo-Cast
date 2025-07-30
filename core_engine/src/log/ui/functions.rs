use bevy_egui::egui::{self, Color32, ScrollArea, TextFormat, FontId};
use egui::{text::LayoutJob, WidgetText};

use crate::log::{resources::LogRegistry, types::{LogLevel::*, *}, ui::{resources::LogViewerState, types::SelectionMode}};
use crate::ui::custom_egui_widgets::tri_checkbox::TriCheckboxExt;

// === Basics ===

pub fn render_selection_tree_toolbar(ui: &mut egui::Ui, log_registry: &mut LogRegistry) {
    egui::Frame::none()
        .fill(Color32::from_gray(25))
        .stroke(egui::Stroke::new(1.0, Color32::DARK_GRAY))
        .inner_margin(egui::Margin::symmetric(6.0, 4.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let selection_mode = &mut log_registry.selection_mode;

                ui.label("Mode:");

                let selected = matches!(selection_mode, SelectionMode::Span);
                if ui.selectable_label(selected, "↔ Span").clicked() {
                    *selection_mode = SelectionMode::Span;
                }

                let selected = matches!(selection_mode, SelectionMode::Module);
                if ui.selectable_label(selected, "📦 Module").clicked() {
                    *selection_mode = SelectionMode::Module;
                }

                let selected = matches!(selection_mode, SelectionMode::Physical);
                if ui.selectable_label(selected, "📂 Physical").clicked() {
                    *selection_mode = SelectionMode::Physical;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Tools:");

                if ui.button("ToggleAll").clicked() {
                    match log_registry.selection_mode {
                        SelectionMode::Span => log_registry.span_selections.span_roots.values_mut().for_each(|sel| sel.toggle_selection()),
                        SelectionMode::Module => log_registry.module_selections.crates.values_mut().for_each(|sel| sel.toggle_selection()),
                        SelectionMode::Physical => log_registry.physical_selections.crates.values_mut().for_each(|sel| sel.toggle_selection()),
                    };
                }
            });
        });
}

pub fn render_selection_tree(
    ui: &mut egui::Ui,
    log_registry: &mut LogRegistry
) {
    let selection_mode = log_registry.selection_mode;
    let height = ui.available_height();
    let width = ui.available_width();

    ScrollArea::vertical()
        .max_height(height)
        .max_width(width)
        .show(ui, |ui| {
            ui.label("TOP OF SELECTION TREE");

            match selection_mode {
                SelectionMode::Span => {
                    render_span_tree(ui, &mut log_registry.span_registry, &mut log_registry.span_selections);
                }
                SelectionMode::Module => {
                    render_module_tree(ui, &mut log_registry.module_registry, &mut log_registry.module_selections);
                }
                SelectionMode::Physical => {
                    render_physical_tree(ui, &mut log_registry.physical_registry, &mut log_registry.physical_selections);
                }
            }

            ui.label("BOTTOM OF SELECTION TREE");
        }
    );
}

pub fn render_console_toolbar(ui: &mut egui::Ui, log_viewer_state: &mut LogViewerState) {
    egui::Frame::none()
        .fill(Color32::from_gray(25))
        .stroke(egui::Stroke::new(1.0, Color32::DARK_GRAY))
        .inner_margin(egui::Margin::symmetric(6.0, 4.0))
        .show(ui, |ui| {
            let all_levels = [Error, Warn, Info, Debug, Trace];
            let level_symbols = ["E", "W", "I", "D", "T"];
            let level_colors = [
                Color32::RED,
                Color32::YELLOW,
                Color32::GREEN,
                Color32::LIGHT_BLUE,
                Color32::KHAKI,
            ];

            let threshold = &mut log_viewer_state.threshold;
            let level_index = all_levels
                .iter()
                .position(|l| l == threshold)
                .unwrap_or(2);
            let mut slider_value = level_index as f32;

            ui.horizontal(|ui| {
                ui.label("Log Level:");
            
                let slider = egui::Slider::new(&mut slider_value, 0.0..=4.0)
                    .step_by(1.0)
                    .show_value(false);
                let response = ui.add(slider);
            
                let track_rect = response.rect.shrink(6.0);
                let norm = (slider_value / 4.0).clamp(0.0, 1.0);
                let x = track_rect.left() + norm * track_rect.width();
                let y = track_rect.center().y;
                let center = egui::pos2(x, y);
            
                let idx = slider_value.round().clamp(0.0, 4.0) as usize;
                let symbol = level_symbols[idx];
                let color = level_colors[idx];
            
                ui.painter().text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    symbol,
                    egui::TextStyle::Button.resolve(ui.style()),
                    color,
                );
            
                *threshold = all_levels[idx];
            });
        });
}

pub fn render_console(
    ui: &mut egui::Ui,
    log_viewer_state: &LogViewerState,
    log_registry: &mut LogRegistry
) {
    let logs = gather_logs(log_viewer_state, log_registry);
    let row_h = ui.text_style_height(&egui::TextStyle::Monospace);
    let height = ui.available_height();
    let width = ui.available_width();

    ScrollArea::vertical()
        .max_height(height)
        .max_width(width)
        .stick_to_bottom(true)
        .show_rows(ui, row_h, logs.len(), |ui, range| {
            for i in range {
                ui.label(format_log(&logs[i]));
            }
        });
}

// === Utilities ===

pub fn gather_logs(
    state: &LogViewerState,
    registry: &mut LogRegistry,
) -> Vec<LogEntry> {
    let mut out = Vec::new();

    match registry.selection_mode {
        SelectionMode::Span => {
            let log_ids = registry.collect_logs_from_span_selections();

            for log_id in log_ids {
                if let Some(entry) = registry.get_log(&log_id) {
                    out.push(entry.clone());
                }
            }
        }
        SelectionMode::Module => {
            let log_ids = registry.collect_logs_from_module_selections();
            
            for log_id in log_ids {
                if let Some(entry) = registry.get_log(&log_id) {
                    out.push(entry.clone());
                }
            }
        }
        SelectionMode::Physical => {
            let log_ids = registry.collect_logs_from_physical_selections();

            for log_id in log_ids {
                if let Some(entry) = registry.get_log(&log_id) {
                    out.push(entry.clone());
                }
            }
        }
    }

    out.retain(|log| log.lvl >= state.threshold);
    out.sort_by_key(|log| log.ts);
    out
}

pub(super) fn format_log(log: &LogEntry) -> WidgetText {
    use crate::log::types::LogLevel;

    let ns = log.ts;
    let ms = ns / 1_000_000;
    let secs = ms / 1000 % 60;
    let mins = ms / 1000 / 60 % 60;
    let hrs  = ms / 1000 / 60 / 60 % 24;
    let days = ms / 1000 / 60 / 60 / 24;
    let sub_ms = ms % 1000;

    let time = if days > 0 {
        format!("T+{}d:{:02}h:{:02}m:{:02}s.{:03}ms", days, hrs, mins, secs, sub_ms)
    } else if hrs > 0 {
        format!("T+{:02}h:{:02}m:{:02}s.{:03}ms", hrs, mins, secs, sub_ms)
    } else {
        format!("T+{:02}m:{:02}s.{:03}ms", mins, secs, sub_ms)
    };

    let (level_str, level_color) = match log.lvl {
        LogLevel::Error => ("[ERROR]", Color32::RED),
        LogLevel::Warn  => ("[WARN]",  Color32::YELLOW),
        LogLevel::Info  => ("[INFO]",  Color32::LIGHT_GREEN),
        LogLevel::Debug => ("[DEBUG]", Color32::LIGHT_BLUE),
        LogLevel::Trace => ("[TRACE]", Color32::KHAKI),
    };

    let mut job = LayoutJob::default();

    job.append(
        &format!("[{time}]"),
        0.0,
        TextFormat {
            font_id: FontId::monospace(11.0),
            color: Color32::GRAY,
            ..Default::default()
        },
    );

    job.append(
        level_str,
        0.0,
        TextFormat {
            font_id: FontId::monospace(11.0),
            color: level_color,
            ..Default::default()
        },
    );

    job.append(
        &format!(" {}", log.msg),
        0.0,
        TextFormat {
            font_id: FontId::monospace(11.0),
            color: Color32::WHITE,
            ..Default::default()
        },
    );

    WidgetText::from(job)
}

// === Tree Rendering ===

pub fn render_span_tree(
    ui: &mut egui::Ui,
    span_registry: &mut crate::log::types::SpanRegistry,
    span_selections: &mut crate::log::types::SpanPathSelections
) {
    for (root_seg, root_sel) in &mut span_selections.span_roots {
        if let Some(root_node) = span_registry.span_roots.get_mut(root_seg) {
            render_span_branch(ui, root_seg, root_sel, root_node);
        }
    }
}

pub fn render_module_tree(
    ui: &mut egui::Ui,
    module_registry: &mut crate::log::types::ModuleRegistry,
    module_selections: &mut crate::log::types::ModulePathSelections
) {
    ScrollArea::vertical().show(ui, |ui| {
        for (crate_seg, crate_sel) in &mut module_selections.crates {
            if let Some(crate_node) = module_registry.crates.get_mut(crate_seg) {
                render_crate_module_branch(ui, crate_seg, crate_sel, crate_node);
            }
        }
    });
}

pub fn render_physical_tree(
    ui: &mut egui::Ui,
    physical_registry: &mut crate::log::types::PhysicalRegistry,
    physical_selections: &mut crate::log::types::PhysicalPathSelections
) {
    ScrollArea::vertical().show(ui, |ui| {
        for (crate_seg, crate_sel) in &mut physical_selections.crates {
            if let Some(crate_node) = physical_registry.crates.get_mut(crate_seg) {
                render_crate_folder_branch(ui, crate_seg, crate_sel, crate_node);
            }
        }
    });
}

// === Node Rendering ===

// --- Span ---

fn render_span_branch(
    ui: &mut egui::Ui,
    seg: &SpanSegment,
    sel: &mut SpanNodeSelection,
    node: &mut SpanNode,
) {
    let label = seg.name.as_str();
    let mut checked = sel.metadata.explicit_selection_state.consolidate(sel.is_partial()).into();

    ui.spacing_mut().item_spacing.x = 0.0;
    ui.horizontal(|ui| {
        if ui.tri_checkbox(&mut checked).changed() {
            sel.toggle_selection();
        }
        if sel.span_children.is_empty() {
            ui.label(label);
        } else {
            ui.collapsing(label, |ui| {
                ui.vertical(|ui| {
                    for (child_seg, child_sel) in &mut sel.span_children {
                        if let Some(child_node) = node.span_children.get_mut(child_seg) {
                            render_span_branch(ui, child_seg, child_sel, child_node);
                        }
                    }
                });
            });
        }
    });
}

// --- Module ---

fn render_crate_module_branch(
    ui: &mut egui::Ui,
    seg: &CrateModuleSegment,
    sel: &mut CrateModuleNodeSelection,
    node: &mut CrateModuleNode,
) {
    let label = seg.name.as_str();
    let mut checked = sel.metadata.explicit_selection_state.consolidate(sel.is_partial()).into();

    ui.spacing_mut().item_spacing.x = 0.0;
    ui.horizontal(|ui| {
        if ui.tri_checkbox(&mut checked).changed() {
            sel.toggle_selection();
        }
        if sel.modules.is_empty() {
            ui.label(label);
        } else {
            ui.collapsing(label, |ui| {
                ui.vertical(|ui| {
                    for (mod_seg, mod_sel) in &mut sel.modules {
                        if let Some(mod_node) = node.modules.get_mut(mod_seg) {
                            render_module_branch(ui, mod_seg, mod_sel, mod_node);
                        }
                    }
                });
            });
        }
    });
}

fn render_module_branch(
    ui: &mut egui::Ui,
    seg: &ModuleSegment,
    sel: &mut ModuleNodeSelection,
    node: &mut ModuleNode,
) {
    let label = seg.name.as_str();
    let mut checked = sel.metadata.explicit_selection_state.consolidate(sel.is_partial()).into();

    ui.spacing_mut().item_spacing.x = 0.0;
    ui.horizontal(|ui| {
        if ui.tri_checkbox(&mut checked).changed() {
            sel.toggle_selection();
        }
        if sel.modules.is_empty() && sel.sub_modules.is_empty() {
            ui.label(label);
        } else {
            ui.collapsing(label, |ui| {
                ui.vertical(|ui| {
                    for (mod_seg, mod_sel) in &mut sel.modules {
                        if let Some(mod_node) = node.modules.get_mut(mod_seg) {
                            render_module_branch(ui, mod_seg, mod_sel, mod_node);
                        }
                    }
                
                    for (sub_seg, sub_sel) in &mut sel.sub_modules {
                        if let Some(sub_node) = node.sub_modules.get_mut(sub_seg) {
                            render_submodule_branch(ui, sub_seg, sub_sel, sub_node);
                        }
                    }
                });
            });
        }
    });
}

fn render_submodule_branch(
    ui: &mut egui::Ui,
    seg: &SubModuleSegment,
    sel: &mut SubModuleNodeSelection,
    node: &mut SubModuleNode,
) {
    let label = seg.name.as_str();
    let mut checked = sel.metadata.explicit_selection_state.consolidate(sel.is_partial()).into();

    ui.spacing_mut().item_spacing.x = 0.0;
    ui.horizontal(|ui| {
        if ui.tri_checkbox(&mut checked).changed() {
            sel.toggle_selection();
        }
        if sel.sub_modules.is_empty() {
            ui.label(label);
        } else {
            ui.collapsing(label, |ui| {
                ui.vertical(|ui| {
                    for (sub_seg, sub_sel) in &mut sel.sub_modules {
                        if let Some(sub_node) = node.sub_modules.get_mut(sub_seg) {
                            render_submodule_branch(ui, sub_seg, sub_sel, sub_node);
                        }
                    }
                });
            });
        }
    });
}

// --- Physical ---

fn render_crate_folder_branch(
    ui: &mut egui::Ui,
    seg: &CrateFolderSegment,
    sel: &mut CrateFolderNodeSelection,
    node: &mut CrateFolderNode,
) {
    let label = seg.name.as_str();
    let mut checked = sel.metadata.explicit_selection_state.consolidate(sel.is_partial()).into();

    ui.spacing_mut().item_spacing.x = 0.0;
    ui.horizontal(|ui| {
        if ui.tri_checkbox(&mut checked).changed() {
            sel.toggle_selection();
        }
        if sel.folders.is_empty() && sel.files.is_empty() {
            ui.label(label);
        } else {
            ui.collapsing(label, |ui| {
                ui.vertical(|ui| {
                    for (folder_seg, folder_sel) in &mut sel.folders {
                        if let Some(folder_node) = node.folders.get_mut(folder_seg) {
                            render_folder_branch(ui, folder_seg, folder_sel, folder_node);
                        }
                    }
                
                    for (file_seg, file_sel) in &mut sel.files {
                        if let Some(file_node) = node.files.get_mut(file_seg) {
                            render_file_branch(ui, file_seg, file_sel, file_node);
                        }
                    }
                });
            });
        }
    });
}

fn render_folder_branch(
    ui: &mut egui::Ui,
    seg: &FolderSegment,
    sel: &mut FolderNodeSelection,
    node: &mut FolderNode,
) {
    let label = seg.name.as_str();
    let mut checked = sel.metadata.explicit_selection_state.consolidate(sel.is_partial()).into();

    ui.spacing_mut().item_spacing.x = 0.0;
    ui.horizontal(|ui| {
        if ui.tri_checkbox(&mut checked).changed() {
            sel.toggle_selection();
        }
        if sel.folders.is_empty() && sel.files.is_empty() {
            ui.label(label);
        } else {
            ui.collapsing(label, |ui| {
                ui.vertical(|ui| {
                    for (folder_seg, folder_sel) in &mut sel.folders {
                        if let Some(folder_node) = node.folders.get_mut(folder_seg) {
                            render_folder_branch(ui, folder_seg, folder_sel, folder_node);
                        }
                    }
                
                    for (file_seg, file_sel) in &mut sel.files {
                        if let Some(file_node) = node.files.get_mut(file_seg) {
                            render_file_branch(ui, file_seg, file_sel, file_node);
                        }
                    }
                });
            });
        }
    });
}

fn render_file_branch(
    ui: &mut egui::Ui,
    seg: &FileSegment,
    sel: &mut FileNodeSelection,
    node: &mut FileNode,
) {
    let label = seg.name.as_str();
    let mut checked = sel.metadata.explicit_selection_state.consolidate(sel.is_partial()).into();

    ui.spacing_mut().item_spacing.x = 0.0;
    ui.horizontal(|ui| {
        if ui.tri_checkbox(&mut checked).changed() {
            sel.toggle_selection();
        }
        if sel.lines.is_empty() {
            ui.label(label);
        } else {
            ui.collapsing(label, |ui| {
                ui.vertical(|ui| {
                    for (line_seg, line_sel) in &mut sel.lines {
                        if let Some(line_node) = node.lines.get_mut(line_seg) {
                            render_line_leaf(ui, line_seg, line_sel, line_node);
                        }
                    }
                });
            });
        }
    });
}

fn render_line_leaf(
    ui: &mut egui::Ui,
    seg: &LineSegment,
    sel: &mut LineNodeSelection,
    _node: &mut LineNode,
) {
    let label = seg.number.to_string();
    let mut checked = sel.metadata.explicit_selection_state.consolidate(sel.is_partial()).into();

    ui.spacing_mut().item_spacing.x = 0.0;
    ui.horizontal(|ui| {
        if ui.tri_checkbox(&mut checked).changed() {
            sel.toggle_selection();
        }
        ui.label(label);
    });
}
