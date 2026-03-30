use crate::bevy::prelude::*;
#[cfg(not(test))]
use crate::chunk::gpu_density;
use crate::usf::definition::ZoneTypeId;
use crate::usf::phenomenon::MetricSurfaceDebugFieldDefinition;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::Scale;
use crate::usf::zone::ZoneDensityProfile;
use serde::{Deserialize, Serialize};
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

pub(crate) const CHUNK_SPAN_UNITS_I64: i64 = 1_000;
pub(crate) const HALF_CHUNK_SPAN_F32: f32 = 500.0;
pub(crate) const ROOT_AXIS_CELL_COUNT: i64 = 10;
pub(crate) const ROOT_AXIS_PERIOD_UNITS: i64 = CHUNK_SPAN_UNITS_I64 * ROOT_AXIS_CELL_COUNT;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializableGridCoord {
    pub scale_index: u8,
    pub digits: Vec<[i8; 3]>,
}
impl SerializableGridCoord {
    pub(crate) fn from_grid(coord: &GridVec) -> Self {
        let mut canonical = coord.clone();
        canonical.normalize();
        let digits = canonical
            .to_raw_vec_3d()
            .into_iter()
            .map(|xyz| [xyz.x as i8, xyz.y as i8, xyz.z as i8])
            .collect::<Vec<_>>();
        Self {
            scale_index: canonical.scale.index_from_top(),
            digits,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MixedMetricFieldU8 {
    Uniform(u8),
    Brick(Vec<u8>),
}
impl MixedMetricFieldU8 {
    pub(crate) fn from_values(values: Vec<u8>) -> Self {
        if let Some(first) = values.first().copied() {
            if values.iter().all(|value| *value == first) {
                return Self::Uniform(first);
            }
        }
        Self::Brick(values)
    }

    pub(crate) fn expand(&self, expected_len: usize) -> Option<Vec<u8>> {
        match self {
            MixedMetricFieldU8::Uniform(value) => Some(vec![*value; expected_len]),
            MixedMetricFieldU8::Brick(values) => {
                if values.len() == expected_len {
                    Some(values.clone())
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MixedMetricFieldU32 {
    Uniform(u32),
    Brick(Vec<u32>),
}
impl MixedMetricFieldU32 {
    pub(crate) fn from_values(values: Vec<u32>) -> Self {
        if let Some(first) = values.first().copied() {
            if values.iter().all(|value| *value == first) {
                return Self::Uniform(first);
            }
        }
        Self::Brick(values)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PersistedChunkRecord {
    pub world_seed: u64,
    pub active_scale_index: u8,
    pub chunk_coord: SerializableGridCoord,
    #[serde(default)]
    pub zone_type: String,
    #[serde(default)]
    pub zone_density_signature: u64,
    #[serde(default)]
    pub density_field_signature: u64,
    #[serde(default)]
    pub phenomenon_script_id: String,
    pub chunk_seed: u64,
    pub sample_step: u16,
    pub iso_level: u8,
    pub axis_samples: Vec<u16>,
    pub rho_field: MixedMetricFieldU8,
    pub zone_field: MixedMetricFieldU32,
}

pub(crate) fn generate_chunk_record(
    world_seed: u64,
    sample_step: u16,
    iso_level: u8,
    chunk_scale: Scale,
    canonical_coord: &GridVec,
    zone_type: &ZoneTypeId,
    zone_density_profile: ZoneDensityProfile,
    zone_density_signature: u64,
    phenomenon_script_id: &str,
    metric_surface_debug_field: MetricSurfaceDebugFieldDefinition,
) -> PersistedChunkRecord {
    let axis_samples = build_axis_samples(sample_step);
    let axis_points = axis_samples.len();
    let total_points = axis_points * axis_points * axis_points;

    let chunk_seed = derive_chunk_seed(world_seed, canonical_coord);

    let rho_values = sample_density_field_values(
        world_seed,
        chunk_scale,
        canonical_coord,
        &axis_samples,
        zone_density_profile,
        metric_surface_debug_field,
    );
    let zone_id = zone_numeric_id(zone_type);
    let zone_values = vec![zone_id; total_points];

    PersistedChunkRecord {
        world_seed,
        active_scale_index: chunk_scale.index_from_top(),
        chunk_coord: SerializableGridCoord::from_grid(canonical_coord),
        zone_type: zone_type.0.clone(),
        zone_density_signature,
        density_field_signature: density_field_signature(metric_surface_debug_field),
        phenomenon_script_id: phenomenon_script_id.to_ascii_lowercase(),
        chunk_seed,
        sample_step,
        iso_level,
        axis_samples,
        rho_field: MixedMetricFieldU8::from_values(rho_values),
        zone_field: MixedMetricFieldU32::from_values(zone_values),
    }
}

pub(crate) fn sample_density_field_values(
    world_seed: u64,
    chunk_scale: Scale,
    canonical_coord: &GridVec,
    axis_samples: &[u16],
    zone_density_profile: ZoneDensityProfile,
    metric_surface_debug_field: MetricSurfaceDebugFieldDefinition,
) -> Vec<u8> {
    #[cfg(test)]
    {
        let _ = chunk_scale;
        sample_density_field_values_cpu(world_seed, canonical_coord, axis_samples, zone_density_profile, metric_surface_debug_field)
    }

    #[cfg(not(test))]
    {
        match gpu_density::sample_density_field(world_seed, chunk_scale, canonical_coord, axis_samples) {
            Ok(values) => values,
            Err(error) => panic!(
                "USF mod runtime GPU density sampling failed (scale_index={}, coord={:?}): {}",
                chunk_scale.index_from_top(),
                canonical_coord,
                error
            ),
        }
    }
}

#[cfg(test)]
fn sample_density_field_values_cpu(
    world_seed: u64,
    canonical_coord: &GridVec,
    axis_samples: &[u16],
    zone_density_profile: ZoneDensityProfile,
    metric_surface_debug_field: MetricSurfaceDebugFieldDefinition,
) -> Vec<u8> {
    let axis_points = axis_samples.len();
    let total_points = axis_points * axis_points * axis_points;
    let mut rho_values = Vec::with_capacity(total_points);

    for iz in 0..axis_points {
        for iy in 0..axis_points {
            for ix in 0..axis_points {
                let local_offset = Vec3::new(
                    axis_samples[ix] as f32 - HALF_CHUNK_SPAN_F32,
                    axis_samples[iy] as f32 - HALF_CHUNK_SPAN_F32,
                    axis_samples[iz] as f32 - HALF_CHUNK_SPAN_F32,
                );
                let root_native = sample_root_native_position(canonical_coord, local_offset);
                rho_values.push(hash_density_u8(world_seed, root_native, zone_density_profile, metric_surface_debug_field));
            }
        }
    }

    rho_values
}

pub(crate) fn canonical_grid_coord(coord: &GridVec) -> GridVec {
    let mut canonical = coord.clone();
    canonical.normalize();
    canonical
}

pub(crate) fn chunk_file_path(persistence_dir: &str, world_seed: u64, chunk_scale: Scale, canonical_coord: &GridVec, chunk_store_key: &str) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    canonical_coord.hash(&mut hasher);
    let coord_hash = hasher.finish();
    let normalized_store_key = chunk_store_key.trim().to_ascii_lowercase();
    let sanitized_store_key = if normalized_store_key.is_empty() {
        "chunk_store.default".to_string()
    } else {
        normalized_store_key
            .chars()
            .map(|char| {
                if char.is_ascii_alphanumeric() || char == '_' || char == '-' || char == '.' {
                    char
                } else {
                    '_'
                }
            })
            .collect::<String>()
    };
    Path::new(persistence_dir).join(sanitized_store_key).join(format!(
        "ws_{:016x}_as_{:02}_coord_{:016x}.json",
        world_seed,
        chunk_scale.index_from_top(),
        coord_hash
    ))
}

pub(crate) fn load_chunk_record(path: &Path) -> Option<PersistedChunkRecord> {
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice::<PersistedChunkRecord>(&bytes).ok()
}

pub(crate) fn save_chunk_record(path: &Path, record: &PersistedChunkRecord) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("create_dir_all failed: {error}"))?;
    }
    let encoded = serde_json::to_vec_pretty(record).map_err(|error| format!("serialize failed: {error}"))?;
    fs::write(path, encoded).map_err(|error| format!("write failed: {error}"))?;
    Ok(())
}

pub(crate) fn color_from_seed(seed: u64) -> Color {
    let r = ((seed & 0xff) as f32) / 255.0;
    let g = (((seed >> 8) & 0xff) as f32) / 255.0;
    let b = (((seed >> 16) & 0xff) as f32) / 255.0;
    Color::srgb(0.2 + 0.6 * r, 0.2 + 0.6 * g, 0.2 + 0.6 * b)
}

pub(crate) fn density_field_signature(metric_surface_debug_field: MetricSurfaceDebugFieldDefinition) -> u64 {
    const DENSITY_ALGO_REVISION: u64 = 4;
    let mut signature_seed = 0xa3f1_1a89_5d4c_2be7_u64 ^ DENSITY_ALGO_REVISION;
    signature_seed ^= CHUNK_SPAN_UNITS_I64 as u64;
    signature_seed ^= (ROOT_AXIS_CELL_COUNT as u64) << 8;
    signature_seed ^= (ROOT_AXIS_PERIOD_UNITS as u64) << 16;
    signature_seed ^= 0x4750_555f_4445_4d4f_u64; // "GPU_DEMO"
    signature_seed ^= metric_surface_debug_field.coarse_span_units.to_bits();
    signature_seed ^= metric_surface_debug_field.detail_span_units.to_bits();
    signature_seed ^= (metric_surface_debug_field.coarse_weight.to_bits() as u64) << 1;
    signature_seed ^= (metric_surface_debug_field.detail_weight.to_bits() as u64) << 2;
    signature_seed ^= (metric_surface_debug_field.bias.to_bits() as u64) << 3;
    signature_seed ^= (metric_surface_debug_field.gain.to_bits() as u64) << 4;
    signature_seed ^= (metric_surface_debug_field.center.to_bits() as u64) << 5;
    signature_seed ^= metric_surface_debug_field.seed_salt_primary;
    signature_seed ^= metric_surface_debug_field.seed_salt_detail;
    mix64(signature_seed)
}

pub(crate) fn sample_root_native_position(canonical_coord: &GridVec, local_offset: Vec3) -> (f64, f64, f64) {
    let mut sample = UnitVec::new(canonical_coord.clone(), local_offset);
    while sample.grid_offset.scale != Scale::MAX {
        sample.zoom_out();
    }
    let root = sample.grid_offset.xyz;
    (
        root.x as f64 * CHUNK_SPAN_UNITS_I64 as f64 + sample.unit_offset.x as f64,
        root.y as f64 * CHUNK_SPAN_UNITS_I64 as f64 + sample.unit_offset.y as f64,
        root.z as f64 * CHUNK_SPAN_UNITS_I64 as f64 + sample.unit_offset.z as f64,
    )
}

pub(crate) fn wrap_root_native_position((x, y, z): (f64, f64, f64)) -> (f64, f64, f64) {
    (wrap_root_native_axis(x), wrap_root_native_axis(y), wrap_root_native_axis(z))
}

fn build_axis_samples(step: u16) -> Vec<u16> {
    let step = step.clamp(1, 1_000);
    let mut samples = Vec::new();
    let mut cursor = 0_u16;
    while cursor < 1_000 {
        samples.push(cursor);
        let next = cursor.saturating_add(step);
        if next == cursor {
            break;
        }
        cursor = next.min(1_000);
    }
    if samples.last().copied() != Some(1_000) {
        samples.push(1_000);
    }
    samples
}

fn derive_chunk_seed(world_seed: u64, canonical_coord: &GridVec) -> u64 {
    let mut hasher = DefaultHasher::new();
    world_seed.hash(&mut hasher);
    canonical_coord.hash(&mut hasher);
    let raw = hasher.finish();
    if raw == 0 { 0x9e37_79b9_7f4a_7c15 } else { raw }
}

fn hash_density_u8(
    world_seed: u64,
    root_native: (f64, f64, f64),
    _zone_density_profile: ZoneDensityProfile,
    metric_surface_debug_field: MetricSurfaceDebugFieldDefinition,
) -> u8 {
    let (wx, wy, wz) = wrap_root_native_position(root_native);

    let seed = mix64(world_seed ^ metric_surface_debug_field.seed_salt_primary);
    let base = value_noise_3d(seed, wx, wy, wz, metric_surface_debug_field.coarse_span_units);
    let detail = value_noise_3d(
        seed ^ metric_surface_debug_field.seed_salt_detail,
        wx,
        wy,
        wz,
        metric_surface_debug_field.detail_span_units,
    );
    let weight_sum = (metric_surface_debug_field.coarse_weight + metric_surface_debug_field.detail_weight).max(f32::MIN_POSITIVE);
    let combined = ((base * metric_surface_debug_field.coarse_weight) + (detail * metric_surface_debug_field.detail_weight)) / weight_sum;
    let shaped = ((combined - metric_surface_debug_field.bias) * metric_surface_debug_field.gain + metric_surface_debug_field.center).clamp(0.0, 1.0);

    (shaped * 255.0).round() as u8
}

fn zone_numeric_id(zone_type: &ZoneTypeId) -> u32 {
    let mut state = 0x13d7_4b29_11f2_7a67_u64;
    for byte in zone_type.0.as_bytes() {
        state = mix64(state ^ (*byte as u64));
    }
    (state & 0x0000_ffff) as u32
}

#[inline]
fn fold_signed(value: i64) -> u64 {
    value as u64
}

#[inline]
fn mix64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[inline]
fn wrap_root_native_axis(value: f64) -> f64 {
    let period = ROOT_AXIS_PERIOD_UNITS as f64;
    if !value.is_finite() || period <= 0.0 {
        return 0.0;
    }
    value.rem_euclid(period)
}

fn value_noise_3d(seed: u64, gx: f64, gy: f64, gz: f64, cell_size: f64) -> f32 {
    let cell_size = cell_size.max(f64::EPSILON);
    let sx = gx / cell_size;
    let sy = gy / cell_size;
    let sz = gz / cell_size;

    let cx0 = sx.floor() as i64;
    let cy0 = sy.floor() as i64;
    let cz0 = sz.floor() as i64;
    let cx1 = cx0 + 1;
    let cy1 = cy0 + 1;
    let cz1 = cz0 + 1;

    let tx = smoothstep01((sx - cx0 as f64) as f32);
    let ty = smoothstep01((sy - cy0 as f64) as f32);
    let tz = smoothstep01((sz - cz0 as f64) as f32);

    let c000 = lattice_noise01(seed, cx0, cy0, cz0);
    let c100 = lattice_noise01(seed, cx1, cy0, cz0);
    let c010 = lattice_noise01(seed, cx0, cy1, cz0);
    let c110 = lattice_noise01(seed, cx1, cy1, cz0);
    let c001 = lattice_noise01(seed, cx0, cy0, cz1);
    let c101 = lattice_noise01(seed, cx1, cy0, cz1);
    let c011 = lattice_noise01(seed, cx0, cy1, cz1);
    let c111 = lattice_noise01(seed, cx1, cy1, cz1);

    let x00 = lerp(c000, c100, tx);
    let x10 = lerp(c010, c110, tx);
    let x01 = lerp(c001, c101, tx);
    let x11 = lerp(c011, c111, tx);
    let y0 = lerp(x00, x10, ty);
    let y1 = lerp(x01, x11, ty);
    lerp(y0, y1, tz)
}

#[inline]
fn lattice_noise01(seed: u64, x: i64, y: i64, z: i64) -> f32 {
    let mut state = mix64(seed ^ 0x5f35_d3a1_c9b4_e227_u64);
    state = mix64(state ^ fold_signed(x));
    state = mix64(state ^ fold_signed(y));
    state = mix64(state ^ fold_signed(z));
    ((state >> 40) as u32) as f32 / ((1_u32 << 24) - 1) as f32
}

#[inline]
fn smoothstep01(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::pos::types::GridXyz;

    #[derive(Clone)]
    struct TestSettings {
        world_seed: u64,
        sample_step: u16,
        iso_level: u8,
        persistence_dir: String,
    }

    fn test_settings() -> TestSettings {
        TestSettings {
            world_seed: 42,
            sample_step: 64,
            iso_level: 128,
            persistence_dir: std::env::temp_dir().join("usf_mod_runtime_surface_tests").to_string_lossy().to_string(),
        }
    }

    fn rho_values(record: &PersistedChunkRecord) -> Vec<u8> {
        let axis_points = record.axis_samples.len();
        let total = axis_points * axis_points * axis_points;
        record.rho_field.expand(total).expect("rho field should expand")
    }

    fn test_zone() -> (ZoneTypeId, ZoneDensityProfile) {
        (
            ZoneTypeId::new("forest"),
            ZoneDensityProfile {
                density_multiplier: 0.72,
                density_offset: 0.14,
                density_floor: 0.05,
                density_ceil: 0.88,
            },
        )
    }

    fn test_phenomenon_id() -> &'static str {
        "phenomenon.test.surface_metric"
    }

    fn test_metric_surface_debug_field() -> MetricSurfaceDebugFieldDefinition {
        MetricSurfaceDebugFieldDefinition::default()
    }

    fn grid_index(ix: usize, iy: usize, iz: usize, axis_points: usize) -> usize {
        ix + axis_points * (iy + axis_points * iz)
    }

    #[test]
    fn rho_sampling_matches_across_adjacent_chunk_borders() {
        let settings = test_settings();
        let left = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let right = left.clone() + IVec3::new(1, 0, 0);
        let (zone_type, zone_density_profile) = test_zone();
        let zone_density_signature = zone_density_profile.signature();

        let left_record = generate_chunk_record(
            settings.world_seed,
            settings.sample_step,
            settings.iso_level,
            Scale::MAX,
            &left,
            &zone_type,
            zone_density_profile,
            zone_density_signature,
            test_phenomenon_id(),
            test_metric_surface_debug_field(),
        );
        let right_record = generate_chunk_record(
            settings.world_seed,
            settings.sample_step,
            settings.iso_level,
            Scale::MAX,
            &right,
            &zone_type,
            zone_density_profile,
            zone_density_signature,
            test_phenomenon_id(),
            test_metric_surface_debug_field(),
        );

        let axis_points = left_record.axis_samples.len();
        assert_eq!(axis_points, right_record.axis_samples.len());

        let left_values = rho_values(&left_record);
        let right_values = rho_values(&right_record);
        let lx = axis_points - 1;
        let rx = 0;

        for iz in 0..axis_points {
            for iy in 0..axis_points {
                let left_idx = grid_index(lx, iy, iz, axis_points);
                let right_idx = grid_index(rx, iy, iz, axis_points);
                assert_eq!(left_values[left_idx], right_values[right_idx], "border seam mismatch at (y={}, z={})", iy, iz);
            }
        }
    }

    #[test]
    fn rho_sampling_loops_across_top_level_wrap_boundary() {
        let settings = test_settings();
        let left = GridVec::new_root(GridXyz::new_local(4, 0, 0));
        let right = GridVec::new_root(GridXyz::new_local(-5, 0, 0));
        let (zone_type, zone_density_profile) = test_zone();
        let zone_density_signature = zone_density_profile.signature();

        let left_record = generate_chunk_record(
            settings.world_seed,
            settings.sample_step,
            settings.iso_level,
            Scale::MAX,
            &left,
            &zone_type,
            zone_density_profile,
            zone_density_signature,
            test_phenomenon_id(),
            test_metric_surface_debug_field(),
        );
        let right_record = generate_chunk_record(
            settings.world_seed,
            settings.sample_step,
            settings.iso_level,
            Scale::MAX,
            &right,
            &zone_type,
            zone_density_profile,
            zone_density_signature,
            test_phenomenon_id(),
            test_metric_surface_debug_field(),
        );

        let axis_points = left_record.axis_samples.len();
        assert_eq!(axis_points, right_record.axis_samples.len());

        let left_values = rho_values(&left_record);
        let right_values = rho_values(&right_record);
        let lx = axis_points - 1;
        let rx = 0;

        for iz in 0..axis_points {
            for iy in 0..axis_points {
                let left_idx = grid_index(lx, iy, iz, axis_points);
                let right_idx = grid_index(rx, iy, iz, axis_points);
                assert_eq!(
                    left_values[left_idx], right_values[right_idx],
                    "top-level wrap seam mismatch at (y={}, z={})",
                    iy, iz
                );
            }
        }
    }

    #[test]
    fn persistence_roundtrip_is_stable() {
        let settings = test_settings();
        let coord = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let (zone_type, zone_density_profile) = test_zone();
        let record = generate_chunk_record(
            settings.world_seed,
            settings.sample_step,
            settings.iso_level,
            Scale::MAX,
            &coord,
            &zone_type,
            zone_density_profile,
            zone_density_profile.signature(),
            test_phenomenon_id(),
            test_metric_surface_debug_field(),
        );

        let path = Path::new(&settings.persistence_dir).join("roundtrip_chunk.json");
        save_chunk_record(&path, &record).expect("save should succeed");
        let loaded = load_chunk_record(&path).expect("load should succeed");
        assert_eq!(record, loaded);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn density_sampling_is_scale_invariant_for_shared_root_space_points() {
        let settings = test_settings();
        let parent = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let child = GridVec::new(parent.clone(), GridXyz::new_local(3, 4, 4));
        let (_zone_type, zone_density_profile) = test_zone();
        let child_digit = child.xyz;

        for child_local in [-500_i32, -300, 0, 300, 500] {
            let child_offset = Vec3::splat(child_local as f32);
            let parent_offset = Vec3::new(
                child_digit.x as f32 * 100.0 + child_offset.x / 10.0,
                child_digit.y as f32 * 100.0 + child_offset.y / 10.0,
                child_digit.z as f32 * 100.0 + child_offset.z / 10.0,
            );

            let child_root_native = sample_root_native_position(&child, child_offset);
            let parent_root_native = sample_root_native_position(&parent, parent_offset);
            let wrapped_child = wrap_root_native_position(child_root_native);
            let wrapped_parent = wrap_root_native_position(parent_root_native);
            let abs_diff = (
                (wrapped_child.0 - wrapped_parent.0).abs(),
                (wrapped_child.1 - wrapped_parent.1).abs(),
                (wrapped_child.2 - wrapped_parent.2).abs(),
            );

            assert!(
                abs_diff.0 <= 1e-6 && abs_diff.1 <= 1e-6 && abs_diff.2 <= 1e-6,
                "root-native mismatch for child_local={child_local}: child={wrapped_child:?}, parent={wrapped_parent:?}"
            );

            let child_density = hash_density_u8(settings.world_seed, child_root_native, zone_density_profile, test_metric_surface_debug_field());
            let parent_density = hash_density_u8(settings.world_seed, parent_root_native, zone_density_profile, test_metric_surface_debug_field());
            assert_eq!(
                child_density, parent_density,
                "density mismatch for child_local={child_local}: child={child_density}, parent={parent_density}"
            );
        }
    }

    #[test]
    fn density_sampling_is_scale_invariant_for_fractional_shared_points() {
        let settings = test_settings();
        let parent = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let child = GridVec::new(parent.clone(), GridXyz::new_local(-2, 1, 3));
        let (_zone_type, zone_density_profile) = test_zone();
        let child_digit = child.xyz;

        let child_offsets = [
            Vec3::new(-437.25, 118.125, -42.5),
            Vec3::new(-221.75, -339.875, 271.625),
            Vec3::new(19.375, -75.625, 413.5),
            Vec3::new(348.5, 244.25, -196.125),
        ];

        for child_offset in child_offsets {
            let parent_offset = Vec3::new(
                child_digit.x as f32 * 100.0 + child_offset.x / 10.0,
                child_digit.y as f32 * 100.0 + child_offset.y / 10.0,
                child_digit.z as f32 * 100.0 + child_offset.z / 10.0,
            );

            let child_root_native = sample_root_native_position(&child, child_offset);
            let parent_root_native = sample_root_native_position(&parent, parent_offset);
            let wrapped_child = wrap_root_native_position(child_root_native);
            let wrapped_parent = wrap_root_native_position(parent_root_native);
            let abs_diff = (
                (wrapped_child.0 - wrapped_parent.0).abs(),
                (wrapped_child.1 - wrapped_parent.1).abs(),
                (wrapped_child.2 - wrapped_parent.2).abs(),
            );

            assert!(
                abs_diff.0 <= 1e-3 && abs_diff.1 <= 1e-3 && abs_diff.2 <= 1e-3,
                "root-native mismatch for child_offset={child_offset:?}: child={wrapped_child:?}, parent={wrapped_parent:?}"
            );

            let child_density = hash_density_u8(settings.world_seed, child_root_native, zone_density_profile, test_metric_surface_debug_field());
            let parent_density = hash_density_u8(settings.world_seed, parent_root_native, zone_density_profile, test_metric_surface_debug_field());
            assert_eq!(
                child_density, parent_density,
                "fractional density mismatch for child_offset={child_offset:?}: child={child_density}, parent={parent_density}"
            );
        }
    }
}
