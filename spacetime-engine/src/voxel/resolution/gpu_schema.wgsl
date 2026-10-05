#define_import_path spacetime_engine::voxel::gpu_schema

const BLOCK: u32 = 8u;
const REGULAR_LATTICE: u32 = 9u;
const REGULAR_LATTICE_LEN: u32 = 729u;
const FACE_LATTICE: u32 = 17u;
const FACE_LATTICE_LEN: u32 = 289u;
const HASH_DIGITS: u32 = 20u;
const MAX_COARSE_BANDS: u32 = 35u;
const MAX_FINE_BANDS: u32 = 36u;

const REG_CLASS_OFFSET: u32 = 0u;
const REG_COUNTS_OFFSET: u32 = 256u;
const REG_TRI_OFFSET: u32 = 272u;
const REG_VERTEX_OFFSET: u32 = 512u;
const TRANS_CLASS_OFFSET: u32 = 3584u;
const TRANS_COUNTS_OFFSET: u32 = 4096u;
const TRANS_TRI_OFFSET: u32 = 4152u;
const TRANS_VERTEX_OFFSET: u32 = 6168u;

struct GpuCoarseBand {
    params: vec4<f32>,
    seeds: vec4<u32>,
}

struct GpuFineBand {
    params: vec4<f32>,
    base_offset: vec4<f32>,
    band_meta: vec4<i32>,
    prefix: vec4<u32>,
    digits_x: array<vec4<i32>, 5>,
    digits_y: array<vec4<i32>, 5>,
    digits_z: array<vec4<i32>, 5>,
}

struct GpuCaveChart {
    base_cell: vec4<i32>,
    fraction_and_wavelength: vec4<f32>,
    seed: vec4<u32>,
}

struct GpuTerrainDescriptor {
    chart_origin_and_spacing: vec4<f32>,
    anchor_direction_and_inverse_radius: vec4<f32>,
    extent_uv_radius: vec4<f32>,
    reference_relief_and_cave_depths: vec4<f32>,
    terrain_meta: vec4<u32>,
    semantic_meta: vec4<i32>,
    seed_meta: vec4<u32>,
    coarse: array<GpuCoarseBand, 35>,
    fine: array<GpuFineBand, 36>,
    caves: array<GpuCaveChart, 5>,
}

struct GpuTerrainDispatch {
    ranges: vec4<u32>,
    descriptor: GpuTerrainDescriptor,
}

struct SurfaceVertex {
    position: vec3<f32>,
    normal: vec3<f32>,
}
