//! Host-managed developer scripting workspace.
//!
//! The developer console owns the UI shell. This domain owns bounded script
//! execution, document revisions, live/default storage, and the embedded editor.
//!
//! Rhai code never receives Bevy World/ECS/filesystem authority. Repository
//! `scripts/` files are immutable source defaults from the in-game editor's
//! perspective. The editor works on a separate writable live copy; builds embed
//! the source defaults so deployed games can materialize defaults + live trees.
//! The runtime receives only explicit typed policy inputs.

use std::{collections::BTreeMap, path::Path};

use bevy::prelude::*;
use rhai::{AST, Engine, FuncArgs, Scope};

use crate::console::{
    AppConsoleExt, ConsoleCommandInvocation, ConsoleCommandResult, ConsoleCommandSpec,
};

mod host;
mod storage;
mod ui;

use host::bounded_engine;
use storage::{ScriptWorkspaceRoots, collect_rhai_sources, prepare_workspace_roots};
pub(crate) use ui::draw_script_workspace;

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
const DEFAULT_SCRATCH_PATH: &str = "scratch/experiment.rhai";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScriptTarget {
    None,
    FreecamSpeed,
}

impl ScriptTarget {
    fn for_path(path: &str) -> Self {
        match path {
            FREECAM_SCRIPT_PATH => Self::FreecamSpeed,
            _ => Self::None,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::None => "unbound",
            Self::FreecamSpeed => "debug.freecam.speed",
        }
    }

    const fn contract(self) -> &'static str {
        match self {
            Self::None => "No host contract. Compile-only script.",
            Self::FreecamSpeed => {
                "fn transform(value) -> finite number; value is freecam speed in m/s."
            }
        }
    }

    const fn supports_live(self) -> bool {
        !matches!(self, Self::None)
    }

    const fn fallback_source(self) -> &'static str {
        match self {
            Self::None => DEFAULT_SCRATCH_SOURCE,
            Self::FreecamSpeed => DEFAULT_SCALAR_SOURCE,
        }
    }

    const fn api_help(self) -> &'static str {
        match self {
            Self::None => {
                "Common: Rhai standard math + value.pow(...), lerp, saturate, smoothstep, remap; print/debug/log_* route into the developer console."
            }
            Self::FreecamSpeed => {
                "transform(value): value is m/s. Common math/logging helpers are available."
            }
        }
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
        let fallback_source = target.fallback_source();
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

    /// A candidate proves the current editor buffer only; editing invalidates it.
    fn compile_candidate(&mut self, engine: &Engine) -> Result<Option<f64>, String> {
        match compile_and_validate(engine, self.target, &self.source, self.preview_input) {
            Ok((ast, preview)) => {
                self.candidate_source = Some(self.source.clone());
                self.candidate_ast = Some(ast);
                self.preview_output = preview;
                self.diagnostic = match preview {
                    Some(value) => format!("Compiled successfully. Preview = {value:.6}"),
                    None => "Compiled successfully.".to_string(),
                };
                Ok(preview)
            }
            Err(error) => {
                self.invalidate_candidate();
                self.preview_output = None;
                self.diagnostic = error.clone();
                Err(error)
            }
        }
    }

    /// Only a candidate for the exact current buffer may become runtime authority.
    fn commit(&mut self, engine: &Engine) -> Result<u64, String> {
        if self.candidate_source.as_deref() != Some(self.source.as_str())
            || self.candidate_ast.is_none()
        {
            self.compile_candidate(engine)?;
        }
        let Some(ast) = self.candidate_ast.take() else {
            return Err("compiled candidate disappeared before commit".to_string());
        };
        self.candidate_source = None;
        self.committed_ast = ast;
        self.committed_source.clone_from(&self.source);
        self.revision = self.revision.saturating_add(1);
        self.diagnostic = format!(
            "Committed runtime revision {} for {}.",
            self.revision, self.path
        );
        Ok(self.revision)
    }

    fn invalidate_candidate(&mut self) {
        self.candidate_source = None;
        self.candidate_ast = None;
    }

    fn revert_to_committed(&mut self) {
        self.source.clone_from(&self.committed_source);
        self.invalidate_candidate();
        self.diagnostic = format!(
            "Reverted editor buffer to runtime revision {}.",
            self.revision
        );
    }

    fn replace_with_saved(&mut self, source: String, location: &Path) {
        self.source = source.clone();
        self.saved_source = source;
        self.invalidate_candidate();
        self.diagnostic = format!("Reloaded live copy {}.", location.display());
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

    match target {
        ScriptTarget::None => Ok((ast, None)),
        ScriptTarget::FreecamSpeed => {
            let preview = call_f64(engine, &ast, "transform", (preview_input,))?;
            Ok((ast, Some(preview)))
        }
    }
}

fn call_f64(
    engine: &Engine,
    ast: &AST,
    function: &str,
    args: impl FuncArgs,
) -> Result<f64, String> {
    let mut scope = Scope::new();
    let output = engine
        .call_fn::<f64>(&mut scope, ast, function, args)
        .map_err(|error| format!("{function}(...) failed: {error}"))?;
    if !output.is_finite() {
        return Err(format!("{function}(...) returned a non-finite number"));
    }
    Ok(output)
}

#[derive(Resource)]
pub(crate) struct DeveloperScriptWorkbench {
    engine: Engine,
    documents: BTreeMap<String, ScriptDocument>,
    open_tabs: Vec<String>,
    active_path: String,
    roots: ScriptWorkspaceRoots,
    bootstrap_warning: Option<String>,
    scratch_counter: u32,
}

impl Default for DeveloperScriptWorkbench {
    fn default() -> Self {
        let engine = bounded_engine();
        let bootstrap = prepare_workspace_roots();

        let mut sources = BTreeMap::<String, String>::new();
        collect_rhai_sources(
            &bootstrap.roots.live_root,
            &bootstrap.roots.live_root,
            &mut sources,
        );

        sources
            .entry(FREECAM_SCRIPT_PATH.to_string())
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

        let active_path = if documents.contains_key(FREECAM_SCRIPT_PATH) {
            FREECAM_SCRIPT_PATH.to_string()
        } else {
            documents
                .keys()
                .next()
                .cloned()
                .unwrap_or_else(|| DEFAULT_SCRATCH_PATH.to_string())
        };

        let mut open_tabs = Vec::new();
        for path in [FREECAM_SCRIPT_PATH, DEFAULT_SCRATCH_PATH] {
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
            roots: bootstrap.roots,
            bootstrap_warning: bootstrap.warning,
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

    pub(crate) fn apply_live_scalar(&self, value: f64) -> f64 {
        let Some(document) = self.document_for_target(ScriptTarget::FreecamSpeed) else {
            return value;
        };
        if !document.live_enabled {
            return value;
        }
        call_f64(&self.engine, &document.committed_ast, "transform", (value,)).unwrap_or(value)
    }

    fn document_for_target(&self, target: ScriptTarget) -> Option<&ScriptDocument> {
        self.documents
            .values()
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
        let document = self
            .documents
            .get_mut(&path)
            .ok_or_else(|| "no active script document".to_string())?;
        document.compile_candidate(&self.engine)
    }

    fn commit_active(&mut self) -> Result<u64, String> {
        let path = self.active_path.clone();
        let document = self
            .documents
            .get_mut(&path)
            .ok_or_else(|| "no active script document".to_string())?;
        document.commit(&self.engine)
    }

    fn revert_active_to_committed(&mut self) {
        if let Some(document) = self.active_mut() {
            document.revert_to_committed();
        }
    }

    fn reload_active_from_saved(&mut self) {
        let path = self.active_path.clone();
        match self.roots.read_live(&path) {
            Ok((source, location)) => {
                if let Some(document) = self.active_mut() {
                    document.replace_with_saved(source, &location);
                }
            }
            Err(error) => {
                if let Some(document) = self.active_mut() {
                    document.diagnostic = error;
                }
            }
        }
    }

    fn save_active(&mut self) -> Result<(), String> {
        let path = self.active_path.clone();
        let document = self
            .documents
            .get(&path)
            .ok_or_else(|| "no active script document".to_string())?;
        let source = document.source.clone();
        let location = self.roots.write_live(&path, &source)?;
        let document = self
            .documents
            .get_mut(&path)
            .expect("active document exists");
        document.saved_source = source;
        document.diagnostic = format!("Saved live copy {}.", location.display());
        Ok(())
    }

    fn reset_active_to_default(&mut self) -> Result<(), String> {
        let path = self.active_path.clone();
        let source = self.roots.reset_live_to_default(&path)?;
        let document = self
            .documents
            .get_mut(&path)
            .ok_or_else(|| "no active script document".to_string())?;
        document.source = source.clone();
        document.saved_source = source;
        document.invalidate_candidate();
        document.diagnostic =
            "Reset LIVE file to shipped default. Commit separately to activate it.".to_string();
        Ok(())
    }

    fn active_has_default(&self) -> bool {
        self.roots.has_default(&self.active_path)
    }
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
    let mut lines = vec![
        format!(
            "{} | target={} | revision={} | file-dirty={} | runtime-dirty={} | live={}",
            document.path,
            document.target.label(),
            document.revision,
            document.source_dirty(),
            document.runtime_dirty(),
            document.live_enabled,
        ),
        format!(
            "script workspace = {} | live={} | defaults={}",
            workbench.roots.mode,
            workbench.roots.live_root.display(),
            workbench.roots.defaults_root.display(),
        ),
        document.diagnostic.clone(),
    ];
    if let Some(warning) = workbench.bootstrap_warning.as_ref() {
        lines.push(format!("workspace bootstrap warning: {warning}"));
    }
    ConsoleCommandResult::lines(lines)
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
