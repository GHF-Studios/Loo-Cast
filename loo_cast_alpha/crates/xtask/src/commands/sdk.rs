use anyhow::{Context, Result, bail};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use xshell::Shell;

use crate::LAUNCHER_CRATE;
use crate::commands::build::build;
use crate::utils::build_target::BuildTarget;
use crate::utils::fs::{cargo_artifact_dir, discover_workspace_crates, executable_name};
use crate::utils::options::XtaskOptions;
use crate::utils::profile::Profile;

pub fn clean(_sh: &Shell, root: &Path, profile: Profile, target: BuildTarget, options: XtaskOptions) -> Result<()> {
    let artifact_dir = cargo_artifact_dir(root, profile, target, options.use_vendored_toolchain);
    if artifact_dir.is_dir() {
        fs::remove_dir_all(&artifact_dir).with_context(|| format!("failed to remove '{}'", artifact_dir.display()))?;
    }
    println!("cleaned runtime build artifacts: {}", artifact_dir.display());
    Ok(())
}

pub fn run(sh: &Shell, root: &Path, profile: Profile, target: BuildTarget, options: XtaskOptions) -> Result<()> {
    build(sh, root, profile, target, options, false)?;
    let launcher = cargo_artifact_dir(root, profile, target, options.use_vendored_toolchain).join(bin_name(LAUNCHER_CRATE));
    run_binary(&launcher, root, profile)
}

pub fn debug(sh: &Shell, root: &Path, profile: Profile, target: BuildTarget, options: XtaskOptions) -> Result<()> {
    build(sh, root, profile, target, options, false)?;
    let game = cargo_artifact_dir(root, profile, target, options.use_vendored_toolchain).join(executable_name(target));
    run_binary(&game, root, profile)
}

pub fn contribute(root: &Path, options: XtaskOptions) -> Result<()> {
    ensure_git_repo(root)?;
    ensure_develop_branch(root)?;

    let commit_message = env::var("LOOCAST_CONTRIBUTE_MESSAGE")
        .ok()
        .map(|message| message.trim().to_string())
        .filter(|message| !message.is_empty())
        .unwrap_or_else(|| "contribute sdk snapshot".to_string());

    let mut add_cmd = Command::new("git");
    add_cmd.arg("add");
    for path in contribution_paths(root)? {
        add_cmd.arg(path);
    }
    let add_status = add_cmd.current_dir(root).status().context("failed to stage contribute files")?;
    if !add_status.success() {
        bail!("git add failed with status {add_status}");
    }

    if !has_staged_changes(root)? {
        println!("contribute: no staged changes");
        return Ok(());
    }

    if !options.contribute_apply {
        println!("contribute dry-run: staged files are ready.");
        println!(
            "run again with '--contribute-apply' to commit{}",
            if options.contribute_push { " and push" } else { "" }
        );
        println!("optional env override: LOOCAST_CONTRIBUTE_MESSAGE='your message'");
        return Ok(());
    }

    let commit_status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .current_dir(root)
        .status()
        .context("failed to run git commit")?;
    if !commit_status.success() {
        bail!("git commit failed with status {commit_status}");
    }

    if options.contribute_push {
        let push_status = Command::new("git")
            .arg("push")
            .arg("origin")
            .arg("develop")
            .current_dir(root)
            .status()
            .context("failed to run git push origin develop")?;
        if !push_status.success() {
            bail!("git push origin develop failed with status {push_status}");
        }
    }

    Ok(())
}

fn ensure_git_repo(root: &Path) -> Result<()> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .current_dir(root)
        .output()
        .context("failed to run git rev-parse")?;
    if !output.status.success() {
        bail!("contribute requires a git repository");
    }
    Ok(())
}

fn ensure_develop_branch(root: &Path) -> Result<()> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .current_dir(root)
        .output()
        .context("failed to get current branch")?;
    if !output.status.success() {
        bail!("failed to get current branch");
    }
    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch != "develop" {
        bail!("contribute is restricted to branch 'develop' (current branch: '{branch}')");
    }
    Ok(())
}

fn has_staged_changes(root: &Path) -> Result<bool> {
    let status = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--quiet")
        .current_dir(root)
        .status()
        .context("failed to inspect staged changes")?;
    Ok(!status.success())
}

fn run_binary(binary: &Path, root: &Path, profile: Profile) -> Result<()> {
    if !binary.is_file() {
        bail!("missing executable '{}'", binary.display());
    }
    let status = Command::new(binary)
        .current_dir(root)
        .env("RUST_BACKTRACE", "1")
        .env("BUILD_PROFILE", profile.as_str())
        .status()
        .with_context(|| format!("failed to run '{}'", binary.display()))?;
    if !status.success() {
        bail!("executable exited with status {status}");
    }
    Ok(())
}

fn contribution_paths(root: &Path) -> Result<Vec<String>> {
    let mut paths = vec![
        "Cargo.toml".to_string(),
        "Cargo.lock".to_string(),
        "rust-toolchain.toml".to_string(),
        ".cargo".to_string(),
        "docs".to_string(),
        "crates/launcher/src/main.rs".to_string(),
    ];

    for crate_dir in discover_workspace_crates(root)? {
        if !crate_dir.join(".loo_cast_mod").is_file() {
            continue;
        }
        let Some(name) = crate_dir.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        paths.push(format!("crates/{name}"));
    }

    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn bin_name(name: &str) -> String {
    if cfg!(target_os = "windows") {
        format!("{name}.exe")
    } else {
        name.to_string()
    }
}
