use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::usf::definition::DptSchema;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DptChunkKey {
    pub scale: Scale,
    pub coord: GridVec,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct DptChunkRecord {
    pub schema_revision: u64,
    pub metrics: Vec<f32>,
}

#[derive(Resource, Debug, Default)]
pub struct DptStore {
    pub chunks: HashMap<DptChunkKey, DptChunkRecord>,
}
impl DptStore {
    pub fn get_chunk(&self, key: &DptChunkKey) -> Option<&DptChunkRecord> {
        self.chunks.get(key)
    }

    pub fn ensure_chunk(&mut self, key: DptChunkKey, schema: &DptSchema) -> &DptChunkRecord {
        self.chunks.entry(key.clone()).or_insert_with(|| DptChunkRecord {
            schema_revision: schema.revision,
            metrics: deterministic_metric_vector(&key, schema.metrics.len()),
        })
    }
}

fn deterministic_metric_vector(key: &DptChunkKey, metric_count: usize) -> Vec<f32> {
    let sample = normalized_chunk_center(&key.coord);

    let elevation = clamp01(
        0.50 + 0.38 * coherent_wave(scale_point(sample, 0.42), 0x4f1b_bcdd_91a2_77c3) + 0.16 * coherent_wave(scale_point(sample, 1.37), 0x9e37_79b9_7f4a_7c15),
    );
    let temperature = clamp01(0.53 + 0.31 * coherent_wave(scale_point(sample, 0.23), 0x0f12_34ab_cd56_78ef) - 0.28 * (elevation - 0.5));
    let humidity =
        clamp01(0.50 + 0.34 * coherent_wave(offset_and_scale_point(sample, [13.0, -7.0, 3.0], 0.29), 0xbadc_0ffe_e0dd_f00d) - 0.26 * (temperature - 0.5));
    let vegetation_density = clamp01(0.66 * humidity + 0.18 * (1.0 - elevation) + 0.16 * (1.0 - (temperature - 0.55).abs() * 2.0));

    let mut metrics = Vec::with_capacity(metric_count);
    for idx in 0..metric_count {
        let value = match idx {
            0 => temperature,
            1 => humidity,
            2 => elevation,
            3 => vegetation_density,
            _ => {
                let seed = mix64((idx as u64).wrapping_mul(0x94d0_49bb_1331_11eb));
                clamp01(0.5 + 0.5 * coherent_wave(scale_point(sample, 0.41 + idx as f64 * 0.07), seed))
            }
        };
        metrics.push(value);
    }

    metrics
}

fn normalized_chunk_center(coord: &GridVec) -> [f64; 3] {
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;
    let mut z = 0.0_f64;
    let mut divisor = 1.0_f64;

    for xyz in coord.to_raw_vec_3d() {
        x += xyz.x as f64 / divisor;
        y += xyz.y as f64 / divisor;
        z += xyz.z as f64 / divisor;
        divisor *= 10.0_f64;
    }

    // Sample at chunk center so all chunks at a given scale produce stable interior values.
    let half_cell = 5.0_f64 / divisor;
    [x + half_cell, y + half_cell, z + half_cell]
}

#[inline]
fn scale_point(point: [f64; 3], scale: f64) -> [f64; 3] {
    [point[0] * scale, point[1] * scale, point[2] * scale]
}

#[inline]
fn offset_and_scale_point(point: [f64; 3], offset: [f64; 3], scale: f64) -> [f64; 3] {
    [(point[0] + offset[0]) * scale, (point[1] + offset[1]) * scale, (point[2] + offset[2]) * scale]
}

fn coherent_wave(point: [f64; 3], seed: u64) -> f32 {
    let mut total = 0.0_f64;
    let mut total_amp = 0.0_f64;
    let mut amp = 1.0_f64;
    let mut freq = 1.0_f64;

    for octave in 0..4_u64 {
        let phase_x = seed_phase(seed, 0x1000_0000_0000_0000 ^ octave);
        let phase_y = seed_phase(seed, 0x2000_0000_0000_0000 ^ octave);
        let phase_z = seed_phase(seed, 0x3000_0000_0000_0000 ^ octave);
        let wave = ((point[0] * freq + phase_x).sin() + (point[1] * freq + phase_y).cos() + (point[2] * freq + phase_z).sin()) / 3.0;
        total += wave * amp;
        total_amp += amp;
        amp *= 0.5;
        freq *= 2.0;
    }

    if total_amp == 0.0 {
        return 0.0;
    }

    (total / total_amp).clamp(-1.0, 1.0) as f32
}

#[inline]
fn seed_phase(seed: u64, salt: u64) -> f64 {
    seed_unit(seed, salt) * std::f64::consts::TAU
}

#[inline]
fn seed_unit(seed: u64, salt: u64) -> f64 {
    (mix64(seed ^ salt) as f64) / (u64::MAX as f64)
}

#[inline]
fn clamp01(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

#[inline]
fn mix64(mut state: u64) -> u64 {
    state ^= state >> 30;
    state = state.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    state ^= state >> 27;
    state = state.wrapping_mul(0x94d0_49bb_1331_11eb);
    state ^ (state >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::pos::types::GridXyz;

    fn key(coord: GridVec) -> DptChunkKey {
        DptChunkKey { scale: coord.scale, coord }
    }

    fn mean_abs_diff(a: &[f32], b: &[f32]) -> f32 {
        let len = a.len().min(b.len()).max(1);
        let sum = a.iter().zip(b.iter()).map(|(lhs, rhs)| (lhs - rhs).abs()).sum::<f32>();
        sum / len as f32
    }

    #[test]
    fn dpt_metric_vector_is_deterministic() {
        let coord = GridVec::new(GridVec::new_root(GridXyz::new_local(0, 0, 0)), GridXyz::new_local(1, -2, 3));
        let key = key(coord);

        let a = deterministic_metric_vector(&key, 6);
        let b = deterministic_metric_vector(&key, 6);
        assert_eq!(a, b);
    }

    #[test]
    fn dpt_metric_vector_values_stay_in_unit_interval() {
        let coord = GridVec::new(GridVec::new_root(GridXyz::new_local(2, -1, 1)), GridXyz::new_local(-3, 4, 0));
        let key = key(coord);
        let metrics = deterministic_metric_vector(&key, 8);

        for metric in metrics {
            assert!(metric.is_finite());
            assert!((0.0..=1.0).contains(&metric));
        }
    }

    #[test]
    fn dpt_neighboring_chunks_have_smoother_transition_than_far_chunks() {
        let base = GridVec::new(GridVec::new_root(GridXyz::new_local(0, 0, 0)), GridXyz::new_local(0, 0, 0));
        let neighbor = base.clone() + IVec3::new(1, 0, 0);
        let far = base.clone() + IVec3::new(4, 4, 4);

        let base_metrics = deterministic_metric_vector(&key(base), 6);
        let neighbor_metrics = deterministic_metric_vector(&key(neighbor), 6);
        let far_metrics = deterministic_metric_vector(&key(far), 6);

        let near_delta = mean_abs_diff(&base_metrics, &neighbor_metrics);
        let far_delta = mean_abs_diff(&base_metrics, &far_metrics);
        assert!(near_delta < far_delta);
    }

    #[test]
    fn dpt_parent_child_metrics_remain_coherent() {
        let parent = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let child = GridVec::new(parent.clone(), GridXyz::new_local(0, 0, 0));

        let parent_metrics = deterministic_metric_vector(&key(parent), 6);
        let child_metrics = deterministic_metric_vector(&key(child), 6);
        let delta = mean_abs_diff(&parent_metrics, &child_metrics);

        assert!(delta < 0.22);
    }
}

pub(crate) struct DptPlugin;
impl Plugin for DptPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DptStore>();
    }
}
