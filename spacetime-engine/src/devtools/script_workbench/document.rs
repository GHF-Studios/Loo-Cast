//! Target contracts and draft-to-committed document lifecycle.

use rhai::{AST, Engine, FuncArgs, Scope};
use std::path::Path;

pub(super) const DEFAULT_SCALAR_SOURCE: &str = r#"// Pure scalar host contract.
//
// Input/output units are defined by the bound host policy.
fn transform(value) {
    value
}
"#;

pub(super) const DEFAULT_SCRATCH_SOURCE: &str = r#"// Session script.
// Scratch scripts need not expose a host policy contract.

fn hello() {
    "hello from Loo-Cast"
}
"#;

pub(super) const FREECAM_SCRIPT_PATH: &str = "debug/freecam_speed.rhai";
pub(super) const DEFAULT_SCRATCH_PATH: &str = "scratch/experiment.rhai";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ScriptTarget {
    None,
    FreecamSpeed,
}

impl ScriptTarget {
    pub(super) fn for_path(path: &str) -> Self {
        match path {
            FREECAM_SCRIPT_PATH => Self::FreecamSpeed,
            _ => Self::None,
        }
    }

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::None => "unbound",
            Self::FreecamSpeed => "debug.freecam.speed",
        }
    }

    pub(super) const fn contract(self) -> &'static str {
        match self {
            Self::None => "No host contract. Compile-only script.",
            Self::FreecamSpeed => {
                "fn transform(value) -> finite number; value is freecam speed in m/s."
            }
        }
    }

    pub(super) const fn supports_live(self) -> bool {
        !matches!(self, Self::None)
    }

    pub(super) const fn fallback_source(self) -> &'static str {
        match self {
            Self::None => DEFAULT_SCRATCH_SOURCE,
            Self::FreecamSpeed => DEFAULT_SCALAR_SOURCE,
        }
    }

    pub(super) const fn api_help(self) -> &'static str {
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

pub(super) struct ScriptDocument {
    pub(super) path: String,
    pub(super) target: ScriptTarget,
    pub(super) source: String,
    pub(super) saved_source: String,
    pub(super) committed_source: String,
    pub(super) committed_ast: AST,
    pub(super) candidate_source: Option<String>,
    pub(super) candidate_ast: Option<AST>,
    pub(super) revision: u64,
    pub(super) diagnostic: String,
    pub(super) preview_input: f64,
    pub(super) preview_output: Option<f64>,
    pub(super) live_enabled: bool,
}

impl ScriptDocument {
    pub(super) fn from_source(engine: &Engine, path: String, source: String) -> Self {
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

    pub(super) fn source_dirty(&self) -> bool {
        self.source != self.saved_source
    }

    pub(super) fn runtime_dirty(&self) -> bool {
        self.source != self.committed_source
    }

    /// A candidate proves the current editor buffer only; editing invalidates it.
    pub(super) fn compile_candidate(&mut self, engine: &Engine) -> Result<Option<f64>, String> {
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
    pub(super) fn commit(&mut self, engine: &Engine) -> Result<u64, String> {
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

    pub(super) fn invalidate_candidate(&mut self) {
        self.candidate_source = None;
        self.candidate_ast = None;
    }

    pub(super) fn revert_to_committed(&mut self) {
        self.source.clone_from(&self.committed_source);
        self.invalidate_candidate();
        self.diagnostic = format!(
            "Reverted editor buffer to runtime revision {}.",
            self.revision
        );
    }

    pub(super) fn replace_with_saved(&mut self, source: String, location: &Path) {
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

pub(super) fn call_f64(
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
