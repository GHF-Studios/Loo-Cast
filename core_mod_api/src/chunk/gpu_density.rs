use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use bytemuck::{Pod, Zeroable};
use std::borrow::Cow;
use std::sync::{Mutex, OnceLock};

const CHUNK_SPAN_UNITS_F32: f32 = 1_000.0;
const HALF_CHUNK_SPAN_F32: f32 = 500.0;
const WORKGROUP_SIZE: u32 = 64;
const MAP_ALIGNMENT_BYTES: usize = wgpu::MAP_ALIGNMENT as usize;

static GPU_DENSITY_CONTEXT: OnceLock<Result<GpuDensityContext, String>> = OnceLock::new();

#[derive(Clone, Copy, Debug)]
pub struct GpuDensitySemantics {
    pub coarse_span_units: f32,
    pub detail_span_units: f32,
    pub coarse_weight: f32,
    pub detail_weight: f32,
    pub bias: f32,
    pub gain: f32,
    pub center: f32,
    pub seed_salt_coarse: u64,
    pub seed_salt_detail: u64,
    pub zone_density_multiplier: f32,
    pub zone_density_offset: f32,
    pub zone_density_floor: f32,
    pub zone_density_ceil: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GpuDensityParams {
    world_seed_lo: u32,
    world_seed_hi: u32,
    axis_len: u32,
    _pad0: u32,
    root_origin_x: f32,
    root_origin_y: f32,
    root_origin_z: f32,
    local_unit_scale: f32,
    coarse_span_units: f32,
    detail_span_units: f32,
    coarse_weight: f32,
    detail_weight: f32,
    bias: f32,
    gain: f32,
    center: f32,
    _pad1: f32,
    seed_salt_coarse_lo: u32,
    seed_salt_coarse_hi: u32,
    seed_salt_detail_lo: u32,
    seed_salt_detail_hi: u32,
    zone_density_multiplier: f32,
    zone_density_offset: f32,
    zone_density_floor: f32,
    zone_density_ceil: f32,
}

struct GpuDensityContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    bind_group_layout: wgpu::BindGroupLayout,
    pipeline: wgpu::ComputePipeline,
    dispatch_lock: Mutex<()>,
}

pub fn sample_density_field(
    world_seed: u64,
    chunk_scale: Scale,
    canonical_coord: &GridVec,
    axis_samples: &[u16],
    semantics: GpuDensitySemantics,
) -> Result<Vec<u8>, String> {
    if axis_samples.is_empty() {
        return Ok(Vec::new());
    }

    let axis_offsets = axis_samples.iter().map(|sample| *sample as f32 - HALF_CHUNK_SPAN_F32).collect::<Vec<_>>();
    let axis_len = axis_offsets.len();
    let total_points = axis_len
        .checked_mul(axis_len)
        .and_then(|value| value.checked_mul(axis_len))
        .ok_or_else(|| format!("GPU density axis overflow for axis_len={axis_len}"))?;
    let output_raw_bytes = total_points
        .checked_mul(std::mem::size_of::<u32>())
        .ok_or_else(|| format!("GPU density output byte overflow for total_points={total_points}"))?;
    let output_padded_bytes = align_up(output_raw_bytes, MAP_ALIGNMENT_BYTES);

    let (root_origin_x, root_origin_y, root_origin_z) = root_chunk_origin_native(canonical_coord);
    let local_unit_scale = local_to_root_unit_scale(chunk_scale);
    let params = GpuDensityParams {
        world_seed_lo: (world_seed & 0xffff_ffff) as u32,
        world_seed_hi: (world_seed >> 32) as u32,
        axis_len: axis_len as u32,
        _pad0: 0,
        root_origin_x,
        root_origin_y,
        root_origin_z,
        local_unit_scale,
        coarse_span_units: semantics.coarse_span_units.max(1e-6),
        detail_span_units: semantics.detail_span_units.max(1e-6),
        coarse_weight: semantics.coarse_weight.max(0.0),
        detail_weight: semantics.detail_weight.max(0.0),
        bias: semantics.bias,
        gain: semantics.gain,
        center: semantics.center,
        _pad1: 0.0,
        seed_salt_coarse_lo: (semantics.seed_salt_coarse & 0xffff_ffff) as u32,
        seed_salt_coarse_hi: (semantics.seed_salt_coarse >> 32) as u32,
        seed_salt_detail_lo: (semantics.seed_salt_detail & 0xffff_ffff) as u32,
        seed_salt_detail_hi: (semantics.seed_salt_detail >> 32) as u32,
        zone_density_multiplier: semantics.zone_density_multiplier,
        zone_density_offset: semantics.zone_density_offset,
        zone_density_floor: semantics.zone_density_floor.min(semantics.zone_density_ceil),
        zone_density_ceil: semantics.zone_density_ceil.max(semantics.zone_density_floor),
    };

    let context = gpu_density_context()?;
    context.sample(params, &axis_offsets, total_points, output_raw_bytes, output_padded_bytes)
}

fn gpu_density_context() -> Result<&'static GpuDensityContext, String> {
    match GPU_DENSITY_CONTEXT.get_or_init(init_gpu_density_context) {
        Ok(context) => Ok(context),
        Err(error) => Err(error.clone()),
    }
}

fn init_gpu_density_context() -> Result<GpuDensityContext, String> {
    futures::executor::block_on(async move {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .map_err(|error| format!("request_adapter failed: {error}"))?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("USF Manifestation Density GPU Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(|error| format!("request_device failed: {error}"))?;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("USF Manifestation Density Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(GPU_DENSITY_SHADER)),
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("USF Manifestation Density Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(wgpu::BufferSize::new(std::mem::size_of::<GpuDensityParams>() as u64).expect("Uniform size must be > 0")),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("USF Manifestation Density Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("USF Manifestation Density Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        Ok(GpuDensityContext {
            device,
            queue,
            bind_group_layout,
            pipeline,
            dispatch_lock: Mutex::new(()),
        })
    })
}

impl GpuDensityContext {
    fn sample(
        &self,
        params: GpuDensityParams,
        axis_offsets: &[f32],
        total_points: usize,
        output_raw_bytes: usize,
        output_padded_bytes: usize,
    ) -> Result<Vec<u8>, String> {
        let _dispatch_guard = self.dispatch_lock.lock().map_err(|_| "GPU density dispatch lock poisoned".to_string())?;

        let params_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("USF Manifestation Density Params Buffer"),
            size: std::mem::size_of::<GpuDensityParams>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&params_buffer, 0, bytemuck::bytes_of(&params));

        let axis_buffer_bytes = bytemuck::cast_slice(axis_offsets);
        let axis_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("USF Manifestation Density Axis Buffer"),
            size: axis_buffer_bytes.len() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&axis_buffer, 0, axis_buffer_bytes);

        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("USF Manifestation Density Output Buffer"),
            size: output_padded_bytes as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let readback_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("USF Manifestation Density Readback Buffer"),
            size: output_padded_bytes as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("USF Manifestation Density Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: axis_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: output_buffer.as_entire_binding(),
                },
            ],
        });

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("USF Manifestation Density Command Encoder"),
        });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("USF Manifestation Density Compute Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            let workgroups_x = ((total_points as u32).saturating_add(WORKGROUP_SIZE - 1)) / WORKGROUP_SIZE;
            compute_pass.dispatch_workgroups(workgroups_x.max(1), 1, 1);
        }
        encoder.copy_buffer_to_buffer(&output_buffer, 0, &readback_buffer, 0, output_padded_bytes as u64);
        self.queue.submit(std::iter::once(encoder.finish()));

        let (map_sender, map_receiver) = std::sync::mpsc::channel();
        let readback_slice = readback_buffer.slice(..);
        readback_slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = map_sender.send(result);
        });

        self.device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|error| format!("GPU density poll failed: {error}"))?;
        let map_result = map_receiver.recv().map_err(|error| format!("GPU density map callback failed: {error}"))?;
        map_result.map_err(|error| format!("GPU density map_async failed: {error}"))?;

        let mapped_bytes = readback_slice.get_mapped_range();
        let words = bytemuck::cast_slice::<u8, u32>(&mapped_bytes[..output_raw_bytes]);
        let densities = words.iter().map(|value| *value as u8).collect::<Vec<_>>();
        drop(mapped_bytes);
        readback_buffer.unmap();

        Ok(densities)
    }
}

fn align_up(value: usize, alignment: usize) -> usize {
    if alignment == 0 {
        return value;
    }
    let remainder = value % alignment;
    if remainder == 0 { value } else { value + (alignment - remainder) }
}

fn local_to_root_unit_scale(chunk_scale: Scale) -> f32 {
    10.0_f32.powi(-(chunk_scale.index_from_top() as i32))
}

fn root_chunk_origin_native(canonical_coord: &GridVec) -> (f32, f32, f32) {
    let mut origin_x = 0.0_f64;
    let mut origin_y = 0.0_f64;
    let mut origin_z = 0.0_f64;
    let mut weight = CHUNK_SPAN_UNITS_F32 as f64;

    for digit in canonical_coord.to_raw_vec_3d() {
        origin_x += digit.x as f64 * weight;
        origin_y += digit.y as f64 * weight;
        origin_z += digit.z as f64 * weight;
        weight /= 10.0;
    }

    (origin_x as f32, origin_y as f32, origin_z as f32)
}

const GPU_DENSITY_SHADER: &str = r#"
struct Params {
    world_seed_lo: u32,
    world_seed_hi: u32,
    axis_len: u32,
    _pad0: u32,
    root_origin_x: f32,
    root_origin_y: f32,
    root_origin_z: f32,
    local_unit_scale: f32,
    coarse_span_units: f32,
    detail_span_units: f32,
    coarse_weight: f32,
    detail_weight: f32,
    bias: f32,
    gain: f32,
    center: f32,
    _pad1: f32,
    seed_salt_coarse_lo: u32,
    seed_salt_coarse_hi: u32,
    seed_salt_detail_lo: u32,
    seed_salt_detail_hi: u32,
    zone_density_multiplier: f32,
    zone_density_offset: f32,
    zone_density_floor: f32,
    zone_density_ceil: f32,
};

@group(0) @binding(0)
var<uniform> params: Params;
@group(0) @binding(1)
var<storage, read> axis_offsets: array<f32>;
@group(0) @binding(2)
var<storage, read_write> out_density: array<u32>;

const ROOT_AXIS_PERIOD_UNITS: f32 = 1000.0 * 10.0;

fn mix32(value: u32) -> u32 {
    var state = value;
    state = state ^ (state >> 16u);
    state = state * 0x7feb352du;
    state = state ^ (state >> 15u);
    state = state * 0x846ca68bu;
    state = state ^ (state >> 16u);
    return state;
}

fn lattice_noise01(seed: u32, x: i32, y: i32, z: i32) -> f32 {
    var state = mix32(seed ^ 0x5f35d3a1u);
    state = mix32(state ^ bitcast<u32>(x));
    state = mix32(state ^ bitcast<u32>(y));
    state = mix32(state ^ bitcast<u32>(z));
    let bits24 = state >> 8u;
    return f32(bits24) * (1.0 / 16777215.0);
}

fn smoothstep01(t: f32) -> f32 {
    let clamped = clamp(t, 0.0, 1.0);
    return clamped * clamped * (3.0 - 2.0 * clamped);
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

fn value_noise_3d(seed: u32, gx: f32, gy: f32, gz: f32, cell_size: f32) -> f32 {
    let safe_cell_size = max(cell_size, 1e-6);
    let sx = gx / safe_cell_size;
    let sy = gy / safe_cell_size;
    let sz = gz / safe_cell_size;

    let cx0 = i32(floor(sx));
    let cy0 = i32(floor(sy));
    let cz0 = i32(floor(sz));
    let cx1 = cx0 + 1;
    let cy1 = cy0 + 1;
    let cz1 = cz0 + 1;

    let tx = smoothstep01(sx - f32(cx0));
    let ty = smoothstep01(sy - f32(cy0));
    let tz = smoothstep01(sz - f32(cz0));

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
    return lerp(y0, y1, tz);
}

fn wrap_root_native_axis(value: f32) -> f32 {
    return value - ROOT_AXIS_PERIOD_UNITS * floor(value / ROOT_AXIS_PERIOD_UNITS);
}

@compute @workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let axis_len = params.axis_len;
    let total_points = axis_len * axis_len * axis_len;
    let index = global_id.x;
    if index >= total_points {
        return;
    }

    let plane = axis_len * axis_len;
    let iz = index / plane;
    let rem = index - iz * plane;
    let iy = rem / axis_len;
    let ix = rem - iy * axis_len;

    let local_x = axis_offsets[ix];
    let local_y = axis_offsets[iy];
    let local_z = axis_offsets[iz];

    let gx = params.root_origin_x + local_x * params.local_unit_scale;
    let gy = params.root_origin_y + local_y * params.local_unit_scale;
    let gz = params.root_origin_z + local_z * params.local_unit_scale;

    let wx = wrap_root_native_axis(gx);
    let wy = wrap_root_native_axis(gy);
    let wz = wrap_root_native_axis(gz);

    let world_seed = mix32(params.world_seed_lo ^ params.world_seed_hi);
    let coarse_seed = mix32(world_seed ^ params.seed_salt_coarse_lo ^ params.seed_salt_coarse_hi);
    let detail_seed = mix32(world_seed ^ params.seed_salt_detail_lo ^ params.seed_salt_detail_hi);

    let base = value_noise_3d(coarse_seed, wx, wy, wz, params.coarse_span_units);
    let detail = value_noise_3d(detail_seed, wx, wy, wz, params.detail_span_units);
    let weight_sum = max(params.coarse_weight + params.detail_weight, 1e-6);
    let combined = ((base * params.coarse_weight) + (detail * params.detail_weight)) / weight_sum;
    let shaped = clamp((combined - params.bias) * params.gain + params.center, 0.0, 1.0);
    let zoned_raw = shaped * params.zone_density_multiplier + params.zone_density_offset;
    let zoned = clamp(
        zoned_raw,
        min(params.zone_density_floor, params.zone_density_ceil),
        max(params.zone_density_floor, params.zone_density_ceil),
    );

    out_density[index] = u32(round(zoned * 255.0));
}
"#;
