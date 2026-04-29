use anyhow::{Context, Result, bail};
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::utils::git::find_git_dir;

pub fn setup_sdk(root: &Path) -> Result<()> {
    SetupSdk { root }.run()
}

struct SetupSdk<'a> {
    root: &'a Path,
}

impl SetupSdk<'_> {
    fn run(&self) -> Result<()> {
        println!("Running SDK setup steps");
        println!();
        println!("[1/1] Install git hooks");
        self.setup_git_hooks()?;
        println!();
        println!("SDK setup complete.");
        Ok(())
    }

    fn setup_git_hooks(&self) -> Result<()> {
        let Some(git_dir) = find_git_dir(self.root) else {
            bail!("failed to find a .git directory from '{}'", self.root.display());
        };
        let hooks_dir = git_dir.join("hooks");
        fs::create_dir_all(&hooks_dir).with_context(|| format!("failed to create '{}'", hooks_dir.display()))?;

        let pre_push_hook = hooks_dir.join("pre-push");
        if pre_push_hook.exists() {
            fs::remove_file(&pre_push_hook).with_context(|| format!("failed to remove '{}'", pre_push_hook.display()))?;
        }
        fs::write(&pre_push_hook, "cargo xtask audit\n").with_context(|| format!("failed to write '{}'", pre_push_hook.display()))?;

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
}
