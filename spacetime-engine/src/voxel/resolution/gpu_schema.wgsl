#define_import_path spacetime_engine::voxel::gpu_schema

const BLOCK: u32 = 8u;
const REGULAR_LATTICE: u32 = 9u;
const REGULAR_LATTICE_LEN: u32 = 729u;
const FACE_LATTICE: u32 = 17u;
const FACE_LATTICE_LEN: u32 = 289u;

const REG_CLASS_OFFSET: u32 = 0u;
const REG_COUNTS_OFFSET: u32 = 256u;
const REG_TRI_OFFSET: u32 = 272u;
const REG_VERTEX_OFFSET: u32 = 512u;
const TRANS_CLASS_OFFSET: u32 = 3584u;
const TRANS_COUNTS_OFFSET: u32 = 4096u;
const TRANS_TRI_OFFSET: u32 = 4152u;
const TRANS_VERTEX_OFFSET: u32 = 6168u;

struct GpuTerrainDescriptor {
    chart_origin_and_spacing: vec4<f32>,
    anchor_direction_and_inverse_radius: vec4<f32>,
    extent_uv_radius: vec4<f32>,
    terrain_meta: vec4<u32>,
}

struct GpuTerrainDispatch {
    ranges: vec4<u32>,
    descriptor: GpuTerrainDescriptor,
}

struct SurfaceVertex {
    position: vec3<f32>,
    normal: vec3<f32>,
}
