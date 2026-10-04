//! GPU-native binary terrain presentation.
//!
//! Canonical terrain authority stays on the CPU/USF side. This module receives
//! one bounded semantic chart per binary clipmap block, evaluates density and
//! Transvoxel topology entirely on the GPU, and writes directly into Bevy's
//! MeshAllocator slabs. No density/geometry readback is performed.
//!
//! gpu-binary-presentation-production-v1
// gpu-terrain-table-readonly-binding-repair-v1

use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

use bevy::{
    asset::{load_internal_asset, uuid_handle, AssetId, RenderAssetUsages},
    core_pipeline::schedule::camera_driver,
    math::{DVec3, IVec4, UVec4, Vec3, Vec4},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    render::{
        diagnostic::RecordDiagnostics,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        mesh::allocator::{MeshAllocator, MeshAllocatorSettings},
        render_resource::{
            binding_types::{
                storage_buffer, storage_buffer_read_only, uniform_buffer,
            },
            *,
        },
        renderer::{RenderContext, RenderGraph, RenderQueue},
        Render, RenderApp, RenderStartup,
    },
    shader::{Shader, ShaderCacheError},
};

use crate::{
    spatial::{SpatialScale, SPATIAL_SCALE_MAX, SPATIAL_SCALE_MIN},
    voxel::{CelestialBodyProfile, CelestialVoxelField},
};

const GPU_TERRAIN_DENSITY_SHADER: Handle<Shader> =
    uuid_handle!("56f43359-c86b-4cd9-a561-9f07f86d0a51");
const GPU_TERRAIN_TOPOLOGY_SHADER: Handle<Shader> =
    uuid_handle!("96b866c0-a48a-4558-9e24-b8773dcf569e");
// gpu-terrain-split-pipeline-v1
// gpu-terrain-memory-pressure-repair-v1
// gpu-terrain-frontier-stability-v1
// gpu-terrain-tracy-observability-v1

const BLOCK_SUBDIVISIONS: usize = 8;
const REGULAR_MAX_TRIANGLES: usize = BLOCK_SUBDIVISIONS * BLOCK_SUBDIVISIONS
    * BLOCK_SUBDIVISIONS * 5;
const REGULAR_MAX_VERTICES: usize =
    3 * BLOCK_SUBDIVISIONS * (BLOCK_SUBDIVISIONS + 1) * (BLOCK_SUBDIVISIONS + 1);
const REGULAR_MAX_INDICES: usize = REGULAR_MAX_TRIANGLES * 3;
const TRANSITION_MAX_TRIANGLES_PER_FACE: usize =
    BLOCK_SUBDIVISIONS * BLOCK_SUBDIVISIONS * 12;
const TRANSITION_MAX_VERTICES_PER_FACE: usize =
    TRANSITION_MAX_TRIANGLES_PER_FACE * 3;
const TRANSITION_MAX_INDICES_PER_FACE: usize =
    TRANSITION_MAX_TRIANGLES_PER_FACE * 3;
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
pub(crate) struct GpuTerrainBlock {
    mesh: Handle<Mesh>,
    build_id: u64,
    descriptor: GpuTerrainDescriptor,
}

impl GpuTerrainBlock {
    pub(crate) fn new(
        mesh: Handle<Mesh>,
        build_id: u64,
        descriptor: GpuTerrainDescriptor,
    ) -> Self {
        Self {
            mesh,
            build_id,
            descriptor,
        }
    }
}

#[derive(Resource)]
pub(crate) struct GpuTerrainRuntime {
    next_build_id: u64,
    completed: Arc<Mutex<VecDeque<u64>>>,
}

impl GpuTerrainRuntime {
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
struct GpuTerrainCompletionSink(Arc<Mutex<VecDeque<u64>>>);

#[derive(Resource, Default)]
struct GpuTerrainRenderState {
    processed: HashMap<AssetId<Mesh>, u64>,
    pending: Vec<GpuTerrainBlock>,
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

pub(crate) struct GpuTerrainPresentationPlugin;

impl Plugin for GpuTerrainPresentationPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            GPU_TERRAIN_DENSITY_SHADER,
            "gpu_density.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            GPU_TERRAIN_TOPOLOGY_SHADER,
            "gpu_transvoxel.wgsl",
            Shader::from_wgsl
        );

        let completed = Arc::new(Mutex::new(VecDeque::new()));
        app.insert_resource(GpuTerrainRuntime {
            next_build_id: 0,
            completed: completed.clone(),
        });
        app.add_plugins(ExtractComponentPlugin::<GpuTerrainBlock>::default());

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .insert_resource(GpuTerrainCompletionSink(completed))
            .init_resource::<GpuTerrainRenderState>()
            .add_systems(
                RenderStartup,
                (init_gpu_terrain_pipeline, init_gpu_terrain_buffers),
            )
            .add_systems(Render, prepare_gpu_terrain_blocks)
            .add_systems(
                RenderGraph,
                compute_gpu_terrain.before(camera_driver),
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

        // gpu-terrain-frontier-stability-v1
        // Bound general-slab growth so streaming terrain cannot trigger
        // progressively larger hundreds-of-MiB relocation copies.
        settings.slab_allocator_settings.min_slab_size = 8 * 1024 * 1024;
        settings.slab_allocator_settings.max_slab_size = 64 * 1024 * 1024;
        settings.slab_allocator_settings.large_threshold = 32 * 1024 * 1024;
    }
}

pub(super) fn configure(app: &mut App) {
    app.add_plugins(GpuTerrainPresentationPlugin);
}

/// Fixed-capacity GPU allocation shell.
///
/// This contains no CPU-generated terrain topology. Its sole purpose is to give
/// Bevy's MeshAllocator persistent vertex/index ranges that the compute shader
/// owns afterwards.
pub(crate) fn allocation_mesh(
    transition_face_count: u32,
) -> Mesh {
    // gpu-terrain-memory-pressure-repair-v1
    //
    // Regular topology exists for every block. Transition capacity is reserved
    // only for faces that actually bridge a 2:1 LOD boundary. This keeps the
    // fixed draw-allocation contract without multiplying every block by the
    // six-face worst case.
    let transition_face_count =
        transition_face_count.min(6) as usize;
    let max_vertices = REGULAR_MAX_VERTICES
        + TRANSITION_MAX_VERTICES_PER_FACE * transition_face_count;
    let max_indices = REGULAR_MAX_INDICES
        + TRANSITION_MAX_INDICES_PER_FACE * transition_face_count;

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0_f32; 3]; max_vertices],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0_f32, 1.0, 0.0]; max_vertices],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0_f32; 2]; max_vertices],
    )
    .with_inserted_indices(Indices::U32(vec![0; max_indices]))
}

fn profile_id(profile: CelestialBodyProfile) -> u32 {
    match profile {
        CelestialBodyProfile::Lunar => 0,
        CelestialBodyProfile::Rocky => 1,
        CelestialBodyProfile::Stellar => 2,
    }
}

fn detail_parameters(profile: CelestialBodyProfile) -> (f64, f64, f64, u32) {
    match profile {
        CelestialBodyProfile::Lunar => (0.82, 0.040, 1.70, 0x4C55_4E41),
        CelestialBodyProfile::Rocky => (0.72, 0.0025, 1.65, 0x524F_434B),
        CelestialBodyProfile::Stellar => (0.48, 0.008, 1.30, 0x5354_4152),
    }
}

#[inline]
fn mix_u32(mut state: u32, input: u32) -> u32 {
    state ^= input.wrapping_mul(0x85EB_CA6B);
    state ^= state >> 16;
    state = state.wrapping_mul(0x7FEB_352D);
    state ^= state >> 15;
    state
}

fn scale_layer_seed(seed: u32, exponent: i8) -> u32 {
    mix_u32(
        seed ^ 0xA17E_5CA1,
        (i32::from(exponent) - i32::from(SPATIAL_SCALE_MIN)) as u32,
    )
}

fn zero_prefix_state(seed: u32, zero_scale_count: usize) -> u32 {
    let mut state = seed ^ 0x517C_C1B7;
    for _ in 0..zero_scale_count {
        state = mix_u32(state, 0);
        state = mix_u32(state, 0);
        state = mix_u32(state, 0);
    }
    state
}

fn canonical_axis(value: f64) -> Option<(i64, f32)> {
    const CHUNK: f64 = 1_000.0;
    const HALF: f64 = 500.0;

    if !value.is_finite() {
        return None;
    }
    let carry_f = ((value + HALF) / CHUNK).floor();
    if carry_f < i64::MIN as f64 || carry_f > i64::MAX as f64 {
        return None;
    }
    let mut chunk = carry_f as i64;
    let remainder = value - chunk as f64 * CHUNK;
    let stored = remainder as f32;
    if !stored.is_finite() {
        return None;
    }
    let carry2 = ((f64::from(stored) + HALF) / CHUNK).floor() as i64;
    let mut local = (f64::from(stored) - carry2 as f64 * CHUNK) as f32;
    chunk = chunk.checked_add(carry2)?;

    if local >= 500.0 {
        local -= 1_000.0;
        chunk = chunk.checked_add(1)?;
    } else if local < -500.0 {
        local += 1_000.0;
        chunk = chunk.checked_sub(1)?;
    }
    Some((chunk, local))
}

fn balanced_digits(mut value: i64, count: usize) -> Option<[i32; HASH_DIGIT_WINDOW]> {
    let mut out = [0_i32; HASH_DIGIT_WINDOW];
    for slot in out.iter_mut().take(count) {
        let parent = (i128::from(value) + 5).div_euclid(10);
        let digit = i128::from(value) - parent * 10;
        if !(-5..5).contains(&digit) {
            return None;
        }
        *slot = digit as i32;
        value = i64::try_from(parent).ok()?;
    }
    (value == 0).then_some(out)
}

fn pack_digits(values: [i32; HASH_DIGIT_WINDOW]) -> [IVec4; HASH_DIGIT_PACKS] {
    std::array::from_fn(|pack| {
        let base = pack * 4;
        IVec4::new(
            values[base],
            values[base + 1],
            values[base + 2],
            values[base + 3],
        )
    })
}

fn fine_band(
    anchor_pre_fine_surface: DVec3,
    level: SpatialScale,
    amplitude_metres: f64,
    seed: u32,
) -> Option<GpuFineBand> {
    let metres_per_native = level.metres_per_native();
    let native = anchor_pre_fine_surface / metres_per_native;
    let (cx, ox) = canonical_axis(native.x)?;
    let (cy, oy) = canonical_axis(native.y)?;
    let (cz, oz) = canonical_axis(native.z)?;

    let digit_count = (i16::from(SPATIAL_SCALE_MAX)
        - i16::from(level.exponent())
        + 1) as usize;
    let digit_len = digit_count.min(HASH_DIGIT_WINDOW);
    let prefix_count = digit_count.saturating_sub(digit_len);

    let x = balanced_digits(cx, digit_len)?;
    let y = balanced_digits(cy, digit_len)?;
    let z = balanced_digits(cz, digit_len)?;

    let broad_seed = seed ^ 0xA341_316C;
    let fine_seed = seed ^ 0xC801_3EA4;

    Some(GpuFineBand {
        params: Vec4::new(
            metres_per_native as f32,
            amplitude_metres as f32,
            0.0,
            0.0,
        ),
        base_offset: Vec4::new(ox, oy, oz, 0.0),
        meta: IVec4::new(
            i32::from(level.exponent()),
            digit_len as i32,
            0,
            0,
        ),
        prefix: UVec4::new(
            zero_prefix_state(broad_seed, prefix_count),
            zero_prefix_state(fine_seed, prefix_count),
            0,
            0,
        ),
        digits_x: pack_digits(x),
        digits_y: pack_digits(y),
        digits_z: pack_digits(z),
    })
}

fn cave_chart(
    anchor: DVec3,
    wavelength: f32,
    offset: Vec3,
    seed: u32,
) -> Option<GpuCaveChart> {
    let phase = anchor / f64::from(wavelength)
        + DVec3::new(
            f64::from(offset.x),
            f64::from(offset.y),
            f64::from(offset.z),
        );
    let floor = phase.floor();
    let cell = IVec4::new(
        i32::try_from(floor.x as i64).ok()?,
        i32::try_from(floor.y as i64).ok()?,
        i32::try_from(floor.z as i64).ok()?,
        0,
    );
    let fraction = phase - floor;
    Some(GpuCaveChart {
        base_cell: cell,
        fraction_and_wavelength: Vec4::new(
            fraction.x as f32,
            fraction.y as f32,
            fraction.z as f32,
            wavelength,
        ),
        seed: UVec4::new(seed, 0, 0, 0),
    })
}

fn finite_vec3(value: DVec3) -> Option<Vec3> {
    let out = Vec3::new(value.x as f32, value.y as f32, value.z as f32);
    out.is_finite().then_some(out)
}

pub(crate) fn descriptor_for_block(
    field: CelestialVoxelField,
    origin_local_metres: DVec3,
    extent_metres: f64,
    spacing_metres: f64,
    transition_bits: u8,
) -> Option<GpuTerrainDescriptor> {
    if !origin_local_metres.is_finite()
        || !extent_metres.is_finite()
        || extent_metres <= 0.0
        || !spacing_metres.is_finite()
        || spacing_metres <= 0.0
        || extent_metres > f64::from(f32::MAX)
    {
        return None;
    }

    let center = origin_local_metres + DVec3::splat(extent_metres * 0.5);
    let center_radius = center.length();
    let anchor_direction = if center_radius.is_finite() && center_radius > f64::EPSILON {
        Vec3::new(
            (center.x / center_radius) as f32,
            (center.y / center_radius) as f32,
            (center.z / center_radius) as f32,
        )
        .normalize_or_zero()
    } else {
        Vec3::Y
    };
    let anchor_direction = if anchor_direction == Vec3::ZERO {
        Vec3::Y
    } else {
        anchor_direction
    };

    let sampler = field.presentation_sampler(spacing_metres)?;
    let anchor_pre_fine_surface =
        sampler.pre_fine_surface_local_metres(anchor_direction).ok()?;
    let anchor_radius = anchor_pre_fine_surface.length();
    if !anchor_radius.is_finite() || anchor_radius <= f64::EPSILON {
        return None;
    }

    let chart_origin_delta =
        finite_vec3(origin_local_metres - anchor_pre_fine_surface)?;
    let inverse_anchor_radius = (1.0 / anchor_radius) as f32;
    let extent_f32 = extent_metres as f32;
    let spacing_f32 = spacing_metres as f32;
    if !extent_f32.is_finite() || !spacing_f32.is_finite() {
        return None;
    }

    let floor = field.surface_detail_scale().exponent();
    let root = field.coarsest_detail_scale().exponent();
    let profile = field.profile();
    let seed = field.seed();
    let body_radius = field
        .radius_metres()
        .clamp(0.0, f64::from(f32::MAX)) as f32;

    let mut coarse = [GpuCoarseBand::default(); MAX_COARSE_BANDS];
    let mut coarse_count = 0usize;
    let mut fine = [GpuFineBand::default(); MAX_FINE_BANDS];
    let mut fine_count = 0usize;
    let (frequency_factor, amplitude, growth, salt) =
        detail_parameters(profile);

    let coarse_lower = floor.max(1);
    if coarse_lower <= root {
        for raw in (coarse_lower..=root).rev() {
            if coarse_count >= MAX_COARSE_BANDS {
                return None;
            }
            let level = SpatialScale::new(raw)?;
            let metres_per_native = level.metres_per_native();
            let depth = i32::from(root - raw).max(0);
            let amplitude_native = amplitude * growth.powi(depth);
            let angular_frequency = (
                field.radius_metres() / metres_per_native * frequency_factor
            )
                .max(4.0)
                .min(f64::from(f32::MAX)) as f32;
            coarse[coarse_count] = GpuCoarseBand {
                params: Vec4::new(
                    (amplitude_native * metres_per_native) as f32,
                    angular_frequency,
                    0.0,
                    0.0,
                ),
                seeds: UVec4::new(
                    scale_layer_seed(seed ^ salt, raw),
                    0,
                    0,
                    0,
                ),
            };
            coarse_count += 1;
        }
    }

    let fine_upper = root.min(0);
    if floor <= fine_upper {
        for raw in (floor..=fine_upper).rev() {
            if fine_count >= MAX_FINE_BANDS {
                return None;
            }
            let level = SpatialScale::new(raw)?;
            let metres_per_native = level.metres_per_native();
            let depth = i32::from(root - raw).max(0);
            let amplitude_native = amplitude * growth.powi(depth);
            let band_seed = scale_layer_seed(seed ^ salt, raw);
            fine[fine_count] = fine_band(
                anchor_pre_fine_surface,
                level,
                amplitude_native * metres_per_native,
                band_seed,
            )?;
            fine_count += 1;
        }
    }

    let central_half_extent =
        DVec3::splat(extent_metres * 0.5 + spacing_metres);
    let include_caves = profile == CelestialBodyProfile::Rocky
        && field.presentation_caves_may_intersect_aabb(
            center,
            central_half_extent,
        );

    let mut caves = [GpuCaveChart::default(); 5];
    if include_caves {
        caves[0] = cave_chart(
            anchor_pre_fine_surface,
            520.0,
            Vec3::new(13.7, -5.1, 8.9),
            seed ^ 0x4341_5645,
        )?;
        caves[1] = cave_chart(
            anchor_pre_fine_surface,
            390.0,
            Vec3::new(-7.4, 19.2, -11.6),
            seed ^ 0x5455_4E4C,
        )?;
        caves[2] = cave_chart(
            anchor_pre_fine_surface,
            240.0,
            Vec3::new(-21.3, 4.8, 15.2),
            seed ^ 0x4252_414E,
        )?;
        caves[3] = cave_chart(
            anchor_pre_fine_surface,
            310.0,
            Vec3::new(6.6, -17.9, 2.7),
            seed ^ 0x4348_4D42,
        )?;
        caves[4] = cave_chart(
            anchor_pre_fine_surface,
            680.0,
            Vec3::new(31.7, -14.1, 9.3),
            seed ^ 0x4348_414D,
        )?;
    }

    let uv_x =
        (origin_local_metres.x.rem_euclid(UV_PHASE_WRAP_METRES) * 0.5) as f32;
    let uv_z =
        (origin_local_metres.z.rem_euclid(UV_PHASE_WRAP_METRES) * 0.5) as f32;

    Some(GpuTerrainDescriptor {
        chart_origin_and_spacing:
            chart_origin_delta.extend(spacing_f32),
        anchor_direction_and_inverse_radius:
            anchor_direction.extend(inverse_anchor_radius),
        extent_uv_radius: Vec4::new(
            extent_f32,
            uv_x,
            uv_z,
            body_radius,
        ),
        meta: UVec4::new(
            profile_id(profile),
            u32::from(transition_bits),
            coarse_count as u32,
            fine_count as u32,
        ),
        semantic_meta: IVec4::new(
            i32::from(floor),
            if include_caves { 1 } else { 0 },
            0,
            0,
        ),
        seed_meta: UVec4::new(seed, 0, 0, 0),
        coarse,
        fine,
        caves,
    })
}

fn init_gpu_terrain_pipeline(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
) {
    let density_layout = BindGroupLayoutDescriptor::new(
        "voxel GPU density",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                uniform_buffer::<GpuTerrainDispatch>(false),
                storage_buffer::<Vec<f32>>(false),
            ),
        ),
    );
    let topology_layout = BindGroupLayoutDescriptor::new(
        "voxel GPU Transvoxel",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                uniform_buffer::<GpuTerrainDispatch>(false),
                storage_buffer_read_only::<Vec<u32>>(false),
                storage_buffer_read_only::<Vec<f32>>(false),
                storage_buffer::<Vec<f32>>(false),
                storage_buffer::<Vec<u32>>(false),
            ),
        ),
    );

    let density_pipeline = pipeline_cache.queue_compute_pipeline(
        ComputePipelineDescriptor {
            label: Some("voxel GPU density".into()),
            layout: vec![density_layout.clone()],
            shader: GPU_TERRAIN_DENSITY_SHADER.clone(),
            ..default()
        },
    );
    let topology_pipeline = pipeline_cache.queue_compute_pipeline(
        ComputePipelineDescriptor {
            label: Some("voxel GPU Transvoxel".into()),
            layout: vec![topology_layout.clone()],
            shader: GPU_TERRAIN_TOPOLOGY_SHADER.clone(),
            ..default()
        },
    );

    commands.insert_resource(GpuTerrainPipeline {
        density_layout,
        density_pipeline,
        topology_layout,
        topology_pipeline,
    });
}


fn pack_transvoxel_tables() -> Vec<u32> {
    use transvoxel_data::{
        regular_cell_data::{
            REGULAR_CELL_CLASS, REGULAR_CELL_DATA, REGULAR_VERTEX_DATA,
        },
        transition_cell_data::{
            TRANSITION_CELL_CLASS, TRANSITION_CELL_DATA,
            TRANSITION_VERTEX_DATA,
        },
    };

    let mut out = Vec::with_capacity(12_312);
    out.extend(REGULAR_CELL_CLASS.into_iter().map(u32::from));
    out.extend(REGULAR_CELL_DATA.into_iter().map(|data| {
        u32::from(
            (data.get_vertex_count() << 4)
                | data.get_triangle_count(),
        )
    }));
    for data in REGULAR_CELL_DATA {
        out.extend(data.vertex_index.into_iter().map(u32::from));
    }
    for row in REGULAR_VERTEX_DATA {
        out.extend(row.into_iter().map(u32::from));
    }

    out.extend(TRANSITION_CELL_CLASS.into_iter().map(u32::from));
    out.extend(TRANSITION_CELL_DATA.into_iter().map(|data| {
        u32::from(
            (data.get_vertex_count() << 4)
                | data.get_triangle_count(),
        )
    }));
    for data in TRANSITION_CELL_DATA {
        out.extend(data.vertex_index.into_iter().map(u32::from));
    }
    for row in TRANSITION_VERTEX_DATA {
        out.extend(row.into_iter().map(u32::from));
    }

    debug_assert_eq!(out.len(), 12_312);
    out
}

fn init_gpu_terrain_buffers(
    mut commands: Commands,
    render_device: Res<bevy::render::renderer::RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    let mut tables = StorageBuffer::from(pack_transvoxel_tables());
    tables.set_label(Some("canonical Transvoxel lookup tables"));
    tables.write_buffer(&render_device, &render_queue);

    let mut scratch = StorageBuffer::from(vec![0.0_f32; 7665]);
    scratch.set_label(Some("GPU terrain density scratch"));
    scratch.write_buffer(&render_device, &render_queue);

    commands.insert_resource(GpuTransvoxelTables(tables));
    commands.insert_resource(GpuTerrainScratch(scratch));
}


fn prepare_gpu_terrain_blocks(
    blocks: Query<&GpuTerrainBlock>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<GpuTerrainPipeline>,
    mut state: ResMut<GpuTerrainRenderState>,
    mut failure_reported: Local<bool>,
) {
    state.pending.clear();
    state.extracted_blocks = blocks.iter().count();

    let mut failed = None;
    let density_ready = match pipeline_cache
        .get_compute_pipeline_state(pipeline.density_pipeline)
    {
        CachedPipelineState::Ok(_) => true,
        CachedPipelineState::Err(ShaderCacheError::ShaderNotLoaded(_)) => false,
        CachedPipelineState::Err(err) => {
            failed = Some(("density", err));
            false
        }
        _ => false,
    };
    let topology_ready = match pipeline_cache
        .get_compute_pipeline_state(pipeline.topology_pipeline)
    {
        CachedPipelineState::Ok(_) => true,
        CachedPipelineState::Err(ShaderCacheError::ShaderNotLoaded(_)) => false,
        CachedPipelineState::Err(err) => {
            failed = Some(("Transvoxel topology", err));
            false
        }
        _ => false,
    };

    if let Some((stage, err)) = failed {
        if !*failure_reported {
            error!(
                stage,
                ?err,
                "GPU binary terrain compute pipeline failed; binary terrain presentation cannot advance"
            );
            *failure_reported = true;
        }
        return;
    }

    if !density_ready || !topology_ready {
        return;
    }
    *failure_reported = false;

    for block in &blocks {
        let id = block.mesh.id();
        if state.processed.get(&id) == Some(&block.build_id) {
            continue;
        }
        state.pending.push(block.clone());
    }
}



#[cfg(feature = "profiling-tracy")]
fn emit_gpu_terrain_pressure(
    mesh_allocator: &MeshAllocator,
    extracted_blocks: usize,
    pending_blocks: usize,
    dispatched_blocks: usize,
    transition_faces: u64,
    density_sample_calls: u64,
    fine_band_visits: u64,
    processed_records: usize,
) {
    let Some(client) = tracy_client::Client::running() else {
        return;
    };

    client.plot(
        tracy_client::plot_name!("GPU terrain/extracted blocks"),
        extracted_blocks as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/pending blocks"),
        pending_blocks as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/dispatched blocks/frame"),
        dispatched_blocks as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/transition faces/frame"),
        transition_faces as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/density samples/frame"),
        density_sample_calls as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/fine-band visits/frame"),
        fine_band_visits as f64,
    );

    // These MeshAllocator values cover the renderer's shared mesh slab domain,
    // which is useful for spotting terrain-driven allocator pressure.
    client.plot(
        tracy_client::plot_name!("GPU terrain/mesh slab bytes"),
        mesh_allocator.slabs_size() as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/mesh slab count"),
        mesh_allocator.slab_count() as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/index allocations"),
        mesh_allocator.index_allocation_count() as f64,
    );

    // Lower bound only: transition-capable shells carry more fixed indices.
    client.plot(
        tracy_client::plot_name!("GPU terrain/min fixed draw indices/frame"),
        extracted_blocks
            .saturating_mul(REGULAR_MAX_INDICES) as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/processed records"),
        processed_records as f64,
    );
}

fn compute_gpu_terrain(
    mut render_context: RenderContext,
    mesh_allocator: Res<MeshAllocator>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<GpuTerrainPipeline>,
    tables: Res<GpuTransvoxelTables>,
    scratch: Res<GpuTerrainScratch>,
    render_queue: Res<RenderQueue>,
    completion_sink: Res<GpuTerrainCompletionSink>,
    mut state: ResMut<GpuTerrainRenderState>,
) {
    let Some(density_pipeline) =
        pipeline_cache.get_compute_pipeline(pipeline.density_pipeline)
    else {
        return;
    };
    let Some(topology_pipeline) =
        pipeline_cache.get_compute_pipeline(pipeline.topology_pipeline)
    else {
        return;
    };
    let Some(table_buffer) = tables.0.buffer() else {
        return;
    };
    let Some(scratch_buffer) = scratch.0.buffer() else {
        return;
    };

    // RenderDiagnosticsPlugin is automatically present in Bevy Tracy builds.
    // These become actual GPU timestamp zones in Tracy.
    let diagnostics = render_context.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();

    let pending = std::mem::take(&mut state.pending);
    let pending_count = pending.len();
    let mut dispatched = 0usize;
    let mut transition_faces = 0u64;
    let mut density_sample_calls = 0u64;
    let mut fine_band_visits = 0u64;

    for block in pending {
        let mesh_id = block.mesh.id();
        let Some(vertex_slice) =
            mesh_allocator.mesh_vertex_slice(&mesh_id)
        else {
            continue;
        };
        let Some(index_slice) =
            mesh_allocator.mesh_index_slice(&mesh_id)
        else {
            continue;
        };

        let block_transition_faces =
            block.descriptor.meta.y.count_ones() as u64;

        // 9^3 regular samples. Each enabled transition face evaluates 17^2
        // samples, with density + six finite-difference gradient neighbours.
        let block_density_calls = 729u64
            + block_transition_faces.saturating_mul(289u64 * 7u64);

        transition_faces =
            transition_faces.saturating_add(block_transition_faces);
        density_sample_calls =
            density_sample_calls.saturating_add(block_density_calls);
        fine_band_visits = fine_band_visits.saturating_add(
            block_density_calls
                .saturating_mul(u64::from(block.descriptor.meta.w)),
        );

        let dispatch = GpuTerrainDispatch {
            ranges: UVec4::new(
                vertex_slice.range.start * GPU_VERTEX_FLOATS,
                vertex_slice.range.end * GPU_VERTEX_FLOATS,
                index_slice.range.start,
                index_slice.range.end,
            ),
            descriptor: block.descriptor,
        };
        let mut uniform = UniformBuffer::from(dispatch);
        uniform.write_buffer(
            render_context.render_device(),
            &render_queue,
        );

        let density_bind_group =
            render_context.render_device().create_bind_group(
                Some("voxel GPU density block"),
                &pipeline_cache.get_bind_group_layout(
                    &pipeline.density_layout,
                ),
                &BindGroupEntries::sequential((
                    &uniform,
                    scratch_buffer.as_entire_buffer_binding(),
                )),
            );

        {
            let mut pass =
                render_context.command_encoder().begin_compute_pass(
                    &ComputePassDescriptor {
                        label: Some("voxel GPU density"),
                        ..default()
                    },
                );
            let gpu_span = diagnostics.time_span(
                &mut pass,
                "voxel GPU terrain/density",
            );
            pass.set_pipeline(density_pipeline);
            pass.set_bind_group(0, &density_bind_group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
            gpu_span.end(&mut pass);
        }

        let topology_bind_group =
            render_context.render_device().create_bind_group(
                Some("voxel GPU Transvoxel block"),
                &pipeline_cache.get_bind_group_layout(
                    &pipeline.topology_layout,
                ),
                &BindGroupEntries::sequential((
                    &uniform,
                    table_buffer.as_entire_buffer_binding(),
                    scratch_buffer.as_entire_buffer_binding(),
                    vertex_slice.buffer.as_entire_buffer_binding(),
                    index_slice.buffer.as_entire_buffer_binding(),
                )),
            );

        {
            let mut pass =
                render_context.command_encoder().begin_compute_pass(
                    &ComputePassDescriptor {
                        label: Some("voxel GPU Transvoxel"),
                        ..default()
                    },
                );
            let gpu_span = diagnostics.time_span(
                &mut pass,
                "voxel GPU terrain/topology",
            );
            pass.set_pipeline(topology_pipeline);
            pass.set_bind_group(0, &topology_bind_group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
            gpu_span.end(&mut pass);
        }

        dispatched = dispatched.saturating_add(1);
        state.processed.insert(mesh_id, block.build_id);
        completion_sink
            .0
            .lock()
            .expect("GPU terrain completion queue poisoned")
            .push_back(block.build_id);
    }

    #[cfg(feature = "profiling-tracy")]
    emit_gpu_terrain_pressure(
        &mesh_allocator,
        state.extracted_blocks,
        pending_count,
        dispatched,
        transition_faces,
        density_sample_calls,
        fine_band_visits,
        state.processed.len(),
    );
}


