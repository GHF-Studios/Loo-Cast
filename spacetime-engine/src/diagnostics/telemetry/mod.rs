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

    let physics = runtime.physics;
    metrics.insert(
        "physics.steps-window".to_owned(),
        physics.physics_steps_window as f64,
    );
    metrics.insert(
        "physics.render-frames-window".to_owned(),
        physics.render_frames_window as f64,
    );
    metrics.insert(
        "physics.steps-per-frame.last".to_owned(),
        physics.steps_per_frame_last as f64,
    );
    metrics.insert(
        "physics.steps-per-frame.avg".to_owned(),
        physics.steps_per_frame_average,
    );
    metrics.insert(
        "physics.steps-per-frame.max".to_owned(),
        physics.steps_per_frame_max as f64,
    );

    insert_collision_timer_metrics(
        &mut metrics,
        "broad-phase",
        physics.broad_phase_step_average_ms,
        physics.broad_phase_step_max_ms,
        physics.broad_phase_step_last_ms,
        physics.broad_phase_frame_average_ms,
        physics.broad_phase_frame_max_ms,
        physics.broad_phase_frame_last_ms,
    );
    insert_collision_timer_metrics(
        &mut metrics,
        "narrow-phase",
        physics.narrow_phase_step_average_ms,
        physics.narrow_phase_step_max_ms,
        physics.narrow_phase_step_last_ms,
        physics.narrow_phase_frame_average_ms,
        physics.narrow_phase_frame_max_ms,
        physics.narrow_phase_frame_last_ms,
    );

    metrics.insert(
        "physics.collision.contact-count.step-avg".to_owned(),
        physics.contact_count_step_average,
    );
    metrics.insert(
        "physics.collision.contact-count.step-max".to_owned(),
        physics.contact_count_step_max as f64,
    );
    metrics.insert(
        "physics.collision.contact-count.last-step".to_owned(),
        physics.contact_count_step_last as f64,
    );
    metrics.insert(
        "physics.solver.total.step-avg-ms".to_owned(),
        physics.solver_step_average_ms,
    );
    metrics.insert(
        "physics.solver.total.step-max-ms".to_owned(),
        physics.solver_step_max_ms,
    );
    metrics.insert(
        "physics.solver.total.last-step-ms".to_owned(),
        physics.solver_step_last_ms,
    );
    metrics.insert(
        "physics.solver.contact-constraints.step-avg".to_owned(),
        physics.solver_constraint_count_step_average,
    );
    metrics.insert(
        "physics.solver.contact-constraints.step-max".to_owned(),
        physics.solver_constraint_count_step_max as f64,
    );
    metrics.insert(
        "physics.solver.contact-constraints.last-step".to_owned(),
        physics.solver_constraint_count_step_last as f64,
    );
    metrics.insert(
        "physics.contact-pairs.active".to_owned(),
        physics.active_contact_pairs as f64,
    );
    metrics.insert(
        "physics.contact-pairs.active-touching".to_owned(),
        physics.active_touching_pairs as f64,
    );
    metrics.insert(
        "physics.contact-pairs.sleeping".to_owned(),
        physics.sleeping_contact_pairs as f64,
    );
    metrics.insert(
        "physics.contact-pairs.sleeping-touching".to_owned(),
        physics.sleeping_touching_pairs as f64,
    );
    metrics.insert(
        "physics.bodies.dynamic".to_owned(),
        physics.dynamic_bodies as f64,
    );
    metrics.insert(
        "physics.bodies.kinematic".to_owned(),
        physics.kinematic_bodies as f64,
    );
    metrics.insert(
        "physics.bodies.static".to_owned(),
        physics.static_bodies as f64,
    );
    metrics.insert(
        "physics.bodies.sleeping".to_owned(),
        physics.sleeping_bodies as f64,
    );
    metrics.insert(
        "physics.colliders".to_owned(),
        physics.collider_count as f64,
    );

    telemetry.0.publish_metrics(metrics);
}

fn insert_collision_timer_metrics(
    metrics: &mut BTreeMap<String, f64>,
    phase: &str,
    step_average_ms: f64,
    step_max_ms: f64,
    step_last_ms: f64,
    frame_average_ms: f64,
    frame_max_ms: f64,
    frame_last_ms: f64,
) {
    let prefix = format!("physics.collision.{phase}");
    metrics.insert(format!("{prefix}.step-avg-ms"), step_average_ms);
    metrics.insert(format!("{prefix}.step-max-ms"), step_max_ms);
    metrics.insert(format!("{prefix}.last-step-ms"), step_last_ms);
    metrics.insert(format!("{prefix}.frame-avg-ms"), frame_average_ms);
    metrics.insert(format!("{prefix}.frame-max-ms"), frame_max_ms);
    metrics.insert(format!("{prefix}.last-frame-ms"), frame_last_ms);
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
