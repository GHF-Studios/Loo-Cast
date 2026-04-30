use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::Command;

struct AuditStep {
    name: &'static str,
    program: &'static str,
    args: &'static [&'static str],
}

const AUDIT_STEPS: &[AuditStep] = &[
    AuditStep {
        name: "Formatting check",
        program: "cargo",
        args: &["fmt", "--all", "--check"],
    },
    AuditStep {
        name: "Clippy lint check",
        program: "cargo",
        args: &["clippy", "--workspace", "--all-targets", "--no-deps", "--", "-D", "warnings"],
    },
    AuditStep {
        name: "Workspace tests",
        program: "cargo",
        args: &["test", "--workspace", "--lib", "--bins"],
    },
];

pub fn audit(root: &Path) -> Result<()> {
    println!("Running audit ({} steps)", AUDIT_STEPS.len());
    for (index, step) in AUDIT_STEPS.iter().enumerate() {
        println!();
        println!("[{}/{}] {}", index + 1, AUDIT_STEPS.len(), step.name);
        let rendered = render_command(step.program, step.args);
        println!("> {rendered}");

        let status = Command::new(step.program)
            .args(step.args)
            .current_dir(root)
            .status()
            .with_context(|| format!("failed to execute `{rendered}`"))?;

        if !status.success() {
            bail!("audit failed in step {}: {} (`{}` exited with {status})", index + 1, step.name, rendered);
        }

        println!("ok: {}", step.name);
    }

    println!();
    println!("Audit passed.");
    Ok(())
}

fn render_command(program: &str, args: &[&str]) -> String {
    if args.is_empty() {
        program.to_string()
    } else {
        format!("{program} {}", args.join(" "))
    }
}
