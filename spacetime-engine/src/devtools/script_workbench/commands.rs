//! Developer console ingress for script workspace operations.

use super::workspace::DeveloperScriptWorkbench;
use crate::console::{
    AppConsoleExt, ConsoleCommandInvocation, ConsoleCommandResult, ConsoleCommandSpec,
};
use bevy::prelude::*;

pub(crate) fn configure(app: &mut App) {
    app.init_resource::<DeveloperScriptWorkbench>()
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script status",
                aliases: &["script status"],
                usage: "debug script status",
                summary: "Show the active script buffer/runtime state.",
            },
            script_status_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script compile",
                aliases: &["script compile"],
                usage: "debug script compile",
                summary: "Compile/validate the active script buffer.",
            },
            script_compile_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script commit",
                aliases: &["script commit"],
                usage: "debug script commit",
                summary: "Atomically commit the active script buffer as a runtime revision.",
            },
            script_commit_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script revert",
                aliases: &["script revert"],
                usage: "debug script revert",
                summary: "Revert the active editor buffer to its committed runtime revision.",
            },
            script_revert_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "debug script save",
                aliases: &["script save"],
                usage: "debug script save",
                summary: "Save the active script buffer to the host-managed scripts directory.",
            },
            script_save_command,
        );
}

fn script_status_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: debug script status");
    }
    let workbench = world.resource::<DeveloperScriptWorkbench>();
    let Some(document) = workbench.active() else {
        return ConsoleCommandResult::error("no active script document");
    };
    let mut lines = vec![
        format!(
            "{} | target={} | revision={} | file-dirty={} | runtime-dirty={} | live={}",
            document.path,
            document.target.label(),
            document.revision,
            document.source_dirty(),
            document.runtime_dirty(),
            document.live_enabled,
        ),
        format!(
            "script workspace = {} | live={} | defaults={}",
            workbench.roots.mode,
            workbench.roots.live_root.display(),
            workbench.roots.defaults_root.display(),
        ),
        document.diagnostic.clone(),
    ];
    if let Some(warning) = workbench.bootstrap_warning.as_ref() {
        lines.push(format!("workspace bootstrap warning: {warning}"));
    }
    ConsoleCommandResult::lines(lines)
}

fn script_compile_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: debug script compile");
    }
    match world
        .resource_mut::<DeveloperScriptWorkbench>()
        .compile_active()
    {
        Ok(Some(value)) => ConsoleCommandResult::success(format!(
            "active script compiled; preview output={value:.6}"
        )),
        Ok(None) => ConsoleCommandResult::success("active script compiled"),
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn script_commit_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: debug script commit");
    }
    match world
        .resource_mut::<DeveloperScriptWorkbench>()
        .commit_active()
    {
        Ok(revision) => {
            ConsoleCommandResult::success(format!("active script committed as revision {revision}"))
        }
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn script_revert_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: debug script revert");
    }
    world
        .resource_mut::<DeveloperScriptWorkbench>()
        .revert_active_to_committed();
    ConsoleCommandResult::success("active script reverted to committed runtime source")
}

fn script_save_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: debug script save");
    }
    match world
        .resource_mut::<DeveloperScriptWorkbench>()
        .save_active()
    {
        Ok(()) => ConsoleCommandResult::success("active script saved"),
        Err(error) => ConsoleCommandResult::error(error),
    }
}
