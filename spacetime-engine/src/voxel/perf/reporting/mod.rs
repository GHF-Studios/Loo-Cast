//! Periodic aggregation/logging of voxel realization performance and residency state.

use super::*;

#[derive(Debug, Default)]
struct ScaleLiveCounts {
    active: usize,
    dense: usize,
    surfaces: usize,
    render_aggregates: usize,
    colliders: usize,
}

pub(crate) fn report_voxel_perf(
    config: Res<EngineConfig>,
    fixed_time: Res<Time<Fixed>>,
    virtual_time: Res<Time<Virtual>>,
    view: Res<UsfViewFrame>,
    meshes: Res<Assets<Mesh>>,
    mut stats: ResMut<VoxelPerfStats>,
    worlds: Query<(&UsfScaleLayer, &VoxelWorld)>,
    render_aggregates: Query<(
        &UsfScaleLayer,
        &VoxelRenderAggregate,
        Option<&Collider>,
        Option<&RigidBody>,
    )>,
    aggregate_mesh_presentations: Query<(), (With<Mesh3d>, With<VoxelRenderAggregatePresentation>)>,
    generation: Query<(), With<VoxelAggregateGenerationTask>>,
    derived: Query<(), With<VoxelDerivedTask>>,
) {
    let now = Instant::now();
    if let Some(previous) = stats.last_frame_at.replace(now) {
        let frame_us = now
            .duration_since(previous)
            .as_micros()
            .min(u128::from(u64::MAX)) as u64;
        stats.frames += 1;
        stats.frame_us = stats.frame_us.saturating_add(frame_us);
        stats.frame_max_us = stats.frame_max_us.max(frame_us);
    }

    let report_started_at = *stats.report_started_at.get_or_insert(now);
    if now.duration_since(report_started_at) < Duration::from_secs(1) {
        return;
    }

    let mut per_scale = BTreeMap::<i8, ScaleLiveCounts>::new();
    let mut resident_materializations = 0usize;
    let mut active_materializations = 0usize;
    let mut inactive_cached = 0usize;
    let mut pending_materializations = 0usize;
    let mut dense_materializations = 0usize;
    let mut active_dense_materializations = 0usize;
    let mut surface_caches = 0usize;
    let mut active_surface_caches = 0usize;
    let mut dirty_materializations = 0usize;

    for (layer, world) in &worlds {
        let store = world.materializations();
        resident_materializations += store.resident_count();
        active_materializations += store.active_count();
        inactive_cached += store.inactive_count();
        pending_materializations += store.pending_count();
        dense_materializations += store.dense_count();
        active_dense_materializations += store.active_dense_count();
        surface_caches += store.surface_cache_count();
        active_surface_caches += store.active_surface_count();
        dirty_materializations += store.dirty_derived_count();

        let scale = per_scale.entry(layer.scale().exponent()).or_default();
        scale.active += store.active_count();
        scale.dense += store.active_dense_count();
        scale.surfaces += store.active_surface_count();
    }

    let mut aggregate_members = 0usize;
    let mut aggregate_max_members = 0usize;
    let mut colliders = 0usize;
    let mut rigid_bodies = 0usize;
    let mut collider_wanted = 0usize;
    let mut collider_missing_visible = 0usize;

    for (layer, aggregate, collider, rigid_body) in &render_aggregates {
        let member_count = aggregate.member_count();
        aggregate_members += member_count;
        aggregate_max_members = aggregate_max_members.max(member_count);

        let scale = per_scale.entry(layer.scale().exponent()).or_default();
        scale.render_aggregates += 1;

        if collider.is_some() {
            colliders += 1;
            scale.colliders += 1;
        }
        if rigid_body.is_some() {
            rigid_bodies += 1;
        }

        if aggregate_collider_proximity_squared(
            &view,
            aggregate.scope(),
            layer,
            config.voxel.manifestation.physics_interaction_radius_native,
        )
        .is_some()
        {
            collider_wanted += 1;
            if collider.is_none() {
                collider_missing_visible += 1;
            }
        }
    }

    let render_aggregate_count = render_aggregates.iter().count();
    let aggregate_avg_members = if render_aggregate_count == 0 {
        0.0
    } else {
        aggregate_members as f64 / render_aggregate_count as f64
    };

    let frame_avg_ms = if stats.frames == 0 {
        0.0
    } else {
        stats.frame_us as f64 / stats.frames as f64 / 1000.0
    };
    let fixed_steps_avg = if stats.fixed_step_samples == 0 {
        0.0
    } else {
        stats.fixed_steps_total as f64 / stats.fixed_step_samples as f64
    };
    let generation_avg_ms = if stats.generated == 0 {
        0.0
    } else {
        stats.generation_us as f64 / stats.generated as f64 / 1000.0
    };
    let derived_avg_ms = if stats.derived == 0 {
        0.0
    } else {
        stats.derived_us as f64 / stats.derived as f64 / 1000.0
    };
    let aggregate_rebuild_avg_ms = if stats.render_aggregate_rebuilds == 0 {
        0.0
    } else {
        stats.render_aggregate_rebuild_us as f64 / stats.render_aggregate_rebuilds as f64 / 1000.0
    };

    info!(
        resident_materializations,
        active_materializations,
        inactive_cached,
        pending_materializations,
        dense_materializations,
        active_dense_materializations,
        surface_caches,
        active_surface_caches,
        dirty_materializations,
        // Compatibility names retained in logs while the monitor transitions.
        reserved_chunks = active_materializations,
        loaded_chunks = active_dense_materializations,
        surface_chunks = active_surface_caches,
        render_aggregates = render_aggregate_count,
        aggregate_mesh_presentations = aggregate_mesh_presentations.iter().count(),
        aggregate_members,
        aggregate_avg_members,
        aggregate_max_members,
        mesh_assets = meshes.len(),
        colliders,
        rigid_bodies,
        collider_wanted,
        collider_missing_visible,
        manifestation_group_base_chunks_per_axis =
            config.voxel.manifestation.grouping.base_chunks_per_axis,
        manifestation_group_native_units_per_axis =
            config.voxel.manifestation.grouping.base_chunks_per_axis as f32
                * super::super::MATERIALIZATION_CHUNK_SIZE as f32,
        manifestation_rebuild_budget_per_frame =
            config.voxel.manifestation.rebuild_budget_per_frame,
        physics_interaction_radius_native =
            config.voxel.manifestation.physics_interaction_radius_native,
        generation_in_flight = generation.iter().count(),
        derived_in_flight = derived.iter().count(),
        frame_avg_ms,
        frame_max_ms = stats.frame_max_us as f64 / 1000.0,
        fixed_steps_avg,
        fixed_steps_max = stats.fixed_steps_max,
        fixed_timestep_ms = fixed_time.timestep().as_secs_f64() * 1000.0,
        fixed_overstep_ms = fixed_time.overstep().as_secs_f64() * 1000.0,
        virtual_delta_ms = virtual_time.delta_secs_f64() * 1000.0,
        virtual_max_delta_ms = virtual_time.max_delta().as_secs_f64() * 1000.0,
        generated = stats.generated,
        generation_avg_ms,
        generation_max_ms = stats.generation_max_us as f64 / 1000.0,
        derived = stats.derived,
        derived_avg_ms,
        derived_max_ms = stats.derived_max_us as f64 / 1000.0,
        render_aggregate_rebuilds = stats.render_aggregate_rebuilds,
        aggregate_rebuild_avg_ms,
        aggregate_rebuild_max_ms = stats.render_aggregate_rebuild_max_us as f64 / 1000.0,
        demand_rebuilds = stats.demand_rebuilds,
        worker_limit_per_stage = per_stage_in_flight_limit(),
        ?per_scale,
        "voxel perf"
    );

    *stats = VoxelPerfStats {
        report_started_at: Some(now),
        last_frame_at: Some(now),
        ..default()
    };
}
