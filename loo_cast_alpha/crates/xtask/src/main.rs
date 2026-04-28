mod commands;
mod utils;

use anyhow::{Context, Result, bail};
use std::env;
use std::path::{Path, PathBuf};
use xshell::Shell;

use crate::commands::admin::clean_sdk;
use crate::commands::build::build;
use crate::commands::cloc::run_cloc;
use crate::commands::deploy::deploy_stub;
use crate::commands::gource::run_gource;
use crate::commands::help::print_help;
use crate::commands::package::package;
use crate::commands::sdk;
use crate::utils::build_target::BuildTarget;
use crate::utils::live_repo::find_live_repo_root;
use crate::utils::options::XtaskOptions;
use crate::utils::profile::Profile;

const CORE_ENGINE_CRATE: &str = "core_engine";
const LAUNCHER_CRATE: &str = "launcher";
const XTASK_CRATE: &str = "xtask";

fn main() -> Result<()> {
    let sh = Shell::new().context("failed to create shell")?;
    let cwd = env::current_dir().context("failed to get current directory")?;
    sh.change_dir(&cwd);

    let mut args = env::args().skip(1);
    let Some(task) = args.next() else {
        print_help();
        return Ok(());
    };
    let options = parse_options(task.as_str(), args)?;

    match task.as_str() {
        "help" => print_help(),
        // SDK/modding/end-user layer (locked toolchain)
        "clean" => sdk::clean(&sh, &cwd, Profile::Fastdev, BuildTarget::Host, options)?,
        "build" => build(&sh, &cwd, Profile::Fastdev, BuildTarget::Host, options, false)?,
        "run" => sdk::run(&sh, &cwd, Profile::Fastdev, BuildTarget::Host, options)?,
        "debug" => sdk::debug(&sh, &cwd, Profile::Fastdev, BuildTarget::Host, options)?,
        "contribute" => {
            let live_root = require_live_repo_root(&cwd, "contribute")?;
            sdk::contribute(&live_root, options)?;
        }
        // Admin/developer layer (can mutate toolchain/sdk)
        "clean_sdk" => {
            let live_root = require_live_repo_root(&cwd, "clean_sdk")?;
            clean_sdk(&live_root, Profile::Fastdev, BuildTarget::Host)?;
        }
        "build_sdk" => {
            let live_root = require_live_repo_root(&cwd, "build_sdk")?;
            let mut sdk_options = options;
            sdk_options.clean_sdk = true;
            package(&sh, &live_root, Profile::Fastdev, BuildTarget::Host, sdk_options)?;
        }
        "cloc" => run_cloc(&cwd)?,
        "gource" => run_gource(&cwd)?,
        "deploy" => deploy_stub(),
        other => bail!("unknown xtask command '{other}'. Use `cargo xtask help`."),
    }

    Ok(())
}

fn require_live_repo_root(cwd: &Path, task: &str) -> Result<PathBuf> {
    find_live_repo_root(cwd).ok_or_else(|| {
        anyhow::anyhow!(
            "task '{}' requires running inside the live repo checkout (with .git + crates/xtask). current dir: '{}'",
            task,
            cwd.display()
        )
    })
}

fn parse_options(task: &str, args: impl Iterator<Item = String>) -> Result<XtaskOptions> {
    let mut options = XtaskOptions::default();
    for arg in args {
        match arg.as_str() {
            "--clean-sdk" | "--CLEAN_SDK" | "--clean_sdk" => options.clean_sdk = true,
            "--host-toolchain" => options.use_vendored_toolchain = false,
            "--vendored-toolchain" => options.use_vendored_toolchain = true,
            "--contribute-apply" => options.contribute_apply = true,
            "--contribute-no-push" => options.contribute_push = false,
            other => bail!("unknown option '{other}' for task '{task}'. Use `cargo xtask help`."),
        }
    }
    Ok(options)
}
