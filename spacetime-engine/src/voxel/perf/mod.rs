//! Lightweight voxel realization counters used alongside Tracy/Vapor diagnostics.

use std::{
    collections::BTreeMap,
    num::NonZeroUsize,
    time::{Duration, Instant},
};

use avian3d::prelude::{Collider, RigidBody};
use bevy::{prelude::*, time::Virtual};

use crate::{
    config::EngineConfig,
    spatial::{UsfScaleLayer, UsfViewFrame},
};

use super::{
    VoxelWorld,
    async_pipeline::VoxelDerivedTask,
    manifestation::{
        VoxelRenderAggregate, VoxelRenderAggregatePresentation,
        aggregate_collider_proximity_squared,
    },
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
    report_started_at: Option<Instant>,
    last_frame_at: Option<Instant>,
    generated: u64,
    generation_us: u64,
    generation_max_us: u64,
    derived: u64,
    derived_us: u64,
    derived_max_us: u64,
    render_aggregate_rebuilds: u64,
    render_aggregate_rebuild_us: u64,
    render_aggregate_rebuild_max_us: u64,
    demand_rebuilds: u64,
    frames: u64,
    frame_us: u64,
    frame_max_us: u64,
    fixed_step_samples: u64,
    fixed_steps_total: u64,
    fixed_steps_max: u32,
}

#[derive(Resource, Debug, Default)]
pub(crate) struct FixedStepProbe {
    steps_this_frame: u32,
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

    pub(crate) fn record_render_aggregate_rebuild(&mut self, us: u64) {
        self.render_aggregate_rebuilds += 1;
        self.render_aggregate_rebuild_us = self.render_aggregate_rebuild_us.saturating_add(us);
        self.render_aggregate_rebuild_max_us = self.render_aggregate_rebuild_max_us.max(us);
    }

    pub(crate) fn record_demand_rebuild(&mut self) {
        self.demand_rebuilds += 1;
    }
}

mod reporting;
mod systems;

pub(crate) use reporting::report_voxel_perf;
pub(crate) use systems::{count_fixed_step, sample_fixed_steps};
