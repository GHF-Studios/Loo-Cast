//! Temporary aggregate counters until full profiling is integrated.
use bevy::prelude::*;
use std::num::NonZeroUsize;

use super::{
    VoxelChunk, async_pipeline::VoxelDerivedTask, streaming::VoxelAggregateGenerationTask,
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

pub(crate) fn report_voxel_perf(
    time: Res<Time>,
    mut stats: ResMut<VoxelPerfStats>,
    chunks: Query<(), With<VoxelChunk>>,
    generation: Query<(), With<VoxelAggregateGenerationTask>>,
    derived: Query<(), With<VoxelDerivedTask>>,
) {
    stats.elapsed += time.delta_secs();
    if stats.elapsed < 1.0 {
        return;
    }

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
        loaded_chunks = chunks.iter().count(),
        generation_in_flight = generation.iter().count(),
        derived_in_flight = derived.iter().count(),
        generated = stats.generated,
        generation_avg_ms,
        generation_max_ms = stats.generation_max_us as f64 / 1000.0,
        derived = stats.derived,
        derived_avg_ms,
        derived_max_ms = stats.derived_max_us as f64 / 1000.0,
        uniform_shortcuts = stats.uniform_shortcuts,
        demand_rebuilds = stats.demand_rebuilds,
        worker_limit_per_stage = per_stage_in_flight_limit(),
        "voxel perf"
    );

    *stats = VoxelPerfStats::default();
}
