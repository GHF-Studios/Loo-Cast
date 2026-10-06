//! Console ingress for typed runtime variables.

use super::*;

pub(crate) fn configure(app: &mut App) {
    app.register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "config get",
            aliases: &["cvar get"],
            usage: "config get <path>",
            summary: "Read a typed runtime variable and its compiled default.",
        },
        &[ConsoleArgumentCompletion::RuntimeVariablePath],
        config_get,
    )
    .register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "config set",
            aliases: &["cvar set"],
            usage: "config set <path> <value>",
            summary: "Set a typed engine override or local debug control.",
        },
        &[
            ConsoleArgumentCompletion::RuntimeVariablePath,
            ConsoleArgumentCompletion::RuntimeVariableValue { path_argument: 0 },
        ],
        config_set,
    )
    .register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "config reset",
            aliases: &["cvar reset"],
            usage: "config reset <path>",
            summary: "Reset a typed runtime override/control to its owning default.",
        },
        &[ConsoleArgumentCompletion::RuntimeVariablePath],
        config_reset,
    )
    .register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "config list",
            aliases: &["cvar list", "cvars"],
            usage: "config list [prefix]",
            summary: "List registered typed runtime variables and metadata.",
        },
        &[ConsoleArgumentCompletion::RuntimeVariablePath],
        config_list,
    );
}

fn config_get(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    if invocation.args().len() != 1 {
        return ConsoleCommandResult::error("usage: config get <path>");
    }
    let path = &invocation.args()[0];
    let binding = {
        let registry = world.resource::<RuntimeVariableRegistry>();
        registry.binding(path)
    };
    let Some(binding) = binding else {
        return ConsoleCommandResult::error(format!("unknown runtime variable `{path}`"));
    };

    match binding.current(world) {
        Ok(current) => {
            let spec = binding.spec();
            ConsoleCommandResult::success(format!(
                "{} = {} | default={} | type={}{} | authority={}",
                spec.path,
                current,
                binding.default_value(),
                spec.value_type.label(),
                spec.units
                    .map_or(String::new(), |units| format!(" {units}")),
                spec.authority.label(),
            ))
        }
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn config_set(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    if invocation.args().len() != 2 {
        return ConsoleCommandResult::error("usage: config set <path> <value>");
    }
    let path = &invocation.args()[0];
    let raw = &invocation.args()[1];
    let binding = {
        let registry = world.resource::<RuntimeVariableRegistry>();
        registry.binding(path)
    };
    let Some(binding) = binding else {
        return ConsoleCommandResult::error(format!("unknown runtime variable `{path}`"));
    };

    if let Err(error) = binding.set(world, raw) {
        return ConsoleCommandResult::error(error);
    }
    match binding.current(world) {
        Ok(current) => {
            ConsoleCommandResult::success(format!("{} = {}", binding.spec().path, current))
        }
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn config_reset(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    if invocation.args().len() != 1 {
        return ConsoleCommandResult::error("usage: config reset <path>");
    }
    let path = &invocation.args()[0];
    let binding = {
        let registry = world.resource::<RuntimeVariableRegistry>();
        registry.binding(path)
    };
    let Some(binding) = binding else {
        return ConsoleCommandResult::error(format!("unknown runtime variable `{path}`"));
    };

    match binding.reset(world) {
        Ok(()) => ConsoleCommandResult::success(format!(
            "{} reset; layered/effective state reconciles at its owning runtime boundary",
            binding.spec().path
        )),
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn config_list(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: config list [prefix]");
    }
    let prefix = invocation
        .args()
        .first()
        .map_or(String::new(), |value| normalize_path(value));

    let lines = {
        let registry = world.resource::<RuntimeVariableRegistry>();
        registry
            .matching(&prefix)
            .map(|binding| {
                let spec = binding.spec();
                let domain = spec
                    .domain
                    .label()
                    .map_or(String::new(), |domain| format!(" {domain}"));
                let units = spec.units.map_or("", |units| units);
                format!(
                    "{:<52} {:<12} {:<22} {}{} — {}",
                    spec.path,
                    spec.value_type.label(),
                    spec.authority.label(),
                    units,
                    domain,
                    spec.summary,
                )
            })
            .collect::<Vec<_>>()
    };

    if lines.is_empty() {
        ConsoleCommandResult::error(format!(
            "no runtime variables match `{}`",
            invocation.args().first().map_or("", String::as_str)
        ))
    } else {
        ConsoleCommandResult::lines(lines)
    }
}
