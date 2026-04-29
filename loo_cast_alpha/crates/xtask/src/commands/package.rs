use anyhow::Result;
use std::path::Path;
use xshell::Shell;

use crate::CORE_ENGINE_CRATE;
use crate::commands::build::build;
use crate::utils::build_target::BuildTarget;
use crate::utils::fs::{
    cargo_artifact_dir, clean_dir, copy_dir_recursive, copy_file, copy_optional_symbol, discover_workspace_crates, executable_name, staged_build_dir,
};
use crate::utils::profile::Profile;

pub fn package(sh: &Shell, root: &Path, profile: Profile, target: BuildTarget) -> Result<()> {
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
