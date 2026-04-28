use anyhow::{bail, Result};
use xshell::{cmd, Shell};

use crate::{LINUX_RELEASE_TARGET, WINDOWS_RELEASE_TARGET, XTASK_CRATE};
use crate::utils::build_target::BuildTarget;
use crate::utils::profile::Profile;

pub fn build(sh: &Shell, profile: Profile, target: BuildTarget) -> Result<()> {
    match (profile, target) {
        (Profile::Dev, BuildTarget::Host) => cmd!(sh, "cargo build --workspace --exclude {XTASK_CRATE}").run()?,
        (Profile::Fastdev, BuildTarget::Host) => cmd!(sh, "cargo build --workspace --exclude {XTASK_CRATE} --profile fastdev").run()?,
        (Profile::Release, BuildTarget::Host) => cmd!(sh, "cargo build --workspace --exclude {XTASK_CRATE} --release").run()?,
        (Profile::Release, BuildTarget::LinuxRelease) => {
            cmd!(sh, "cargo build --workspace --exclude {XTASK_CRATE} --release --target {LINUX_RELEASE_TARGET}").run()?
        }
        (Profile::Release, BuildTarget::WindowsRelease) => cmd!(
            sh,
            "cargo build --workspace --exclude {XTASK_CRATE} --release --target {WINDOWS_RELEASE_TARGET}"
        )
            .run()?,
        _ => bail!("only release profile supports explicit target tasks"),
    }
    Ok(())
}