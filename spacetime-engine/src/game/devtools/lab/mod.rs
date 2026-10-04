//! Live developer override/preset composition.
//!
//!
//! This layer composes existing typed runtime-variable adapters. It does not
//! become a second config/cvar authority. Presets are reversible transactions:
//! they capture the pre-preset baseline, apply through validated setters, and
//! rebuild effective values when presets are removed.

use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::*;

use crate::console::{
    AppConsoleExt, ConsoleArgumentCompletion, ConsoleCommandInvocation,
    ConsoleCommandResult, ConsoleCommandSpec, RuntimeVariableBinding,
    RuntimeVariableRegistry,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct DeveloperPresetAssignment {
    pub path: &'static str,
    pub value: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DeveloperPresetSpec {
    pub name: &'static str,
    pub summary: &'static str,
    pub assignments: &'static [DeveloperPresetAssignment],
}

#[derive(Resource, Default)]
pub(crate) struct DeveloperPresetRegistry {
    presets: BTreeMap<String, DeveloperPresetSpec>,
}

impl DeveloperPresetRegistry {
    fn register(&mut self, spec: DeveloperPresetSpec) {
        let name = normalize(spec.name);
        assert!(!name.is_empty(), "developer preset names must not be empty");
        assert!(
            !self.presets.contains_key(&name),
            "duplicate developer preset `{name}`"
        );
        self.presets.insert(name, spec);
    }

    fn spec(&self, name: &str) -> Option<DeveloperPresetSpec> {
        self.presets.get(&normalize(name)).copied()
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub(crate) struct DeveloperPresetState {
    active: Vec<String>,
    baselines: BTreeMap<String, String>,
}

impl DeveloperPresetState {
    pub(crate) fn is_active(&self, name: &str) -> bool {
        let name = normalize(name);
        self.active.iter().any(|active| active == &name)
    }

    pub(crate) fn active(&self) -> &[String] {
        &self.active
    }

    pub(crate) fn overridden_path_count(&self) -> usize {
        self.baselines.len()
    }
}

const FREECAM_ASSIGNMENTS: &[DeveloperPresetAssignment] = &[
    DeveloperPresetAssignment {
        path: "debug.freecam.enabled",
        value: "true",
    },
    DeveloperPresetAssignment {
        path: "debug.freecam.control_policy",
        value: "exclusive",
    },
    DeveloperPresetAssignment {
        path: "debug.freecam.projection_policy",
        value: "follow",
    },
    DeveloperPresetAssignment {
        path: "debug.freecam.view_demand",
        value: "frozen",
    },
];

const FREECAM_PRESET: DeveloperPresetSpec = DeveloperPresetSpec {
    name: "freecam",
    summary: "Detached view-only camera with exclusive controls and frozen sparse view demand.",
    assignments: FREECAM_ASSIGNMENTS,
};

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

fn runtime_binding(world: &World, path: &str) -> Result<RuntimeVariableBinding, String> {
    world
        .resource::<RuntimeVariableRegistry>()
        .binding(path)
        .ok_or_else(|| format!("developer preset references unknown runtime variable `{path}`"))
}

fn current_value(world: &mut World, path: &str) -> Result<String, String> {
    let binding = runtime_binding(world, path)?;
    binding.current(world)
}

fn set_value(world: &mut World, path: &str, value: &str) -> Result<(), String> {
    let binding = runtime_binding(world, path)?;
    binding.set(world, value)
}

fn restore_snapshot(world: &mut World, snapshot: &BTreeMap<String, String>) {
    for (path, value) in snapshot {
        if let Err(error) = set_value(world, path, value) {
            error!(%path, %error, "failed to rollback Developer Lab preset transaction");
        }
    }
}

fn active_specs(
    world: &World,
    state: &DeveloperPresetState,
) -> Result<Vec<DeveloperPresetSpec>, String> {
    let registry = world.resource::<DeveloperPresetRegistry>();
    state
        .active
        .iter()
        .map(|name| {
            registry
                .spec(name)
                .ok_or_else(|| format!("active developer preset `{name}` is no longer registered"))
        })
        .collect()
}

fn paths_touched_by_active(
    world: &World,
    state: &DeveloperPresetState,
) -> Result<BTreeSet<String>, String> {
    let mut touched = BTreeSet::new();
    for spec in active_specs(world, state)? {
        for assignment in spec.assignments {
            touched.insert(normalize(assignment.path));
        }
    }
    Ok(touched)
}

fn rebuild_effective_values(
    world: &mut World,
    state: &DeveloperPresetState,
) -> Result<(), String> {
    let baselines = state
        .baselines
        .iter()
        .map(|(path, value)| (path.clone(), value.clone()))
        .collect::<Vec<_>>();
    let specs = active_specs(world, state)?;

    for (path, value) in baselines {
        set_value(world, &path, &value)?;
    }
    for spec in specs {
        for assignment in spec.assignments {
            set_value(world, assignment.path, assignment.value)?;
        }
    }
    Ok(())
}

pub(crate) fn preset_active(world: &World, name: &str) -> bool {
    world.resource::<DeveloperPresetState>().is_active(name)
}

pub(crate) fn apply_preset(world: &mut World, name: &str) -> Result<bool, String> {
    let normalized = normalize(name);
    let spec = world
        .resource::<DeveloperPresetRegistry>()
        .spec(&normalized)
        .ok_or_else(|| format!("unknown developer preset `{name}`"))?;

    let previous_state = world.resource::<DeveloperPresetState>().clone();
    if previous_state.is_active(&normalized) {
        return Ok(false);
    }

    let mut next_state = previous_state.clone();
    let mut before = BTreeMap::new();

    for assignment in spec.assignments {
        let path = normalize(assignment.path);
        let current = current_value(world, &path)?;
        before.insert(path.clone(), current.clone());
        next_state.baselines.entry(path).or_insert(current);
    }
    for path in previous_state.baselines.keys() {
        if !before.contains_key(path) {
            before.insert(path.clone(), current_value(world, path)?);
        }
    }

    next_state.active.push(normalized);
    if let Err(error) = rebuild_effective_values(world, &next_state) {
        restore_snapshot(world, &before);
        return Err(error);
    }

    *world.resource_mut::<DeveloperPresetState>() = next_state;
    Ok(true)
}

pub(crate) fn clear_preset(
    world: &mut World,
    name: Option<&str>,
) -> Result<bool, String> {
    let previous_state = world.resource::<DeveloperPresetState>().clone();
    if previous_state.active.is_empty() {
        return Ok(false);
    }

    let mut next_state = previous_state.clone();
    let changed = match name {
        None | Some("all") => {
            next_state.active.clear();
            true
        }
        Some(name) => {
            let normalized = normalize(name);
            let before_len = next_state.active.len();
            next_state.active.retain(|active| active != &normalized);
            before_len != next_state.active.len()
        }
    };
    if !changed {
        return Ok(false);
    }

    let mut before = BTreeMap::new();
    for path in previous_state.baselines.keys() {
        before.insert(path.clone(), current_value(world, path)?);
    }

    if let Err(error) = rebuild_effective_values(world, &next_state) {
        restore_snapshot(world, &before);
        return Err(error);
    }

    let still_touched = paths_touched_by_active(world, &next_state)?;
    next_state
        .baselines
        .retain(|path, _| still_touched.contains(path));

    *world.resource_mut::<DeveloperPresetState>() = next_state;
    Ok(true)
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

fn normalize(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace('-', "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_state_tracks_activation_order() {
        let mut state = DeveloperPresetState::default();
        state.active.push("a".into());
        state.active.push("b".into());
        assert_eq!(state.active(), &["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn registry_accepts_named_presets() {
        let mut registry = DeveloperPresetRegistry::default();
        registry.register(DeveloperPresetSpec {
            name: "test",
            summary: "test",
            assignments: &[],
        });
        assert!(registry.spec("test").is_some());
    }
}
