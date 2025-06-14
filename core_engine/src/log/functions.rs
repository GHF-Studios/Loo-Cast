use bevy_egui::egui::{self, Color32, RichText, ScrollArea};
use crate::log::types::{LogSpan, LogTree, LogEvent};
use std::collections::HashSet;

fn format_log_line(log: &LogEvent) -> RichText {
    let base = format!("{} — {}", log.level, log.message);

    match log.level {
        tracing::Level::ERROR => RichText::new(base).color(Color32::RED),
        tracing::Level::WARN => RichText::new(base).color(Color32::YELLOW),
        tracing::Level::INFO => RichText::new(base).color(Color32::LIGHT_GREEN),
        tracing::Level::DEBUG => RichText::new(base).color(Color32::LIGHT_BLUE),
        tracing::Level::TRACE => RichText::new(base).color(Color32::KHAKI),
    }
}

pub(super) fn render_log_tree(ui: &mut egui::Ui, tree: &LogTree) {
    // Eliminate duplicate root spans
    let mut visited = HashSet::new();
    ScrollArea::vertical().show(ui, |ui| {
        for root in &tree.root_spans {
            if visited.contains(root) {
                continue;
            }
            if let Some(span) = tree.spans.get(root) {
                render_span(ui, span, tree, 0, &mut visited);
            }
        }
    });
}

fn render_span(
    ui: &mut egui::Ui,
    span: &LogSpan,
    tree: &LogTree,
    indent: usize,
    visited: &mut HashSet<tracing::span::Id>,
) {
    visited.insert(span.id.clone());

    let indent_str = "  ".repeat(indent);
    let header = format!("{indent_str}📍 {}", span.name);

    ui.collapsing(header, |ui| {
        for (key, value) in &span.fields {
            ui.label(RichText::new(format!("{key}: {value}")).monospace());
        }

        ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
            for log in &span.logs {
                ui.label(format_log_line(log));
            }
        });

        for child_id in &span.children {
            if let Some(child) = tree.spans.get(child_id) {
                if !visited.contains(&child.id) {
                    render_span(ui, child, tree, indent + 1, visited);
                }
            }
        }
    });
}
