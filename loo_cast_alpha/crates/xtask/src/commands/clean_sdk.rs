use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;

use crate::commands::setup_sdk::{PRE_COMMIT_HOOK_CONTENTS, PRE_PUSH_HOOK_CONTENTS};
use crate::utils::git::find_git_dir;

pub fn clean_sdk(root: &Path) -> Result<()> {
    CleanSdk { root }.run()
}

struct CleanSdk<'a> {
    root: &'a Path,
}

impl CleanSdk<'_> {
    fn run(&self) -> Result<()> {
        println!("Running SDK cleanup steps");
        println!();
        println!("[1/1] Remove managed git hooks");
        let summary = self.clean_git_hooks()?;
        println!();
        println!(
            "SDK cleanup complete. Removed: {}, retained (unmanaged): {}, missing: {}.",
            summary.removed, summary.retained_unmanaged, summary.missing
        );
        Ok(())
    }

    fn clean_git_hooks(&self) -> Result<HookCleanupSummary> {
        let Some(git_dir) = find_git_dir(self.root) else {
            bail!("failed to find a .git directory from '{}'", self.root.display());
        };
        let hooks_dir = git_dir.join("hooks");
        let managed_hooks = [("pre-commit", PRE_COMMIT_HOOK_CONTENTS), ("pre-push", PRE_PUSH_HOOK_CONTENTS)];

        let mut summary = HookCleanupSummary::default();
        for (hook_name, expected_contents) in managed_hooks {
            let hook_path = hooks_dir.join(hook_name);
            if !hook_path.exists() {
                println!("missing {hook_name} hook: {}", hook_path.display());
                summary.missing += 1;
                continue;
            }

            let actual_contents = fs::read_to_string(&hook_path).with_context(|| format!("failed to read '{}'", hook_path.display()))?;
            if is_managed_hook(&actual_contents, expected_contents) {
                fs::remove_file(&hook_path).with_context(|| format!("failed to remove '{}'", hook_path.display()))?;
                println!("removed managed {hook_name} hook: {}", hook_path.display());
                summary.removed += 1;
            } else {
                println!(
                    "retained unmanaged {hook_name} hook: {} (contents differ from setup_sdk managed hook)",
                    hook_path.display()
                );
                summary.retained_unmanaged += 1;
            }
        }

        Ok(summary)
    }
}

#[derive(Default)]
struct HookCleanupSummary {
    removed: usize,
    retained_unmanaged: usize,
    missing: usize,
}

fn is_managed_hook(actual_contents: &str, expected_contents: &str) -> bool {
    normalize_newlines(actual_contents).trim_end() == normalize_newlines(expected_contents).trim_end()
}

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}
