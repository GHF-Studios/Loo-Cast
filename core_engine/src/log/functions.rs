use bevy_egui::egui::{self, Color32, RichText, ScrollArea, TextFormat, FontId};
use egui::{text::LayoutJob, WidgetText};

use crate::log::arena::{Arena, TreeKind, LocKind, Level, NodeIdx, Log};
use crate::log::resources::*;
use crate::log::types::LogPathSegment;
use tracing::Metadata;

use std::path::{Component, Path, PathBuf};

pub fn resolve_log_location_path(meta: &Metadata<'_>) -> Vec<LogPathSegment> {
    let file_path = meta.file().expect("Missing file path in log metadata");
    let module_path = meta.module_path().expect("Missing module path in log metadata");
    let line = meta.line().expect("Missing line number in log metadata");

    let module_segments: Vec<&str> = module_path.split("::").collect();
    let crate_name = module_segments.first().expect("Empty module path");

    let mut path = Vec::new();
    path.push(LogPathSegment::Crate(crate_name.to_string()));
    for seg in &module_segments[1..] {
        path.push(LogPathSegment::Module(seg.to_string()));
    }

    let normalized = file_path.replace('\\', "/");
    let file_name = normalized
        .split('/')
        .last()
        .expect("Failed to extract file name from path");
    path.push(LogPathSegment::File(file_name.to_string()));

    path.push(LogPathSegment::Line(line));

    path
}

pub fn render_log_tree(ui: &mut egui::Ui, arena: &Arena) {
    ScrollArea::vertical().show(ui, |ui| {
        for root in arena.roots() {
            render_node(ui, arena, root, 0);
        }
    });
}

fn render_node(ui: &mut egui::Ui, arena: &Arena, idx: NodeIdx, depth: usize) {
    let indent = " ".repeat(depth);
    match arena.kind(idx) {
        TreeKind::Span => {
            let name = arena.tok_str(arena.name_tok(idx));
            ui.collapsing(format!("{indent}↔{}", name), |ui| {
                for child in arena.child_iter(idx) {
                    render_node(ui, arena, child, depth + 1);
                }
            });
        }
        TreeKind::Loc(loc_kind) => match loc_kind {
            LocKind::Crate => {
                let name = arena.tok_str(arena.name_tok(idx));
                ui.collapsing(format!("{indent}📦{}", name), |ui| {
                    for child in arena.child_iter(idx) {
                        render_node(ui, arena, child, depth + 1);
                    }
                });
            }
            LocKind::Module => {
                let name = arena.tok_str(arena.name_tok(idx));
                ui.collapsing(format!("{indent}📂{}", name), |ui| {
                    for child in arena.child_iter(idx) {
                        render_node(ui, arena, child, depth + 1);
                    }
                });
            }
            LocKind::File => {
                let name = arena.tok_str(arena.name_tok(idx));
                ui.collapsing(format!("{indent}📄{}", name), |ui| {
                    for child in arena.child_iter(idx) {
                        render_node(ui, arena, child, depth + 1);
                    }
                });
            }
            LocKind::Line => {
                let (l, c) = arena.line_col(idx).unwrap_or((0, 0));
                ui.collapsing(format!("{indent}📑{l}:{c}"), |ui| {
                    ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                        for log in arena.logs(idx).iter() {
                            ui.label(format_log_line(log));
                        }
                    });
                });
            }
            LocKind::SubModule => {
                let name = arena.tok_str(arena.name_tok(idx));
                ui.collapsing(format!("{indent}📂{}", name), |ui| {
                    for child in arena.child_iter(idx) {
                        render_node(ui, arena, child, depth + 1);
                    }
                });
            }
        },
    }
}

/// Entrypoint to render the correct tree UI based on mode.
pub fn render_selectable_tree(
    ui:    &mut egui::Ui,
    arena: &Arena,
    state: &mut LogViewerState,
) {
    for root in arena.roots() {
        if arena.kind(root) == state.tree_mode {
            paint_branch(ui, arena, state, root);
        }
    }
}

/// Render one node and recurse into its children.
fn paint_branch(
    ui:    &mut egui::Ui,
    arena: &Arena,
    state: &mut LogViewerState,
    idx:   NodeIdx,
) {
    ui.indent(idx, |ui| {
        ui.horizontal(|ui| {
            let mut checked = state.selected.contains(&idx);
            if ui.checkbox(&mut checked, "").changed() {
                if checked { state.selected.insert(idx); }
                else       { state.selected.remove(&idx); }
            }

            let (icon, label) = match arena.kind(idx) {
                TreeKind::Span => ("↔", arena.tok_str(arena.name_tok(idx))),
                TreeKind::Loc(loc_kind) => match loc_kind {
                    LocKind::Crate  => ("📦", arena.tok_str(arena.name_tok(idx))),
                    LocKind::Module => ("📂", arena.tok_str(arena.name_tok(idx))),
                    LocKind::File   => ("📄", arena.tok_str(arena.name_tok(idx))),
                    LocKind::Line => {
                        let (l, c) = arena.line_col(idx).unwrap_or((0, 0));
                        ("📑", format!("{l}:{c}").into())
                    }
                    LocKind::SubModule => ("📂", arena.tok_str(arena.name_tok(idx))),
                },
            };

            ui.collapsing(format!("{icon} {label}"), |ui| {
                for child in arena.child_iter(idx) {
                    paint_branch(ui, arena, state, child);
                }
            });
        });
    });
}

/// Recursively collect all logs from selected nodes.
pub fn gather_logs(arena: &Arena, state: &LogViewerState) -> Vec<Log> {
    let mut out = Vec::new();
    for &node in &state.selected {
        collect_logs(arena, node, &mut out);
    }
    out.retain(|log| log.lvl >= state.threshold);
    out.sort_by_key(|l| l.ts);
    out
}

fn collect_logs(arena: &Arena, idx: NodeIdx, v: &mut Vec<Log>) {
    if arena.kind(idx) == TreeKind::Loc(LocKind::Line) {
        v.extend(arena.logs(idx).iter().cloned());
    }
    for child in arena.child_iter(idx) {
        collect_logs(arena, child, v);
    }
}

/// Format timestamp + level + message.
pub fn format_log_line(log: &Log) -> WidgetText {
    use crate::log::arena::Level;

    let ns = log.ts;
    let ms = ns / 1_000_000;
    let secs = ms / 1000 % 60;
    let mins = ms / 1000 / 60 % 60;
    let hrs  = ms / 1000 / 60 / 60 % 24;
    let days = ms / 1000 / 60 / 60 / 24;
    let sub_ms = ms % 1000;

    let time = if days > 0 {
        format!("T+ {}d:{:02}h:{:02}m:{:02}s.{:03}ms", days, hrs, mins, secs, sub_ms)
    } else if hrs > 0 {
        format!("T+ {:02}h:{:02}m:{:02}s.{:03}ms", hrs, mins, secs, sub_ms)
    } else {
        format!("T+ {:02}m:{:02}s.{:03}ms", mins, secs, sub_ms)
    };

    let (level_str, level_color) = match log.lvl {
        Level::Error => ("[ERROR]", Color32::RED),
        Level::Warn  => ("[WARN]",  Color32::YELLOW),
        Level::Info  => ("[INFO]",  Color32::LIGHT_GREEN),
        Level::Debug => ("[DEBUG]", Color32::LIGHT_BLUE),
        Level::Trace => ("[TRACE]", Color32::KHAKI),
    };

    let mut job = LayoutJob::default();

    job.append(
        &format!("[{time}]"),
        0.0,
        TextFormat {
            font_id: FontId::monospace(12.0),
            color: Color32::GRAY,
            ..Default::default()
        },
    );

    job.append(
        level_str,
        0.0,
        TextFormat {
            font_id: FontId::monospace(12.0),
            color: level_color,
            ..Default::default()
        },
    );

    job.append(
        &format!(" — {}", log.msg),
        0.0,
        TextFormat {
            font_id: FontId::monospace(12.0),
            color: Color32::WHITE,
            ..Default::default()
        },
    );

    WidgetText::from(job)
}

/// Reusable log level slider UI
pub fn right_panel_filter_ui(ui: &mut egui::Ui, threshold: &mut Level) {
    use Level::*;

    let all_levels = [Error, Warn, Info, Debug, Trace];
    let level_symbols = ["E", "W", "I", "D", "T"];
    let level_colors = [
        Color32::RED,
        Color32::YELLOW,
        Color32::GREEN,
        Color32::LIGHT_BLUE,
        Color32::KHAKI,
    ];
    
    let level_index = all_levels
        .iter()
        .position(|l| l == threshold)
        .unwrap_or(2);
    let mut slider_value = level_index as f32;

    egui::Frame::none()
        .fill(Color32::from_gray(30))
        .stroke(egui::Stroke::new(1.0, Color32::DARK_GRAY))
        .rounding(egui::Rounding::same(4.0))
        .inner_margin(egui::Margin::symmetric(6.0, 4.0))
        .show(ui, |ui| {
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

                ui.painter().circle_filled(center, 11.0, color);
                ui.painter().text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    symbol,
                    egui::TextStyle::Button.resolve(ui.style()),
                    Color32::BLACK,
                );

                *threshold = all_levels[idx];
            });
        });
}
