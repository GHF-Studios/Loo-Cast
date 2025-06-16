use bevy_egui::egui::{self, Color32, RichText, ScrollArea, TextFormat, FontId};
use egui::{text::LayoutJob, WidgetText};

use crate::log::arena::{Arena, Kind, Level, NodeIdx, Log};
use crate::log::resources::*;

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
    out.retain(|log| log.lvl >= state.threshold);
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

                // Reserve space first
                let slider = egui::Slider::new(&mut slider_value, 0.0..=4.0)
                    .step_by(1.0)
                    .show_value(false);
                let response = ui.add(slider);

                // Compute actual knob position manually
                let track_rect = response.rect.shrink(6.0); // account for padding/margin
                let knob_range = 4.0;
                let norm = (slider_value / knob_range).clamp(0.0, 1.0);

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

