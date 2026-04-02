use crate::bevy::prelude::*;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::runtime::manifestation_field::{ROOT_AXIS_PERIOD_UNITS, canonical_grid_coord, sample_root_native_position};
use crate::usf::scale::Scale;
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChunkInstanceCullCandidate {
    pub entity: Entity,
    pub coarse_depth: u8,
    pub distance_sq: f32,
    pub front_dot: f32,
}

pub(crate) fn select_visible_chunk_instance_entities(candidates: &[ChunkInstanceCullCandidate], active_budget: usize) -> HashSet<Entity> {
    let mut grouped = BTreeMap::<u8, Vec<ChunkInstanceCullCandidate>>::new();
    for candidate in candidates.iter().copied() {
        grouped.entry(candidate.coarse_depth).or_default().push(candidate);
    }

    let mut selected = HashSet::<Entity>::new();
    for (depth, mut entries) in grouped {
        entries.sort_by(|a, b| {
            b.front_dot
                .total_cmp(&a.front_dot)
                .then_with(|| a.distance_sq.total_cmp(&b.distance_sq))
                .then_with(|| a.entity.to_bits().cmp(&b.entity.to_bits()))
        });
        let budget = chunk_instance_band_budget(depth, active_budget).min(entries.len());
        for candidate in entries.into_iter().take(budget) {
            selected.insert(candidate.entity);
        }
    }

    selected
}

#[inline]
pub(crate) fn chunk_center_in_active_native_space(chunk_coord: &GridVec, player_root_native: (f64, f64, f64), active_scale_index: i16) -> Vec3 {
    let chunk_canonical = canonical_grid_coord(chunk_coord);
    let chunk_root_native = sample_root_native_position(&chunk_canonical, Vec3::ZERO);
    let root_to_active = 10.0_f64.powi(active_scale_index as i32);

    let dx = wrap_root_native_delta_axis(chunk_root_native.0 - player_root_native.0) * root_to_active;
    let dy = wrap_root_native_delta_axis(chunk_root_native.1 - player_root_native.1) * root_to_active;
    let dz = wrap_root_native_delta_axis(chunk_root_native.2 - player_root_native.2) * root_to_active;
    Vec3::new(dx as f32, dy as f32, dz as f32)
}

#[inline]
pub(crate) fn chunk_instance_scale_in_active_native_space(chunk_scale: Scale, active_scale_index: i16) -> f32 {
    let exponent = active_scale_index - chunk_scale.index_from_top() as i16;
    let scale = 10.0_f64.powi(exponent as i32);
    if !scale.is_finite() || scale <= 0.0 {
        return f32::MIN_POSITIVE;
    }
    scale.clamp(f32::MIN_POSITIVE as f64, f32::MAX as f64) as f32
}

#[inline]
fn wrap_root_native_delta_axis(delta: f64) -> f64 {
    let period = ROOT_AXIS_PERIOD_UNITS as f64;
    if !delta.is_finite() || period <= f64::EPSILON {
        return 0.0;
    }
    let half = period * 0.5;
    ((delta + half).rem_euclid(period)) - half
}

#[inline]
fn chunk_instance_band_budget(coarse_depth: u8, active_budget: usize) -> usize {
    match coarse_depth {
        0 => active_budget.max(1),
        1 => (active_budget / 3).max(6),
        2 => (active_budget / 6).max(2),
        3 => (active_budget / 12).max(1),
        _ => 1,
    }
}
