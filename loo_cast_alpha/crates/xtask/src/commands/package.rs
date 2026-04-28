use anyhow::{Error, Result};
use std::fs;
use std::path::Path;
use xshell::Shell;

use crate::commands::build::build;
use crate::utils::build_target::BuildTarget;
use crate::utils::fs::{
    cargo_artifact_dir, copy_dir_recursive, copy_file, copy_optional_symbol, discover_workspace_crates, executable_name, sdk_root_dir, staged_build_dir,
};
use crate::utils::options::XtaskOptions;
use crate::utils::profile::Profile;
use crate::{CORE_ENGINE_CRATE, LAUNCHER_CRATE};

pub fn package(sh: &Shell, root: &Path, profile: Profile, target: BuildTarget, options: XtaskOptions) -> Result<()> {
    let build_dir = staged_build_dir(root, profile, target);
    fs::create_dir_all(&build_dir)?;

    let sdk_root = sdk_root_dir(root, profile, target);
    if options.clean_sdk {
        reset_dir(&sdk_root)?;
    } else {
        fs::create_dir_all(&sdk_root)?;
    }

    stage_sdk_seed(root, &build_dir, options.clean_sdk)?;
    stage_sdk_workspace(root, &build_dir, options.clean_sdk)?;
    stage_sdk_bootstrap(root, &build_dir, options.clean_sdk)?;

    build(sh, root, profile, target, options, true)?;

    let artifact_dir = cargo_artifact_dir(root, profile, target, options.use_vendored_toolchain);
    let core_engine_name = executable_name(target);
    let source_executable = artifact_dir.join(core_engine_name);
    let staged_executable = build_dir.join(core_engine_name);

    copy_or_skip_if_busy(&source_executable, &staged_executable, "game executable")?;
    copy_optional_symbol(&artifact_dir.join("core_engine.pdb"), &build_dir.join("core_engine.pdb"))?;
    copy_optional_symbol(&artifact_dir.join("core_engine.debug"), &build_dir.join("core_engine.debug"))?;
    let launcher_name = bin_name(LAUNCHER_CRATE, target);
    copy_or_skip_if_busy(&artifact_dir.join(&launcher_name), &build_dir.join(&launcher_name), "launcher executable")?;
    copy_optional_symbol(&artifact_dir.join("launcher.pdb"), &build_dir.join("launcher.pdb"))?;
    copy_optional_symbol(&artifact_dir.join("launcher.debug"), &build_dir.join("launcher.debug"))?;
    stage_assets(root, &build_dir)?;

    println!("packaged game executable: {}", staged_executable.display());
    println!("packaged launcher executable: {}", build_dir.join(launcher_name).display());
    println!("packaged assets root: {}", build_dir.join("assets").display());
    println!("packaged sdk workspace root: {}", build_dir.join("sdk").join("workspace").display());
    println!("packaged sdk bootstrap root: {}", build_dir.join("sdk").join("bootstrap").display());
    println!("packaged sdk seed root: {}", build_dir.join("sdk").join("seed").display());
    Ok(())
}

fn stage_assets(root: &Path, build_dir: &Path) -> Result<()> {
    let assets_root = build_dir.join("assets");
    reset_dir(&assets_root)?;

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

fn stage_sdk_workspace(root: &Path, build_dir: &Path, clean_sdk: bool) -> Result<()> {
    let sdk_root = build_dir.join("sdk");
    let workspace_root = sdk_root.join("workspace");
    if clean_sdk {
        reset_dir(&workspace_root)?;
    } else {
        fs::create_dir_all(&workspace_root)?;
    }

    copy_file(&root.join("Cargo.toml"), &workspace_root.join("Cargo.toml"))?;
    copy_file(&root.join("Cargo.lock"), &workspace_root.join("Cargo.lock"))?;

    let toolchain_file = root.join("rust-toolchain.toml");
    if toolchain_file.is_file() {
        copy_file(&toolchain_file, &workspace_root.join("rust-toolchain.toml"))?;
    }

    let cargo_config_dir = root.join(".cargo");
    if cargo_config_dir.is_dir() {
        copy_dir_recursive(&cargo_config_dir, &workspace_root.join(".cargo"))?;
    }

    let crates_dir = root.join("crates");
    if crates_dir.is_dir() {
        copy_dir_recursive(&crates_dir, &workspace_root.join("crates"))?;
    }

    let bootstrap_dir = sdk_root.join("bootstrap");
    fs::create_dir_all(bootstrap_dir)?;
    Ok(())
}

fn stage_sdk_seed(root: &Path, build_dir: &Path, clean_sdk: bool) -> Result<()> {
    let seed_src = root.join("vendor").join("sdk-seed");
    let seed_dst = build_dir.join("sdk").join("seed");
    if clean_sdk {
        reset_dir(&seed_dst)?;
    } else {
        fs::create_dir_all(&seed_dst)?;
    }
    if seed_src.is_dir() {
        copy_dir_recursive(&seed_src, &seed_dst)?;
    }
    Ok(())
}

fn stage_sdk_bootstrap(root: &Path, build_dir: &Path, clean_sdk: bool) -> Result<()> {
    let vendor_root = root.join("vendor").join("rustup");
    let bootstrap_root = build_dir.join("sdk").join("bootstrap");
    if clean_sdk {
        reset_dir(&bootstrap_root)?;
    } else {
        fs::create_dir_all(&bootstrap_root)?;
    }

    let linux_src = vendor_root.join("x86_64-unknown-linux-gnu").join("rustup-init");
    if linux_src.is_file() {
        copy_file(&linux_src, &bootstrap_root.join("rustup-init"))?;
    }

    let windows_src = vendor_root.join("x86_64-pc-windows-gnu").join("rustup-init.exe");
    if windows_src.is_file() {
        copy_file(&windows_src, &bootstrap_root.join("rustup-init.exe"))?;
    }

    Ok(())
}

fn bin_name(name: &str, target: BuildTarget) -> String {
    if target.is_windows() { format!("{name}.exe") } else { name.to_string() }
}

fn reset_dir(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)?;
    Ok(())
}

fn copy_or_skip_if_busy(source: &Path, destination: &Path, label: &str) -> Result<()> {
    match copy_file(source, destination) {
        Ok(()) => Ok(()),
        Err(err) => {
            if is_text_file_busy(&err) {
                eprintln!("warning: skipping copy for {label} because destination is busy: {}", destination.display());
                Ok(())
            } else {
                Err(err)
            }
        }
    }
}

fn is_text_file_busy(err: &Error) -> bool {
    err.chain().any(|cause| {
        cause
            .downcast_ref::<std::io::Error>()
            .and_then(std::io::Error::raw_os_error)
            .is_some_and(|code| code == 26 || code == 32)
    })
}
