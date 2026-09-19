//! Avian runtime profiling and bounded high-cardinality physics telemetry.
//!
//! The hot path records only Avian's already-computed diagnostics once per
//! `PhysicsSchedule`. More expensive body/contact/AABB inventories are sampled
//! at low frequency and only when a Vapor telemetry consumer is connected.

use std::time::Duration;

use avian3d::{
    collision::CollisionDiagnostics,
    dynamics::solver::SolverDiagnostics,
    prelude::*,
};
use bevy::prelude::*;
use serde_json::{Value, json};

use super::{DiagnosticsCadence, PhysicsRuntimeDiagnostics, VaporTelemetry};

const DETAIL_SAMPLE_INTERVAL_SECONDS: f32 = 1.0;
const MAX_BODY_ROWS: usize = 256;
const MAX_COLLIDER_ROWS: usize = 256;
const MAX_CONTACT_PAIR_ROWS: usize = 512;

#[derive(Resource, Debug, Default)]
pub(super) struct PhysicsTelemetryAccumulator {
    physics_steps: u64,
    render_frames: u64,
    steps_since_frame: u32,
    steps_per_frame_last: u32,
    steps_per_frame_max: u32,

    broad_step_total: Duration,
    broad_step_max: Duration,
    broad_step_last: Duration,
    broad_since_frame: Duration,
    broad_frame_total: Duration,
    broad_frame_max: Duration,
    broad_frame_last: Duration,

    narrow_step_total: Duration,
    narrow_step_max: Duration,
    narrow_step_last: Duration,
    narrow_since_frame: Duration,
    narrow_frame_total: Duration,
    narrow_frame_max: Duration,
    narrow_frame_last: Duration,

    contact_count_total: u64,
    contact_count_max: u32,
    contact_count_last: u32,

    solver_step_total: Duration,
    solver_step_max: Duration,
    solver_step_last: Duration,
    solver_constraint_count_total: u64,
    solver_constraint_count_max: u32,
    solver_constraint_count_last: u32,
}

pub(super) fn record_physics_step(
    diagnostics: Res<CollisionDiagnostics>,
    solver: Res<SolverDiagnostics>,
    mut accumulator: ResMut<PhysicsTelemetryAccumulator>,
) {
    accumulator.physics_steps = accumulator.physics_steps.saturating_add(1);
    accumulator.steps_since_frame = accumulator.steps_since_frame.saturating_add(1);

    accumulator.broad_step_total += diagnostics.broad_phase;
    accumulator.broad_step_max = accumulator.broad_step_max.max(diagnostics.broad_phase);
    accumulator.broad_step_last = diagnostics.broad_phase;
    accumulator.broad_since_frame += diagnostics.broad_phase;

    accumulator.narrow_step_total += diagnostics.narrow_phase;
    accumulator.narrow_step_max = accumulator.narrow_step_max.max(diagnostics.narrow_phase);
    accumulator.narrow_step_last = diagnostics.narrow_phase;
    accumulator.narrow_since_frame += diagnostics.narrow_phase;

    accumulator.contact_count_total = accumulator
        .contact_count_total
        .saturating_add(diagnostics.contact_count as u64);
    accumulator.contact_count_max = accumulator.contact_count_max.max(diagnostics.contact_count);
    accumulator.contact_count_last = diagnostics.contact_count;

    let solver_duration = solver_step_duration(&solver);
    accumulator.solver_step_total += solver_duration;
    accumulator.solver_step_max = accumulator.solver_step_max.max(solver_duration);
    accumulator.solver_step_last = solver_duration;
    accumulator.solver_constraint_count_total = accumulator
        .solver_constraint_count_total
        .saturating_add(solver.contact_constraint_count as u64);
    accumulator.solver_constraint_count_max = accumulator
        .solver_constraint_count_max
        .max(solver.contact_constraint_count);
    accumulator.solver_constraint_count_last = solver.contact_constraint_count;
}

/// Finalizes the fixed-physics work performed during the current render frame.
///
/// Bevy runs its fixed main loop before `Update`, so this system sees all
/// `PhysicsSchedule` iterations that were required for the frame.
pub(super) fn finalize_physics_frame(mut accumulator: ResMut<PhysicsTelemetryAccumulator>) {
    accumulator.render_frames = accumulator.render_frames.saturating_add(1);
    accumulator.steps_per_frame_last = accumulator.steps_since_frame;
    accumulator.steps_per_frame_max = accumulator
        .steps_per_frame_max
        .max(accumulator.steps_since_frame);

    let broad_frame = accumulator.broad_since_frame;
    accumulator.broad_frame_last = broad_frame;
    accumulator.broad_frame_total += broad_frame;
    accumulator.broad_frame_max = accumulator.broad_frame_max.max(broad_frame);

    let narrow_frame = accumulator.narrow_since_frame;
    accumulator.narrow_frame_last = narrow_frame;
    accumulator.narrow_frame_total += narrow_frame;
    accumulator.narrow_frame_max = accumulator.narrow_frame_max.max(narrow_frame);

    accumulator.steps_since_frame = 0;
    accumulator.broad_since_frame = Duration::ZERO;
    accumulator.narrow_since_frame = Duration::ZERO;
}

pub(super) fn sample_runtime(
    accumulator: &mut PhysicsTelemetryAccumulator,
    contact_graph: &ContactGraph,
    bodies: impl Iterator<Item = (RigidBody, bool)>,
    collider_count: usize,
) -> PhysicsRuntimeDiagnostics {
    let mut result = PhysicsRuntimeDiagnostics {
        physics_steps_window: accumulator.physics_steps,
        render_frames_window: accumulator.render_frames,
        steps_per_frame_last: accumulator.steps_per_frame_last,
        steps_per_frame_max: accumulator.steps_per_frame_max,
        steps_per_frame_average: ratio(accumulator.physics_steps, accumulator.render_frames),

        broad_phase_step_average_ms: duration_average_ms(
            accumulator.broad_step_total,
            accumulator.physics_steps,
        ),
        broad_phase_step_max_ms: duration_ms(accumulator.broad_step_max),
        broad_phase_step_last_ms: duration_ms(accumulator.broad_step_last),
        broad_phase_frame_average_ms: duration_average_ms(
            accumulator.broad_frame_total,
            accumulator.render_frames,
        ),
        broad_phase_frame_max_ms: duration_ms(accumulator.broad_frame_max),
        broad_phase_frame_last_ms: duration_ms(accumulator.broad_frame_last),

        narrow_phase_step_average_ms: duration_average_ms(
            accumulator.narrow_step_total,
            accumulator.physics_steps,
        ),
        narrow_phase_step_max_ms: duration_ms(accumulator.narrow_step_max),
        narrow_phase_step_last_ms: duration_ms(accumulator.narrow_step_last),
        narrow_phase_frame_average_ms: duration_average_ms(
            accumulator.narrow_frame_total,
            accumulator.render_frames,
        ),
        narrow_phase_frame_max_ms: duration_ms(accumulator.narrow_frame_max),
        narrow_phase_frame_last_ms: duration_ms(accumulator.narrow_frame_last),

        contact_count_step_average: ratio(
            accumulator.contact_count_total,
            accumulator.physics_steps,
        ),
        contact_count_step_max: accumulator.contact_count_max,
        contact_count_step_last: accumulator.contact_count_last,

        solver_step_average_ms: duration_average_ms(
            accumulator.solver_step_total,
            accumulator.physics_steps,
        ),
        solver_step_max_ms: duration_ms(accumulator.solver_step_max),
        solver_step_last_ms: duration_ms(accumulator.solver_step_last),
        solver_constraint_count_step_average: ratio(
            accumulator.solver_constraint_count_total,
            accumulator.physics_steps,
        ),
        solver_constraint_count_step_max: accumulator.solver_constraint_count_max,
        solver_constraint_count_step_last: accumulator.solver_constraint_count_last,

        active_contact_pairs: contact_graph.active_pairs().len(),
        active_touching_pairs: contact_graph.iter_active_touching().count(),
        sleeping_contact_pairs: contact_graph.sleeping_pairs().len(),
        sleeping_touching_pairs: contact_graph.iter_sleeping_touching().count(),
        collider_count,
        ..default()
    };

    for (body, sleeping) in bodies {
        match body {
            RigidBody::Dynamic => result.dynamic_bodies += 1,
            RigidBody::Kinematic => result.kinematic_bodies += 1,
            RigidBody::Static => result.static_bodies += 1,
        }
        if sleeping {
            result.sleeping_bodies += 1;
        }
    }

    *accumulator = PhysicsTelemetryAccumulator::default();
    result
}

pub(super) fn collect_physics_detail_diagnostics(
    time: Res<Time>,
    mut cadence: ResMut<DiagnosticsCadence>,
    telemetry: Option<Res<VaporTelemetry>>,
    contact_graph: Res<ContactGraph>,
    bodies: Query<(
        Entity,
        Option<&Name>,
        &RigidBody,
        Has<Sleeping>,
        Option<&Transform>,
        Option<&Position>,
        Option<&LinearVelocity>,
        Option<&AngularVelocity>,
    )>,
    colliders: Query<(Entity, Option<&Name>, &ColliderAabb, Option<&ColliderOf>)>,
    entity_meta: Query<(Option<&Name>, Option<&RigidBody>)>,
) {
    cadence.physics_detail_elapsed += time.delta_secs();
    if cadence.physics_detail_elapsed < DETAIL_SAMPLE_INTERVAL_SECONDS {
        return;
    }
    cadence.physics_detail_elapsed %= DETAIL_SAMPLE_INTERVAL_SECONDS;

    let Some(telemetry) = telemetry else {
        return;
    };

    telemetry
        .0
        .publish_snapshot("physics.body-inventory", body_inventory_snapshot(&bodies));
    telemetry.0.publish_snapshot(
        "physics.collider-extents",
        collider_extent_snapshot(&colliders, &entity_meta),
    );
    telemetry.0.publish_snapshot(
        "physics.contact-pairs",
        contact_pair_snapshot(&contact_graph, &colliders, &entity_meta),
    );
}

fn body_inventory_snapshot(
    bodies: &Query<(
        Entity,
        Option<&Name>,
        &RigidBody,
        Has<Sleeping>,
        Option<&Transform>,
        Option<&Position>,
        Option<&LinearVelocity>,
        Option<&AngularVelocity>,
    )>,
) -> Value {
    let mut rows = bodies
        .iter()
        .map(
            |(entity, name, body, sleeping, transform, position, linear, angular)| {
                let transform_position = transform.map(|transform| transform.translation);
                let physics_position = position.map(|position| position.0);
                let mismatch = transform_position
                    .zip(physics_position)
                    .map(|(transform, physics)| transform.distance(physics));

                json!({
                    "entity": entity_label(entity),
                    "name": name.map(Name::as_str).unwrap_or(""),
                    "kind": rigid_body_kind(*body),
                    "sleeping": sleeping,
                    "transform_x": transform_position.map(|value| finite_f32(value.x)),
                    "transform_y": transform_position.map(|value| finite_f32(value.y)),
                    "transform_z": transform_position.map(|value| finite_f32(value.z)),
                    "physics_x": physics_position.map(|value| finite_f32(value.x)),
                    "physics_y": physics_position.map(|value| finite_f32(value.y)),
                    "physics_z": physics_position.map(|value| finite_f32(value.z)),
                    "position_delta": mismatch.map(finite_f32),
                    "linear_speed": linear.map(|velocity| finite_f32(velocity.0.length())),
                    "angular_speed": angular.map(|velocity| finite_f32(velocity.0.length())),
                })
            },
        )
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        body_rank(right.get("kind").and_then(Value::as_str))
            .cmp(&body_rank(left.get("kind").and_then(Value::as_str)))
            .then_with(|| {
                left.get("name")
                    .and_then(Value::as_str)
                    .cmp(&right.get("name").and_then(Value::as_str))
            })
    });

    bounded_rows(rows, MAX_BODY_ROWS)
}

fn collider_extent_snapshot(
    colliders: &Query<(Entity, Option<&Name>, &ColliderAabb, Option<&ColliderOf>)>,
    entity_meta: &Query<(Option<&Name>, Option<&RigidBody>)>,
) -> Value {
    let mut rows = colliders
        .iter()
        .map(|(entity, name, aabb, collider_of)| {
            let size = aabb.size();
            let center = aabb.center();
            let volume = size.x.abs() * size.y.abs() * size.z.abs();
            let body_entity = collider_of.map(|owner| owner.body);
            let body_meta = body_entity.and_then(|body| entity_meta.get(body).ok());
            let body_name = body_meta
                .and_then(|(name, _)| name)
                .map(Name::as_str)
                .unwrap_or("");
            let body_kind = body_meta
                .and_then(|(_, body)| body)
                .map(|body| rigid_body_kind(*body))
                .unwrap_or("none");

            json!({
                "entity": entity_label(entity),
                "name": name.map(Name::as_str).unwrap_or(""),
                "body": body_entity.map(entity_label).unwrap_or_default(),
                "body_name": body_name,
                "body_kind": body_kind,
                "size_x": finite_f32(size.x),
                "size_y": finite_f32(size.y),
                "size_z": finite_f32(size.z),
                "volume": finite_f32(volume),
                "center_x": finite_f32(center.x),
                "center_y": finite_f32(center.y),
                "center_z": finite_f32(center.z),
            })
        })
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        json_f64(right, "volume")
            .total_cmp(&json_f64(left, "volume"))
            .then_with(|| {
                left.get("name")
                    .and_then(Value::as_str)
                    .cmp(&right.get("name").and_then(Value::as_str))
            })
    });

    bounded_rows(rows, MAX_COLLIDER_ROWS)
}

fn contact_pair_snapshot(
    contact_graph: &ContactGraph,
    colliders: &Query<(Entity, Option<&Name>, &ColliderAabb, Option<&ColliderOf>)>,
    entity_meta: &Query<(Option<&Name>, Option<&RigidBody>)>,
) -> Value {
    let mut rows = contact_graph
        .active_pairs()
        .iter()
        .map(|pair| contact_pair_row("active", pair, colliders, entity_meta))
        .chain(
            contact_graph
                .sleeping_pairs()
                .iter()
                .map(|pair| contact_pair_row("sleeping", pair, colliders, entity_meta)),
        )
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        json_bool(right, "touching")
            .cmp(&json_bool(left, "touching"))
            .then_with(|| json_u64(right, "points").cmp(&json_u64(left, "points")))
            .then_with(|| {
                json_f64(right, "largest_aabb_volume")
                    .total_cmp(&json_f64(left, "largest_aabb_volume"))
            })
    });

    let total = rows.len();
    let shown = total.min(MAX_CONTACT_PAIR_ROWS);
    rows.truncate(shown);
    json!({
        "summary": {
            "active": contact_graph.active_pairs().len(),
            "active_touching": contact_graph.iter_active_touching().count(),
            "sleeping": contact_graph.sleeping_pairs().len(),
            "sleeping_touching": contact_graph.iter_sleeping_touching().count(),
            "total": total,
            "shown": shown,
            "truncated": total > shown,
        },
        "items": rows,
    })
}

fn contact_pair_row(
    state: &'static str,
    pair: &ContactPair,
    colliders: &Query<(Entity, Option<&Name>, &ColliderAabb, Option<&ColliderOf>)>,
    entity_meta: &Query<(Option<&Name>, Option<&RigidBody>)>,
) -> Value {
    let collider1 = collider_summary(pair.collider1, colliders, entity_meta);
    let collider2 = collider_summary(pair.collider2, colliders, entity_meta);
    let points = pair
        .manifolds
        .iter()
        .map(|manifold| manifold.points.len())
        .sum::<usize>();
    let deepest = pair.find_deepest_contact().map(|contact| contact.penetration);
    let largest_aabb_volume = collider1.volume.max(collider2.volume);

    json!({
        "state": state,
        "touching": pair.is_touching(),
        "constraints": pair.generates_constraints(),
        "collider1": collider1.entity,
        "collider1_name": collider1.name,
        "body1_kind": collider1.body_kind,
        "collider1_aabb_volume": finite_f32(collider1.volume),
        "collider2": collider2.entity,
        "collider2_name": collider2.name,
        "body2_kind": collider2.body_kind,
        "collider2_aabb_volume": finite_f32(collider2.volume),
        "manifolds": pair.manifolds.len(),
        "points": points,
        "deepest_penetration": deepest.map(finite_f32),
        "normal_impulse": finite_f32(pair.total_normal_impulse_magnitude()),
        "largest_aabb_volume": finite_f32(largest_aabb_volume),
    })
}

struct ColliderSummary {
    entity: String,
    name: String,
    body_kind: &'static str,
    volume: f32,
}

fn collider_summary(
    entity: Entity,
    colliders: &Query<(Entity, Option<&Name>, &ColliderAabb, Option<&ColliderOf>)>,
    entity_meta: &Query<(Option<&Name>, Option<&RigidBody>)>,
) -> ColliderSummary {
    let Ok((_, name, aabb, collider_of)) = colliders.get(entity) else {
        return ColliderSummary {
            entity: entity_label(entity),
            name: String::new(),
            body_kind: "unknown",
            volume: 0.0,
        };
    };

    let size = aabb.size();
    let body_kind = collider_of
        .and_then(|owner| entity_meta.get(owner.body).ok())
        .and_then(|(_, body)| body)
        .map(|body| rigid_body_kind(*body))
        .unwrap_or("none");

    ColliderSummary {
        entity: entity_label(entity),
        name: name.map(Name::as_str).unwrap_or("").to_owned(),
        body_kind,
        volume: size.x.abs() * size.y.abs() * size.z.abs(),
    }
}

fn bounded_rows(mut rows: Vec<Value>, limit: usize) -> Value {
    let total = rows.len();
    let shown = total.min(limit);
    rows.truncate(shown);
    json!({
        "summary": {
            "total": total,
            "shown": shown,
            "truncated": total > shown,
        },
        "items": rows,
    })
}

fn entity_label(entity: Entity) -> String {
    format!("{entity:?}")
}

fn rigid_body_kind(body: RigidBody) -> &'static str {
    match body {
        RigidBody::Dynamic => "dynamic",
        RigidBody::Kinematic => "kinematic",
        RigidBody::Static => "static",
    }
}

fn body_rank(kind: Option<&str>) -> u8 {
    match kind {
        Some("dynamic") => 3,
        Some("kinematic") => 2,
        Some("static") => 1,
        _ => 0,
    }
}


fn solver_step_duration(diagnostics: &SolverDiagnostics) -> Duration {
    diagnostics.prepare_constraints
        + diagnostics.update_velocity_increments
        + diagnostics.integrate_velocities
        + diagnostics.warm_start
        + diagnostics.solve_constraints
        + diagnostics.integrate_positions
        + diagnostics.relax_velocities
        + diagnostics.apply_restitution
        + diagnostics.finalize
        + diagnostics.store_impulses
        + diagnostics.swept_ccd
}

fn duration_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn duration_average_ms(total: Duration, count: u64) -> f64 {
    if count == 0 {
        0.0
    } else {
        duration_ms(total) / count as f64
    }
}

fn ratio(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn finite_f32(value: f32) -> Option<f32> {
    value.is_finite().then_some(value)
}

fn json_f64(value: &Value, key: &str) -> f64 {
    value.get(key).and_then(Value::as_f64).unwrap_or_default()
}

fn json_u64(value: &Value, key: &str) -> u64 {
    value.get(key).and_then(Value::as_u64).unwrap_or_default()
}

fn json_bool(value: &Value, key: &str) -> bool {
    value.get(key).and_then(Value::as_bool).unwrap_or_default()
}
