//! Source-style runtime key bindings over the existing player input adapter.
//!
//! #56 source-style-runtime-keymap-v1
//!
//! `bind` owns only device-to-action/command mapping. Bound console commands
//! re-enter the same console dispatcher and therefore inherit the authority of
//! the command they invoke; key bindings are not a second simulation ingress.

use bevy::prelude::*;

use crate::{
    console::{
        AppConsoleExt, ConsoleArgumentCompletion, ConsoleCommandInvocation, ConsoleCommandResult,
        ConsoleCommandSpec,
    },
    game::player::{BINDABLE_INPUT_NAMES, PLAYER_BIND_TARGETS, PlayerInputBindings},
};

pub(super) fn configure(app: &mut App) {
    app.register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "bind",
            aliases: &[],
            usage: "bind [<key> [<action|command...>]]",
            summary: "List/query/set Source-style runtime key bindings.",
        },
        &[
            ConsoleArgumentCompletion::Static(BINDABLE_INPUT_NAMES),
            ConsoleArgumentCompletion::CommandTail(PLAYER_BIND_TARGETS),
        ],
        bind_command,
    )
    .register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "unbind",
            aliases: &[],
            usage: "unbind <key>",
            summary: "Remove one runtime key binding.",
        },
        &[ConsoleArgumentCompletion::Static(BINDABLE_INPUT_NAMES)],
        unbind_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "unbindall",
            aliases: &[],
            usage: "unbindall",
            summary: "Remove every runtime player/console key binding.",
        },
        unbind_all_command,
    );
}

fn bind_command(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    let args = invocation.args();
    if args.is_empty() {
        let lines = world.resource::<PlayerInputBindings>().binding_lines();
        return if lines.is_empty() {
            ConsoleCommandResult::success("no runtime key bindings")
        } else {
            ConsoleCommandResult::lines(lines)
        };
    }

    if args.len() == 1 {
        let key = &args[0];
        return match world
            .resource::<PlayerInputBindings>()
            .binding_for_name(key)
        {
            Ok(Some(target)) => ConsoleCommandResult::success(format!("{key} = {target}")),
            Ok(None) => ConsoleCommandResult::success(format!("{key} is unbound")),
            Err(error) => ConsoleCommandResult::error(error),
        };
    }

    let key = &args[0];
    let target = args[1..].join(" ");
    match world
        .resource_mut::<PlayerInputBindings>()
        .bind_named(key, &target)
    {
        Ok(canonical) => ConsoleCommandResult::success(format!("{key} -> {canonical}")),
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn unbind_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() != 1 {
        return ConsoleCommandResult::error("usage: unbind <key>");
    }
    let key = &invocation.args()[0];
    match world
        .resource_mut::<PlayerInputBindings>()
        .unbind_named(key)
    {
        Ok(Some(previous)) => {
            ConsoleCommandResult::success(format!("{key} unbound (was {previous})"))
        }
        Ok(None) => ConsoleCommandResult::success(format!("{key} was already unbound")),
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn unbind_all_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: unbindall");
    }
    world.resource_mut::<PlayerInputBindings>().clear_all();
    ConsoleCommandResult::success("all runtime key bindings cleared")
}
