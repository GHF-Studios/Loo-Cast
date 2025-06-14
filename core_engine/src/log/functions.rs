//! UI helpers & misc queries.

use bevy_egui::egui::{self, Color32, RichText, ScrollArea};
use crate::log::arena::{Arena, Kind, Level, NodeIdx, Log};
use crate::log::resources::{LogTreeHandle, LogViewerState};

pub fn render_log_tree(ui: &mut egui::Ui, arena: &Arena) {
    ScrollArea::vertical().show(ui, |ui| {
        for root in arena.roots() {
            render_node(ui, arena, root, 0);
        }
    });
}

fn render_node(ui: &mut egui::Ui, arena: &Arena, idx: NodeIdx, depth: usize) {
    let indent = "  ".repeat(depth);
    match arena.kind(idx) {
        Kind::Span | Kind::Module | Kind::File => {
            let name = arena.tok_str(arena.name_tok(idx));
            ui.collapsing(format!("{indent}📂 {}", name), |ui| {
                for child in arena.child_iter(idx) {
                    render_node(ui, arena, child, depth + 1);
                }
            });
        }
        Kind::Line => {
            let (l, c) = arena.line_col(idx).unwrap_or((0, 0));
            ui.collapsing(format!("{indent}📄 {l}:{c}"), |ui| {
                ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    for log in arena.logs(idx).iter() {
                        ui.label(format_log_line(log));
                    }
                });
            });
        }
    }
}

pub fn render_selectable_tree(
    ui:    &mut egui::Ui,
    arena: &Arena,
    state: &mut LogViewerState,
) {
    for root in arena.roots() {
        paint_branch(ui, arena, state, root);
    }
}

fn paint_branch(
    ui:    &mut egui::Ui,
    arena: &Arena,
    state: &mut LogViewerState,
    idx:   NodeIdx,
) {
    ui.indent(idx, |ui| {
        ui.horizontal(|ui| {
            // checkbox
            let mut checked = state.selected.contains(&idx);
            if ui.checkbox(&mut checked, "").changed() {
                if checked { state.selected.insert(idx); }
                else       { state.selected.remove(&idx); }
            }

            // collapsible header (arrow + icon + text)
            let (icon, label) = match arena.kind(idx) {
                Kind::Span | Kind::Module => ("📂", arena.tok_str(arena.name_tok(idx))),
                Kind::File               => ("📄", arena.tok_str(arena.name_tok(idx))),
                Kind::Line => {
                    let (l, c) = arena.line_col(idx).unwrap_or((0, 0));
                    ("📑", format!("{l}:{c}").into())
                }
            };

            ui.collapsing(format!("{icon} {label}"), |ui| {
                for child in arena.child_iter(idx) {
                    paint_branch(ui, arena, state, child);
                }
            });
        });
    });
}

pub fn gather_logs(arena: &Arena, state: &LogViewerState) -> Vec<Log> {
    let mut out = Vec::new();
    for &node in &state.selected {
        collect_logs(arena, node, &mut out);
    }
    out.sort_by_key(|l| l.ts);
    out
}

fn collect_logs(arena: &Arena, idx: NodeIdx, v: &mut Vec<Log>) {
    if arena.kind(idx) == Kind::Line {
        v.extend(arena.logs(idx).iter().cloned());
    }
    for child in arena.child_iter(idx) {
        collect_logs(arena, child, v);
    }
}

pub fn format_log_line(log: &Log) -> RichText {
    let base = format!("{:?} — {}", log.lvl, log.msg);
    match log.lvl {
        Level::Error => RichText::new(base).color(Color32::RED),
        Level::Warn  => RichText::new(base).color(Color32::YELLOW),
        Level::Info  => RichText::new(base).color(Color32::LIGHT_GREEN),
        Level::Debug => RichText::new(base).color(Color32::LIGHT_BLUE),
        Level::Trace => RichText::new(base).color(Color32::KHAKI),
    }
}
