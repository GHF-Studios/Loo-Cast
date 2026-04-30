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

        self.install_hook(
            &hooks_dir,
            "pre-commit",
            "#!/usr/bin/env sh\nset -e\ncargo fmt --manifest-path loo_cast_alpha/Cargo.toml --all\n",
        )?;
        self.install_hook(
            &hooks_dir,
            "pre-push",
            "#!/usr/bin/env sh\nset -e\ncargo run --manifest-path loo_cast_alpha/Cargo.toml -p xtask -- audit\n",
        )?;
        Ok(())
    }

    fn install_hook(&self, hooks_dir: &Path, hook_name: &str, hook_contents: &str) -> Result<()> {
        let hook_path = hooks_dir.join(hook_name);
        if hook_path.exists() {
            fs::remove_file(&hook_path).with_context(|| format!("failed to remove '{}'", hook_path.display()))?;
        }
        fs::write(&hook_path, hook_contents).with_context(|| format!("failed to write '{}'", hook_path.display()))?;

        #[cfg(unix)]
        {
            let mut permissions = fs::metadata(&hook_path)
                .with_context(|| format!("failed to read permissions for '{}'", hook_path.display()))?
                .permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&hook_path, permissions).with_context(|| format!("failed to set execute permissions on '{}'", hook_path.display()))?;
        }

        println!("installed {hook_name} hook: {}", hook_path.display());
        println!("hook command: {}", hook_contents.trim_end());
        Ok(())
    }
}
