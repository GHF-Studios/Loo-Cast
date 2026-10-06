//! Document selection and status listing.

use super::*;
use std::collections::BTreeMap;

pub(super) fn draw_script_explorer(ui: &mut egui::Ui, workbench: &mut DeveloperScriptWorkbench) {
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
