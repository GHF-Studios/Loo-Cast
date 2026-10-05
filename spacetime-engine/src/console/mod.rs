//! Shared developer-console command and diagnostic transport.
//!
//! The console has one command registry/dispatcher and multiple frontends:
//! - the in-game egui overlay;
//! - native stdin.
//!
//! Command results become structured console records consumed by both the
//! overlay and terminal sinks. Bevy/tracing diagnostics remain tracing events;
//! the overlay observes them through an additional [`bevy::log::LogPlugin`]
//! layer while Bevy's normal formatter continues to own terminal log output.
//!
//! Raw process stdout/stderr are deliberately not intercepted. Engine code that
//! should participate in the shared diagnostic stream should use tracing.

use crate::input_focus::{InputFocus, InputFocusSet};
use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

mod command;
mod overlay;
mod runtime_variables;
mod transport;

pub use command::{
    AppConsoleExt, ConsoleArgumentCompletion, ConsoleCommandHandler, ConsoleCommandInvocation,
    ConsoleCommandRegistry, ConsoleCommandResult, ConsoleCommandSource, ConsoleCommandSpec,
    ConsoleFocusDisposition,
};
use command::{clear_command, echo_command, help_command, parse_command};
use overlay::{ConsoleOverlay, draw_console, toggle_console};
#[cfg(not(target_arch = "wasm32"))]
use transport::start_terminal_input;
use transport::{ConsoleRecord, ConsoleRecordOrigin, write_terminal_record};
pub(crate) use transport::{ConsoleTransport, console_log_layer};

pub use runtime_variables::{
    AppRuntimeVariableExt, RuntimeVariableAuthority, RuntimeVariableBinding, RuntimeVariableDomain,
    RuntimeVariableRegistry, RuntimeVariableSpec, RuntimeVariableValueType,
};

const CONSOLE_FOCUS_OWNER: &str = "developer_console";

pub struct DeveloperConsolePlugin;

impl Plugin for DeveloperConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConsoleTransport>()
            .init_resource::<ConsoleOverlay>()
            .init_resource::<ConsoleCommandRegistry>()
            .init_resource::<RuntimeVariableRegistry>()
            .init_resource::<InputFocus>()
            .add_systems(PreUpdate, toggle_console.before(InputFocusSet::Resolve))
            .add_systems(Update, dispatch_console_commands)
            .add_systems(PostUpdate, flush_console_records)
            .add_systems(EguiPrimaryContextPass, draw_console);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Startup, start_terminal_input);

        app.register_console_command(
            ConsoleCommandSpec {
                name: "help",
                aliases: &["?", "commands"],
                usage: "help [command]",
                summary: "List commands or show detailed help for one command.",
            },
            help_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "clear",
                aliases: &["cls"],
                usage: "clear",
                summary: "Clear console frontends.",
            },
            clear_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "echo",
                aliases: &[],
                usage: "echo <text...>",
                summary: "Print text to all console frontends.",
            },
            echo_command,
        );

        runtime_variables::configure(app);
    }
}

fn dispatch_console_commands(world: &mut World) {
    let submissions = world.resource::<ConsoleTransport>().drain_commands();

    for submission in submissions {
        let transport = world.resource::<ConsoleTransport>().clone();
        transport.publish(ConsoleRecord::command(submission.source, &submission.raw));

        let invocation = match parse_command(&submission.raw, submission.source) {
            Ok(invocation) => invocation,
            Err(error) => {
                transport.publish(ConsoleRecord::error(error));
                continue;
            }
        };

        let tokens = invocation.tokens();
        let resolved = {
            let registry = world.resource::<ConsoleCommandRegistry>();
            registry.resolve_tokens(&tokens)
        };

        let Some((command, consumed, canonical)) = resolved else {
            transport.publish(ConsoleRecord::error(format!(
                "unknown command path `{}` — type `help` to list commands",
                tokens.join(" ")
            )));
            continue;
        };
        let invocation = invocation.resolved(canonical, consumed);

        match (command.handler)(world, &invocation) {
            ConsoleCommandResult::Silent => {}
            ConsoleCommandResult::Success { lines, focus } => {
                for line in lines {
                    transport.publish(ConsoleRecord::output(line));
                }
                apply_focus_disposition(world, invocation.source(), focus);
            }
            ConsoleCommandResult::Error(error) => {
                transport.publish(ConsoleRecord::error(error));
            }
            ConsoleCommandResult::Clear => {
                transport.publish(ConsoleRecord::clear());
            }
        }
    }
}

fn apply_focus_disposition(
    world: &mut World,
    source: ConsoleCommandSource,
    focus: ConsoleFocusDisposition,
) {
    if source != ConsoleCommandSource::Overlay || focus != ConsoleFocusDisposition::ReturnToGameplay
    {
        return;
    }

    {
        let mut console = world.resource_mut::<ConsoleOverlay>();
        console.close_for_gameplay();
    }

    let mut input_focus = world.resource_mut::<InputFocus>();
    input_focus.set_modal_claim(CONSOLE_FOCUS_OWNER, false);
    input_focus.request_gameplay_resume();
}

fn flush_console_records(transport: Res<ConsoleTransport>, mut overlay: ResMut<ConsoleOverlay>) {
    for record in transport.drain_records() {
        if record.origin == ConsoleRecordOrigin::Command {
            write_terminal_record(&record);
        }

        overlay.accept_record(record);
    }
}
