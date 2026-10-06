//! Reversible typed-variable preset application.

use super::*;

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

fn rebuild_effective_values(world: &mut World, state: &DeveloperPresetState) -> Result<(), String> {
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

/// The state resource advances only after all typed setters succeed. Restore
/// the exact pre-transaction values if any adapter rejects the new composition.
fn commit_preset_state(
    world: &mut World,
    next_state: DeveloperPresetState,
    before: &BTreeMap<String, String>,
) -> Result<(), String> {
    if let Err(error) = rebuild_effective_values(world, &next_state) {
        restore_snapshot(world, before);
        return Err(error);
    }
    *world.resource_mut::<DeveloperPresetState>() = next_state;
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
    commit_preset_state(world, next_state, &before)?;
    Ok(true)
}

pub(crate) fn clear_preset(world: &mut World, name: Option<&str>) -> Result<bool, String> {
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

    let still_touched = paths_touched_by_active(world, &next_state)?;
    commit_preset_state(world, next_state, &before)?;
    world
        .resource_mut::<DeveloperPresetState>()
        .baselines
        .retain(|path, _| still_touched.contains(path));
    Ok(true)
}
