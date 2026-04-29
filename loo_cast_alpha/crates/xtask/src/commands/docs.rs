use anyhow::Result;
use xshell::{Shell, cmd};

pub fn build_docs(sh: &Shell, open: bool) -> Result<()> {
    if open {
        cmd!(sh, "cargo doc --open --no-deps").run()?;
    } else {
        cmd!(sh, "cargo doc --no-deps").run()?;
    }
    Ok(())
}
