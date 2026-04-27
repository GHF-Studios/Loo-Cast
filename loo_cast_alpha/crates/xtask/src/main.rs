use anyhow::{Context, Result, bail};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use xshell::{Shell, cmd};

const CORE_ENGINE_CRATE: &str = "core_engine";
const XTASK_CRATE: &str = "xtask";
const LINUX_RELEASE_TARGET: &str = "x86_64-unknown-linux-gnu";
const WINDOWS_RELEASE_TARGET: &str = "x86_64-pc-windows-msvc";

#[derive(Copy, Clone)]
enum Profile {
    Dev,
    Fastdev,
    Release,
}

impl Profile {
    fn as_str(self) -> &'static str {
        match self {
            Profile::Dev => "dev",
            Profile::Fastdev => "fastdev",
            Profile::Release => "release",
        }
    }

    fn artifact_dir_name(self) -> &'static str {
        match self {
            Profile::Dev => "debug",
            Profile::Fastdev => "fastdev",
            Profile::Release => "release",
        }
    }
}

#[derive(Copy, Clone)]
enum BuildTarget {
    Host,
    LinuxRelease,
    WindowsRelease,
}

impl BuildTarget {
    fn triple(self) -> Option<&'static str> {
        match self {
            BuildTarget::Host => None,
            BuildTarget::LinuxRelease => Some(LINUX_RELEASE_TARGET),
            BuildTarget::WindowsRelease => Some(WINDOWS_RELEASE_TARGET),
        }
    }

    fn is_windows(self) -> bool {
        match self {
            BuildTarget::Host => cfg!(target_os = "windows"),
            BuildTarget::LinuxRelease => false,
            BuildTarget::WindowsRelease => true,
        }
    }
}

fn main() -> Result<()> {
    let sh = Shell::new().context("failed to create shell")?;
    let root = env::current_dir().context("failed to get current directory")?;
    sh.change_dir(&root);

    let mut args = env::args().skip(1);
    let Some(task) = args.next() else {
        print_help();
        return Ok(());
    };
    if args.next().is_some() {
        bail!("task '{task}' does not take extra arguments");
    }

    match task.as_str() {
        "help" => print_help(),
        "build_dev" => build(&sh, Profile::Dev, BuildTarget::Host)?,
        "build_fastdev" => build(&sh, Profile::Fastdev, BuildTarget::Host)?,
        "build_release" => build(&sh, Profile::Release, BuildTarget::Host)?,
        "build_linux_release" => build(&sh, Profile::Release, BuildTarget::LinuxRelease)?,
        "build_windows_release" => build(&sh, Profile::Release, BuildTarget::WindowsRelease)?,
        "package_dev" => package(&sh, &root, Profile::Dev, BuildTarget::Host)?,
        "package_fastdev" => package(&sh, &root, Profile::Fastdev, BuildTarget::Host)?,
        "package_release" => package(&sh, &root, Profile::Release, BuildTarget::Host)?,
        "package_linux_release" => package(&sh, &root, Profile::Release, BuildTarget::LinuxRelease)?,
        "package_windows_release" => package(&sh, &root, Profile::Release, BuildTarget::WindowsRelease)?,
        "run_dev" => run(&sh, &root, Profile::Dev)?,
        "run_fastdev" => run(&sh, &root, Profile::Fastdev)?,
        "run_release" => run(&sh, &root, Profile::Release)?,
        "cloc" => run_cloc(&root)?,
        "gource" => run_gource(&root)?,
        "deploy" => deploy_stub(),
        other => bail!("unknown xtask command '{other}'. Use `cargo xtask help`."),
    }

    Ok(())
}

fn print_help() {
    println!(
        "Loo-Cast alpha xtask

Commands:
  help
  build_dev
  build_fastdev
  build_release
  build_linux_release
  build_windows_release
  package_dev
  package_fastdev
  package_release
  package_linux_release
  package_windows_release
  run_dev
  run_fastdev
  run_release
  cloc
  gource
  deploy"
    );
}

fn build(sh: &Shell, profile: Profile, target: BuildTarget) -> Result<()> {
    match (profile, target) {
        (Profile::Dev, BuildTarget::Host) => cmd!(sh, "cargo build --workspace --exclude {XTASK_CRATE}").run()?,
        (Profile::Fastdev, BuildTarget::Host) => cmd!(sh, "cargo build --workspace --exclude {XTASK_CRATE} --profile fastdev").run()?,
        (Profile::Release, BuildTarget::Host) => cmd!(sh, "cargo build --workspace --exclude {XTASK_CRATE} --release").run()?,
        (Profile::Release, BuildTarget::LinuxRelease) => {
            cmd!(sh, "cargo build --workspace --exclude {XTASK_CRATE} --release --target {LINUX_RELEASE_TARGET}").run()?
        }
        (Profile::Release, BuildTarget::WindowsRelease) => cmd!(
            sh,
            "cargo build --workspace --exclude {XTASK_CRATE} --release --target {WINDOWS_RELEASE_TARGET}"
        )
        .run()?,
        _ => bail!("only release profile supports explicit target tasks"),
    }
    Ok(())
}

fn package(sh: &Shell, root: &Path, profile: Profile, target: BuildTarget) -> Result<()> {
    build(sh, profile, target)?;

    let build_dir = staged_build_dir(root, profile, target);
    clean_dir(&build_dir)?;

    let artifact_dir = cargo_artifact_dir(root, profile, target);
    let executable_name = executable_name(target);
    let source_executable = artifact_dir.join(executable_name);
    let staged_executable = build_dir.join(executable_name);

    copy_file(&source_executable, &staged_executable)?;
    copy_optional_symbol(&artifact_dir.join("core_engine.pdb"), &build_dir.join("core_engine.pdb"))?;
    copy_optional_symbol(&artifact_dir.join("core_engine.debug"), &build_dir.join("core_engine.debug"))?;
    stage_assets(root, &build_dir)?;

    println!("packaged executable: {}", staged_executable.display());
    println!("packaged assets root: {}", build_dir.join("assets").display());
    Ok(())
}

fn run(sh: &Shell, root: &Path, profile: Profile) -> Result<()> {
    package(sh, root, profile, BuildTarget::Host)?;
    let build_dir = staged_build_dir(root, profile, BuildTarget::Host);
    let executable = build_dir.join(executable_name(BuildTarget::Host));

    let status = Command::new(&executable)
        .current_dir(&build_dir)
        .env("RUST_BACKTRACE", "1")
        .env("BUILD_PROFILE", profile.as_str())
        .status()
        .with_context(|| format!("failed to execute '{}'", executable.display()))?;
    if !status.success() {
        bail!("staged executable exited with status {status}");
    }
    Ok(())
}

fn run_cloc(root: &Path) -> Result<()> {
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

fn run_gource(root: &Path) -> Result<()> {
    let gource_exe = root.join("gource.exe");
    let gource_program = if gource_exe.is_file() { gource_exe } else { PathBuf::from("gource") };

    let year_seconds = env::var("YEAR_SECONDS").ok().and_then(|value| value.parse::<f64>().ok()).unwrap_or(30.0);
    let seconds_per_day = format!("{:.6}", year_seconds / 365.2425);
    let auto_skip_seconds = env::var("AUTO_SKIP_SECONDS").unwrap_or_else(|_| "1".to_string());
    let file_idle_time = env::var("FILE_IDLE_TIME").unwrap_or_else(|_| "0".to_string());
    let max_file_lag = env::var("MAX_FILE_LAG").unwrap_or_else(|_| "0.15".to_string());
    let follow_user = env::var("FOLLOW_USER")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| git_user_name(root));

    let mut command = Command::new(&gource_program);
    command
        .current_dir(root)
        .arg("--title")
        .arg("Loo-Cast History")
        .arg("--seconds-per-day")
        .arg(seconds_per_day)
        .arg("--auto-skip-seconds")
        .arg(auto_skip_seconds)
        .arg("--file-idle-time")
        .arg(file_idle_time)
        .arg("--max-file-lag")
        .arg(max_file_lag)
        .arg("--hide")
        .arg("mouse,progress,filenames,dirnames,usernames")
        .arg("--highlight-users")
        .arg("--highlight-dirs")
        .arg("--camera-mode")
        .arg("overview")
        .arg("--multi-sampling")
        .arg("--stop-at-end");
    if let Some(follow_user) = follow_user {
        command.arg("--follow-user").arg(follow_user);
    }

    let status = command
        .status()
        .with_context(|| format!("failed to execute '{}'", gource_program.as_path().display()))?;
    if !status.success() {
        bail!("gource exited with status {status}");
    }
    Ok(())
}

fn deploy_stub() {
    println!("deploy task is currently a stub.");
    println!("future plan: semver-aware deploy pipeline for steam + github.");
    println!("future plan: multi-platform release artifacts for linux and windows.");
    println!("stub exits successfully by design.");
}

fn staged_build_dir(root: &Path, profile: Profile, target: BuildTarget) -> PathBuf {
    let mut path = root.join("build").join(profile.as_str());
    if let Some(triple) = target.triple() {
        path.push(triple);
    }
    path
}

fn cargo_artifact_dir(root: &Path, profile: Profile, target: BuildTarget) -> PathBuf {
    let mut path = root.join("target");
    if let Some(triple) = target.triple() {
        path.push(triple);
    }
    path.join(profile.artifact_dir_name())
}

fn executable_name(target: BuildTarget) -> &'static str {
    if target.is_windows() { "core_engine.exe" } else { "core_engine" }
}

fn clean_dir(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path).with_context(|| format!("failed to remove '{}'", path.display()))?;
    }
    fs::create_dir_all(path).with_context(|| format!("failed to create '{}'", path.display()))?;
    Ok(())
}

fn stage_assets(root: &Path, build_dir: &Path) -> Result<()> {
    let assets_root = build_dir.join("assets");

    let core_engine_assets = root.join("crates").join(CORE_ENGINE_CRATE).join("assets");
    if core_engine_assets.is_dir() {
        copy_dir_recursive(&core_engine_assets, &assets_root.join(CORE_ENGINE_CRATE))?;
    }

    for crate_dir in discover_workspace_crates(root)? {
        let crate_name = crate_dir.file_name().and_then(|name| name.to_str()).unwrap_or_default().to_string();
        if crate_name == CORE_ENGINE_CRATE {
            continue;
        }
        if !crate_dir.join(".loo_cast_mod").is_file() {
            continue;
        }
        let crate_assets = crate_dir.join("assets");
        if !crate_assets.is_dir() {
            continue;
        }
        copy_dir_recursive(&crate_assets, &assets_root.join(crate_name))?;
    }
    Ok(())
}

fn discover_workspace_crates(root: &Path) -> Result<Vec<PathBuf>> {
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

fn copy_file(source: &Path, destination: &Path) -> Result<()> {
    if !source.is_file() {
        bail!("missing file '{}'", source.display());
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).with_context(|| format!("failed to create '{}'", parent.display()))?;
    }
    fs::copy(source, destination).with_context(|| format!("failed to copy '{}' -> '{}'", source.display(), destination.display()))?;
    Ok(())
}

fn copy_optional_symbol(source: &Path, destination: &Path) -> Result<()> {
    if source.is_file() {
        copy_file(source, destination)?;
    }
    Ok(())
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
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

fn git_user_name(root: &Path) -> Option<String> {
    let output = Command::new("git").arg("config").arg("user.name").current_dir(root).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let user_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if user_name.is_empty() { None } else { Some(user_name) }
}
