use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;
use std::process::Command;

struct AuditStep {
    name: String,
    command: String,
}

const AUDIT_CONFIG_PATH: &str = "crates/xtask/audit_steps.toml";

pub fn audit(root: &Path) -> Result<()> {
    let config_path = root.join(AUDIT_CONFIG_PATH);
    let steps = parse_audit_config(&config_path)?;

    println!("Running audit ({} steps)", steps.len());
    for (index, step) in steps.iter().enumerate() {
        println!();
        println!("[{}/{}] {}", index + 1, steps.len(), step.name);
        println!("> {}", step.command);

        let command_parts: Vec<&str> = step.command.split_whitespace().collect();
        let Some((program, args)) = command_parts.split_first() else {
            bail!("audit config error: step '{}' has an empty command", step.name);
        };

        let status = Command::new(program)
            .args(args)
            .current_dir(root)
            .status()
            .with_context(|| format!("failed to execute `{}`", step.command))?;

        if !status.success() {
            bail!("audit failed in step {}: {} (`{}` exited with {status})", index + 1, step.name, step.command);
        }

        println!("ok: {}", step.name);
    }

    println!();
    println!("Audit passed.");
    Ok(())
}

fn parse_audit_config(config_path: &Path) -> Result<Vec<AuditStep>> {
    let raw = fs::read_to_string(config_path).with_context(|| format!("failed to read '{}'", config_path.display()))?;

    let mut steps = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_command: Option<String> = None;

    for (line_idx, raw_line) in raw.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line == "[[steps]]" {
            finalize_step(&mut steps, &mut current_name, &mut current_command, config_path, line_idx + 1)?;
            continue;
        }

        if let Some(name) = line.strip_prefix("name =") {
            current_name = Some(parse_toml_string(name, config_path, line_idx + 1, "name")?);
            continue;
        }

        if let Some(command) = line.strip_prefix("command =") {
            current_command = Some(parse_toml_string(command, config_path, line_idx + 1, "command")?);
            continue;
        }

        bail!("invalid audit config line at {}:{}: '{}'", config_path.display(), line_idx + 1, line);
    }

    finalize_step(&mut steps, &mut current_name, &mut current_command, config_path, raw.lines().count() + 1)?;

    if steps.is_empty() {
        bail!("audit config '{}' contains no steps", config_path.display());
    }

    Ok(steps)
}

fn finalize_step(
    steps: &mut Vec<AuditStep>,
    current_name: &mut Option<String>,
    current_command: &mut Option<String>,
    config_path: &Path,
    line_number: usize,
) -> Result<()> {
    match (current_name.take(), current_command.take()) {
        (None, None) => Ok(()),
        (Some(name), Some(command)) => {
            if command.split_whitespace().next().is_none() {
                bail!("audit config step '{}' has an empty command at {}:{}", name, config_path.display(), line_number);
            }
            steps.push(AuditStep { name, command });
            Ok(())
        }
        (Some(name), None) => bail!("audit config step '{}' is missing 'command' at {}:{}", name, config_path.display(), line_number),
        (None, Some(_)) => bail!("audit config step is missing 'name' at {}:{}", config_path.display(), line_number),
    }
}

fn parse_toml_string(raw_value: &str, config_path: &Path, line_number: usize, key: &str) -> Result<String> {
    let value = raw_value.trim();
    if !value.starts_with('"') || !value.ends_with('"') || value.len() < 2 {
        bail!("expected quoted string for '{}' at {}:{}", key, config_path.display(), line_number);
    }
    Ok(value[1..value.len() - 1].to_string())
}
