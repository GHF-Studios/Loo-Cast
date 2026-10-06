//! GPU build backend for binary terrain presentation.
//!
//! Canonical terrain authority stays on the CPU/semantic side. This module receives
//! one bounded semantic chart per binary clipmap block, evaluates density and
//! Transvoxel topology entirely on the GPU, and writes directly into Bevy's
//! MeshAllocator slabs. No density/geometry readback is performed.
//!
//! ## Module map
//!
//! - `descriptor`: CPU projection of canonical field parameters into one bounded GPU block
//!   descriptor.
//! - `render`: Render-world allocation, shader admission, dispatch and completion reporting.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex},
};

use bevy::{
    asset::{AssetId, RenderAssetUsages, load_internal_asset, uuid_handle},
    core_pipeline::schedule::camera_driver,
    math::{DVec3, IVec4, UVec4, Vec3, Vec4},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    render::{
        Render, RenderApp, RenderStartup,
        diagnostic::RecordDiagnostics,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        mesh::allocator::{MeshAllocator, MeshAllocatorSettings},
        render_resource::{
            binding_types::{storage_buffer, storage_buffer_read_only, uniform_buffer},
            *,
        },
        renderer::{RenderContext, RenderGraph, RenderQueue},
    },
    shader::{Shader, ShaderCacheError, load_shader_library},
};

mod descriptor;
mod render;

pub(crate) use descriptor::descriptor_for_block;
use render::{
    execute_gpu_terrain_builds, init_gpu_terrain_buffers, init_gpu_terrain_pipeline,
    prepare_gpu_terrain_builds,
};

use crate::{
    spatial::{SPATIAL_SCALE_MAX, SPATIAL_SCALE_MIN, SpatialScale},
    voxel::{
        CelestialBodyProfile, CelestialVoxelField,
        base::{CAVE_MAX_DEPTH_METRES, CAVE_START_DEPTH_METRES},
    },
};

const GPU_TERRAIN_DENSITY_SHADER: Handle<Shader> =
    uuid_handle!("56f43359-c86b-4cd9-a561-9f07f86d0a51");
const GPU_TERRAIN_TOPOLOGY_SHADER: Handle<Shader> =
    uuid_handle!("96b866c0-a48a-4558-9e24-b8773dcf569e");

const BLOCK_SUBDIVISIONS: usize = 8;
const REGULAR_MAX_TRIANGLES: usize =
    BLOCK_SUBDIVISIONS * BLOCK_SUBDIVISIONS * BLOCK_SUBDIVISIONS * 5;
const REGULAR_MAX_VERTICES: usize =
    3 * BLOCK_SUBDIVISIONS * (BLOCK_SUBDIVISIONS + 1) * (BLOCK_SUBDIVISIONS + 1);
const REGULAR_MAX_INDICES: usize = REGULAR_MAX_TRIANGLES * 3;
const TRANSITION_MAX_TRIANGLES_PER_FACE: usize = BLOCK_SUBDIVISIONS * BLOCK_SUBDIVISIONS * 12;
const TRANSITION_MAX_VERTICES_PER_FACE: usize = TRANSITION_MAX_TRIANGLES_PER_FACE * 3;
const TRANSITION_MAX_INDICES_PER_FACE: usize = TRANSITION_MAX_TRIANGLES_PER_FACE * 3;
const GPU_VERTEX_FLOATS: u32 = 8;

const MAX_COARSE_BANDS: usize = 35;
const MAX_FINE_BANDS: usize = 36;
const HASH_DIGIT_WINDOW: usize = 20;
const HASH_DIGIT_PACKS: usize = HASH_DIGIT_WINDOW / 4;
const UV_PHASE_WRAP_METRES: f64 = 65_536.0;

#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub(crate) struct GpuCoarseBand {
    /// x = amplitude metres, y = angular frequency.
    params: Vec4,
    /// x = keyed residual seed.
    seeds: UVec4,
}

#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub(crate) struct GpuFineBand {
    /// x = metres/native, y = amplitude metres.
    params: Vec4,
    /// xyz = canonical native offset in [-500, 500).
    base_offset: Vec4,
    /// x = semantic leaf exponent, y = number of low balanced-decimal digits.
    meta: IVec4,
    /// x = broad prefix state, y = fine prefix state.
    prefix: UVec4,
    digits_x: [IVec4; HASH_DIGIT_PACKS],
    digits_y: [IVec4; HASH_DIGIT_PACKS],
    digits_z: [IVec4; HASH_DIGIT_PACKS],
}

#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub(crate) struct GpuCaveChart {
    /// xyz = integer cell containing the CPU-resolved chart anchor.
    base_cell: IVec4,
    /// xyz = sub-cell anchor phase, w = wavelength in metres.
    fraction_and_wavelength: Vec4,
    /// x = keyed noise seed.
    seed: UVec4,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub(crate) struct GpuTerrainDescriptor {
    /// xyz = block origin relative to the CPU-resolved pre-fine surface anchor.
    /// w = regular sample spacing in metres.
    chart_origin_and_spacing: Vec4,
    /// xyz = anchor direction, w = 1 / anchor radius. No absolute radius/position.
    anchor_direction_and_inverse_radius: Vec4,
    /// x = block extent, y/z = bounded UV phase, w = semantic body radius scalar.
    extent_uv_radius: Vec4,
    /// x = CPU-resolved pre-fine relief; y/z = cave start/max depth metres.
    reference_relief_and_cave_depths: Vec4,
    /// x = profile, y = transition bits, z = coarse count, w = fine count.
    meta: UVec4,
    /// x = semantic surface floor exponent, y = cave evaluation enabled.
    semantic_meta: IVec4,
    /// x = body seed.
    seed_meta: UVec4,
    coarse: [GpuCoarseBand; MAX_COARSE_BANDS],
    fine: [GpuFineBand; MAX_FINE_BANDS],
    caves: [GpuCaveChart; 5],
}

impl Default for GpuTerrainDescriptor {
    fn default() -> Self {
        Self {
            chart_origin_and_spacing: Vec4::ZERO,
            anchor_direction_and_inverse_radius: Vec4::ZERO,
            extent_uv_radius: Vec4::ZERO,
            reference_relief_and_cave_depths: Vec4::ZERO,
            meta: UVec4::ZERO,
            semantic_meta: IVec4::ZERO,
            seed_meta: UVec4::ZERO,
            coarse: [GpuCoarseBand::default(); MAX_COARSE_BANDS],
            fine: [GpuFineBand::default(); MAX_FINE_BANDS],
            caves: [GpuCaveChart::default(); 5],
        }
    }
}

#[derive(Component, ExtractComponent, Clone)]
pub(crate) struct GpuTerrainBuild {
    mesh: Handle<Mesh>,
    build_id: u64,
    descriptor: GpuTerrainDescriptor,
}

impl GpuTerrainBuild {
    pub(crate) fn new(mesh: Handle<Mesh>, build_id: u64, descriptor: GpuTerrainDescriptor) -> Self {
        Self {
            mesh,
            build_id,
            descriptor,
        }
    }
}

#[derive(Resource)]
pub(crate) struct GpuTerrainBuilds {
    next_build_id: u64,
    completed: Arc<Mutex<VecDeque<u64>>>,
}

impl GpuTerrainBuilds {
    pub(crate) fn next_build_id(&mut self) -> u64 {
        self.next_build_id = self.next_build_id.wrapping_add(1).max(1);
        self.next_build_id
    }

    pub(crate) fn drain_completed(&self) -> Vec<u64> {
        let mut completed = self
            .completed
            .lock()
            .expect("GPU terrain completion queue poisoned");
        completed.drain(..).collect()
    }
}

#[derive(Resource, Clone)]
struct GpuTerrainBuildCompletionSink(Arc<Mutex<VecDeque<u64>>>);

#[derive(Resource, Default)]
struct GpuTerrainBuildState {
    processed: HashMap<AssetId<Mesh>, u64>,
    pending: Vec<GpuTerrainBuild>,
    extracted_blocks: usize,
}

#[derive(Resource)]
struct GpuTerrainPipeline {
    density_layout: BindGroupLayoutDescriptor,
    density_pipeline: CachedComputePipelineId,
    topology_layout: BindGroupLayoutDescriptor,
    topology_pipeline: CachedComputePipelineId,
}

#[derive(Resource)]
struct GpuTransvoxelTables(StorageBuffer<Vec<u32>>);

#[derive(Resource)]
struct GpuTerrainScratch(StorageBuffer<Vec<f32>>);

#[derive(Debug, Clone, ShaderType)]
struct GpuTerrainDispatch {
    /// x/y = vertex float range, z/w = index range.
    ranges: UVec4,
    descriptor: GpuTerrainDescriptor,
}

pub(crate) struct GpuTerrainBackendPlugin;

impl Plugin for GpuTerrainBackendPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "../gpu_schema.wgsl");
        load_internal_asset!(
            app,
            GPU_TERRAIN_DENSITY_SHADER,
            "../gpu_density.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            GPU_TERRAIN_TOPOLOGY_SHADER,
            "../gpu_transvoxel.wgsl",
            Shader::from_wgsl
        );

        let completed = Arc::new(Mutex::new(VecDeque::new()));
        app.insert_resource(GpuTerrainBuilds {
            next_build_id: 0,
            completed: completed.clone(),
        });
        app.add_plugins(ExtractComponentPlugin::<GpuTerrainBuild>::default());

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .insert_resource(GpuTerrainBuildCompletionSink(completed))
            .init_resource::<GpuTerrainBuildState>()
            .add_systems(
                RenderStartup,
                (init_gpu_terrain_pipeline, init_gpu_terrain_buffers),
            )
            .add_systems(Render, prepare_gpu_terrain_builds)
            .add_systems(
                RenderGraph,
                execute_gpu_terrain_builds.before(camera_driver),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        let mut settings = render_app
            .world_mut()
            .resource_mut::<MeshAllocatorSettings>();
        settings.extra_buffer_usages |= BufferUsages::STORAGE;

        // Bound general-slab growth so streaming terrain cannot trigger
        // progressively larger hundreds-of-MiB relocation copies.
        settings.slab_allocator_settings.min_slab_size = 8 * 1024 * 1024;
        settings.slab_allocator_settings.max_slab_size = 64 * 1024 * 1024;
        settings.slab_allocator_settings.large_threshold = 32 * 1024 * 1024;
    }
}

pub(super) fn configure(app: &mut App) {
    app.add_plugins(GpuTerrainBackendPlugin);
}

/// Fixed-capacity GPU build target allocation.
///
/// This contains no CPU-generated terrain topology. Its sole purpose is to give
/// Bevy's MeshAllocator persistent vertex/index ranges that the compute shader
/// owns afterwards.
pub(crate) fn allocation_mesh(transition_face_count: u32) -> Mesh {
    //
    // Regular topology exists for every block. Transition capacity is reserved
    // only for faces that actually bridge a 2:1 LOD boundary. This keeps the
    // fixed draw-allocation contract without multiplying every block by the
    // six-face worst case.
    let transition_face_count = transition_face_count.min(6) as usize;
    let max_vertices =
        REGULAR_MAX_VERTICES + TRANSITION_MAX_VERTICES_PER_FACE * transition_face_count;
    let max_indices = REGULAR_MAX_INDICES + TRANSITION_MAX_INDICES_PER_FACE * transition_face_count;

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0_f32; 3]; max_vertices])
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0_f32, 1.0, 0.0]; max_vertices],
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0_f32; 2]; max_vertices])
    .with_inserted_indices(Indices::U32(vec![0; max_indices]))
}
