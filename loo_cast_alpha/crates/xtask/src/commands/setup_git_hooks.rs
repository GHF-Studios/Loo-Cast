use anyhow::{Context, Result, bail};
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::utils::git::find_git_dir;

const PRE_PUSH_HOOK_CONTENT: &str = "cargo xtask audit\n";

pub fn setup_git_hooks(root: &Path) -> Result<()> {
    let Some(git_dir) = find_git_dir(root) else {
        bail!("failed to find a .git directory from '{}'", root.display());
    };
    let hooks_dir = git_dir.join("hooks");
    fs::create_dir_all(&hooks_dir).with_context(|| format!("failed to create '{}'", hooks_dir.display()))?;

    let pre_push_hook = hooks_dir.join("pre-push");
    if pre_push_hook.exists() {
        fs::remove_file(&pre_push_hook).with_context(|| format!("failed to remove '{}'", pre_push_hook.display()))?;
    }
    fs::write(&pre_push_hook, PRE_PUSH_HOOK_CONTENT).with_context(|| format!("failed to write '{}'", pre_push_hook.display()))?;

    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(&pre_push_hook)
            .with_context(|| format!("failed to read permissions for '{}'", pre_push_hook.display()))?
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&pre_push_hook, permissions).with_context(|| format!("failed to set execute permissions on '{}'", pre_push_hook.display()))?;
    }

    println!("installed pre-push hook: {}", pre_push_hook.display());
    println!("hook command: cargo xtask audit");
    Ok(())
}
