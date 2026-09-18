//! Temporary aggregate counters until full profiling is integrated.

use std::{collections::BTreeMap, num::NonZeroUsize};

use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::spatial::{UsfScaleLayer, UsfViewFrame};

use super::{
    VoxelChunk, VoxelChunkPhysicsLod, VoxelChunkPresentation, VoxelMaterializationChunkAddress,
    VoxelWorld,
    async_pipeline::{VoxelDerivedPurpose, VoxelDerivedTask, collider_proximity_squared},
    streaming::VoxelAggregateGenerationTask,
};

pub(crate) fn per_stage_in_flight_limit() -> usize {
    let threads = std::thread::available_parallelism()
        .unwrap_or(NonZeroUsize::new(4).unwrap())
        .get();
    (threads / 2).max(2)
}

#[derive(Resource, Debug, Default)]
pub(crate) struct VoxelPerfStats {
    elapsed: f32,
    generated: u64,
    generation_us: u64,
    generation_max_us: u64,
    derived: u64,
    derived_us: u64,
    derived_max_us: u64,
    uniform_shortcuts: u64,
    demand_rebuilds: u64,
    frames: u64,
    frame_us: u64,
    frame_max_us: u64,
}

impl VoxelPerfStats {
    pub(crate) fn record_generation(&mut self, us: u64) {
        self.generated += 1;
        self.generation_us = self.generation_us.saturating_add(us);
        self.generation_max_us = self.generation_max_us.max(us);
    }

    pub(crate) fn record_derived(&mut self, us: u64) {
        self.derived += 1;
        self.derived_us = self.derived_us.saturating_add(us);
        self.derived_max_us = self.derived_max_us.max(us);
    }

    pub(crate) fn record_uniform_shortcut(&mut self) {
        self.uniform_shortcuts += 1;
    }

    pub(crate) fn record_demand_rebuild(&mut self) {
        self.demand_rebuilds += 1;
    }
}

#[derive(Debug, Default)]
struct ScaleLiveCounts {
    reserved: usize,
    generated: usize,
    mesh_presentations: usize,
    colliders: usize,
    collider_wanted: usize,
    collider_pending: usize,
    collider_waiting: usize,
    collider_missing_visible: usize,
}

pub(crate) fn report_voxel_perf(
    time: Res<Time>,
    view: Res<UsfViewFrame>,
    meshes: Res<Assets<Mesh>>,
    mut stats: ResMut<VoxelPerfStats>,
    worlds: Query<(&UsfScaleLayer, &VoxelWorld)>,
    chunks: Query<(
        &UsfScaleLayer,
        &VoxelChunk,
        &VoxelChunkPresentation,
        &VoxelChunkPhysicsLod,
        &VoxelMaterializationChunkAddress,
        Option<&VoxelDerivedTask>,
        Option<&Collider>,
        Option<&RigidBody>,
    )>,
    presentation_meshes: Query<(), With<Mesh3d>>,
    generation: Query<(), With<VoxelAggregateGenerationTask>>,
) {
    let frame_us = (time.delta_secs() * 1_000_000.0) as u64;
    stats.frames += 1;
    stats.frame_us = stats.frame_us.saturating_add(frame_us);
    stats.frame_max_us = stats.frame_max_us.max(frame_us);
    stats.elapsed += time.delta_secs();
    if stats.elapsed < 1.0 {
        return;
    }

    let mut per_scale = BTreeMap::<i8, ScaleLiveCounts>::new();
    let mut reserved_chunks = 0usize;
    for (layer, world) in &worlds {
        let reserved = world.len();
        reserved_chunks += reserved;
        per_scale
            .entry(layer.scale().exponent())
            .or_default()
            .reserved += reserved;
    }

    let mut loaded_chunks = 0usize;
    let mut mesh_presentations = 0usize;
    let mut colliders = 0usize;
    let mut rigid_bodies = 0usize;
    let mut dirty_chunks = 0usize;
    let mut derived_in_flight = 0usize;
    let mut geometry_in_flight = 0usize;
    let mut collider_only_in_flight = 0usize;
    let mut collider_wanted = 0usize;
    let mut collider_ready = 0usize;
    let mut collider_pending = 0usize;
    let mut collider_waiting = 0usize;
    let mut collider_missing_visible = 0usize;

    for (layer, chunk, presentation, physics_lod, address, task, collider, rigid_body) in &chunks {
        loaded_chunks += 1;
        let scale = per_scale.entry(layer.scale().exponent()).or_default();
        scale.generated += 1;

        if chunk.needs_remesh() {
            dirty_chunks += 1;
        }

        let has_mesh = presentation_meshes.get(presentation.0).is_ok();
        if has_mesh {
            mesh_presentations += 1;
            scale.mesh_presentations += 1;
        }
        if collider.is_some() {
            colliders += 1;
            scale.colliders += 1;
        }
        if rigid_body.is_some() {
            rigid_bodies += 1;
        }

        if let Some(task) = task {
            derived_in_flight += 1;
            match task.purpose {
                VoxelDerivedPurpose::Geometry => geometry_in_flight += 1,
                VoxelDerivedPurpose::ColliderOnly => collider_only_in_flight += 1,
            }
        }

        if collider_proximity_squared(&view, address, layer).is_some() {
            collider_wanted += 1;
            scale.collider_wanted += 1;
            if collider.is_some() {
                collider_ready += 1;
            } else if task.is_some() {
                collider_pending += 1;
                scale.collider_pending += 1;
            } else if physics_lod.requested && physics_lod.built_revision == Some(chunk.revision())
            {
                // A completed request at the current geometry revision with a
                // visible render surface but no Collider means collider creation
                // returned None. This is distinct from queue starvation.
                if has_mesh {
                    collider_missing_visible += 1;
                    scale.collider_missing_visible += 1;
                }
            } else {
                collider_waiting += 1;
                scale.collider_waiting += 1;
            }
        }
    }

    let frame_avg_ms = if stats.frames == 0 {
        0.0
    } else {
        stats.frame_us as f64 / stats.frames as f64 / 1000.0
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
    info!(
        reserved_chunks,
        reserved_not_generated = reserved_chunks.saturating_sub(loaded_chunks),
        loaded_chunks,
        mesh_presentations,
        mesh_assets = meshes.len(),
        colliders,
        rigid_bodies,
        dirty_chunks,
        generation_in_flight = generation.iter().count(),
        derived_in_flight,
        geometry_in_flight,
        collider_only_in_flight,
        collider_outside = loaded_chunks.saturating_sub(collider_wanted),
        collider_wanted,
        collider_ready,
        collider_pending,
        collider_waiting,
        collider_missing_visible,
        frame_avg_ms,
        frame_max_ms = stats.frame_max_us as f64 / 1000.0,
        generated = stats.generated,
        generation_avg_ms,
        generation_max_ms = stats.generation_max_us as f64 / 1000.0,
        derived = stats.derived,
        derived_avg_ms,
        derived_max_ms = stats.derived_max_us as f64 / 1000.0,
        uniform_shortcuts = stats.uniform_shortcuts,
        demand_rebuilds = stats.demand_rebuilds,
        worker_limit_per_stage = per_stage_in_flight_limit(),
        ?per_scale,
        "voxel perf"
    );

    *stats = VoxelPerfStats::default();
}
