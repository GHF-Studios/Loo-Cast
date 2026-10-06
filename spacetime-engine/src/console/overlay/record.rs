//! Structured diagnostic record styling.

use super::*;

pub(super) fn console_record_layout(record: &ConsoleRecord) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let font = egui::FontId::new(12.5, egui::FontFamily::Monospace);

    let timestamp = egui::Color32::from_rgb(105, 112, 120);
    let separator = egui::Color32::from_rgb(92, 100, 108);
    let target = egui::Color32::from_rgb(100, 184, 214);
    let message = egui::Color32::from_rgb(218, 222, 226);
    let field_key = egui::Color32::from_rgb(192, 146, 224);
    let field_value = egui::Color32::from_rgb(218, 188, 116);
    let command = egui::Color32::from_rgb(96, 190, 220);
    let output = egui::Color32::from_rgb(202, 208, 214);
    let error = egui::Color32::from_rgb(248, 105, 96);

    let mut append = |text: &str, color: egui::Color32| {
        job.append(
            text,
            0.0,
            egui::text::TextFormat {
                font_id: font.clone(),
                color,
                ..default()
            },
        );
    };

    append(&timestamp_label(record.timestamp_millis), timestamp);
    append("  ", separator);

    match record.kind {
        ConsoleRecordKind::Trace(level) => {
            append(&format!("{:<5}", level.label()), level_color(level));
            append("  ", separator);
            append(record.target.as_deref().unwrap_or("<unknown>"), target);
            append("  ", separator);
            append(&record.text, message);

            for (key, value) in &record.fields {
                append("  ", separator);
                append(key, field_key);
                append("=", separator);
                append(value, field_value);
            }
        }
        ConsoleRecordKind::Command => {
            append("CMD  ", command);
            append(
                &format!(
                    "[{}]",
                    record.source.map_or("?", ConsoleCommandSource::label)
                ),
                separator,
            );
            append("  › ", command);
            append(&record.text, message);
        }
        ConsoleRecordKind::Output => {
            append("OUT  ", separator);
            append(&record.text, output);
        }
        ConsoleRecordKind::Error => {
            append("ERR  ", error);
            append(&record.text, error);
        }
        ConsoleRecordKind::Clear => {}
    }

    job
}

fn level_color(level: ConsoleLogLevel) -> egui::Color32 {
    match level {
        ConsoleLogLevel::Trace => egui::Color32::from_rgb(172, 132, 205),
        ConsoleLogLevel::Debug => egui::Color32::from_rgb(104, 158, 220),
        ConsoleLogLevel::Info => egui::Color32::from_rgb(108, 190, 126),
        ConsoleLogLevel::Warn => egui::Color32::from_rgb(236, 185, 82),
        ConsoleLogLevel::Error => egui::Color32::from_rgb(248, 105, 96),
    }
}
