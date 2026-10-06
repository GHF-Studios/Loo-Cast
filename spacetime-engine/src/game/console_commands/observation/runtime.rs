//! Bounded navigation history and worker telemetry commands.

use super::*;

pub(in crate::game::console_commands) fn navtrace_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: navtrace [<count>|clear|on|off|status]");
    }

    let arg = invocation.args().first().map(String::as_str);
    match arg {
        Some(value) if value.eq_ignore_ascii_case("clear") => {
            world.resource_mut::<NavigationFlightRecorder>().clear();
            ConsoleCommandResult::success("navtrace cleared")
        }
        Some(value) if value.eq_ignore_ascii_case("on") => {
            world
                .resource_mut::<NavigationFlightRecorder>()
                .set_enabled(true);
            ConsoleCommandResult::success("navtrace enabled")
        }
        Some(value) if value.eq_ignore_ascii_case("off") => {
            world
                .resource_mut::<NavigationFlightRecorder>()
                .set_enabled(false);
            ConsoleCommandResult::success("navtrace disabled")
        }
        Some(value) if value.eq_ignore_ascii_case("status") => {
            let recorder = world.resource::<NavigationFlightRecorder>();
            ConsoleCommandResult::success(format!(
                "navtrace {} | {} samples retained",
                if recorder.enabled() {
                    "enabled"
                } else {
                    "disabled"
                },
                recorder.len(),
            ))
        }
        Some(value) => {
            let Ok(count) = value.parse::<usize>() else {
                return ConsoleCommandResult::error("navtrace count must be a positive integer");
            };
            ConsoleCommandResult::lines(world.resource::<NavigationFlightRecorder>().lines(count))
        }
        None => ConsoleCommandResult::lines(world.resource::<NavigationFlightRecorder>().lines(40)),
    }
}

pub(in crate::game::console_commands) fn voxelstream_command(
    world: &mut World,
    _: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    ConsoleCommandResult::success(world.resource::<VoxelStreamingTelemetry>().summary())
}
