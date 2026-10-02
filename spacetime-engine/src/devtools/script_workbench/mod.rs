//! Trusted in-process Rhai developer workbench.
//!
//! developer-lab-rhai-workbench-v1
//!
//! This is deliberately a tiny host contract, not "script the Bevy World".
//! The first proof exposes exactly one pure scalar policy:
//!
//!     fn transform(value) -> number
//!
//! Draft source is compiled/validated before commit. A failed compile/commit
//! never replaces the last working AST. Runtime consumers opt in explicitly.

use std::collections::BTreeMap;

use bevy::prelude::*;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use rhai::{AST, Engine, Scope};

use crate::console::{
    AppConsoleExt, ConsoleCommandInvocation, ConsoleCommandResult, ConsoleCommandSpec,
    RuntimeVariableBinding, RuntimeVariableRegistry,
};

const DEFAULT_SOURCE: &str = r#"// Trusted Developer Lab scalar policy.
//
// Host contract:
//   fn transform(value) -> finite number
//
// Nothing in this script has ECS/World/filesystem authority.
fn transform(value) {
    value
}
"#;

const SCRIPT_MAX_OPERATIONS: u64 = 5_000;
const SCRIPT_MAX_CALL_LEVELS: usize = 12;
const SCRIPT_MAX_VARIABLES: usize = 96;
const SCRIPT_MAX_FUNCTIONS: usize = 48;
const SCRIPT_MAX_STRING_BYTES: usize = 16 * 1024;
const SCRIPT_MAX_ARRAY_SIZE: usize = 256;
const SCRIPT_MAX_MAP_SIZE: usize = 128;

fn bounded_engine() -> Engine {
    let mut engine = Engine::new();
    engine
        .set_max_operations(SCRIPT_MAX_OPERATIONS)
        .set_max_call_levels(SCRIPT_MAX_CALL_LEVELS)
        .set_max_expr_depths(32, 16)
        .set_max_variables(SCRIPT_MAX_VARIABLES)
        .set_max_functions(SCRIPT_MAX_FUNCTIONS)
        .set_max_modules(0)
        .set_max_string_size(SCRIPT_MAX_STRING_BYTES)
        .set_max_array_size(SCRIPT_MAX_ARRAY_SIZE)
        .set_max_map_size(SCRIPT_MAX_MAP_SIZE);
    engine
}

#[derive(Resource)]
pub(crate) struct DeveloperScriptWorkbench {
    engine: Engine,
    source: String,
    committed_source: String,
    committed_ast: AST,
    candidate_source: Option<String>,
    candidate_ast: Option<AST>,
    revision: u64,
    diagnostic: String,
    preview_input: f64,
    preview_output: Option<f64>,
    live_enabled: bool,
}

impl Default for DeveloperScriptWorkbench {
    fn default() -> Self {
        let engine = bounded_engine();
        let committed_ast = engine
            .compile(DEFAULT_SOURCE)
            .expect("built-in Developer Lab Rhai source must compile");
        let mut value = Self {
            engine,
            source: DEFAULT_SOURCE.to_string(),
            committed_source: DEFAULT_SOURCE.to_string(),
            committed_ast,
            candidate_source: None,
            candidate_ast: None,
            revision: 1,
            diagnostic: "Committed revision 1 — identity scalar policy.".to_string(),
            preview_input: 2.0,
            preview_output: Some(2.0),
            live_enabled: false,
        };
        value.refresh_preview_from_committed();
        value
    }
}

impl DeveloperScriptWorkbench {
    pub(crate) const fn revision(&self) -> u64 {
        self.revision
    }

    pub(crate) const fn live_enabled(&self) -> bool {
        self.live_enabled
    }

    pub(crate) fn set_live_enabled(&mut self, enabled: bool) {
        self.live_enabled = enabled;
    }

    pub(crate) fn dirty(&self) -> bool {
        self.source != self.committed_source
    }

    fn evaluate_ast(&self, ast: &AST, input: f64) -> Result<f64, String> {
        let mut scope = Scope::new();
        let output = self
            .engine
            .call_fn::<f64>(&mut scope, ast, "transform", (input,))
            .map_err(|error| format!("transform(value) failed: {error}"))?;
        if !output.is_finite() {
            return Err("transform(value) returned a non-finite number".to_string());
        }
        Ok(output)
    }

    pub(crate) fn evaluate_committed(&self, input: f64) -> Result<f64, String> {
        self.evaluate_ast(&self.committed_ast, input)
    }

    /// Runtime consumers use the committed AST only. Failure falls back to the
    /// authored value instead of injecting broken developer policy.
    pub(crate) fn apply_live_scalar(&self, value: f64) -> f64 {
        if !self.live_enabled {
            return value;
        }
        self.evaluate_committed(value).unwrap_or(value)
    }

    fn compile_source(&self, source: &str) -> Result<(AST, f64), String> {
        let ast = self
            .engine
            .compile(source)
            .map_err(|error| format!("compile error: {error}"))?;
        let preview = self.evaluate_ast(&ast, self.preview_input)?;
        Ok((ast, preview))
    }

    pub(crate) fn compile_draft(&mut self) -> Result<f64, String> {
        match self.compile_source(&self.source) {
            Ok((ast, preview)) => {
                self.candidate_source = Some(self.source.clone());
                self.candidate_ast = Some(ast);
                self.preview_output = Some(preview);
                self.diagnostic =
                    format!("Draft compiled successfully. Preview = {preview:.6}");
                Ok(preview)
            }
            Err(error) => {
                self.candidate_source = None;
                self.candidate_ast = None;
                self.preview_output = None;
                self.diagnostic = error.clone();
                Err(error)
            }
        }
    }

    pub(crate) fn commit_draft(&mut self) -> Result<u64, String> {
        let candidate_matches_source =
            self.candidate_source.as_deref() == Some(self.source.as_str())
                && self.candidate_ast.is_some();

        if !candidate_matches_source {
            self.compile_draft()?;
        }

        let Some(ast) = self.candidate_ast.take() else {
            return Err("compiled Rhai candidate disappeared before commit".to_string());
        };
        self.candidate_source = None;
        self.committed_ast = ast;
        self.committed_source.clone_from(&self.source);
        self.revision = self.revision.saturating_add(1);
        self.refresh_preview_from_committed();
        self.diagnostic = format!("Committed live Rhai revision {}.", self.revision);
        Ok(self.revision)
    }

    pub(crate) fn revert_draft(&mut self) {
        self.source.clone_from(&self.committed_source);
        self.candidate_source = None;
        self.candidate_ast = None;
        self.refresh_preview_from_committed();
        self.diagnostic = format!("Draft reverted to revision {}.", self.revision);
    }

    fn refresh_preview_from_committed(&mut self) {
        self.preview_output = self.evaluate_committed(self.preview_input).ok();
    }
}

#[derive(Resource, Default)]
struct DeveloperLabUiState {
    variable_search: String,
    value_drafts: BTreeMap<String, String>,
}

pub(crate) fn configure(app: &mut App) {
    app.init_resource::<DeveloperScriptWorkbench>()
        .init_resource::<DeveloperLabUiState>()
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script status",
                aliases: &["script status"],
                usage: "debug script status",
                summary: "Show the committed Developer Lab Rhai revision.",
            },
            script_status_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script compile",
                aliases: &["script compile"],
                usage: "debug script compile",
                summary: "Compile/validate the current in-editor Rhai draft without committing it.",
            },
            script_compile_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script commit",
                aliases: &["script commit"],
                usage: "debug script commit",
                summary: "Atomically replace the live scalar policy with the validated Rhai draft.",
            },
            script_commit_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script revert",
                aliases: &["script revert"],
                usage: "debug script revert",
                summary: "Discard the Rhai draft and restore the committed source.",
            },
            script_revert_command,
        );
}

fn script_status_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: debug script status");
    }
    let workbench = world.resource::<DeveloperScriptWorkbench>();
    ConsoleCommandResult::lines([
        format!(
            "Rhai revision {} | draft={} | live-consumer={}",
            workbench.revision(),
            if workbench.dirty() { "dirty" } else { "clean" },
            workbench.live_enabled(),
        ),
        workbench.diagnostic.clone(),
    ])
}

fn script_compile_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: debug script compile");
    }
    match world
        .resource_mut::<DeveloperScriptWorkbench>()
        .compile_draft()
    {
        Ok(value) => ConsoleCommandResult::success(format!(
            "Rhai draft compiled; preview output={value:.6}"
        )),
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn script_commit_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: debug script commit");
    }
    match world
        .resource_mut::<DeveloperScriptWorkbench>()
        .commit_draft()
    {
        Ok(revision) => ConsoleCommandResult::success(format!(
            "Rhai scalar policy committed as revision {revision}"
        )),
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn script_revert_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: debug script revert");
    }
    world
        .resource_mut::<DeveloperScriptWorkbench>()
        .revert_draft();
    ConsoleCommandResult::success("Rhai draft reverted to committed source")
}

fn draw_script_workbench(ui: &mut egui::Ui, world: &mut World) {
    world.resource_scope(|_, mut workbench: Mut<DeveloperScriptWorkbench>| {
        ui.horizontal(|ui| {
            ui.heading("Rhai Workbench");
            ui.separator();
            ui.monospace(format!("revision {}", workbench.revision()));
            if workbench.dirty() {
                ui.colored_label(egui::Color32::YELLOW, "DIRTY");
            } else {
                ui.weak("clean");
            }
            ui.separator();
            ui.checkbox(
                &mut workbench.live_enabled,
                "Live output → freecam speed proof consumer",
            );
        });

        ui.horizontal_wrapped(|ui| {
            if ui.button("Compile").clicked() {
                let _ = workbench.compile_draft();
            }
            if ui.button("Commit").clicked() {
                let _ = workbench.commit_draft();
            }
            if ui.button("Revert").clicked() {
                workbench.revert_draft();
            }
            if ui.button("Identity").clicked() {
                workbench.source = "fn transform(value) {\n    value\n}\n".to_string();
            }
            if ui.button("Pow⁴ experiment").clicked() {
                workbench.source =
                    "fn transform(value) {\n    value.pow(4)\n}\n".to_string();
            }

            ui.separator();
            ui.label("Preview input");
            if ui
                .add(egui::DragValue::new(&mut workbench.preview_input).speed(0.1))
                .changed()
            {
                if let Some(ast) = workbench.candidate_ast.as_ref()
                    && workbench.candidate_source.as_deref()
                        == Some(workbench.source.as_str())
                {
                    workbench.preview_output =
                        workbench.evaluate_ast(ast, workbench.preview_input).ok();
                } else {
                    workbench.refresh_preview_from_committed();
                }
            }
            ui.label(format!(
                "→ {}",
                workbench
                    .preview_output
                    .map_or_else(|| "<error>".to_string(), |value| format!("{value:.6}"))
            ));
        });

        let status_color = if workbench.diagnostic.contains("error")
            || workbench.diagnostic.contains("failed")
        {
            egui::Color32::LIGHT_RED
        } else {
            egui::Color32::LIGHT_GREEN
        };
        ui.colored_label(status_color, &workbench.diagnostic);
        ui.weak(
            "Host contract: pure fn transform(value) -> finite number. No ECS, World, filesystem, imports, or semantic mutation.",
        );
        ui.add_space(4.0);

        CodeEditor::default()
            .id_source("developer_lab_rhai_editor")
            .with_rows(24)
            .with_fontsize(14.0)
            .with_theme(ColorTheme::GRUVBOX)
            .with_syntax(Syntax::rust())
            .with_numlines(true)
            .vscroll(true)
            .show(ui, &mut workbench.source);
    });
}

fn draw_runtime_variables(ui: &mut egui::Ui, world: &mut World) {
    let bindings = world
        .resource::<RuntimeVariableRegistry>()
        .bindings_snapshot();

    world.resource_scope(|world, mut state: Mut<DeveloperLabUiState>| {
        ui.horizontal(|ui| {
            ui.heading("Typed Runtime Controls");
            ui.separator();
            ui.label("Search");
            ui.text_edit_singleline(&mut state.variable_search);
        });
        ui.weak(
            "These are the exact #56 runtime-variable bindings used by the console; this GUI does not create a second state authority.",
        );

        let search = state.variable_search.trim().to_ascii_lowercase();
        egui::ScrollArea::vertical()
            .id_salt("developer_lab_runtime_variables")
            .max_height(360.0)
            .show(ui, |ui| {
                for (path, binding) in &bindings {
                    let spec = binding.spec();
                    if !search.is_empty()
                        && !path.to_ascii_lowercase().contains(&search)
                        && !spec.summary.to_ascii_lowercase().contains(&search)
                    {
                        continue;
                    }

                    let current = binding.current(world);
                    let default = binding.default_value();
                    let draft = state
                        .value_drafts
                        .entry(path.clone())
                        .or_insert_with(|| current.clone().unwrap_or_else(|_| default.clone()));

                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.monospace(path);
                            ui.separator();
                            ui.weak(format!(
                                "{} · {}{}",
                                spec.value_type.label(),
                                spec.authority.label(),
                                spec.units
                                    .map_or(String::new(), |units| format!(" · {units}")),
                            ));
                        });
                        ui.label(spec.summary);
                        match &current {
                            Ok(value) => {
                                ui.small(format!("effective: {value} · default: {default}"));
                            }
                            Err(error) => {
                                ui.colored_label(
                                    egui::Color32::LIGHT_RED,
                                    format!("unavailable: {error}"),
                                );
                            }
                        }

                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(draft)
                                    .desired_width(260.0)
                                    .hint_text("typed value"),
                            );

                            let apply = ui.button("Apply").clicked();
                            let reset = ui.button("Reset").clicked();
                            let sync = ui.button("↻").on_hover_text("Copy effective value").clicked();

                            if sync {
                                if let Ok(value) = binding.current(world) {
                                    *draft = value;
                                }
                            }
                            if apply {
                                let proposed = draft.clone();
                                if let Err(error) = binding.set(world, &proposed) {
                                    *draft = format!("<error: {error}>");
                                } else if let Ok(value) = binding.current(world) {
                                    *draft = value;
                                }
                            }
                            if reset {
                                if let Err(error) = binding.reset(world) {
                                    *draft = format!("<error: {error}>");
                                } else if let Ok(value) = binding.current(world) {
                                    *draft = value;
                                }
                            }
                        });
                    });
                    ui.add_space(4.0);
                }
            });
    });
}

/// Full docked Developer Lab surface.
///
/// Script workbench and runtime controls intentionally share one tab so the
/// developer can change a policy implementation and its surrounding parameters
/// without bouncing between unrelated debug surfaces.
pub(crate) fn draw_developer_lab(ui: &mut egui::Ui, world: &mut World) {
    ui.heading("Developer Lab");
    ui.weak("Live experiments are explicit, reversible, and downstream of real engine authority.");
    ui.separator();

    draw_script_workbench(ui, world);
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);
    draw_runtime_variables(ui, world);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_script_is_identity_and_bounded() {
        let workbench = DeveloperScriptWorkbench::default();
        assert_eq!(workbench.evaluate_committed(3.5).unwrap(), 3.5);
        assert!(!workbench.live_enabled());
    }

    #[test]
    fn broken_draft_cannot_replace_committed_ast() {
        let mut workbench = DeveloperScriptWorkbench::default();
        workbench.source = "fn nope(".to_string();
        assert!(workbench.commit_draft().is_err());
        assert_eq!(workbench.evaluate_committed(7.0).unwrap(), 7.0);
        assert_eq!(workbench.revision(), 1);
    }

    #[test]
    fn valid_commit_replaces_scalar_policy_atomically() {
        let mut workbench = DeveloperScriptWorkbench::default();
        workbench.source = "fn transform(value) { value * 2.0 }".to_string();
        assert_eq!(workbench.commit_draft().unwrap(), 2);
        assert_eq!(workbench.evaluate_committed(3.0).unwrap(), 6.0);
    }
}
