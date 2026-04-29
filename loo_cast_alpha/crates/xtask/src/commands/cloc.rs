use anyhow::{Context, Result, bail};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::XTASK_CRATE;
use crate::utils::fs::discover_workspace_crates;

pub fn run_cloc(root: &Path) -> Result<()> {
    let cloc_binary_name = if cfg!(target_os = "windows") {
        "cloc-1.98-windows.exe"
    } else if cfg!(target_os = "linux") {
        "cloc-2.10-preview-linux"
    } else {
        bail!("cloc task is unsupported on '{}'; only windows and linux are supported", env::consts::OS);
    };

    let cloc_binary_path = root.join(cloc_binary_name);
    if !cloc_binary_path.is_file() {
        bail!("expected cloc binary '{}' at '{}'", cloc_binary_name, cloc_binary_path.display());
    }

    let mut inputs: Vec<PathBuf> = discover_workspace_crates(root)?
        .into_iter()
        .filter(|path| path.file_name().and_then(|name| name.to_str()) != Some(XTASK_CRATE))
        .collect();
    let documents = root.join("documents");
    if documents.is_dir() {
        inputs.push(documents);
    }
    if inputs.is_empty() {
        bail!("no input paths found for cloc");
    }

    let status = Command::new(&cloc_binary_path)
        .args(inputs.iter())
        .current_dir(root)
        .status()
        .with_context(|| format!("failed to execute '{}'", cloc_binary_path.display()))?;
    if !status.success() {
        bail!("cloc exited with status {status}");
    }
    Ok(())
}
