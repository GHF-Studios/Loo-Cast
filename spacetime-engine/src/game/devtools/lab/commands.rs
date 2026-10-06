//! Console adapters for developer presets.

use super::*;

pub(crate) fn configure(app: &mut App) {
    app.init_resource::<DeveloperPresetRegistry>()
        .init_resource::<DeveloperPresetState>();

    {
        let mut presets = app.world_mut().resource_mut::<DeveloperPresetRegistry>();
        if presets.spec(FREECAM_PRESET.name).is_none() {
            presets.register(FREECAM_PRESET);
        }
    }

    app.register_console_command(
        ConsoleCommandSpec {
            name: "debug preset list",
            aliases: &[],
            usage: "debug preset list",
            summary: "List named Developer Lab presets and active state.",
        },
        preset_list_command,
    )
    .register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "debug preset apply",
            aliases: &["debug preset on"],
            usage: "debug preset apply <name>",
            summary: "Apply a reversible named Developer Lab preset.",
        },
        &[ConsoleArgumentCompletion::Static(&["freecam"])],
        preset_apply_command,
    )
    .register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "debug preset clear",
            aliases: &["debug preset off"],
            usage: "debug preset clear <name|all>",
            summary: "Remove one/all presets and rebuild remaining overrides.",
        },
        &[ConsoleArgumentCompletion::Static(&["freecam", "all"])],
        preset_clear_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "debug lab status",
            aliases: &["debug lab"],
            usage: "debug lab status",
            summary: "Show active presets and override footprint.",
        },
        lab_status_command,
    );
}

fn preset_list_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: debug preset list");
    }

    let state = world.resource::<DeveloperPresetState>();
    let registry = world.resource::<DeveloperPresetRegistry>();
    ConsoleCommandResult::lines(registry.presets.values().map(|spec| {
        format!(
            "{} {:<14} — {}",
            if state.is_active(spec.name) { "*" } else { " " },
            spec.name,
            spec.summary,
        )
    }))
}

fn preset_apply_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let [name] = invocation.args() else {
        return ConsoleCommandResult::error("usage: debug preset apply <name>");
    };
    match apply_preset(world, name) {
        Ok(true) => ConsoleCommandResult::success_and_return_to_gameplay(format!(
            "developer preset `{}` applied",
            normalize(name)
        )),
        Ok(false) => ConsoleCommandResult::success(format!(
            "developer preset `{}` already active",
            normalize(name)
        )),
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn preset_clear_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let [name] = invocation.args() else {
        return ConsoleCommandResult::error("usage: debug preset clear <name|all>");
    };
    let target = if name.eq_ignore_ascii_case("all") {
        None
    } else {
        Some(name.as_str())
    };
    match clear_preset(world, target) {
        Ok(true) => ConsoleCommandResult::success_and_return_to_gameplay(format!(
            "developer preset `{}` cleared",
            target.unwrap_or("all")
        )),
        Ok(false) => ConsoleCommandResult::success(format!(
            "developer preset `{}` was not active",
            target.unwrap_or("all")
        )),
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn lab_status_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: debug lab status");
    }
    let state = world.resource::<DeveloperPresetState>();
    let active = if state.active().is_empty() {
        "<none>".to_string()
    } else {
        state.active().join(" -> ")
    };

    ConsoleCommandResult::lines([
        format!("active developer presets = {active}"),
        format!(
            "preset-controlled runtime variables = {}",
            state.overridden_path_count()
        ),
        "later active presets win when assignments overlap; clearing rebuilds from captured baselines"
            .to_string(),
    ])
}
