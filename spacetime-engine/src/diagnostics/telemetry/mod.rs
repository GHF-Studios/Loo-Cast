//! Vapor telemetry publication for typed runtime/world diagnostic snapshots.

use super::*;

pub(super) fn publish_runtime_telemetry(telemetry: &VaporTelemetry, runtime: &RuntimeDiagnostics) {
    let mut metrics = BTreeMap::new();
    insert_metric(&mut metrics, "frame.fps", runtime.frame.fps);
    insert_metric(&mut metrics, "frame.time-ms", runtime.frame.frame_time_ms);
    insert_metric(
        &mut metrics,
        "frame.average-time-ms",
        runtime.frame.average_frame_time_ms,
    );
    insert_metric(
        &mut metrics,
        "frame.one-percent-low-fps",
        runtime.frame.one_percent_low_fps,
    );
    insert_metric(
        &mut metrics,
        "system.process-cpu-percent",
        runtime.system.process_cpu_percent,
    );
    insert_metric(
        &mut metrics,
        "system.cpu-percent",
        runtime.system.system_cpu_percent,
    );
    insert_metric(
        &mut metrics,
        "system.process-memory-gib",
        runtime.system.process_memory_gib,
    );
    insert_metric(
        &mut metrics,
        "system.memory-percent",
        runtime.system.system_memory_percent,
    );
    telemetry.0.publish_metrics(metrics);
}

pub(super) fn publish_world_telemetry(world: &World) {
    let Some(telemetry) = world.get_resource::<VaporTelemetry>() else {
        return;
    };
    let runtime = world.resource::<RuntimeDiagnostics>();
    let memory = world.resource::<EcsMemoryDiagnostics>();
    let mut metrics = BTreeMap::new();

    metrics.insert("world.entities".to_owned(), runtime.world.entities as f64);
    metrics.insert(
        "world.component-instances".to_owned(),
        runtime.world.component_instances as f64,
    );
    metrics.insert(
        "world.archetypes".to_owned(),
        runtime.world.archetypes as f64,
    );
    metrics.insert(
        "ecs.inline-component-bytes".to_owned(),
        memory.inline_component_bytes as f64,
    );
    telemetry.0.publish_metrics(metrics);

    let components = memory
        .largest_components
        .iter()
        .map(|component| {
            serde_json::json!({
                "name": &component.name,
                "instances": component.instances,
                "inline_size_bytes": component.inline_size_bytes,
                "inline_bytes": component.inline_bytes,
            })
        })
        .collect();
    telemetry
        .0
        .publish_snapshot("ecs.component-memory", serde_json::Value::Array(components));
}

fn insert_metric(metrics: &mut BTreeMap<String, f64>, name: &str, value: Option<f64>) {
    if let Some(value) = value {
        metrics.insert(name.to_owned(), value);
    }
}
