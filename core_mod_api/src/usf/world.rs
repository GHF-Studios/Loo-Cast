use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::usf::content::UsfActiveContentProfile;
use crate::usf::definition::{DptSchema, ZoneTypeId};
use crate::usf::dpt::{DptChunkKey, deterministic_metric_vector, normalized_chunk_center};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use crate::usf::zlm::ZlmRegistry;

#[derive(Debug, Clone, PartialEq)]
pub struct UsfChunkSample {
    pub schema_revision: u64,
    pub schema_metric_count: u32,
    pub zlm_revision: u64,
    pub metrics: Vec<f32>,
    pub zone_type: ZoneTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MetricCombineOp {
    #[default]
    Sum,
    Min,
    Max,
}
impl MetricCombineOp {
    #[inline]
    fn combine(self, current: f32, contribution: f32) -> f32 {
        match self {
            MetricCombineOp::Sum => current + contribution,
            MetricCombineOp::Min => current.min(contribution),
            MetricCombineOp::Max => current.max(contribution),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetricPatchPrimitive {
    Sphere { center: [f64; 3], radius: f64, amplitude: f32 },
    Aabb { min: [f64; 3], max: [f64; 3], amplitude: f32 },
}
impl MetricPatchPrimitive {
    #[inline]
    fn sample(&self, point: [f64; 3]) -> Option<f32> {
        match self {
            MetricPatchPrimitive::Sphere { center, radius, amplitude } => {
                if !radius.is_finite() || *radius <= f64::EPSILON {
                    return None;
                }
                let dx = point[0] - center[0];
                let dy = point[1] - center[1];
                let dz = point[2] - center[2];
                let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                if distance > *radius {
                    return None;
                }
                let falloff = (1.0 - (distance / *radius)).clamp(0.0, 1.0) as f32;
                Some(*amplitude * falloff)
            }
            MetricPatchPrimitive::Aabb { min, max, amplitude } => {
                if point[0] < min[0] || point[0] > max[0] || point[1] < min[1] || point[1] > max[1] || point[2] < min[2] || point[2] > max[2] {
                    return None;
                }
                Some(*amplitude)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricPatch {
    pub patch_id: String,
    pub primitive: MetricPatchPrimitive,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UsfMetricProgram {
    pub combine_op: MetricCombineOp,
    pub patches_by_id: HashMap<String, MetricPatch>,
}
impl Default for UsfMetricProgram {
    fn default() -> Self {
        Self {
            combine_op: MetricCombineOp::Sum,
            patches_by_id: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UsfWorldScale {
    pub scale: Scale,
    pub schema_revision: u64,
    pub zlm_revision: u64,
    pub metric_programs_by_name: HashMap<String, UsfMetricProgram>,
    pub sampled_chunks: HashMap<GridVec, UsfChunkSample>,
}
impl UsfWorldScale {
    fn new(scale: Scale) -> Self {
        Self {
            scale,
            schema_revision: 0,
            zlm_revision: 0,
            metric_programs_by_name: HashMap::new(),
            sampled_chunks: HashMap::new(),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct UsfWorld {
    pub scales: HashMap<Scale, UsfWorldScale>,
}
impl Default for UsfWorld {
    fn default() -> Self {
        let mut scales = HashMap::<Scale, UsfWorldScale>::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            scales.insert(scale, UsfWorldScale::new(scale));
        }
        if scales.len() != Scale::SCALE_LEVEL_COUNT as usize {
            panic!(
                "USF world bootstrap failed: expected {} scale buckets, got {}",
                Scale::SCALE_LEVEL_COUNT,
                scales.len()
            );
        }
        Self { scales }
    }
}
impl UsfWorld {
    pub fn clear_transient_samples(&mut self) {
        for scale in self.scales.values_mut() {
            scale.sampled_chunks.clear();
        }
    }

    pub fn sample_chunk_with_scale_binding(
        &mut self,
        coord: &GridVec,
        active_content_profile: &UsfActiveContentProfile,
        zlm_registry: &ZlmRegistry,
    ) -> Option<UsfChunkSample> {
        let canonical_coord = canonical_grid_coord(coord);
        let scale = canonical_coord.scale;
        let schema = active_content_profile.schema_for_scale(scale)?;
        let zlm_revision = zlm_registry.maps_by_scale.get(&scale).map(|definition| definition.revision).unwrap_or_default();

        let scale_state = self.scales.entry(scale).or_insert_with(|| UsfWorldScale::new(scale));
        if let Some(existing) = scale_state.sampled_chunks.get(&canonical_coord) {
            if existing.schema_revision == schema.revision
                && existing.schema_metric_count == schema.metrics.len() as u32
                && existing.zlm_revision == zlm_revision
            {
                return Some(existing.clone());
            }
        }

        let mut metrics = deterministic_metric_vector(
            &DptChunkKey {
                scale,
                coord: canonical_coord.clone(),
            },
            schema,
        );
        apply_metric_programs(scale_state, schema, &canonical_coord, &mut metrics);
        let zone_type = zlm_registry.classify_with_scale_binding(scale, schema, &metrics, active_content_profile);

        let sample = UsfChunkSample {
            schema_revision: schema.revision,
            schema_metric_count: schema.metrics.len() as u32,
            zlm_revision,
            metrics,
            zone_type,
        };
        scale_state.schema_revision = sample.schema_revision;
        scale_state.zlm_revision = sample.zlm_revision;
        scale_state.sampled_chunks.insert(canonical_coord, sample.clone());
        Some(sample)
    }

    pub fn set_metric_combine_op(&mut self, scale: Scale, metric_name: &str, combine_op: MetricCombineOp) {
        let metric_name = normalize_metric_name(metric_name);
        let scale_state = self.scales.entry(scale).or_insert_with(|| UsfWorldScale::new(scale));
        scale_state.metric_programs_by_name.entry(metric_name).or_default().combine_op = combine_op;
        scale_state.sampled_chunks.clear();
    }

    pub fn upsert_metric_patch(&mut self, scale: Scale, metric_name: &str, patch: MetricPatch) {
        let metric_name = normalize_metric_name(metric_name);
        let scale_state = self.scales.entry(scale).or_insert_with(|| UsfWorldScale::new(scale));
        let program = scale_state.metric_programs_by_name.entry(metric_name).or_default();
        let patch_id = patch.patch_id.clone();
        program.patches_by_id.insert(patch_id, patch);
        scale_state.sampled_chunks.clear();
    }

    pub fn remove_metric_patch(&mut self, scale: Scale, metric_name: &str, patch_id: &str) -> bool {
        let metric_name = normalize_metric_name(metric_name);
        let patch_id = patch_id.trim().to_ascii_lowercase();
        let Some(scale_state) = self.scales.get_mut(&scale) else {
            return false;
        };
        let Some(program) = scale_state.metric_programs_by_name.get_mut(metric_name.as_str()) else {
            return false;
        };
        let removed = program.patches_by_id.remove(patch_id.as_str()).is_some();
        if removed {
            scale_state.sampled_chunks.clear();
        }
        removed
    }
}

fn apply_metric_programs(scale_state: &UsfWorldScale, schema: &DptSchema, coord: &GridVec, metric_values: &mut [f32]) {
    let sample_point = normalized_chunk_center(coord);
    for (index, metric_definition) in schema.metrics.iter().enumerate() {
        let Some(current_value) = metric_values.get_mut(index) else {
            continue;
        };
        let metric_name = normalize_metric_name(metric_definition.name.as_str());
        let Some(program) = scale_state.metric_programs_by_name.get(metric_name.as_str()) else {
            continue;
        };
        let mut value = *current_value;
        for patch in program.patches_by_id.values() {
            let Some(contribution) = patch.primitive.sample(sample_point) else {
                continue;
            };
            value = program.combine_op.combine(value, contribution);
        }
        *current_value = value.clamp(0.0, 1.0);
    }
}

#[inline]
fn normalize_metric_name(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

#[inline]
fn canonical_grid_coord(coord: &GridVec) -> GridVec {
    let mut normalized = coord.clone();
    normalized.normalize();
    normalized
}

fn validate_usf_world_scales_system(world: Res<UsfWorld>) {
    if world.scales.len() != Scale::SCALE_LEVEL_COUNT as usize {
        panic!(
            "USF world validation failed: expected {} scale buckets, got {}",
            Scale::SCALE_LEVEL_COUNT,
            world.scales.len()
        );
    }
}

pub(crate) struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfWorld>().add_systems(Startup, validate_usf_world_scales_system);
    }
}
