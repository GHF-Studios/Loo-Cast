//! Deterministic local and canonical noise primitives for voxel bases.

use std::cell::Cell;

use bevy::{
    math::DVec3,
    prelude::{Vec2, Vec3},
};

use crate::spatial::{SPATIAL_SCALE_MAX, SpatialScale};

use super::super::VoxelQueryPosition;
use super::TERRAIN_DIRECT_LOCAL_LIMIT;

pub(super) fn volumetric_noise(
    world_origin: VoxelQueryPosition,
    point: VoxelQueryPosition,
    frequency: f32,
    seed: u32,
) -> f32 {
    let frequency = frequency.max(f32::EPSILON);
    if let Ok(local) = point.relative_to(world_origin, TERRAIN_DIRECT_LOCAL_LIMIT) {
        value_noise_3d(local * frequency, seed)
    } else {
        semantic_value_noise_3d(point, canonical_cell_size(1.0 / frequency), seed)
    }
}

//
// Most canonical surface evaluation is ordinary value-noise interpolation.
// Within one 9^3 clipmap block, dozens of independent terrain domains repeatedly
// sample the same integer noise cell. Cache the COMPLETE eight-corner cell per
// durable worker thread: one lookup per value_noise_3d call instead of eight
// repeated hash_noise_3d evaluations. Collisions only recompute.

mod local;
mod semantic;

use semantic::SEMANTIC_NOISE_DIAGNOSTICS;

pub(super) use local::value_noise;
pub(crate) use local::value_noise_3d;
pub(crate) use semantic::{
    PreparedSemanticNoisePoint, SemanticNoiseCornerCache, semantic_value_noise_3d_cached,
    semantic_value_noise_3d_cached_prepared,
};
pub(super) use semantic::{
    canonical_cell_size, mix, scale_layer_seed, semantic_value_noise, semantic_value_noise_3d,
};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct SemanticNoiseDiagnosticSnapshot {
    pub(crate) prepared_point_attempts: u64,
    pub(crate) prepared_point_successes: u64,
    pub(crate) prepared_point_nonfinite_failures: u64,
    pub(crate) prepared_point_range_failures: u64,
    pub(crate) prepared_point_overflow_failures: u64,
    pub(crate) prepared_cell_address_failures: u64,

    pub(crate) fast_path_completions: u64,
    pub(crate) generic_fallbacks: u64,

    pub(crate) cell_20_hits: u64,
    pub(crate) cell_20_misses: u64,
    pub(crate) cell_5_hits: u64,
    pub(crate) cell_5_misses: u64,
    pub(crate) cell_empty_misses: u64,
    pub(crate) cell_collision_misses: u64,

    pub(crate) corner_hits: u64,
    pub(crate) corner_misses: u64,
    pub(crate) corner_empty_misses: u64,
    pub(crate) corner_collision_misses: u64,
    pub(crate) corner_hash_computes: u64,
    pub(crate) compact_axis_decompositions: u64,
    pub(crate) compact_digit_steps: u64,
}

impl SemanticNoiseDiagnosticSnapshot {
    pub(crate) fn delta_since(self, before: Self) -> Self {
        macro_rules! delta {
            ($field:ident) => {
                self.$field.saturating_sub(before.$field)
            };
        }

        Self {
            prepared_point_attempts: delta!(prepared_point_attempts),
            prepared_point_successes: delta!(prepared_point_successes),
            prepared_point_nonfinite_failures: delta!(prepared_point_nonfinite_failures),
            prepared_point_range_failures: delta!(prepared_point_range_failures),
            prepared_point_overflow_failures: delta!(prepared_point_overflow_failures),
            prepared_cell_address_failures: delta!(prepared_cell_address_failures),
            fast_path_completions: delta!(fast_path_completions),
            generic_fallbacks: delta!(generic_fallbacks),
            cell_20_hits: delta!(cell_20_hits),
            cell_20_misses: delta!(cell_20_misses),
            cell_5_hits: delta!(cell_5_hits),
            cell_5_misses: delta!(cell_5_misses),
            cell_empty_misses: delta!(cell_empty_misses),
            cell_collision_misses: delta!(cell_collision_misses),
            corner_hits: delta!(corner_hits),
            corner_misses: delta!(corner_misses),
            corner_empty_misses: delta!(corner_empty_misses),
            corner_collision_misses: delta!(corner_collision_misses),
            corner_hash_computes: delta!(corner_hash_computes),
            compact_axis_decompositions: delta!(compact_axis_decompositions),
            compact_digit_steps: delta!(compact_digit_steps),
        }
    }
}

#[derive(Debug)]
struct SemanticNoiseDiagnosticsStorage {
    prepared_point_attempts: Cell<u64>,
    prepared_point_successes: Cell<u64>,
    prepared_point_nonfinite_failures: Cell<u64>,
    prepared_point_range_failures: Cell<u64>,
    prepared_point_overflow_failures: Cell<u64>,
    prepared_cell_address_failures: Cell<u64>,

    fast_path_completions: Cell<u64>,
    generic_fallbacks: Cell<u64>,

    cell_20_hits: Cell<u64>,
    cell_20_misses: Cell<u64>,
    cell_5_hits: Cell<u64>,
    cell_5_misses: Cell<u64>,
    cell_empty_misses: Cell<u64>,
    cell_collision_misses: Cell<u64>,

    corner_hits: Cell<u64>,
    corner_misses: Cell<u64>,
    corner_empty_misses: Cell<u64>,
    corner_collision_misses: Cell<u64>,
    corner_hash_computes: Cell<u64>,
    compact_axis_decompositions: Cell<u64>,
    compact_digit_steps: Cell<u64>,
}

impl SemanticNoiseDiagnosticsStorage {
    fn new() -> Self {
        Self {
            prepared_point_attempts: Cell::new(0),
            prepared_point_successes: Cell::new(0),
            prepared_point_nonfinite_failures: Cell::new(0),
            prepared_point_range_failures: Cell::new(0),
            prepared_point_overflow_failures: Cell::new(0),
            prepared_cell_address_failures: Cell::new(0),
            fast_path_completions: Cell::new(0),
            generic_fallbacks: Cell::new(0),
            cell_20_hits: Cell::new(0),
            cell_20_misses: Cell::new(0),
            cell_5_hits: Cell::new(0),
            cell_5_misses: Cell::new(0),
            cell_empty_misses: Cell::new(0),
            cell_collision_misses: Cell::new(0),
            corner_hits: Cell::new(0),
            corner_misses: Cell::new(0),
            corner_empty_misses: Cell::new(0),
            corner_collision_misses: Cell::new(0),
            corner_hash_computes: Cell::new(0),
            compact_axis_decompositions: Cell::new(0),
            compact_digit_steps: Cell::new(0),
        }
    }

    #[inline]
    fn bump(counter: &Cell<u64>) {
        counter.set(counter.get().saturating_add(1));
    }

    #[inline]
    fn add(counter: &Cell<u64>, amount: u64) {
        counter.set(counter.get().saturating_add(amount));
    }

    fn snapshot(&self) -> SemanticNoiseDiagnosticSnapshot {
        SemanticNoiseDiagnosticSnapshot {
            prepared_point_attempts: self.prepared_point_attempts.get(),
            prepared_point_successes: self.prepared_point_successes.get(),
            prepared_point_nonfinite_failures: self.prepared_point_nonfinite_failures.get(),
            prepared_point_range_failures: self.prepared_point_range_failures.get(),
            prepared_point_overflow_failures: self.prepared_point_overflow_failures.get(),
            prepared_cell_address_failures: self.prepared_cell_address_failures.get(),
            fast_path_completions: self.fast_path_completions.get(),
            generic_fallbacks: self.generic_fallbacks.get(),
            cell_20_hits: self.cell_20_hits.get(),
            cell_20_misses: self.cell_20_misses.get(),
            cell_5_hits: self.cell_5_hits.get(),
            cell_5_misses: self.cell_5_misses.get(),
            cell_empty_misses: self.cell_empty_misses.get(),
            cell_collision_misses: self.cell_collision_misses.get(),
            corner_hits: self.corner_hits.get(),
            corner_misses: self.corner_misses.get(),
            corner_empty_misses: self.corner_empty_misses.get(),
            corner_collision_misses: self.corner_collision_misses.get(),
            corner_hash_computes: self.corner_hash_computes.get(),
            compact_axis_decompositions: self.compact_axis_decompositions.get(),
            compact_digit_steps: self.compact_digit_steps.get(),
        }
    }

    #[inline]
    fn record_cell_hit(&self, cell_size: i32) {
        match cell_size {
            20 => Self::bump(&self.cell_20_hits),
            5 => Self::bump(&self.cell_5_hits),
            _ => {}
        }
    }

    #[inline]
    fn record_cell_miss(&self, cell_size: i32, occupied: bool) {
        match cell_size {
            20 => Self::bump(&self.cell_20_misses),
            5 => Self::bump(&self.cell_5_misses),
            _ => {}
        }
        if occupied {
            Self::bump(&self.cell_collision_misses);
        } else {
            Self::bump(&self.cell_empty_misses);
        }
    }

    #[inline]
    fn record_corner_hit(&self) {
        Self::bump(&self.corner_hits);
    }

    #[inline]
    fn record_corner_miss(&self, occupied: bool) {
        Self::bump(&self.corner_misses);
        if occupied {
            Self::bump(&self.corner_collision_misses);
        } else {
            Self::bump(&self.corner_empty_misses);
        }
    }
}

pub(crate) fn semantic_noise_diagnostic_snapshot() -> SemanticNoiseDiagnosticSnapshot {
    SEMANTIC_NOISE_DIAGNOSTICS.with(SemanticNoiseDiagnosticsStorage::snapshot)
}

#[inline]
pub(crate) fn record_fine_residual_fast_path_completion() {
    SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
        SemanticNoiseDiagnosticsStorage::bump(&diagnostics.fast_path_completions);
    });
}

#[inline]
pub(crate) fn record_fine_residual_generic_fallback() {
    SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
        SemanticNoiseDiagnosticsStorage::bump(&diagnostics.generic_fallbacks);
    });
}
