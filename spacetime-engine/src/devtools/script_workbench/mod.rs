//! Host-managed developer scripting workspace.
//!
//! developer-console-script-workspace-v2
//!
//! The developer console owns the UI shell. This module owns script documents,
//! host-contract validation, committed runtime revisions, and the script-editor
//! surface embedded by the console.
//!
//! Rhai code never receives Bevy World/ECS/filesystem authority. The editor host
//! may read/write source files under the repository's `scripts/` directory; the
//! runtime receives only explicit typed policy inputs.

use std::{
    collections::BTreeMap,
    fs,
    path::{Component, Path, PathBuf},
};

use bevy::prelude::*;
use bevy_egui::egui;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use rhai::{AST, Engine, Scope};

use crate::console::{
    AppConsoleExt, ConsoleCommandInvocation, ConsoleCommandResult, ConsoleCommandSpec,
};

const DEFAULT_SCALAR_SOURCE: &str = r#"// Pure scalar host contract.
//
// Input/output units are defined by the bound host policy.
fn transform(value) {
    value
}
"#;

const DEFAULT_SCRATCH_SOURCE: &str = r#"// Session script.
// Scratch scripts need not expose a host policy contract.

fn hello() {
    "hello from Loo-Cast"
}
"#;

const FREECAM_SCRIPT_PATH: &str = "debug/freecam_speed.rhai";
const CELESTIAL_HEIGHT_SCRIPT_PATH: &str = "worldgen/celestial_height.rhai";
const DEFAULT_SCRATCH_PATH: &str = "scratch/experiment.rhai";
const MAX_WORKSPACE_FILES: usize = 256;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScriptTarget {
    None,
    FreecamSpeed,
    CelestialHeight,
}

impl ScriptTarget {
    fn for_path(path: &str) -> Self {
        match path {
            FREECAM_SCRIPT_PATH => Self::FreecamSpeed,
            CELESTIAL_HEIGHT_SCRIPT_PATH => Self::CelestialHeight,
            _ => Self::None,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::None => "unbound",
            Self::FreecamSpeed => "debug.freecam.speed",
            Self::CelestialHeight => "worldgen.celestial.presentation_height",
        }
    }

    const fn contract(self) -> &'static str {
        match self {
            Self::None => "No host contract. Compile-only script.",
            Self::FreecamSpeed => {
                "fn transform(value) -> finite number; value is freecam speed in m/s."
            }
            Self::CelestialHeight => {
                "fn transform(value) -> finite number; value is radial terrain displacement in metres."
            }
        }
    }

    const fn supports_live(self) -> bool {
        !matches!(self, Self::None)
    }
}

struct ScriptDocument {
    path: String,
    target: ScriptTarget,
    source: String,
    saved_source: String,
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

impl ScriptDocument {
    fn from_source(engine: &Engine, path: String, source: String) -> Self {
        let target = ScriptTarget::for_path(&path);
        let fallback_source = if target == ScriptTarget::None {
            DEFAULT_SCRATCH_SOURCE
        } else {
            DEFAULT_SCALAR_SOURCE
        };
        let fallback_ast = engine
            .compile(fallback_source)
            .expect("built-in developer script fallback must compile");

        let (committed_source, committed_ast, diagnostic, preview_output) =
            match compile_and_validate(engine, target, &source, 2.0) {
                Ok((ast, preview)) => (
                    source.clone(),
                    ast,
                    "Loaded source is compiled and committed.".to_string(),
                    preview,
                ),
                Err(error) => (
                    fallback_source.to_string(),
                    fallback_ast,
                    format!(
                        "Loaded source is not runtime-valid; safe fallback remains committed: {error}"
                    ),
                    None,
                ),
            };

        Self {
            path,
            target,
            source: source.clone(),
            saved_source: source,
            committed_source,
            committed_ast,
            candidate_source: None,
            candidate_ast: None,
            revision: 1,
            diagnostic,
            preview_input: 2.0,
            preview_output,
            live_enabled: false,
        }
    }

    fn source_dirty(&self) -> bool {
        self.source != self.saved_source
    }

    fn runtime_dirty(&self) -> bool {
        self.source != self.committed_source
    }
}

fn compile_and_validate(
    engine: &Engine,
    target: ScriptTarget,
    source: &str,
    preview_input: f64,
) -> Result<(AST, Option<f64>), String> {
    let ast = engine
        .compile(source)
        .map_err(|error| format!("compile error: {error}"))?;

    if target == ScriptTarget::None {
        return Ok((ast, None));
    }

    let preview = evaluate_ast(engine, &ast, preview_input)?;
    Ok((ast, Some(preview)))
}

fn evaluate_ast(engine: &Engine, ast: &AST, input: f64) -> Result<f64, String> {
    let mut scope = Scope::new();
    let output = engine
        .call_fn::<f64>(&mut scope, ast, "transform", (input,))
        .map_err(|error| format!("transform(value) failed: {error}"))?;
    if !output.is_finite() {
        return Err("transform(value) returned a non-finite number".to_string());
    }
    Ok(output)
}

/// Immutable source-level snapshot safe to move into background worker jobs.
#[derive(Debug, Clone)]
pub(crate) struct DeveloperScalarPolicySnapshot {
    source: String,
    revision: u64,
}

impl DeveloperScalarPolicySnapshot {
    pub(crate) const fn revision(&self) -> u64 {
        self.revision
    }

    pub(crate) fn compile_runtime(&self) -> Result<DeveloperScalarPolicyRuntime, String> {
        let engine = bounded_engine();
        let ast = engine
            .compile(&self.source)
            .map_err(|error| format!("worker policy compile error: {error}"))?;
        Ok(DeveloperScalarPolicyRuntime { engine, ast })
    }
}

/// One job-local compiled scalar policy.
pub(crate) struct DeveloperScalarPolicyRuntime {
    engine: Engine,
    ast: AST,
}

impl DeveloperScalarPolicyRuntime {
    pub(crate) fn evaluate(&self, input: f64) -> Result<f64, String> {
        evaluate_ast(&self.engine, &self.ast, input)
    }
}

#[derive(Resource)]
pub(crate) struct DeveloperScriptWorkbench {
    engine: Engine,
    documents: BTreeMap<String, ScriptDocument>,
    open_tabs: Vec<String>,
    active_path: String,
    workspace_root: Option<PathBuf>,
    scratch_counter: u32,
}

impl Default for DeveloperScriptWorkbench {
    fn default() -> Self {
        let engine = bounded_engine();
        let workspace_root = locate_workspace_root();

        let mut sources = BTreeMap::<String, String>::new();
        if let Some(root) = workspace_root.as_deref() {
            collect_rhai_sources(root, root, &mut sources);
        }

        sources
            .entry(FREECAM_SCRIPT_PATH.to_string())
            .or_insert_with(|| DEFAULT_SCALAR_SOURCE.to_string());
        sources
            .entry(CELESTIAL_HEIGHT_SCRIPT_PATH.to_string())
            .or_insert_with(|| DEFAULT_SCALAR_SOURCE.to_string());
        sources
            .entry(DEFAULT_SCRATCH_PATH.to_string())
            .or_insert_with(|| DEFAULT_SCRATCH_SOURCE.to_string());

        let documents = sources
            .into_iter()
            .map(|(path, source)| {
                let document = ScriptDocument::from_source(&engine, path.clone(), source);
                (path, document)
            })
            .collect::<BTreeMap<_, _>>();

        let active_path = if documents.contains_key(CELESTIAL_HEIGHT_SCRIPT_PATH) {
            CELESTIAL_HEIGHT_SCRIPT_PATH.to_string()
        } else {
            documents
                .keys()
                .next()
                .cloned()
                .unwrap_or_else(|| DEFAULT_SCRATCH_PATH.to_string())
        };

        let mut open_tabs = Vec::new();
        for path in [
            CELESTIAL_HEIGHT_SCRIPT_PATH,
            FREECAM_SCRIPT_PATH,
            DEFAULT_SCRATCH_PATH,
        ] {
            if documents.contains_key(path) {
                open_tabs.push(path.to_string());
            }
        }
        if open_tabs.is_empty() {
            open_tabs.push(active_path.clone());
        }

        Self {
            engine,
            documents,
            open_tabs,
            active_path,
            workspace_root,
            scratch_counter: 1,
        }
    }
}

impl DeveloperScriptWorkbench {
    fn active(&self) -> Option<&ScriptDocument> {
        self.documents.get(&self.active_path)
    }

    fn active_mut(&mut self) -> Option<&mut ScriptDocument> {
        self.documents.get_mut(&self.active_path)
    }

    pub(crate) fn revision(&self) -> u64 {
        self.active().map_or(0, |document| document.revision)
    }

    pub(crate) fn live_enabled(&self) -> bool {
        self.document_for_target(ScriptTarget::FreecamSpeed)
            .is_some_and(|document| document.live_enabled)
    }

    pub(crate) fn set_live_enabled(&mut self, enabled: bool) {
        if let Some(document) = self.document_for_target_mut(ScriptTarget::FreecamSpeed) {
            document.live_enabled = enabled;
        }
    }

    pub(crate) fn celestial_height_live_enabled(&self) -> bool {
        self.document_for_target(ScriptTarget::CelestialHeight)
            .is_some_and(|document| document.live_enabled)
    }

    pub(crate) fn celestial_height_snapshot(&self) -> Option<DeveloperScalarPolicySnapshot> {
        let document = self.document_for_target(ScriptTarget::CelestialHeight)?;
        document.live_enabled.then(|| DeveloperScalarPolicySnapshot {
            source: document.committed_source.clone(),
            revision: document.revision,
        })
    }

    pub(crate) fn dirty(&self) -> bool {
        self.active().is_some_and(ScriptDocument::runtime_dirty)
    }

    pub(crate) fn evaluate_committed(&self, input: f64) -> Result<f64, String> {
        let document = self
            .active()
            .ok_or_else(|| "no active script document".to_string())?;
        if document.target == ScriptTarget::None {
            return Err("active script has no scalar host contract".to_string());
        }
        evaluate_ast(&self.engine, &document.committed_ast, input)
    }

    pub(crate) fn apply_live_scalar(&self, value: f64) -> f64 {
        let Some(document) = self.document_for_target(ScriptTarget::FreecamSpeed) else {
            return value;
        };
        if !document.live_enabled {
            return value;
        }
        evaluate_ast(&self.engine, &document.committed_ast, value).unwrap_or(value)
    }

    fn document_for_target(&self, target: ScriptTarget) -> Option<&ScriptDocument> {
        self.documents.values().find(|document| document.target == target)
    }

    fn document_for_target_mut(&mut self, target: ScriptTarget) -> Option<&mut ScriptDocument> {
        self.documents
            .values_mut()
            .find(|document| document.target == target)
    }

    fn open_document(&mut self, path: &str) {
        if !self.documents.contains_key(path) {
            return;
        }
        if !self.open_tabs.iter().any(|open| open == path) {
            self.open_tabs.push(path.to_string());
        }
        self.active_path = path.to_string();
    }

    fn close_tab(&mut self, path: &str) {
        if self.open_tabs.len() <= 1 {
            return;
        }
        let Some(index) = self.open_tabs.iter().position(|open| open == path) else {
            return;
        };
        self.open_tabs.remove(index);
        if self.active_path == path {
            let next = index.min(self.open_tabs.len().saturating_sub(1));
            self.active_path.clone_from(&self.open_tabs[next]);
        }
    }

    fn new_scratch(&mut self) -> String {
        loop {
            let path = format!("scratch/untitled_{}.rhai", self.scratch_counter);
            self.scratch_counter = self.scratch_counter.saturating_add(1);
            if self.documents.contains_key(&path) {
                continue;
            }
            self.documents.insert(
                path.clone(),
                ScriptDocument::from_source(
                    &self.engine,
                    path.clone(),
                    DEFAULT_SCRATCH_SOURCE.to_string(),
                ),
            );
            self.open_document(&path);
            return path;
        }
    }

    fn compile_active(&mut self) -> Result<Option<f64>, String> {
        let path = self.active_path.clone();
        let (target, source, preview_input) = {
            let document = self
                .documents
                .get(&path)
                .ok_or_else(|| "no active script document".to_string())?;
            (
                document.target,
                document.source.clone(),
                document.preview_input,
            )
        };

        match compile_and_validate(&self.engine, target, &source, preview_input) {
            Ok((ast, preview)) => {
                let document = self.documents.get_mut(&path).expect("active document exists");
                document.candidate_source = Some(source);
                document.candidate_ast = Some(ast);
                document.preview_output = preview;
                document.diagnostic = match preview {
                    Some(value) => format!("Compiled successfully. Preview = {value:.6}"),
                    None => "Compiled successfully.".to_string(),
                };
                Ok(preview)
            }
            Err(error) => {
                let document = self.documents.get_mut(&path).expect("active document exists");
                document.candidate_source = None;
                document.candidate_ast = None;
                document.preview_output = None;
                document.diagnostic = error.clone();
                Err(error)
            }
        }
    }

    pub(crate) fn compile_draft(&mut self) -> Result<f64, String> {
        Ok(self.compile_active()?.unwrap_or(0.0))
    }

    fn commit_active(&mut self) -> Result<u64, String> {
        let path = self.active_path.clone();
        let needs_compile = {
            let document = self
                .documents
                .get(&path)
                .ok_or_else(|| "no active script document".to_string())?;
            document.candidate_source.as_deref() != Some(document.source.as_str())
                || document.candidate_ast.is_none()
        };
        if needs_compile {
            self.compile_active()?;
        }

        let document = self
            .documents
            .get_mut(&path)
            .ok_or_else(|| "no active script document".to_string())?;
        let Some(ast) = document.candidate_ast.take() else {
            return Err("compiled candidate disappeared before commit".to_string());
        };
        document.candidate_source = None;
        document.committed_ast = ast;
        document.committed_source.clone_from(&document.source);
        document.revision = document.revision.saturating_add(1);
        document.diagnostic = format!(
            "Committed runtime revision {} for {}.",
            document.revision, document.path
        );
        Ok(document.revision)
    }

    pub(crate) fn commit_draft(&mut self) -> Result<u64, String> {
        self.commit_active()
    }

    fn revert_active_to_committed(&mut self) {
        if let Some(document) = self.active_mut() {
            document.source.clone_from(&document.committed_source);
            document.candidate_source = None;
            document.candidate_ast = None;
            document.diagnostic =
                format!("Reverted editor buffer to runtime revision {}.", document.revision);
        }
    }

    pub(crate) fn revert_draft(&mut self) {
        self.revert_active_to_committed();
    }

    fn reload_active_from_saved(&mut self) {
        if let Some(document) = self.active_mut() {
            document.source.clone_from(&document.saved_source);
            document.candidate_source = None;
            document.candidate_ast = None;
            document.diagnostic = "Reloaded editor buffer from saved source.".to_string();
        }
    }

    fn save_active(&mut self) -> Result<(), String> {
        let root = self
            .workspace_root
            .clone()
            .ok_or_else(|| "developer script root is unavailable in this process".to_string())?;
        let path = self.active_path.clone();
        validate_relative_script_path(&path)?;
        let source = self
            .documents
            .get(&path)
            .ok_or_else(|| "no active script document".to_string())?
            .source
            .clone();

        let destination = root.join(Path::new(&path));
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("create script directory failed: {error}"))?;
        }
        fs::write(&destination, &source)
            .map_err(|error| format!("save {} failed: {error}", destination.display()))?;

        let document = self.documents.get_mut(&path).expect("active document exists");
        document.saved_source = source;
        document.diagnostic = format!("Saved {}.", document.path);
        Ok(())
    }
}

fn locate_workspace_root() -> Option<PathBuf> {
    let mut cursor = std::env::current_dir().ok()?;
    loop {
        if cursor.join("spacetime-engine/Cargo.toml").is_file() {
            return Some(cursor.join("scripts"));
        }
        if !cursor.pop() {
            return None;
        }
    }
}

fn collect_rhai_sources(root: &Path, directory: &Path, out: &mut BTreeMap<String, String>) {
    if out.len() >= MAX_WORKSPACE_FILES {
        return;
    }
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        if out.len() >= MAX_WORKSPACE_FILES {
            break;
        }
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            if entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }
            collect_rhai_sources(root, &path, out);
            continue;
        }
        if !file_type.is_file() || path.extension().and_then(|value| value.to_str()) != Some("rhai") {
            continue;
        }
        let Ok(relative) = path.strip_prefix(root) else {
            continue;
        };
        let logical = relative
            .components()
            .filter_map(|component| match component {
                Component::Normal(value) => Some(value.to_string_lossy().into_owned()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("/");
        let Ok(source) = fs::read_to_string(&path) else {
            continue;
        };
        out.insert(logical, source);
    }
}

fn validate_relative_script_path(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    if path.is_absolute() {
        return Err("script paths must be relative to the developer script root".to_string());
    }
    if !path
        .components()
        .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err("script path contains unsupported traversal/components".to_string());
    }
    if path.extension().and_then(|value| value.to_str()) != Some("rhai") {
        return Err("developer script files must end in .rhai".to_string());
    }
    Ok(())
}

pub(crate) fn configure(app: &mut App) {
    app.init_resource::<DeveloperScriptWorkbench>()
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script status",
                aliases: &["script status"],
                usage: "debug script status",
                summary: "Show the active script buffer/runtime state.",
            },
            script_status_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script compile",
                aliases: &["script compile"],
                usage: "debug script compile",
                summary: "Compile/validate the active script buffer.",
            },
            script_compile_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script commit",
                aliases: &["script commit"],
                usage: "debug script commit",
                summary: "Atomically commit the active script buffer as a runtime revision.",
            },
            script_commit_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script revert",
                aliases: &["script revert"],
                usage: "debug script revert",
                summary: "Revert the active editor buffer to its committed runtime revision.",
            },
            script_revert_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script save",
                aliases: &["script save"],
                usage: "debug script save",
                summary: "Save the active script buffer to the host-managed scripts directory.",
            },
            script_save_command,
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
    let Some(document) = workbench.active() else {
        return ConsoleCommandResult::error("no active script document");
    };
    ConsoleCommandResult::lines([
        format!(
            "{} | target={} | revision={} | file-dirty={} | runtime-dirty={} | live={}",
            document.path,
            document.target.label(),
            document.revision,
            document.source_dirty(),
            document.runtime_dirty(),
            document.live_enabled,
        ),
        document.diagnostic.clone(),
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
        .compile_active()
    {
        Ok(Some(value)) => ConsoleCommandResult::success(format!(
            "active script compiled; preview output={value:.6}"
        )),
        Ok(None) => ConsoleCommandResult::success("active script compiled"),
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
        .commit_active()
    {
        Ok(revision) => {
            ConsoleCommandResult::success(format!("active script committed as revision {revision}"))
        }
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
        .revert_active_to_committed();
    ConsoleCommandResult::success("active script reverted to committed runtime source")
}

fn script_save_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: debug script save");
    }
    match world
        .resource_mut::<DeveloperScriptWorkbench>()
        .save_active()
    {
        Ok(()) => ConsoleCommandResult::success("active script saved"),
        Err(error) => ConsoleCommandResult::error(error),
    }
}

/// Starfall/E2-style script workspace embedded by the developer-console shell.
pub(crate) fn draw_script_workspace(
    ui: &mut egui::Ui,
    workbench: &mut DeveloperScriptWorkbench,
) {
    let available_height = ui.available_height().max(240.0);
    let explorer_width = (ui.available_width() * 0.22).clamp(170.0, 260.0);

    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(explorer_width, available_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| draw_script_explorer(ui, workbench),
        );
        ui.separator();
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), available_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| draw_script_editor(ui, workbench),
        );
    });
}

fn draw_script_explorer(ui: &mut egui::Ui, workbench: &mut DeveloperScriptWorkbench) {
    ui.horizontal(|ui| {
        ui.strong("Scripts");
        if ui.small_button("+").on_hover_text("New scratch script").clicked() {
            workbench.new_scratch();
        }
    });

    if let Some(root) = workbench.workspace_root.as_ref() {
        ui.small(
            egui::RichText::new(root.display().to_string())
                .monospace()
                .weak(),
        );
    } else {
        ui.small(egui::RichText::new("in-memory workspace").weak());
    }
    ui.separator();

    let mut grouped = BTreeMap::<String, Vec<String>>::new();
    for path in workbench.documents.keys() {
        let (folder, _) = path.split_once('/').unwrap_or((".", path.as_str()));
        grouped.entry(folder.to_string()).or_default().push(path.clone());
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

fn draw_script_editor(ui: &mut egui::Ui, workbench: &mut DeveloperScriptWorkbench) {
    let tabs = workbench.open_tabs.clone();
    let mut requested_tab = None::<String>;
    let mut requested_close = None::<String>;

    ui.horizontal_wrapped(|ui| {
        for path in tabs {
            let Some(document) = workbench.documents.get(&path) else {
                continue;
            };
            let basename = path.rsplit('/').next().unwrap_or(&path);
            let mut label = basename.to_string();
            if document.source_dirty() {
                label.push('*');
            }
            if document.runtime_dirty() {
                label.push('∆');
            }
            if ui
                .selectable_label(workbench.active_path == path, label)
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
                requested_close = Some(path);
            }
        }
    });

    if let Some(path) = requested_tab {
        workbench.open_document(&path);
    }
    if let Some(path) = requested_close {
        workbench.close_tab(&path);
    }

    ui.separator();

    let Some(active) = workbench.active() else {
        ui.weak("No active script.");
        return;
    };
    let target = active.target;
    let active_path = active.path.clone();
    let source_dirty = active.source_dirty();
    let runtime_dirty = active.runtime_dirty();
    let revision = active.revision;

    let mut do_save = false;
    let mut do_reload_saved = false;
    let mut do_compile = false;
    let mut do_commit = false;
    let mut do_revert = false;

    ui.horizontal_wrapped(|ui| {
        ui.monospace(&active_path);
        ui.separator();
        ui.weak(format!("target: {}", target.label()));
        ui.separator();
        ui.weak(format!("runtime r{revision}"));
        if source_dirty {
            ui.colored_label(egui::Color32::YELLOW, "UNSAVED");
        }
        if runtime_dirty {
            ui.colored_label(egui::Color32::LIGHT_BLUE, "UNCOMMITTED");
        }

        ui.separator();
        do_save = ui.button("Save").clicked();
        do_reload_saved = ui.button("Reload Saved").clicked();
        do_compile = ui.button("Compile").clicked();
        do_commit = ui.button("Commit").clicked();
        do_revert = ui.button("Revert Runtime").clicked();
    });

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

    if do_save {
        if let Err(error) = workbench.save_active()
            && let Some(document) = workbench.active_mut()
        {
            document.diagnostic = error;
        }
    }
    if do_reload_saved {
        workbench.reload_active_from_saved();
    }
    if do_compile {
        let _ = workbench.compile_active();
    }
    if do_commit {
        let _ = workbench.commit_active();
    }
    if do_revert {
        workbench.revert_active_to_committed();
    }

    if target.supports_live() {
        let mut preview_changed = false;
        if let Some(document) = workbench.active_mut() {
            ui.horizontal(|ui| {
                ui.label("Preview input");
                preview_changed = ui
                    .add(egui::DragValue::new(&mut document.preview_input).speed(0.1))
                    .changed();
                ui.label(
                    document
                        .preview_output
                        .map_or_else(|| "→ —".to_string(), |value| format!("→ {value:.6}")),
                );
            });
        }
        if preview_changed {
            let _ = workbench.compile_active();
        }
    }

    ui.separator();

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

    ui.separator();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scratch_script_compiles_without_scalar_contract() {
        let engine = bounded_engine();
        assert!(
            compile_and_validate(
                &engine,
                ScriptTarget::None,
                "fn hello() { 42 }",
                1.0,
            )
            .is_ok()
        );
    }

    #[test]
    fn bound_policy_requires_transform_contract() {
        let engine = bounded_engine();
        assert!(
            compile_and_validate(
                &engine,
                ScriptTarget::CelestialHeight,
                "fn hello() { 42 }",
                1.0,
            )
            .is_err()
        );
    }

    #[test]
    fn broken_candidate_cannot_replace_committed_runtime() {
        let mut workbench = DeveloperScriptWorkbench::default();
        workbench.open_document(CELESTIAL_HEIGHT_SCRIPT_PATH);
        let before = workbench
            .document_for_target(ScriptTarget::CelestialHeight)
            .unwrap()
            .revision;
        workbench.active_mut().unwrap().source = "fn nope(".to_string();
        assert!(workbench.commit_active().is_err());
        assert_eq!(
            workbench
                .document_for_target(ScriptTarget::CelestialHeight)
                .unwrap()
                .revision,
            before,
        );
    }
}
