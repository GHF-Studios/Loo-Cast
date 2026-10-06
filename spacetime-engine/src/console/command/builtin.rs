//! Built-in commands over the shared registry.

use super::{contract::*, registry::ConsoleCommandRegistry};
use bevy::prelude::*;

pub(in crate::console) fn help_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let registry = world.resource::<ConsoleCommandRegistry>();

    if !invocation.args().is_empty() {
        let tokens = invocation.args().to_vec();
        let Some((command, consumed, _)) = registry.resolve_tokens(&tokens) else {
            return ConsoleCommandResult::error(format!(
                "unknown command `{}`",
                invocation.args().join(" ")
            ));
        };
        if consumed != tokens.len() {
            return ConsoleCommandResult::error(format!(
                "unknown command `{}`",
                invocation.args().join(" ")
            ));
        }

        let mut lines = vec![format!("{} — {}", command.spec.usage, command.spec.summary)];
        if !command.spec.aliases.is_empty() {
            lines.push(format!("aliases: {}", command.spec.aliases.join(", ")));
        }
        return ConsoleCommandResult::lines(lines);
    }

    ConsoleCommandResult::lines(
        registry
            .commands
            .values()
            .map(|command| format!("{:<44} {}", command.spec.usage, command.spec.summary)),
    )
}

pub(in crate::console) fn clear_command(
    _: &mut World,
    _: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    ConsoleCommandResult::clear()
}

pub(in crate::console) fn echo_command(
    _: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    ConsoleCommandResult::success(invocation.args().join(" "))
}
