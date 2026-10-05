//! Default/live script roots, materialization, discovery, and path validation.

use std::{
    collections::BTreeMap,
    fs,
    path::{Component, Path, PathBuf},
};

include!(concat!(env!("OUT_DIR"), "/embedded_developer_scripts.rs"));
const MAX_WORKSPACE_FILES: usize = 256;

#[derive(Debug, Clone)]
pub(super) struct ScriptWorkspaceRoots {
    pub(super) defaults_root: PathBuf,
    pub(super) live_root: PathBuf,
    pub(super) mode: &'static str,
}

impl ScriptWorkspaceRoots {
    fn path_in(root: &Path, path: &str) -> Result<PathBuf, String> {
        validate_relative_script_path(path)?;
        Ok(root.join(Path::new(path)))
    }

    pub(super) fn has_default(&self, path: &str) -> bool {
        Self::path_in(&self.defaults_root, path).is_ok_and(|path| path.is_file())
    }

    pub(super) fn read_live(&self, path: &str) -> Result<(String, PathBuf), String> {
        let location = Self::path_in(&self.live_root, path)?;
        let source = fs::read_to_string(&location)
            .map_err(|error| format!("reload {} failed: {error}", location.display()))?;
        Ok((source, location))
    }

    pub(super) fn write_live(&self, path: &str, source: &str) -> Result<PathBuf, String> {
        let location = Self::path_in(&self.live_root, path)?;
        if let Some(parent) = location.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("create script directory failed: {error}"))?;
        }
        fs::write(&location, source)
            .map_err(|error| format!("save {} failed: {error}", location.display()))?;
        Ok(location)
    }

    pub(super) fn reset_live_to_default(&self, path: &str) -> Result<String, String> {
        let default_path = Self::path_in(&self.defaults_root, path)?;
        let source = fs::read_to_string(&default_path).map_err(|error| {
            format!(
                "no shipped/default source for `{path}` at {}: {error}",
                default_path.display()
            )
        })?;
        self.write_live(path, &source)?;
        Ok(source)
    }
}

#[derive(Debug, Clone)]
pub(super) struct ScriptWorkspaceBootstrap {
    pub(super) roots: ScriptWorkspaceRoots,
    pub(super) warning: Option<String>,
}

fn locate_repo_root() -> Option<PathBuf> {
    let mut cursor = std::env::current_dir().ok()?;
    loop {
        if cursor.join("spacetime-engine/Cargo.toml").is_file() {
            return Some(cursor);
        }
        if !cursor.pop() {
            return None;
        }
    }
}

fn environment_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn locate_workspace_roots() -> ScriptWorkspaceRoots {
    if let (Some(defaults_root), Some(live_root)) = (
        environment_path("LOO_CAST_SCRIPT_DEFAULT_ROOT"),
        environment_path("LOO_CAST_SCRIPT_LIVE_ROOT"),
    ) {
        return ScriptWorkspaceRoots {
            defaults_root,
            live_root,
            mode: "environment override",
        };
    }

    if let Some(repo_root) = locate_repo_root() {
        let runtime = repo_root.join(".loo-cast/runtime");
        return ScriptWorkspaceRoots {
            defaults_root: runtime.join("script-defaults"),
            live_root: runtime.join("scripts"),
            mode: "development runtime copy",
        };
    }

    let game_root = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));

    ScriptWorkspaceRoots {
        defaults_root: game_root.join("scripts/defaults"),
        live_root: game_root.join("scripts/live"),
        mode: "deployed runtime copy",
    }
}

pub(super) fn prepare_workspace_roots() -> ScriptWorkspaceBootstrap {
    let roots = locate_workspace_roots();
    let mut warnings = Vec::new();

    if let Err(error) = materialize_embedded_defaults(&roots.defaults_root) {
        warnings.push(error);
    }
    if let Err(error) = seed_missing_live_scripts(&roots.defaults_root, &roots.live_root) {
        warnings.push(error);
    }

    ScriptWorkspaceBootstrap {
        roots,
        warning: (!warnings.is_empty()).then(|| warnings.join(" | ")),
    }
}

fn materialize_embedded_defaults(defaults_root: &Path) -> Result<(), String> {
    fs::create_dir_all(defaults_root)
        .map_err(|error| format!("create script defaults root failed: {error}"))?;

    for &(logical_path, source) in EMBEDDED_DEVELOPER_SCRIPTS {
        validate_relative_script_path(logical_path)?;
        let destination = defaults_root.join(Path::new(logical_path));
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("create default script directory failed: {error}"))?;
        }

        // Defaults represent the scripts shipped in THIS build, so refreshing
        // them is correct. Live files are a separate tree and are never
        // overwritten here.
        fs::write(&destination, source).map_err(|error| {
            format!(
                "materialize default {} failed: {error}",
                destination.display()
            )
        })?;
    }
    Ok(())
}

fn seed_missing_live_scripts(defaults_root: &Path, live_root: &Path) -> Result<(), String> {
    fs::create_dir_all(live_root)
        .map_err(|error| format!("create live script root failed: {error}"))?;

    let mut defaults = BTreeMap::<String, String>::new();
    collect_rhai_sources(defaults_root, defaults_root, &mut defaults);
    for (logical_path, source) in defaults {
        let destination = live_root.join(Path::new(&logical_path));
        if destination.exists() {
            continue;
        }
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("create live script directory failed: {error}"))?;
        }
        fs::write(&destination, source)
            .map_err(|error| format!("seed live {} failed: {error}", destination.display()))?;
    }
    Ok(())
}

pub(super) fn collect_rhai_sources(
    root: &Path,
    directory: &Path,
    out: &mut BTreeMap<String, String>,
) {
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
        if !file_type.is_file() || path.extension().and_then(|value| value.to_str()) != Some("rhai")
        {
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

pub(super) fn validate_relative_script_path(path: &str) -> Result<(), String> {
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
