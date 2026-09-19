//! Bounded Avian runtime diagnostics.
//!
//! The physics hot path records Avian's already-computed diagnostics once per
//! `PhysicsSchedule`; lower-frequency runtime sampling publishes only compact
//! aggregate metrics rather than high-cardinality body/contact inventories.

use std::time::Duration;

use avian3d::{
    collision::CollisionDiagnostics,
    dynamics::solver::SolverDiagnostics,
    prelude::*,
};
use bevy::prelude::*;

use super::PhysicsRuntimeDiagnostics;


#[derive(Resource, Debug, Default)]
pub(super) struct PhysicsDiagnosticsAccumulator {
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
    mut accumulator: ResMut<PhysicsDiagnosticsAccumulator>,
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
pub(super) fn finalize_physics_frame(mut accumulator: ResMut<PhysicsDiagnosticsAccumulator>) {
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
    accumulator: &mut PhysicsDiagnosticsAccumulator,
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

    *accumulator = PhysicsDiagnosticsAccumulator::default();
    result
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

