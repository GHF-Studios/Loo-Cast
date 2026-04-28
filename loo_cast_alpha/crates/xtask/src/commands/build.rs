use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use xshell::Shell;

use crate::XTASK_CRATE;
use crate::utils::build_target::BuildTarget;
use crate::utils::fs::{sdk_cargo_bin, sdk_cargo_home_dir, sdk_root_dir, sdk_rustup_home_dir, sdk_target_dir};
use crate::utils::options::XtaskOptions;
use crate::utils::profile::Profile;

struct VendoredSdk {
    cargo_bin: PathBuf,
    cargo_home: PathBuf,
    rustup_home: PathBuf,
    target_dir: PathBuf,
}

pub fn build(_sh: &Shell, root: &Path, profile: Profile, target: BuildTarget, options: XtaskOptions, allow_toolchain_mutation: bool) -> Result<()> {
    let mut args = vec!["build".to_string(), "--workspace".to_string(), "--exclude".to_string(), XTASK_CRATE.to_string()];

    match (profile, target) {
        (Profile::Fastdev, BuildTarget::Host) => {
            args.push("--profile".to_string());
            args.push("fastdev".to_string());
        }
    }

    let mut command = if options.use_vendored_toolchain {
        let sdk = ensure_vendored_sdk(root, profile, target, allow_toolchain_mutation)?;
        let mut command = Command::new(&sdk.cargo_bin);
        command.env("RUSTUP_HOME", &sdk.rustup_home);
        command.env("CARGO_HOME", &sdk.cargo_home);
        command.env("CARGO_TARGET_DIR", &sdk.target_dir);
        command
    } else {
        Command::new("cargo")
    };

    command.current_dir(root).args(&args);

    let status = command.status().with_context(|| {
        let mode = if options.use_vendored_toolchain { "vendored SDK cargo" } else { "host cargo" };
        format!("failed to start {mode} build command")
    })?;

    if !status.success() {
        bail!("build command exited with status {status}");
    }
    Ok(())
}

fn ensure_vendored_sdk(root: &Path, profile: Profile, target: BuildTarget, allow_toolchain_mutation: bool) -> Result<VendoredSdk> {
    let sdk_root = sdk_root_dir(root, profile, target);
    let cargo_home = sdk_cargo_home_dir(root, profile, target);
    let rustup_home = sdk_rustup_home_dir(root, profile, target);
    let target_dir = sdk_target_dir(root, profile, target);
    fs::create_dir_all(&sdk_root)?;
    fs::create_dir_all(&cargo_home)?;
    fs::create_dir_all(&rustup_home)?;
    fs::create_dir_all(&target_dir)?;

    let rustup_bin = cargo_home.join("bin").join(host_binary_name("rustup"));
    let cargo_bin = sdk_cargo_bin(root, profile, target);
    if !rustup_bin.is_file() {
        if !allow_toolchain_mutation {
            bail!(
                "vendored rustup is missing at '{}'. This SDK is locked; rebuild SDK from admin layer (`cargo xtask build_sdk`).",
                rustup_bin.display()
            );
        }
        bootstrap_vendored_rustup(root, &sdk_root, &rustup_home, &cargo_home)?;
    }
    if !rustup_bin.is_file() {
        bail!("vendored rustup is still missing at '{}'", rustup_bin.display());
    }

    let toolchain = read_toolchain_channel(root);
    if !toolchain_installed(&rustup_bin, &rustup_home, &cargo_home, &toolchain)? {
        if !allow_toolchain_mutation {
            bail!(
                "toolchain '{}' is missing in vendored SDK. Rebuild SDK from admin layer (`cargo xtask build_sdk`).",
                toolchain
            );
        }
        run_rustup(
            &rustup_bin,
            &rustup_home,
            &cargo_home,
            root,
            &["toolchain", "install", &toolchain, "--profile", "minimal", "--allow-downgrade"],
        )?;
    }

    let target_triple = build_target_triple(target)?;
    if !target_installed(&rustup_bin, &rustup_home, &cargo_home, &toolchain, &target_triple)? {
        if !allow_toolchain_mutation {
            bail!(
                "target '{}' is missing in vendored SDK. Rebuild SDK from admin layer (`cargo xtask build_sdk`).",
                target_triple
            );
        }
        run_rustup(
            &rustup_bin,
            &rustup_home,
            &cargo_home,
            root,
            &["target", "add", &target_triple, "--toolchain", &toolchain],
        )?;
    }

    if needs_cranelift_backend(root) && !component_installed(&rustup_bin, &rustup_home, &cargo_home, &toolchain, "rustc-codegen-cranelift-preview")? {
        if !allow_toolchain_mutation {
            bail!("component 'rustc-codegen-cranelift-preview' is missing in vendored SDK. Rebuild SDK from admin layer (`cargo xtask build_sdk`).");
        }
        run_rustup(
            &rustup_bin,
            &rustup_home,
            &cargo_home,
            root,
            &["component", "add", "rustc-codegen-cranelift-preview", "--toolchain", &toolchain],
        )?;
    }

    if !cargo_bin.is_file() {
        bail!("vendored cargo is missing at '{}'", cargo_bin.display());
    }

    Ok(VendoredSdk {
        cargo_bin,
        cargo_home,
        rustup_home,
        target_dir,
    })
}

fn bootstrap_vendored_rustup(root: &Path, sdk_root: &Path, rustup_home: &Path, cargo_home: &Path) -> Result<()> {
    let rustup_init = find_rustup_init(root, sdk_root)?;
    run_command(
        &rustup_init,
        &["-y", "--default-toolchain", "none", "--profile", "minimal", "--no-modify-path"],
        sdk_root,
        Some((rustup_home, cargo_home)),
        true,
    )
}

fn find_rustup_init(root: &Path, sdk_root: &Path) -> Result<PathBuf> {
    let staged = sdk_root.join("bootstrap").join(host_binary_name("rustup-init"));
    if staged.is_file() {
        return Ok(staged);
    }

    let Some(vendor_subdir) = host_rustup_vendor_subdir() else {
        bail!("unsupported host for vendored rustup bootstrap");
    };
    let vendored = root.join("vendor").join("rustup").join(vendor_subdir).join(host_binary_name("rustup-init"));
    if vendored.is_file() {
        return Ok(vendored);
    }

    bail!("missing rustup bootstrap binary. expected '{}' or '{}'", staged.display(), vendored.display());
}

fn run_rustup(rustup_bin: &Path, rustup_home: &Path, cargo_home: &Path, cwd: &Path, args: &[&str]) -> Result<()> {
    run_command(rustup_bin, args, cwd, Some((rustup_home, cargo_home)), false)
}

fn run_command(program: &Path, args: &[&str], cwd: &Path, sdk_homes: Option<(&Path, &Path)>, skip_path_check: bool) -> Result<()> {
    let mut command = Command::new(program);
    command.args(args).current_dir(cwd);
    if let Some((rustup_home, cargo_home)) = sdk_homes {
        command.env("RUSTUP_HOME", rustup_home);
        command.env("CARGO_HOME", cargo_home);
        if skip_path_check {
            command.env("RUSTUP_INIT_SKIP_PATH_CHECK", "yes");
        }
    }
    let status = command
        .status()
        .with_context(|| format!("failed to run '{}'", preview_command(program, args)))?;
    if !status.success() {
        bail!("command failed: '{}' (status {status})", preview_command(program, args));
    }
    Ok(())
}

fn capture_command_stdout(program: &Path, args: &[&str], cwd: &Path, rustup_home: &Path, cargo_home: &Path) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .env("RUSTUP_HOME", rustup_home)
        .env("CARGO_HOME", cargo_home)
        .output()
        .with_context(|| format!("failed to run '{}'", preview_command(program, args)))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "command failed: '{}' (status {})\n{}",
            preview_command(program, args),
            output.status,
            stderr.trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn toolchain_installed(rustup_bin: &Path, rustup_home: &Path, cargo_home: &Path, toolchain: &str) -> Result<bool> {
    let out = capture_command_stdout(rustup_bin, &["toolchain", "list"], cargo_home, rustup_home, cargo_home)?;
    let prefix = format!("{toolchain}-");
    Ok(out
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .any(|entry| entry == toolchain || entry.starts_with(&prefix)))
}

fn target_installed(rustup_bin: &Path, rustup_home: &Path, cargo_home: &Path, toolchain: &str, target: &str) -> Result<bool> {
    let out = capture_command_stdout(
        rustup_bin,
        &["target", "list", "--toolchain", toolchain, "--installed"],
        cargo_home,
        rustup_home,
        cargo_home,
    )?;
    Ok(out.lines().filter_map(|line| line.split_whitespace().next()).any(|entry| entry == target))
}

fn component_installed(rustup_bin: &Path, rustup_home: &Path, cargo_home: &Path, toolchain: &str, component: &str) -> Result<bool> {
    let out = capture_command_stdout(
        rustup_bin,
        &["component", "list", "--toolchain", toolchain, "--installed"],
        cargo_home,
        rustup_home,
        cargo_home,
    )?;
    let prefix = format!("{component}-");
    Ok(out
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .any(|entry| entry == component || entry.starts_with(&prefix)))
}

fn needs_cranelift_backend(root: &Path) -> bool {
    let config_path = root.join(".cargo").join("config.toml");
    let Ok(config) = fs::read_to_string(config_path) else {
        return false;
    };
    config.contains("codegen-backend = \"cranelift\"")
}

fn read_toolchain_channel(root: &Path) -> String {
    let path = root.join("rust-toolchain.toml");
    let Ok(contents) = fs::read_to_string(path) else {
        return "nightly".to_string();
    };
    for line in contents.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("channel") {
            continue;
        }
        let Some((_, rhs)) = trimmed.split_once('=') else {
            continue;
        };
        let rhs = rhs.trim();
        let Some(start) = rhs.find('"') else {
            continue;
        };
        let remaining = &rhs[start + 1..];
        let Some(end) = remaining.find('"') else {
            continue;
        };
        let value = &remaining[..end];
        if !value.is_empty() {
            return value.to_string();
        }
    }
    "nightly".to_string()
}

fn build_target_triple(target: BuildTarget) -> Result<String> {
    if let Some(triple) = target.triple() {
        return Ok(triple.to_string());
    }
    if cfg!(target_os = "linux") {
        return Ok("x86_64-unknown-linux-gnu".to_string());
    }
    if cfg!(target_os = "windows") {
        return Ok("x86_64-pc-windows-msvc".to_string());
    }
    bail!("unsupported host OS for default build target")
}

fn preview_command(program: &Path, args: &[&str]) -> String {
    let mut cmd = program.display().to_string();
    if !args.is_empty() {
        cmd.push(' ');
        cmd.push_str(&args.join(" "));
    }
    cmd
}

fn host_binary_name(name: &str) -> String {
    if cfg!(target_os = "windows") {
        format!("{name}.exe")
    } else {
        name.to_string()
    }
}

fn host_rustup_vendor_subdir() -> Option<&'static str> {
    if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        Some("x86_64-unknown-linux-gnu")
    } else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        Some("x86_64-pc-windows-gnu")
    } else {
        None
    }
}
