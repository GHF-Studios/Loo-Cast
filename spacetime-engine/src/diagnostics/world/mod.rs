//! Low-frequency ECS/world structure and inline-memory sampling.

use super::*;

pub(super) fn collect_world_diagnostics(world: &mut World) {
    let dt = world.resource::<Time>().delta_secs();
    {
        let mut cadence = world.resource_mut::<DiagnosticsCadence>();
        cadence.world_elapsed += dt;
        if cadence.world_elapsed < WORLD_SAMPLE_INTERVAL_SECONDS {
            return;
        }
        cadence.world_elapsed %= WORLD_SAMPLE_INTERVAL_SECONDS;
    }

    let developer_artifact = world.components().component_id::<DeveloperArtifact>();
    let mut result = WorldRuntimeDiagnostics::default();
    let mut component_instances = vec![0usize; world.components().len()];

    for archetype in world.archetypes().iter() {
        if archetype.contains(IS_RESOURCE)
            || developer_artifact.is_some_and(|id| archetype.contains(id))
            || archetype.is_empty()
        {
            continue;
        }

        let entity_count = archetype.len() as usize;
        result.archetypes += 1;
        result.entities += entity_count;
        result.component_instances += entity_count * archetype.component_count();

        for component_id in archetype.iter_components() {
            component_instances[component_id.index()] += entity_count;
        }
    }

    world.resource_mut::<RuntimeDiagnostics>().world = result;

    let mut inline_component_bytes = 0usize;
    let mut largest_components = component_instances
        .into_iter()
        .enumerate()
        .filter_map(|(index, instances)| {
            if instances == 0 {
                return None;
            }
            let info = world.components().get_info(ComponentId::new(index))?;
            let inline_size_bytes = info.layout().size();
            let inline_bytes = inline_size_bytes.saturating_mul(instances);
            inline_component_bytes = inline_component_bytes.saturating_add(inline_bytes);

            Some(ComponentMemoryDiagnostics {
                name: info.name().to_string(),
                instances,
                inline_size_bytes,
                inline_bytes,
            })
        })
        .collect::<Vec<_>>();

    largest_components.sort_by(|a, b| {
        b.inline_bytes
            .cmp(&a.inline_bytes)
            .then_with(|| b.instances.cmp(&a.instances))
            .then_with(|| a.name.cmp(&b.name))
    });
    largest_components.truncate(TOP_COMPONENT_MEMORY_ENTRIES);

    *world.resource_mut::<EcsMemoryDiagnostics>() = EcsMemoryDiagnostics {
        inline_component_bytes,
        largest_components,
    };

    publish_world_telemetry(world);
}
