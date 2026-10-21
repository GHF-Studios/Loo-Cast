//! Structured diagnostic record styling.

use super::*;
use vapor_telemetry::ConsoleStyleRole;

pub(super) fn console_record_layout(record: &ConsoleRecord) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let font = egui::FontId::new(12.5, egui::FontFamily::Monospace);

    let timestamp = egui::Color32::from_rgb(105, 112, 120);
    let separator = egui::Color32::from_rgb(92, 100, 108);
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

    append(
        &timestamp_label(record.semantic.timestamp_unix_ms),
        timestamp,
    );
    append("  ", separator);

    match record.kind {
        ConsoleRecordKind::Trace(_) => {
            let shared_level = record.semantic.level;
            append(
                &format!("{:<5}", shared_level.label()),
                level_color(shared_level.style_role()),
            );
            append("  ", separator);
            append(
                record.semantic.target.as_deref().unwrap_or("<unknown>"),
                egui::Color32::from_rgb(100, 184, 214),
            );
            append("  ", separator);
            append(&record.semantic.message, message);

            for field in &record.semantic.fields {
                append("  ", separator);
                append(&field.key, field_key);
                append("=", separator);
                append(&field.value, field_value);
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
            append(&record.semantic.message, message);
        }
        ConsoleRecordKind::Output => {
            append("OUT  ", separator);
            append(&record.semantic.message, output);
        }
        ConsoleRecordKind::Error => {
            append("ERR  ", error);
            append(&record.semantic.message, error);
        }
        ConsoleRecordKind::Clear => {}
    }

    job
}

fn level_color(role: ConsoleStyleRole) -> egui::Color32 {
    match role {
        ConsoleStyleRole::Trace => egui::Color32::from_rgb(172, 132, 205),
        ConsoleStyleRole::Debug => egui::Color32::from_rgb(104, 158, 220),
        ConsoleStyleRole::Normal => egui::Color32::from_rgb(108, 190, 126),
        ConsoleStyleRole::Warning => egui::Color32::from_rgb(236, 185, 82),
        ConsoleStyleRole::Error => egui::Color32::from_rgb(248, 105, 96),
        ConsoleStyleRole::Success => egui::Color32::LIGHT_GREEN,
        ConsoleStyleRole::Input => egui::Color32::from_rgb(96, 190, 220),
        ConsoleStyleRole::Muted | ConsoleStyleRole::Metadata => {
            egui::Color32::from_rgb(105, 112, 120)
        }
    }
}
