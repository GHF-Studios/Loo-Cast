use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::usf::content::{DPT_SAMPLER_KERNEL_DEFAULT_ID, UsfActiveModpack};
use crate::usf::definition::{DptMetricDefinition, DptSchema};
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
    pub const DEFAULT_DPT_SAMPLER_ID: &'static str = DPT_SAMPLER_KERNEL_DEFAULT_ID;

    pub fn get_chunk(&self, key: &DptChunkKey) -> Option<&DptChunkRecord> {
        self.chunks.get(key)
    }

    pub fn ensure_chunk(&mut self, key: DptChunkKey, schema: &DptSchema) -> &DptChunkRecord {
        self.chunks.entry(key.clone()).or_insert_with(|| DptChunkRecord {
            schema_revision: schema.revision,
            metrics: deterministic_metric_vector(&key, schema),
        })
    }

    pub fn ensure_chunk_for_scale(&mut self, key: DptChunkKey, schema: &DptSchema, active_modpack: &UsfActiveModpack) -> &DptChunkRecord {
        let sampler_id = active_modpack
            .scale_definition_for_scale(key.scale)
            .map(|scale_definition| scale_definition.dpt_sampler_id.as_str())
            .unwrap_or_else(|| {
                panic!(
                    "USF DPT sampling failed: missing scale definition for scale index {}",
                    key.scale.index_from_top()
                )
            });

        self.chunks.entry(key.clone()).or_insert_with(|| DptChunkRecord {
            schema_revision: schema.revision,
            metrics: metric_vector_for_sampler_id(sampler_id, &key, schema),
        })
    }
}

fn metric_vector_for_sampler_id(sampler_id: &str, key: &DptChunkKey, schema: &DptSchema) -> Vec<f32> {
    let _ = sampler_id;
    deterministic_metric_vector(key, schema)
}

pub(crate) fn deterministic_metric_vector(key: &DptChunkKey, schema: &DptSchema) -> Vec<f32> {
    let sample = normalized_chunk_center(&key.coord);
    let root_pos_x = normalized_root_axis(sample[0]);
    let root_pos_y = normalized_root_axis(sample[1]);
    let root_pos_z = normalized_root_axis(sample[2]);

    let elevation = clamp01(
        0.50 + 0.38 * coherent_wave(scale_point(sample, 0.42), 0x4f1b_bcdd_91a2_77c3) + 0.16 * coherent_wave(scale_point(sample, 1.37), 0x9e37_79b9_7f4a_7c15),
    );
    let temperature = clamp01(0.53 + 0.31 * coherent_wave(scale_point(sample, 0.23), 0x0f12_34ab_cd56_78ef) - 0.28 * (elevation - 0.5));
    let humidity =
        clamp01(0.50 + 0.34 * coherent_wave(offset_and_scale_point(sample, [13.0, -7.0, 3.0], 0.29), 0xbadc_0ffe_e0dd_f00d) - 0.26 * (temperature - 0.5));
    let vegetation_density = clamp01(0.66 * humidity + 0.18 * (1.0 - elevation) + 0.16 * (1.0 - (temperature - 0.55).abs() * 2.0));
    let matter_density = clamp01(0.52 * vegetation_density + 0.28 * (1.0 - elevation) + 0.20 * humidity);
    let matter_support = clamp01(0.45 * humidity + 0.35 * vegetation_density + 0.20 * (1.0 - temperature));

    let mut metrics = Vec::with_capacity(schema.metrics.len());
    for metric in &schema.metrics {
        let value = metric_value_for_definition(
            metric,
            sample,
            temperature,
            humidity,
            elevation,
            vegetation_density,
            matter_density,
            matter_support,
            root_pos_x,
            root_pos_y,
            root_pos_z,
        );
        metrics.push(value);
    }

    metrics
}

fn metric_value_for_definition(
    metric: &DptMetricDefinition,
    sample: [f64; 3],
    temperature: f32,
    humidity: f32,
    elevation: f32,
    vegetation_density: f32,
    matter_density: f32,
    matter_support: f32,
    root_pos_x: f32,
    root_pos_y: f32,
    root_pos_z: f32,
) -> f32 {
    let semantics = metric.semantics_tag.trim().to_ascii_lowercase();
    let metric_name = metric.name.trim().to_ascii_lowercase();
    let canonical = match semantics.as_str() {
        "climate.temperature.normalized" => Some(temperature),
        "climate.humidity.normalized" => Some(humidity),
        "terrain.elevation.normalized" => Some(elevation),
        "biosphere.vegetation_density.normalized" => Some(vegetation_density),
        "terrain.solid_fill.normalized" => Some(matter_density),
        "matter.density.normalized" => Some(matter_density),
        "matter.support.normalized" => Some(matter_support),
        "position.root.x.normalized" => Some(root_pos_x),
        "position.root.y.normalized" => Some(root_pos_y),
        "position.root.z.normalized" => Some(root_pos_z),
        _ => None,
    }
    .or_else(|| match metric_name.as_str() {
        "temperature" => Some(temperature),
        "humidity" => Some(humidity),
        "elevation" => Some(elevation),
        "vegetation_density" | "vegetation-density" => Some(vegetation_density),
        "solid_fill" | "solid-fill" | "demo_mass_density" | "demo-mass-density" => Some(matter_density),
        "density" => Some(matter_density),
        "support" => Some(matter_support),
        "root_pos_x" | "root-pos-x" | "position_x" | "position-x" => Some(root_pos_x),
        "root_pos_y" | "root-pos-y" | "position_y" | "position-y" => Some(root_pos_y),
        "root_pos_z" | "root-pos-z" | "position_z" | "position-z" => Some(root_pos_z),
        _ => None,
    });

    if let Some(value) = canonical {
        return value;
    }

    let semantic_hash = string_hash64(semantics.as_str());
    let name_hash = string_hash64(metric_name.as_str());
    let seed = mix64((metric.id.0 as u64).wrapping_mul(0x94d0_49bb_1331_11eb) ^ semantic_hash ^ name_hash);
    let span = 0.41 + (metric.id.0 as f64 * 0.07);
    clamp01(0.5 + 0.5 * coherent_wave(scale_point(sample, span), seed))
}

pub(crate) fn normalized_chunk_center(coord: &GridVec) -> [f64; 3] {
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
fn string_hash64(value: &str) -> u64 {
    let mut state = 0xcbf2_9ce4_8422_2325_u64;
    for byte in value.as_bytes() {
        state ^= *byte as u64;
        state = state.wrapping_mul(0x1000_0000_01b3);
    }
    state
}

#[inline]
fn clamp01(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

#[inline]
fn normalized_root_axis(value: f64) -> f32 {
    // Top-scale periodic axis is [-5..4], so wrap into [-5, 5) and normalize to [0, 1).
    let wrapped = ((value + 5.0).rem_euclid(10.0)) - 5.0;
    clamp01(((wrapped + 5.0) / 10.0) as f32)
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
    use crate::usf::content::{UsfActiveModpack, UsfConfiguredMod, UsfScaleDefinition};
    use crate::usf::definition::{DptMetricDefinition, DptMetricId, DptMetricStorageClass, DptMetricValueType, ZoneTypeId};
    use crate::usf::pos::types::GridXyz;
    use std::collections::HashMap;

    fn key(coord: GridVec) -> DptChunkKey {
        DptChunkKey { scale: coord.scale, coord }
    }

    fn mean_abs_diff(a: &[f32], b: &[f32]) -> f32 {
        let len = a.len().min(b.len()).max(1);
        let sum = a.iter().zip(b.iter()).map(|(lhs, rhs)| (lhs - rhs).abs()).sum::<f32>();
        sum / len as f32
    }

    fn test_schema(metric_count: usize) -> DptSchema {
        let max_scale_index = Scale::SCALE_LEVEL_COUNT.saturating_sub(1);
        let mut metrics = Vec::<DptMetricDefinition>::new();
        for idx in 0..metric_count {
            let (name, semantics_tag) = match idx {
                0 => ("temperature".to_string(), "climate.temperature.normalized".to_string()),
                1 => ("humidity".to_string(), "climate.humidity.normalized".to_string()),
                2 => ("elevation".to_string(), "terrain.elevation.normalized".to_string()),
                3 => ("vegetation_density".to_string(), "biosphere.vegetation_density.normalized".to_string()),
                _ => (format!("test_metric_{idx}"), format!("test.metric.{idx}.normalized")),
            };
            metrics.push(DptMetricDefinition {
                id: DptMetricId(idx as u16),
                name,
                value_type: DptMetricValueType::F32,
                semantics_tag,
                storage_class: DptMetricStorageClass::Brick,
                derived: idx >= 3,
                min_scale_index: 0,
                max_scale_index,
            });
        }

        DptSchema {
            revision: 1,
            metrics,
            fallback_zone: ZoneTypeId::new("void"),
        }
    }

    #[test]
    fn dpt_metric_vector_is_deterministic() {
        let coord = GridVec::new(GridVec::new_root(GridXyz::new_local(0, 0, 0)), GridXyz::new_local(1, -2, 3));
        let key = key(coord);
        let schema = test_schema(6);

        let a = deterministic_metric_vector(&key, &schema);
        let b = deterministic_metric_vector(&key, &schema);
        assert_eq!(a, b);
    }

    #[test]
    fn dpt_metric_vector_values_stay_in_unit_interval() {
        let coord = GridVec::new(GridVec::new_root(GridXyz::new_local(2, -1, 1)), GridXyz::new_local(-3, 4, 0));
        let key = key(coord);
        let metrics = deterministic_metric_vector(&key, &test_schema(8));

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
        let schema = test_schema(6);

        let base_metrics = deterministic_metric_vector(&key(base), &schema);
        let neighbor_metrics = deterministic_metric_vector(&key(neighbor), &schema);
        let far_metrics = deterministic_metric_vector(&key(far), &schema);

        let near_delta = mean_abs_diff(&base_metrics, &neighbor_metrics);
        let far_delta = mean_abs_diff(&base_metrics, &far_metrics);
        assert!(near_delta < far_delta);
    }

    #[test]
    fn dpt_parent_child_metrics_remain_coherent() {
        let parent = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let child = GridVec::new(parent.clone(), GridXyz::new_local(0, 0, 0));
        let schema = test_schema(6);

        let parent_metrics = deterministic_metric_vector(&key(parent), &schema);
        let child_metrics = deterministic_metric_vector(&key(child), &schema);
        let delta = mean_abs_diff(&parent_metrics, &child_metrics);

        assert!(delta < 0.22);
    }

    #[test]
    fn ensure_chunk_for_scale_populates_metrics() {
        let coord = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let key = key(coord.clone());
        let schema = DptSchema {
            revision: 1,
            metrics: vec![
                crate::usf::definition::DptMetricDefinition {
                    id: crate::usf::definition::DptMetricId(0),
                    name: "density".to_string(),
                    value_type: crate::usf::definition::DptMetricValueType::F32,
                    semantics_tag: "matter.density.normalized".to_string(),
                    storage_class: crate::usf::definition::DptMetricStorageClass::Brick,
                    derived: false,
                    min_scale_index: 0,
                    max_scale_index: Scale::SCALE_LEVEL_COUNT.saturating_sub(1),
                },
                crate::usf::definition::DptMetricDefinition {
                    id: crate::usf::definition::DptMetricId(1),
                    name: "support".to_string(),
                    value_type: crate::usf::definition::DptMetricValueType::F32,
                    semantics_tag: "matter.support.normalized".to_string(),
                    storage_class: crate::usf::definition::DptMetricStorageClass::Brick,
                    derived: true,
                    min_scale_index: 0,
                    max_scale_index: Scale::SCALE_LEVEL_COUNT.saturating_sub(1),
                },
            ],
            fallback_zone: crate::usf::definition::ZoneTypeId::new("void"),
        };
        let active_modpack = UsfActiveModpack {
            modpack_id: "debug".to_string(),
            configured_mods: vec![UsfConfiguredMod { mod_id: "demo".to_string() }],
            enabled_mods: std::collections::HashSet::from(["demo".to_string()]),
            resolved_enabled_mods: vec!["demo".to_string()],
            scales_by_index: HashMap::from([(
                coord.scale,
                UsfScaleDefinition {
                    dpt_sampler_id: DptStore::DEFAULT_DPT_SAMPLER_ID.to_string(),
                    dpt_categorizer_id: crate::usf::content::DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID.to_string(),
                    chunk_store_key: "chunk_store.default".to_string(),
                },
            )]),
            known_dpt_samplers: std::collections::HashSet::from([DptStore::DEFAULT_DPT_SAMPLER_ID.to_string()]),
            known_dpt_categorizers: std::collections::HashSet::from([crate::usf::content::DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID.to_string()]),
            schemas_by_scale: HashMap::from([(coord.scale, schema.clone())]),
            known_zone_types: std::collections::HashSet::from([ZoneTypeId::new("void")]),
        };

        let mut store = DptStore::default();
        let stored = store.ensure_chunk_for_scale(key.clone(), &schema, &active_modpack);
        let expected = deterministic_metric_vector(&key, &schema);
        assert_eq!(stored.metrics, expected);
    }
}

pub(crate) struct DptPlugin;
impl Plugin for DptPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DptStore>();
    }
}
