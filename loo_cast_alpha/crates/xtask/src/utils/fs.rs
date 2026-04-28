use anyhow::{Context, Result, bail};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::build_target::BuildTarget;
use crate::utils::profile::Profile;

pub fn staged_build_dir(root: &Path, profile: Profile, target: BuildTarget) -> PathBuf {
    let mut path = root.join("build").join(profile.as_str());
    if let Some(triple) = target.triple() {
        path.push(triple);
    }
    path
}

pub fn sdk_root_dir(root: &Path, profile: Profile, target: BuildTarget) -> PathBuf {
    if let Some(root_from_env) = sdk_root_dir_from_env() {
        return root_from_env;
    }
    staged_build_dir(root, profile, target).join("sdk")
}

pub fn sdk_cargo_home_dir(root: &Path, profile: Profile, target: BuildTarget) -> PathBuf {
    if let Some(path) = env_path("CARGO_HOME") {
        return path;
    }
    sdk_root_dir(root, profile, target).join("cargo-home")
}

pub fn sdk_rustup_home_dir(root: &Path, profile: Profile, target: BuildTarget) -> PathBuf {
    if let Some(path) = env_path("RUSTUP_HOME") {
        return path;
    }
    sdk_root_dir(root, profile, target).join("rustup-home")
}

pub fn sdk_target_dir(root: &Path, profile: Profile, target: BuildTarget) -> PathBuf {
    if let Some(path) = env_path("CARGO_TARGET_DIR") {
        return path;
    }
    sdk_root_dir(root, profile, target).join("target")
}

pub fn sdk_cargo_bin(root: &Path, profile: Profile, target: BuildTarget) -> PathBuf {
    sdk_cargo_home_dir(root, profile, target).join("bin").join(host_binary_name("cargo"))
}

pub fn cargo_artifact_dir(root: &Path, profile: Profile, target: BuildTarget, use_vendored_toolchain: bool) -> PathBuf {
    let mut path = if use_vendored_toolchain {
        sdk_target_dir(root, profile, target)
    } else {
        root.join("target")
    };
    if let Some(triple) = target.triple() {
        path.push(triple);
    }
    path.join(profile.artifact_dir_name())
}

pub fn executable_name(target: BuildTarget) -> &'static str {
    if target.is_windows() { "core_engine.exe" } else { "core_engine" }
}

fn host_binary_name(name: &str) -> String {
    if cfg!(target_os = "windows") {
        format!("{name}.exe")
    } else {
        name.to_string()
    }
}

fn env_path(name: &str) -> Option<PathBuf> {
    let value = env::var_os(name)?;
    if value.is_empty() {
        return None;
    }
    Some(PathBuf::from(value))
}

fn sdk_root_dir_from_env() -> Option<PathBuf> {
    if let Some(cargo_home) = env_path("CARGO_HOME")
        && cargo_home.file_name().and_then(|name| name.to_str()) == Some("cargo-home")
    {
        return cargo_home.parent().map(Path::to_path_buf);
    }
    if let Some(rustup_home) = env_path("RUSTUP_HOME")
        && rustup_home.file_name().and_then(|name| name.to_str()) == Some("rustup-home")
    {
        return rustup_home.parent().map(Path::to_path_buf);
    }
    if let Some(target_dir) = env_path("CARGO_TARGET_DIR")
        && target_dir.file_name().and_then(|name| name.to_str()) == Some("target")
    {
        return target_dir.parent().map(Path::to_path_buf);
    }
    None
}

pub fn copy_file(source: &Path, destination: &Path) -> Result<()> {
    if !source.is_file() {
        bail!("missing file '{}'", source.display());
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).with_context(|| format!("failed to create '{}'", parent.display()))?;
    }
    fs::copy(source, destination).with_context(|| format!("failed to copy '{}' -> '{}'", source.display(), destination.display()))?;
    Ok(())
}

pub fn copy_optional_symbol(source: &Path, destination: &Path) -> Result<()> {
    if source.is_file() {
        copy_file(source, destination)?;
    }
    Ok(())
}

pub fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination).with_context(|| format!("failed to create '{}'", destination.display()))?;

    for entry in fs::read_dir(source).with_context(|| format!("failed to read '{}'", source.display()))? {
        let entry = entry.with_context(|| format!("failed to read entry in '{}'", source.display()))?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry
            .file_type()
            .with_context(|| format!("failed to get file type for '{}'", source_path.display()))?;

        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            copy_file(&source_path, &destination_path)?;
        }
    }
    Ok(())
}

pub fn discover_workspace_crates(root: &Path) -> Result<Vec<PathBuf>> {
    let crates_root = root.join("crates");
    let mut crates = Vec::new();
    for entry in fs::read_dir(&crates_root).with_context(|| format!("failed to read '{}'", crates_root.display()))? {
        let entry = entry.with_context(|| format!("failed to read directory entry in '{}'", crates_root.display()))?;
        let path = entry.path();
        if path.is_dir() && path.join("Cargo.toml").is_file() {
            crates.push(path);
        }
    }
    crates.sort();
    Ok(crates)
}
