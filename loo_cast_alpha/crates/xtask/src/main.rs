mod commands;
mod utils;

use anyhow::{Context, Result, bail};
use std::env;
use xshell::Shell;

use crate::commands::audit::audit;
use crate::commands::build::build;
use crate::commands::cloc::run_cloc;
use crate::commands::deploy::deploy_stub;
use crate::commands::docs::build_docs;
use crate::commands::gource::run_gource;
use crate::commands::help::print_help;
use crate::commands::package::package;
use crate::commands::run::run;
use crate::commands::setup_sdk::setup_sdk;
use crate::utils::build_target::BuildTarget;
use crate::utils::profile::Profile;

const CORE_ENGINE_CRATE: &str = "core_engine";
const XTASK_CRATE: &str = "xtask";
const LINUX_RELEASE_TARGET: &str = "x86_64-unknown-linux-gnu";
const WINDOWS_RELEASE_TARGET: &str = "x86_64-pc-windows-msvc";

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
        "build" => build(&sh, Profile::Fastdev, BuildTarget::Host)?,
        "build_dev" => build(&sh, Profile::Dev, BuildTarget::Host)?,
        "build_fastdev" => build(&sh, Profile::Fastdev, BuildTarget::Host)?,
        "build_release" => build(&sh, Profile::Release, BuildTarget::Host)?,
        "build_linux_release" => build(&sh, Profile::Release, BuildTarget::LinuxRelease)?,
        "build_windows_release" => build(&sh, Profile::Release, BuildTarget::WindowsRelease)?,
        "package" => package(&sh, &root, Profile::Fastdev, BuildTarget::Host)?,
        "package_dev" => package(&sh, &root, Profile::Dev, BuildTarget::Host)?,
        "package_fastdev" => package(&sh, &root, Profile::Fastdev, BuildTarget::Host)?,
        "package_release" => package(&sh, &root, Profile::Release, BuildTarget::Host)?,
        "package_linux_release" => package(&sh, &root, Profile::Release, BuildTarget::LinuxRelease)?,
        "package_windows_release" => package(&sh, &root, Profile::Release, BuildTarget::WindowsRelease)?,
        "run" => run(&sh, &root, Profile::Fastdev)?,
        "run_dev" => run(&sh, &root, Profile::Dev)?,
        "run_fastdev" => run(&sh, &root, Profile::Fastdev)?,
        "run_release" => run(&sh, &root, Profile::Release)?,
        "audit" => audit(&root)?,
        "build_docs" => build_docs(&sh, false)?,
        "open_docs" => build_docs(&sh, true)?,
        "setup_sdk" => setup_sdk(&root)?,
        "cloc" => run_cloc(&root)?,
        "gource" => run_gource(&root)?,
        "deploy" => deploy_stub(),
        other => bail!("unknown xtask command '{other}'. Use `cargo xtask help`."),
    }

    Ok(())
}
