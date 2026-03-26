use crate::bevy::asset::RenderAssetUsages;
use crate::bevy::prelude::*;
use crate::bevy::render::render_resource::PrimitiveTopology;
use crate::chunk::components::{Chunk, ChunkLoader};
use crate::chunk::resources::ChunkManager;
use crate::config::statics::CONFIG;
use crate::player::components::Player;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::Scale;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

const CHUNK_SPAN_UNITS_I64: i64 = 1_000;
const HALF_CHUNK_SPAN_F32: f32 = 500.0;

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfDemoSettings {
    pub enabled: bool,
    pub world_seed: u64,
    pub sample_step: u16,
    pub iso_level: u8,
    pub persistence_dir: String,
}
impl Default for UsfDemoSettings {
    fn default() -> Self {
        Self {
            enabled: CONFIG().get::<bool>("usf_demo/enabled"),
            world_seed: CONFIG().get::<u64>("usf_demo/world_seed"),
            sample_step: CONFIG().get::<u16>("usf_demo/sample_step"),
            iso_level: CONFIG().get::<u8>("usf_demo/iso_level"),
            persistence_dir: CONFIG().get::<String>("usf_demo/persistence_dir"),
        }
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct UsfDemoChunkVisual {
    pub chunk_seed: u64,
    pub sample_step: u16,
}

#[derive(Resource, Debug, Default)]
pub struct UsfDemoChunkStore {
    pub records: HashMap<GridVec, PersistedChunkRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializableGridCoord {
    pub scale_index: u8,
    pub digits: Vec<[i8; 3]>,
}
impl SerializableGridCoord {
    fn from_grid(coord: &GridVec) -> Self {
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
    fn from_values(values: Vec<u8>) -> Self {
        if let Some(first) = values.first().copied() {
            if values.iter().all(|value| *value == first) {
                return Self::Uniform(first);
            }
        }
        Self::Brick(values)
    }

    fn expand(&self, expected_len: usize) -> Option<Vec<u8>> {
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
    fn from_values(values: Vec<u32>) -> Self {
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
    pub chunk_seed: u64,
    pub sample_step: u16,
    pub iso_level: u8,
    pub axis_samples: Vec<u16>,
    pub rho_field: MixedMetricFieldU8,
    pub zone_field: MixedMetricFieldU32,
}

pub(crate) fn sync_chunk_manager_loader_state_system(player_loader_query: Query<&ChunkLoader, With<Player>>, mut chunk_manager: ResMut<ChunkManager>) {
    let Ok(chunk_loader) = player_loader_query.single() else {
        return;
    };

    chunk_manager.active_scale = chunk_loader.scale;
    chunk_manager.loader_origin_grid = chunk_loader.origin_offset.clone();
    chunk_manager.loader_origin_unit = UnitVec::new(chunk_loader.origin_offset.clone(), Vec3::ZERO);
}

pub(crate) fn hydrate_chunk_demo_data_system(
    mut commands: Commands,
    settings: Res<UsfDemoSettings>,
    chunk_manager: Res<ChunkManager>,
    mut chunk_store: ResMut<UsfDemoChunkStore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    added_chunks: Query<(Entity, &Chunk), (Added<Chunk>, Without<UsfDemoChunkVisual>)>,
) {
    if !settings.enabled {
        return;
    }

    for (entity, chunk) in added_chunks.iter() {
        let canonical_coord = canonical_grid_coord(&chunk.coord);
        let chunk_file = chunk_file_path(&settings, chunk_manager.active_scale, &canonical_coord);
        let expected_coord = SerializableGridCoord::from_grid(&canonical_coord);

        let mut record = load_chunk_record(&chunk_file).filter(|loaded| {
            loaded.world_seed == settings.world_seed
                && loaded.active_scale_index == chunk_manager.active_scale.index_from_top()
                && loaded.chunk_coord == expected_coord
                && loaded.sample_step == settings.sample_step
                && loaded.iso_level == settings.iso_level
        });

        if record.is_none() {
            let generated = generate_chunk_record(&settings, chunk_manager.active_scale, &canonical_coord);
            if let Err(error) = save_chunk_record(&chunk_file, &generated) {
                warn!("USF demo persistence write failed for {:?}: {}", chunk_file, error);
            }
            record = Some(generated);
        }

        let record = record.expect("USF demo record should exist after generate/load");
        let mesh = build_chunk_mesh(&record);
        if let Some(mesh) = mesh {
            let mesh_handle = meshes.add(mesh);
            let material_handle = materials.add(StandardMaterial {
                base_color: color_from_seed(record.chunk_seed),
                perceptual_roughness: 0.9,
                metallic: 0.0,
                ..Default::default()
            });
            commands
                .entity(entity)
                .insert((Mesh3d(mesh_handle), MeshMaterial3d(material_handle), Visibility::Visible));
        }

        commands.entity(entity).insert(UsfDemoChunkVisual {
            chunk_seed: record.chunk_seed,
            sample_step: record.sample_step,
        });

        chunk_store.records.insert(canonical_coord, record);
    }
}

pub(crate) fn sync_chunk_demo_visual_transforms_system(
    settings: Res<UsfDemoSettings>,
    player_loader_query: Query<(&ChunkLoader, &Transform), With<Player>>,
    mut chunk_query: Query<(&Chunk, &mut Transform), (With<UsfDemoChunkVisual>, Without<Player>)>,
) {
    if !settings.enabled {
        return;
    }

    let Ok((chunk_loader, player_transform)) = player_loader_query.single() else {
        return;
    };

    let world_rotation = chunk_loader.world_rotation_quat();
    let world_rotation_origin = player_transform.translation;
    let origin_offset = chunk_loader.origin_offset.clone();

    for (chunk, mut transform) in chunk_query.iter_mut() {
        let layer_z = chunk.coord.scale.compute_z();
        let (native_pos, visual_scale) = chunk.coord.clone().to_native_visual(origin_offset.clone());
        let world_pos = Vec3::new(native_pos.x, native_pos.y, native_pos.z + layer_z);
        transform.translation = world_rotation_origin + world_rotation * (world_pos - world_rotation_origin);
        transform.rotation = world_rotation;
        transform.scale = Vec3::splat(visual_scale);
    }
}

pub(crate) fn prune_chunk_demo_store_system(settings: Res<UsfDemoSettings>, loaded_chunks: Query<&Chunk>, mut chunk_store: ResMut<UsfDemoChunkStore>) {
    if !settings.enabled {
        chunk_store.records.clear();
        return;
    }

    let loaded = loaded_chunks.iter().map(|chunk| canonical_grid_coord(&chunk.coord)).collect::<HashSet<_>>();
    chunk_store.records.retain(|coord, _| loaded.contains(coord));
}

fn generate_chunk_record(settings: &UsfDemoSettings, active_scale: Scale, canonical_coord: &GridVec) -> PersistedChunkRecord {
    let axis_samples = build_axis_samples(settings.sample_step);
    let axis_points = axis_samples.len();
    let total_points = axis_points * axis_points * axis_points;

    let chunk_seed = derive_chunk_seed(settings.world_seed, active_scale, canonical_coord);
    let (chunk_x, chunk_y, chunk_z) = chunk_index_at_scale(canonical_coord);

    let mut rho_values = Vec::with_capacity(total_points);
    let mut zone_values = Vec::with_capacity(total_points);

    for iz in 0..axis_points {
        for iy in 0..axis_points {
            for ix in 0..axis_points {
                let gx = chunk_x * CHUNK_SPAN_UNITS_I64 + axis_samples[ix] as i64;
                let gy = chunk_y * CHUNK_SPAN_UNITS_I64 + axis_samples[iy] as i64;
                let gz = chunk_z * CHUNK_SPAN_UNITS_I64 + axis_samples[iz] as i64;
                rho_values.push(hash_density_u8(settings.world_seed, active_scale, gx, gy, gz));
                zone_values.push(hash_zone_id_u32(settings.world_seed, active_scale, gx, gy, gz));
            }
        }
    }

    PersistedChunkRecord {
        world_seed: settings.world_seed,
        active_scale_index: active_scale.index_from_top(),
        chunk_coord: SerializableGridCoord::from_grid(canonical_coord),
        chunk_seed,
        sample_step: settings.sample_step,
        iso_level: settings.iso_level,
        axis_samples,
        rho_field: MixedMetricFieldU8::from_values(rho_values),
        zone_field: MixedMetricFieldU32::from_values(zone_values),
    }
}

fn build_chunk_mesh(record: &PersistedChunkRecord) -> Option<Mesh> {
    let axis_points = record.axis_samples.len();
    if axis_points < 2 {
        return None;
    }

    let total_points = axis_points * axis_points * axis_points;
    let rho_values = record.rho_field.expand(total_points)?;

    let mut points = vec![Vec3::ZERO; total_points];
    let mut signed_field = vec![0.0_f32; total_points];

    for iz in 0..axis_points {
        for iy in 0..axis_points {
            for ix in 0..axis_points {
                let idx = grid_index(ix, iy, iz, axis_points);
                let x = record.axis_samples[ix] as f32 - HALF_CHUNK_SPAN_F32;
                let y = record.axis_samples[iy] as f32 - HALF_CHUNK_SPAN_F32;
                let z = record.axis_samples[iz] as f32 - HALF_CHUNK_SPAN_F32;
                points[idx] = Vec3::new(x, y, z);
                signed_field[idx] = rho_values[idx] as f32 - record.iso_level as f32;
            }
        }
    }

    let cube_corners: [[usize; 3]; 8] = [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0], [0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]];
    let tetrahedra: [[usize; 4]; 6] = [[0, 5, 1, 6], [0, 1, 2, 6], [0, 2, 3, 6], [0, 3, 7, 6], [0, 7, 4, 6], [0, 4, 5, 6]];

    let mut out_positions = Vec::<[f32; 3]>::new();
    let mut out_normals = Vec::<[f32; 3]>::new();
    let mut out_uvs = Vec::<[f32; 2]>::new();

    for iz in 0..(axis_points - 1) {
        for iy in 0..(axis_points - 1) {
            for ix in 0..(axis_points - 1) {
                let mut cube_points = [Vec3::ZERO; 8];
                let mut cube_values = [0.0_f32; 8];
                for (corner_i, [ox, oy, oz]) in cube_corners.iter().copied().enumerate() {
                    let sx = ix + ox;
                    let sy = iy + oy;
                    let sz = iz + oz;
                    let idx = grid_index(sx, sy, sz, axis_points);
                    cube_points[corner_i] = points[idx];
                    cube_values[corner_i] = signed_field[idx];
                }

                for tet in tetrahedra {
                    let p = [cube_points[tet[0]], cube_points[tet[1]], cube_points[tet[2]], cube_points[tet[3]]];
                    let s = [cube_values[tet[0]], cube_values[tet[1]], cube_values[tet[2]], cube_values[tet[3]]];
                    emit_tetra_surface(p, s, &mut out_positions, &mut out_normals, &mut out_uvs);
                }
            }
        }
    }

    if out_positions.is_empty() {
        return None;
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, out_positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, out_normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, out_uvs);
    Some(mesh)
}

fn emit_tetra_surface(points: [Vec3; 4], values: [f32; 4], out_positions: &mut Vec<[f32; 3]>, out_normals: &mut Vec<[f32; 3]>, out_uvs: &mut Vec<[f32; 2]>) {
    let mut inside = [0usize; 4];
    let mut outside = [0usize; 4];
    let mut inside_count = 0usize;
    let mut outside_count = 0usize;

    for i in 0..4 {
        if values[i] <= 0.0 {
            inside[inside_count] = i;
            inside_count += 1;
        } else {
            outside[outside_count] = i;
            outside_count += 1;
        }
    }

    if inside_count == 0 || inside_count == 4 {
        return;
    }

    let edge_point = |a_i: usize, b_i: usize| interpolate_iso(points[a_i], values[a_i], points[b_i], values[b_i]);

    match inside_count {
        1 => {
            let i = inside[0];
            let a = outside[0];
            let b = outside[1];
            let c = outside[2];
            let inside_ref = points[i];
            let outside_ref = (points[a] + points[b] + points[c]) / 3.0;
            push_oriented_triangle(
                edge_point(i, a),
                edge_point(i, b),
                edge_point(i, c),
                inside_ref,
                outside_ref,
                out_positions,
                out_normals,
                out_uvs,
            );
        }
        3 => {
            let o = outside[0];
            let a = inside[0];
            let b = inside[1];
            let c = inside[2];
            let inside_ref = (points[a] + points[b] + points[c]) / 3.0;
            let outside_ref = points[o];
            push_oriented_triangle(
                edge_point(o, a),
                edge_point(o, c),
                edge_point(o, b),
                inside_ref,
                outside_ref,
                out_positions,
                out_normals,
                out_uvs,
            );
        }
        2 => {
            let a = inside[0];
            let b = inside[1];
            let c = outside[0];
            let d = outside[1];
            let inside_ref = (points[a] + points[b]) * 0.5;
            let outside_ref = (points[c] + points[d]) * 0.5;

            let p0 = edge_point(a, c);
            let p1 = edge_point(b, c);
            let p2 = edge_point(b, d);
            let p3 = edge_point(a, d);
            push_oriented_triangle(p0, p1, p2, inside_ref, outside_ref, out_positions, out_normals, out_uvs);
            push_oriented_triangle(p0, p2, p3, inside_ref, outside_ref, out_positions, out_normals, out_uvs);
        }
        _ => {}
    }
}

fn push_oriented_triangle(
    a: Vec3,
    mut b: Vec3,
    mut c: Vec3,
    inside_ref: Vec3,
    outside_ref: Vec3,
    out_positions: &mut Vec<[f32; 3]>,
    out_normals: &mut Vec<[f32; 3]>,
    out_uvs: &mut Vec<[f32; 2]>,
) {
    let mut normal = (b - a).cross(c - a);
    let mut len_sq = normal.length_squared();
    if len_sq <= 1e-10 {
        return;
    }

    let expected_outward = outside_ref - inside_ref;
    if expected_outward.length_squared() > 1e-10 && normal.dot(expected_outward) < 0.0 {
        std::mem::swap(&mut b, &mut c);
        normal = (b - a).cross(c - a);
        len_sq = normal.length_squared();
        if len_sq <= 1e-10 {
            return;
        }
    }

    let n = normal / len_sq.sqrt();
    for p in [a, b, c] {
        out_positions.push([p.x, p.y, p.z]);
        out_normals.push([n.x, n.y, n.z]);
        out_uvs.push([
            ((p.x + HALF_CHUNK_SPAN_F32) / CHUNK_SPAN_UNITS_I64 as f32).clamp(0.0, 1.0),
            ((p.y + HALF_CHUNK_SPAN_F32) / CHUNK_SPAN_UNITS_I64 as f32).clamp(0.0, 1.0),
        ]);
    }
}

#[inline]
fn interpolate_iso(a_pos: Vec3, a_val: f32, b_pos: Vec3, b_val: f32) -> Vec3 {
    let denom = a_val - b_val;
    let t = if denom.abs() <= 1e-6 { 0.5 } else { (a_val / denom).clamp(0.0, 1.0) };
    a_pos + (b_pos - a_pos) * t
}

#[inline]
fn grid_index(ix: usize, iy: usize, iz: usize, axis_points: usize) -> usize {
    ix + axis_points * (iy + axis_points * iz)
}

fn canonical_grid_coord(coord: &GridVec) -> GridVec {
    let mut canonical = coord.clone();
    canonical.normalize();
    canonical
}

fn chunk_index_at_scale(coord: &GridVec) -> (i64, i64, i64) {
    let mut canonical = coord.clone();
    canonical.normalize();
    let mut x = 0_i64;
    let mut y = 0_i64;
    let mut z = 0_i64;
    for digit in canonical.to_raw_vec_3d() {
        x = x * 10 + digit.x as i64;
        y = y * 10 + digit.y as i64;
        z = z * 10 + digit.z as i64;
    }
    (x, y, z)
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

fn derive_chunk_seed(world_seed: u64, active_scale: Scale, canonical_coord: &GridVec) -> u64 {
    let mut hasher = DefaultHasher::new();
    world_seed.hash(&mut hasher);
    active_scale.hash(&mut hasher);
    canonical_coord.hash(&mut hasher);
    let raw = hasher.finish();
    if raw == 0 { 0x9e37_79b9_7f4a_7c15 } else { raw }
}

fn chunk_file_path(settings: &UsfDemoSettings, active_scale: Scale, canonical_coord: &GridVec) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    canonical_coord.hash(&mut hasher);
    let coord_hash = hasher.finish();
    Path::new(&settings.persistence_dir).join(format!(
        "ws_{:016x}_as_{:02}_coord_{:016x}.json",
        settings.world_seed,
        active_scale.index_from_top(),
        coord_hash
    ))
}

fn load_chunk_record(path: &Path) -> Option<PersistedChunkRecord> {
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice::<PersistedChunkRecord>(&bytes).ok()
}

fn save_chunk_record(path: &Path, record: &PersistedChunkRecord) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("create_dir_all failed: {error}"))?;
    }
    let encoded = serde_json::to_vec_pretty(record).map_err(|error| format!("serialize failed: {error}"))?;
    fs::write(path, encoded).map_err(|error| format!("write failed: {error}"))?;
    Ok(())
}

fn color_from_seed(seed: u64) -> Color {
    let r = ((seed & 0xff) as f32) / 255.0;
    let g = (((seed >> 8) & 0xff) as f32) / 255.0;
    let b = (((seed >> 16) & 0xff) as f32) / 255.0;
    Color::srgb(0.2 + 0.6 * r, 0.2 + 0.6 * g, 0.2 + 0.6 * b)
}

fn hash_density_u8(world_seed: u64, active_scale: Scale, gx: i64, gy: i64, gz: i64) -> u8 {
    let mut state = mix64(world_seed ^ 0xa5a5_35f4_9be3_c211_u64);
    state = mix64(state ^ active_scale.index_from_top() as u64);
    state = mix64(state ^ fold_signed(gx));
    state = mix64(state ^ fold_signed(gy));
    state = mix64(state ^ fold_signed(gz));
    (state & 0xff) as u8
}

fn hash_zone_id_u32(world_seed: u64, active_scale: Scale, gx: i64, gy: i64, gz: i64) -> u32 {
    let mut state = mix64(world_seed ^ 0x13d7_4b29_11f2_7a67_u64);
    state = mix64(state ^ (active_scale.index_from_top() as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
    state = mix64(state ^ fold_signed(gx));
    state = mix64(state ^ fold_signed(gy));
    state = mix64(state ^ fold_signed(gz));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::pos::types::GridXyz;

    fn test_settings() -> UsfDemoSettings {
        UsfDemoSettings {
            enabled: true,
            world_seed: 42,
            sample_step: 64,
            iso_level: 128,
            persistence_dir: std::env::temp_dir().join("usf_demo_chunk_tests").to_string_lossy().to_string(),
        }
    }

    fn rho_values(record: &PersistedChunkRecord) -> Vec<u8> {
        let axis_points = record.axis_samples.len();
        let total = axis_points * axis_points * axis_points;
        record.rho_field.expand(total).expect("rho field should expand")
    }

    #[test]
    fn rho_sampling_matches_across_adjacent_chunk_borders() {
        let settings = test_settings();
        let left = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let right = left.clone() + IVec3::new(1, 0, 0);

        let left_record = generate_chunk_record(&settings, Scale::MAX, &left);
        let right_record = generate_chunk_record(&settings, Scale::MAX, &right);

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
    fn persistence_roundtrip_is_stable() {
        let settings = test_settings();
        let coord = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let record = generate_chunk_record(&settings, Scale::MAX, &coord);

        let path = Path::new(&settings.persistence_dir).join("roundtrip_chunk.json");
        save_chunk_record(&path, &record).expect("save should succeed");
        let loaded = load_chunk_record(&path).expect("load should succeed");
        assert_eq!(record, loaded);

        let _ = fs::remove_file(path);
    }
}
