use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::utils::build_target::BuildTarget;
use crate::utils::fs::sdk_root_dir;
use crate::utils::profile::Profile;

pub fn clean_sdk(root: &Path, profile: Profile, target: BuildTarget) -> Result<()> {
    let sdk_root = sdk_root_dir(root, profile, target);
    if sdk_root.exists() {
        fs::remove_dir_all(&sdk_root).with_context(|| format!("failed to remove '{}'", sdk_root.display()))?;
    }
    println!("cleaned sdk runtime/toolchain root: {}", sdk_root.display());
    Ok(())
}
