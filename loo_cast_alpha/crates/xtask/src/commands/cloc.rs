use anyhow::{Context, Result, bail};
use std::env;
use std::path::Path;
use std::process::Command;

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

    let status = Command::new(&cloc_binary_path)
        .arg("--exclude-dir=build,target")
        .arg("--not-match-f=^cloc(?:-.*|\\.sh)$")
        .arg(root)
        .current_dir(root)
        .status()
        .with_context(|| format!("failed to execute '{}'", cloc_binary_path.display()))?;
    if !status.success() {
        bail!("cloc exited with status {status}");
    }
    Ok(())
}
