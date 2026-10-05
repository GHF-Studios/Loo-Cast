//! Voxel realization developer visualization.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    devtools::{DeveloperSet, DeveloperTools, DrawDepth, WorldDrawBatch, WorldDrawFrame},
    spatial::{
        SPATIAL_DEMAND_VISUALIZATION, UsfCapabilityRealization,
        UsfPrimaryInteractionSlice, UsfScaleLayer, UsfScaleRoleMask,
        UsfRuntimeChartState, UsfViewContext, UsfViewRenderAnchor,
    },
};

use super::{
    CelestialVoxelRealization, MATERIALIZATION_CHUNK_SIZE,
    VoxelRealizationDemandSnapshot, VoxelStreaming, VoxelStreamingTelemetry,
    VoxelWorld,
    resolution::CelestialClipmapTelemetry,
    manifestation::{
        VoxelMaterializationPresentation, VoxelMaterializationRuntime,
    },
};

pub(super) fn configure(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (
            collect_voxel_materialization_world_draw
                .in_set(DeveloperSet::CollectWorldDraw),
            terrain_pipeline_census
                .after(crate::spatial::UsfSpatialSet::ViewProjection),
        ),
    );
}


#[derive(Default)]
struct TerrainPipelineCensusState {
    seconds_until_report: f32,
}

#[derive(Default, Clone, Copy)]
struct RuntimeCensus {
    active: usize,
    presentation_ready: usize,
    collision_ready: usize,
    visible_presentations: usize,
}

/// Temporary forensic census for the celestial terrain regression.
///
/// This intentionally changes no demand, residency, generation, capability or
/// presentation policy. It reports where the coarse->fine pipeline stops:
///
/// demand -> residency -> dense generation -> sign transition -> derived
/// surface -> manifestation -> PRESENTATION/COLLISION readiness -> visibility.
fn terrain_pipeline_census(
    time: Res<Time>,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    telemetry: Res<VoxelStreamingTelemetry>,
    clipmap: Res<CelestialClipmapTelemetry>,
    demands: Res<VoxelRealizationDemandSnapshot>,
    worlds: Query<
        (
            Entity,
            &VoxelWorld,
            &UsfScaleLayer,
            Option<&VoxelStreaming>,
        ),
        With<CelestialVoxelRealization>,
    >,
    runtimes: Query<(
        &VoxelMaterializationRuntime,
        Option<&UsfCapabilityRealization>,
    )>,
    presentations: Query<
        (&ChildOf, &Visibility),
        With<VoxelMaterializationPresentation>,
    >,
    mut state: Local<TerrainPipelineCensusState>,
) {
    state.seconds_until_report -= time.delta_secs().max(0.0);
    if state.seconds_until_report > 0.0 {
        return;
    }
    state.seconds_until_report = 3.0;

    let mut runtime_by_world = HashMap::<Entity, RuntimeCensus>::new();

    for (runtime, capability) in &runtimes {
        if !runtime.active() {
            continue;
        }
        let entry = runtime_by_world.entry(runtime.world()).or_default();
        entry.active += 1;

        if let Some(capability) = capability {
            if capability
                .roles()
                .contains(UsfScaleRoleMask::PRESENTATION)
            {
                entry.presentation_ready += 1;
            }
            if capability
                .roles()
                .contains(UsfScaleRoleMask::COLLISION)
            {
                entry.collision_ready += 1;
            }
        }
    }

    for (parent, visibility) in &presentations {
        if matches!(*visibility, Visibility::Hidden) {
            continue;
        }
        let Ok((runtime, _)) = runtimes.get(parent.0) else {
            continue;
        };
        if !runtime.active() {
            continue;
        }
        runtime_by_world
            .entry(runtime.world())
            .or_default()
            .visible_presentations += 1;
    }

    let mut rows = Vec::new();

    for (entity, world, layer, streaming) in &worlds {
        let store = world.materializations();

        let active_keys = store.active_keys().collect::<Vec<_>>();
        let active = active_keys.len();
        let desired = streaming.map_or(active, VoxelStreaming::desired_count);
        let pending_desired =
            streaming.map_or(0, VoxelStreaming::pending_desired_len);
        let warm_inactive = store.inactive_count();
        let total_materializations = store.total_count();

        let mut dense = 0usize;
        let mut sign_transition = 0usize;
        for (_, chunk) in store.active_dense_entries() {
            dense += 1;
            if chunk.has_surface_transition() {
                sign_transition += 1;
            }
        }

        let derived = active_keys
            .iter()
            .filter(|&&key| store.active_derived_revision(key).is_some())
            .count();
        let surfaces = active_keys
            .iter()
            .filter(|&&key| store.active_surface(key).is_some())
            .count();

        let runtime = runtime_by_world.get(&entity).copied().unwrap_or_default();
        let demand_scopes = demands.requests_for(entity).count();

        rows.push((
            layer.scale().exponent(),
            demand_scopes,
            desired,
            pending_desired,
            active,
            warm_inactive,
            total_materializations,
            active.saturating_sub(dense),
            dense,
            sign_transition,
            derived,
            surfaces,
            runtime,
        ));
    }

    rows.sort_by_key(|row| row.0);

    info!(
        view_scale = %view.scale(),
        view_exponent = view.continuous_exponent(),
        interaction_scale = %interaction.scale(),
        interaction_target_scale = %interaction.target_scale(),
        interaction_handoff_pending = interaction.handoff_pending(),
        worker_state = %telemetry.summary(),
        clipmap_state = %clipmap.summary(),
        worlds = rows.len(),
        "terrain_pipeline_census"
    );

    for (
        exponent,
        demand_scopes,
        desired,
        pending_desired,
        active,
        warm_inactive,
        total_materializations,
        pending_generation,
        dense,
        sign_transition,
        derived,
        surfaces,
        runtime,
    ) in rows
    {
        info!(
            scale = exponent,
            demand_scopes,
            desired,
            pending_desired,
            active,
            warm_inactive,
            total_materializations,
            pending_generation,
            dense,
            sign_transition,
            derived,
            surfaces,
            runtime_active = runtime.active,
            presentation_ready = runtime.presentation_ready,
            collision_ready = runtime.collision_ready,
            visible_presentations = runtime.visible_presentations,
            "terrain_pipeline_scale"
        );
    }
}

fn collect_voxel_materialization_world_draw(
    tools: Res<DeveloperTools>,
    spatial_frame: Res<UsfRuntimeChartState>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    worlds: Query<(&VoxelWorld, &UsfScaleLayer)>,
    frame: Res<WorldDrawFrame>,
) {
    if !tools.visualization_enabled(SPATIAL_DEMAND_VISUALIZATION) {
        return;
    }

    let mut batch = WorldDrawBatch::default();
    let size = MATERIALIZATION_CHUNK_SIZE as f32;
    let extent = Vec3::splat(size);
    let color = Color::srgba(0.35, 1.0, 0.38, 0.82);

    for (world, layer) in &worlds {
        // This overlay is drawn in the local physical view, not in all 71
        // numerical charts superimposed on one another.
        if layer.scale() != interaction.target_scale() {
            continue;
        }
        for key in world.materializations().active_keys() {
            let Ok(address) = world.materialization_address(key) else {
                continue;
            };
            let Ok(translation) = address.origin().relative_at_scale_bounded(
                spatial_frame.origin(), layer.scale(), 16_384.0,
            )
            else {
                continue;
            };
            draw_wire_box(&mut batch, translation, translation + extent, color);
        }
    }

    frame.submit(batch);
}

fn draw_wire_box(batch: &mut WorldDrawBatch, min: Vec3, max: Vec3, color: Color) {
    let corners = [
        Vec3::new(min.x, min.y, min.z),
        Vec3::new(max.x, min.y, min.z),
        Vec3::new(min.x, max.y, min.z),
        Vec3::new(max.x, max.y, min.z),
        Vec3::new(min.x, min.y, max.z),
        Vec3::new(max.x, min.y, max.z),
        Vec3::new(min.x, max.y, max.z),
        Vec3::new(max.x, max.y, max.z),
    ];

    for (a, b) in [
        (0, 1),
        (0, 2),
        (1, 3),
        (2, 3),
        (4, 5),
        (4, 6),
        (5, 7),
        (6, 7),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ] {
        batch.line(corners[a], corners[b], color, DrawDepth::Overlay);
    }
}
