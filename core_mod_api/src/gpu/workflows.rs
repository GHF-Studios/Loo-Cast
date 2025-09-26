use core_mod_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Gpu",
    workflows: [
        SetupTextureGenerator, timeout_secs: 1.0, timeout_mode: RealTime {
            user_imports: {
                use bevy::prelude::{Handle, Shader, Res, ResMut, Assets};
                use bevy::ecs::system::SystemState;
                use bevy::render::render_resource::{
                    BindGroupLayout, CachedComputePipelineId,
                    PipelineCache, BindGroupLayoutEntry, ShaderStages,
                    BindingType, StorageTextureAccess, TextureFormat,
                    TextureViewDimension, BufferBindingType, PushConstantRange,
                    CachedPipelineState, Pipeline, ComputePipelineDescriptor
                };
                use bevy::render::render_asset::RenderAssets;
                use bevy::render::renderer::RenderDevice;
                use std::num::NonZeroU64;
                use std::path::PathBuf;

                use crate::core::functions::asset_root;
                use crate::gpu::resources::ShaderRegistry;
                use crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams;
            },
            user_items: {},
            stages: [
                SetupPhase1: Ecs, run_if_paused: true, run_after_startup_finished: false {
                    core_types: [
                        struct MainAccess<'w> {
                            shader_assets: ResMut<'w, Assets<Shader>>,
                            shader_registry: Res<'w, ShaderRegistry>,
                        }
                        struct Input {
                            shader_name: &'static str,
                            shader_path: String
                        }
                        struct Output {
                            shader_name: &'static str,
                            shader_handle: Handle<Shader>,
                        }
                        enum Error {
                            ShaderAlreadyRegistered {
                                shader_name: &'static str
                            },
                            FailedToReadShader {
                                shader_name: &'static str,
                                shader_path: String,
                                error: std::io::Error
                            }
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Result<Output, Error> {
                            let shader_name = input.shader_name;
                            let shader_path = &input.shader_path;

                            let mut shader_assets = main_access.shader_assets;
                            let shader_registry = main_access.shader_registry;

                            if shader_registry.shaders.contains_key(shader_name) {
                                return Err(Error::ShaderAlreadyRegistered { shader_name })
                            }

                            let mut abs_shader_path = PathBuf::from(shader_path);
                            if abs_shader_path.is_relative() {
                                abs_shader_path = asset_root().join(shader_path);
                            }
                            let shader_path = abs_shader_path.to_string_lossy().to_string();

                            let shader_source = std::fs::read_to_string(shader_path.clone())
                                .map_err(|e| Error::FailedToReadShader { shader_path: shader_path.clone(), shader_name, error: e })?;

                            let shader = Shader::from_wgsl(shader_source, shader_path);
                            let shader_handle = shader_assets.add(shader);

                            Ok(Output { shader_name, shader_handle })
                        }
                    ]
                }

                SetupPhase2: RenderWhile, run_if_paused: true, run_after_startup_finished: false {
                    core_types: [
                        struct RenderAccess<'w> {
                            render_device: Res<'w, RenderDevice>,
                            pipeline_cache: Res<'w, PipelineCache>,
                        }
                        struct Input {
                            shader_name: &'static str,
                            shader_handle: Handle<Shader>
                        }
                        struct State {
                            shader_name: &'static str,
                            shader_handle: Handle<Shader>,
                            bind_group_layout: BindGroupLayout,
                            pipeline_id: CachedComputePipelineId,
                        }
                        struct Output {
                            shader_name: &'static str,
                            shader_handle: Handle<Shader>,
                            pipeline_id: CachedComputePipelineId,
                            bind_group_layout: BindGroupLayout,
                        }
                        enum Error {
                            ExpectedComputePipelineGotRenderPipeline {
                                shader_name: String,
                                pipeline_id: CachedComputePipelineId,
                            },
                            FailedToCreatePipeline {
                                shader_name: &'static str,
                                pipeline_cache_err: String,
                            }
                        }
                    ],
                    core_functions: [
                        fn SetupRenderWhile |input, render_access| -> Result<State, Error> {
                            let shader_name = input.shader_name;
                            let shader_handle = input.shader_handle;

                            let render_device = render_access.render_device;
                            let pipeline_cache = render_access.pipeline_cache;

                            let bind_group_layout = render_device.create_bind_group_layout(
                                None,
                                &[
                                    // Texture buffer
                                    BindGroupLayoutEntry {
                                        binding: 0,
                                        visibility: ShaderStages::COMPUTE,
                                        ty: BindingType::StorageTexture {
                                            access: StorageTextureAccess::WriteOnly,
                                            format: TextureFormat::Rgba8Unorm,
                                            view_dimension: TextureViewDimension::D2,
                                        },
                                        count: None,
                                    },
                                    // Param buffer
                                    BindGroupLayoutEntry {
                                        binding: 1,
                                        visibility: ShaderStages::COMPUTE,
                                        ty: BindingType::Buffer {
                                            ty: BufferBindingType::Storage { read_only: true },
                                            has_dynamic_offset: false,
                                            min_binding_size: Some(NonZeroU64::new(std::mem::size_of::<ShaderParams>() as u64).unwrap()),
                                        },
                                        count: None,
                                    },
                                ],
                            );

                            let pipeline_id = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                                label: None,
                                layout: vec![bind_group_layout.clone()],
                                shader: shader_handle.clone(),
                                shader_defs: vec![],
                                entry_point: "main".into(),
                                push_constant_ranges: vec![PushConstantRange {
                                    stages: ShaderStages::COMPUTE,
                                    range: 0..4,
                                }],
                                zero_initialize_workgroup_memory: false
                            });

                            Ok(State { shader_name, shader_handle, bind_group_layout, pipeline_id })
                        }
                        fn RunRenderWhile |state, render_access| -> Result<Outcome<State, Output>, Error> {
                            let shader_name = state.shader_name;
                            let shader_handle = state.shader_handle.clone();
                            let bind_group_layout = state.bind_group_layout.clone();
                            let pipeline_id = state.pipeline_id;

                            let pipeline_cache = render_access.pipeline_cache;

                            match pipeline_cache.get_compute_pipeline_state(pipeline_id) {
                                CachedPipelineState::Queued | CachedPipelineState::Creating(_) => {
                                    Ok(Wait(state))
                                },
                                CachedPipelineState::Err(err) => {
                                    Err(Error::FailedToCreatePipeline {
                                        shader_name,
                                        pipeline_cache_err: format!("{}", err)
                                    })
                                },
                                CachedPipelineState::Ok(pipeline) => {
                                    match pipeline {
                                        Pipeline::RenderPipeline(_) => Err(Error::ExpectedComputePipelineGotRenderPipeline {
                                            shader_name: state.shader_name.to_string(),
                                            pipeline_id: state.pipeline_id
                                        }),
                                        Pipeline::ComputePipeline(_) => Ok(Done(Output {
                                            shader_name,
                                            shader_handle,
                                            pipeline_id,
                                            bind_group_layout
                                        }))
                                    }
                                },
                            }
                        }
                    ]
                }

                SetupPhase3: Ecs, run_if_paused: true, run_after_startup_finished: false {
                    core_types: [
                        struct MainAccess<'w> {
                            shader_registry: ResMut<'w, ShaderRegistry>,
                        }
                        struct Input {
                            shader_name: &'static str,
                            shader_handle: Handle<Shader>,
                            pipeline_id: CachedComputePipelineId,
                            bind_group_layout: BindGroupLayout,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| {
                            let shader_name = input.shader_name;
                            let shader_handle = input.shader_handle;
                            let bind_group_layout = input.bind_group_layout;
                            let pipeline_id = input.pipeline_id;

                            let mut shader_registry = main_access.shader_registry;

                            shader_registry.shaders.insert(shader_name.to_string(), shader_handle);
                            shader_registry.pipelines.insert(shader_name.to_string(), pipeline_id);
                            shader_registry.bind_group_layouts.insert(shader_name.to_string(), bind_group_layout);
                        }
                    ]
                }
            ]
        }

        GenerateTextures, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::{Handle, Res, ResMut, Assets, Image};
                use bevy::render::render_resource::{
                    CachedComputePipelineId, BindGroupLayout,
                    Buffer, TextureView, TextureDescriptor, Extent3d,
                    TextureDimension, TextureFormat, TextureUsages,
                    BufferInitDescriptor, BufferUsages, CommandEncoderDescriptor,
                    ComputePassDescriptor, BindGroupEntry, BindingResource
                };
                use bevy::ecs::system::SystemState;
                use bevy::render::render_asset::RenderAssets;
                use bevy::image::ImageSampler;
                use bevy::render::texture::GpuImage;
                use bevy::render::renderer::{RenderDevice, RenderQueue};
                use bevy::render::render_resource::PipelineCache;
                use crossbeam_channel::Receiver;

                use crate::config::statics::CONFIG;
                use crate::gpu::resources::ShaderRegistry;
            },
            user_items: {
                #[repr(C, align(16))]
                #[derive(Clone, Copy, Debug)]
                pub struct ShaderParams {
                    pub chunk_pos: [i32; 2],           // 8 bytes
                    pub chunk_size: u32,               // 4 bytes
                    pub chunk_scale: i32,              // 4 bytes
                    pub current_view_scale: i32,       // 4 bytes
                    pub _padding0: u32,                // Pad to next 16-byte boundary
                    pub _padding1: [u32; 4],           // Add 16 more to reach 48
                }
                unsafe impl bytemuck::Pod for ShaderParams {}
                unsafe impl bytemuck::Zeroable for ShaderParams {}

                pub struct BatchedGeneratorParams {
                    pub shader_name: &'static str,
                    pub pipeline_id: CachedComputePipelineId,
                    pub bind_group_layout: BindGroupLayout,
                    pub texture_size: u32,
                    pub texture_handles: Vec<Handle<Image>>,
                    pub param_buffers: Vec<Buffer>,
                }

                pub struct PreparedBatchedGenerator {
                    pub shader_name: &'static str,
                    pub pipeline_id: CachedComputePipelineId,
                    pub bind_group_layout: BindGroupLayout,
                    pub texture_size: u32,
                    pub texture_handles: Vec<Handle<Image>>,
                    pub texture_views: Vec<TextureView>,
                    pub param_buffers: Vec<Buffer>,
                }

                pub struct DispatchedBatch {
                    pub shader_name: &'static str,
                    pub texture_handles: Vec<Handle<Image>>,
                    pub receiver: Receiver<()>
                }
            },
            stages: [
                PrepareBatch: Ecs, run_if_paused: false, run_after_startup_finished: false {
                    core_types: [
                        struct MainAccess<'w> {
                            render_device: Res<'w, RenderDevice>,
                            images: ResMut<'w, Assets<Image>>,
                            shader_registry: Res<'w, ShaderRegistry>,
                        }
                        struct Input {
                            shader_name: &'static str,
                            texture_size: u32,
                            param_data: Vec<ShaderParams>,
                        }
                        struct Output {
                            params: BatchedGeneratorParams,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Output {
                            let shader_name = input.shader_name;
                            let render_device = main_access.render_device;
                            let mut images = main_access.images;
                            let shader_registry = main_access.shader_registry;

                            let pipeline_id = *shader_registry.pipelines.get(shader_name).unwrap_or_else(|| unreachable!("Pipeline for shader '{}' not found", shader_name));
                            let bind_group_layout = shader_registry.bind_group_layouts.get(shader_name).unwrap_or_else(|| unreachable!("BindGroupLayout for shader '{}' not found", shader_name)).clone();

                            let mut texture_handles = Vec::new();
                            let mut param_buffers = Vec::new();

                            for param in &input.param_data {
                                // --- Create the texture ---
                                let texture = Image {
                                    texture_descriptor: TextureDescriptor {
                                        label: Some("Chunk Texture"),
                                        size: Extent3d {
                                            width: input.texture_size,
                                            height: input.texture_size,
                                            depth_or_array_layers: 1,
                                        },
                                        mip_level_count: 1,
                                        sample_count: 1,
                                        dimension: TextureDimension::D2,
                                        format: TextureFormat::Rgba8Unorm,
                                        usage: TextureUsages::COPY_DST
                                            | TextureUsages::TEXTURE_BINDING
                                            | TextureUsages::STORAGE_BINDING,
                                        view_formats: &[],
                                    },
                                    sampler: ImageSampler::nearest(),
                                    data: vec![0; input.texture_size as usize * input.texture_size as usize * 4].into(),
                                    ..Default::default()
                                };
                                texture_handles.push(images.add(texture));

                                // --- Create the param buffer ---
                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("Param Buffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers.push(buffer);
                            }

                            Output {
                                params: BatchedGeneratorParams {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout,
                                    texture_size: input.texture_size,
                                    texture_handles,
                                    param_buffers,
                                }
                            }
                        }
                    ]
                }

                GetTextureViews: RenderWhile, run_if_paused: false, run_after_startup_finished: false {
                    core_types: [
                        struct RenderAccess<'w> {
                            gpu_images: Res<'w, RenderAssets<GpuImage>>,
                        }
                        struct Input {
                            params: BatchedGeneratorParams,
                        }
                        struct State {
                            params: BatchedGeneratorParams,
                        }
                        struct Output {
                            prepared: PreparedBatchedGenerator,
                        }
                    ],
                    core_functions: [
                        fn SetupRenderWhile |input, render_access| -> State {
                            State { params: input.params }
                        }
                        fn RunRenderWhile |state, render_access| -> Outcome<State, Output> {
                            let mut texture_views = Vec::new();
                            let gpu_images = render_access.gpu_images;

                            for handle in &state.params.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }

                            Done(Output {
                                prepared: PreparedBatchedGenerator {
                                    shader_name: state.params.shader_name,
                                    pipeline_id: state.params.pipeline_id,
                                    bind_group_layout: state.params.bind_group_layout,
                                    texture_size: state.params.texture_size,
                                    texture_handles: state.params.texture_handles,
                                    texture_views,
                                    param_buffers: state.params.param_buffers,
                                }
                            })
                        }
                    ]
                }

                DispatchBatch: Render, run_if_paused: false, run_after_startup_finished: false {
                    core_types: [
                        struct RenderAccess<'w> {
                            render_device: Res<'w, RenderDevice>,
                            queue: Res<'w, RenderQueue>,
                            pipeline_cache: Res<'w, PipelineCache>,
                        }
                        struct Input {
                            prepared: PreparedBatchedGenerator,
                        }
                        struct Output {
                            dispatched: DispatchedBatch,
                        }
                    ],
                    core_functions: [
                        fn RunRender |input, render_access| -> Output {
                            let prepared = &input.prepared;
                            let render_device = render_access.render_device;
                            let queue = render_access.queue;
                            let pipeline = render_access.pipeline_cache
                                .get_compute_pipeline(prepared.pipeline_id)
                                .expect("Pipeline missing");

                            let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor { label: None });

                            for (view, buffer) in prepared.texture_views.iter().zip(&prepared.param_buffers) {
                                let bind_group = render_device.create_bind_group(
                                    Some("Compute Bind Group"),
                                    &prepared.bind_group_layout,
                                    &[
                                        BindGroupEntry { binding: 0, resource: BindingResource::TextureView(view) },
                                        BindGroupEntry { binding: 1, resource: buffer.as_entire_binding() },
                                    ]
                                );

                                let texture_size = prepared.texture_size;
                                let (width, height) = (texture_size, texture_size);
                                let workgroup_size_x = CONFIG().get::<u32>("gpu/texture_generator/workgroup_size_x");
                                let workgroup_size_y = CONFIG().get::<u32>("gpu/texture_generator/workgroup_size_y");

                                let dispatch_x = width.div_ceil(workgroup_size_x);
                                let dispatch_y = height.div_ceil(workgroup_size_y);

                                let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor { label: None, timestamp_writes: None });
                                compute_pass.set_pipeline(pipeline);
                                compute_pass.set_bind_group(0, &bind_group, &[]);
                                compute_pass.dispatch_workgroups(dispatch_x, dispatch_y, 1);
                                drop(compute_pass);
                            }

                            queue.submit(Some(encoder.finish()));

                            let (sender, receiver) = crossbeam_channel::unbounded();
                            queue.on_submitted_work_done(move || {
                                let _ = sender.send(());
                            });

                            Output {
                                dispatched: DispatchedBatch {
                                    shader_name: prepared.shader_name,
                                    texture_handles: prepared.texture_handles.clone(),
                                    receiver
                                }
                            }
                        }
                    ]
                }

                WaitForBatch: EcsWhile, run_if_paused: false, run_after_startup_finished: false {
                    core_types: [
                        struct MainAccess {}
                        struct Input {
                            dispatched: DispatchedBatch,
                        }
                        struct State {
                            dispatched: DispatchedBatch,
                        }
                        struct Output {
                            shader_name: &'static str,
                            texture_handles: Vec<Handle<Image>>,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> State {
                            State { dispatched: input.dispatched }
                        }
                        fn RunEcsWhile |state, main_access| -> Outcome<State, Output> {
                            match state.dispatched.receiver.try_recv() {
                                Ok(_) => Done(Output {
                                    shader_name: state.dispatched.shader_name,
                                    texture_handles: state.dispatched.texture_handles.clone(),
                                }),
                                Err(crossbeam_channel::TryRecvError::Empty) => Wait(state),
                                Err(_) => unreachable!("GPU dispatch failed"),
                            }
                        }
                    ]
                }
            ]
        }

        GenerateChunkTextures, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::{Handle, Res, ResMut, Assets, Image};
                use bevy::render::render_resource::{
                    CachedComputePipelineId, BindGroupLayout, TextureView,
                    Buffer, BufferInitDescriptor, BufferUsages, BindingResource,
                    TextureDescriptor, Extent3d, TextureDimension, TextureFormat, TextureUsages,
                    CommandEncoderDescriptor, ComputePassDescriptor, BindGroupEntry,
                };
                use bevy::render::renderer::{RenderDevice, RenderQueue};
                use bevy::render::render_asset::RenderAssets;
                use bevy::image::ImageSampler;
                use bevy::render::render_resource::PipelineCache;
                use bevy::render::texture::GpuImage;
                use crossbeam_channel::Receiver;

                use crate::config::statics::CONFIG;
                use crate::gpu::resources::ShaderRegistry;
                use crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams;
                use crate::utils::progress::Progress;
            },

            user_items: {
                #[derive(Clone)]
                pub struct PreparedRenderExecutorInput {
                    pub shader_name: &'static str,
                    pub pipeline_id: CachedComputePipelineId,
                    pub bind_group_layout: BindGroupLayout,
                    pub texture_size: u32,
                    pub texture_handles: Vec<Handle<Image>>,
                    pub param_buffers: Vec<Buffer>,
                }

                #[derive(Clone)]
                pub struct ChunkRenderExecutor {
                    pub shader_name: &'static str,
                    pub pipeline_id: CachedComputePipelineId,
                    pub bind_group_layout: BindGroupLayout,
                    pub texture_size: u32,
                    pub texture_handles: Vec<Handle<Image>>,
                    pub param_buffers: Vec<Buffer>,
                    pub texture_views: Vec<TextureView>,
                    pub receiver: Option<Receiver<()>>,
                }

                pub fn new_chunk_texture(texture_size: u32) -> Image {
                    Image {
                        texture_descriptor: TextureDescriptor {
                            label: Some("RenderTexture"),
                            size: Extent3d {
                                width: texture_size,
                                height: texture_size,
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: TextureDimension::D2,
                            format: TextureFormat::Rgba8Unorm,
                            usage: TextureUsages::COPY_DST
                                | TextureUsages::TEXTURE_BINDING
                                | TextureUsages::STORAGE_BINDING,
                            view_formats: &[],
                        },
                        sampler: ImageSampler::nearest(),
                        data: vec![0; (texture_size * texture_size * 4) as usize].into(),
                        ..Default::default()
                    }
                }
            },

            stages: [
                PrepareRenderExecutor: Ecs, run_if_paused: false, run_after_startup_finished: false {
                    core_types: [
                        struct MainAccess<'w> {
                            render_device: Res<'w, RenderDevice>,
                            images: ResMut<'w, Assets<Image>>,
                            shader_registry: Res<'w, ShaderRegistry>,
                        }
                        struct Input {
                            shader_name: &'static str,
                            texture_size: u32,
                            param_data_scale_quecto_meter_000001: Vec<ShaderParams>,
                            param_data_scale_quecto_meter_00001: Vec<ShaderParams>,
                            param_data_scale_quecto_meter_0001: Vec<ShaderParams>,
                            param_data_scale_quecto_meter_001: Vec<ShaderParams>,
                            param_data_scale_quecto_meter_01: Vec<ShaderParams>,
                            param_data_scale_quecto_meter_1: Vec<ShaderParams>,
                            param_data_scale_quecto_meter_10: Vec<ShaderParams>,
                            param_data_scale_quecto_meter_100: Vec<ShaderParams>,
                            param_data_scale_ronto_meter_1: Vec<ShaderParams>,
                            param_data_scale_ronto_meter_10: Vec<ShaderParams>,
                            param_data_scale_ronto_meter_100: Vec<ShaderParams>,
                            param_data_scale_yocto_meter_1: Vec<ShaderParams>,
                            param_data_scale_yocto_meter_10: Vec<ShaderParams>,
                            param_data_scale_yocto_meter_100: Vec<ShaderParams>,
                            param_data_scale_zepto_meter_1: Vec<ShaderParams>,
                            param_data_scale_zepto_meter_10: Vec<ShaderParams>,
                            param_data_scale_zepto_meter_100: Vec<ShaderParams>,
                            param_data_scale_atto_meter_1: Vec<ShaderParams>,
                            param_data_scale_atto_meter_10: Vec<ShaderParams>,
                            param_data_scale_atto_meter_100: Vec<ShaderParams>,
                            param_data_scale_femto_meter_1: Vec<ShaderParams>,
                            param_data_scale_femto_meter_10: Vec<ShaderParams>,
                            param_data_scale_femto_meter_100: Vec<ShaderParams>,
                            param_data_scale_pico_meter_1: Vec<ShaderParams>,
                            param_data_scale_pico_meter_10: Vec<ShaderParams>,
                            param_data_scale_pico_meter_100: Vec<ShaderParams>,
                            param_data_scale_nano_meter_1: Vec<ShaderParams>,
                            param_data_scale_nano_meter_10: Vec<ShaderParams>,
                            param_data_scale_nano_meter_100: Vec<ShaderParams>,
                            param_data_scale_micro_meter_1: Vec<ShaderParams>,
                            param_data_scale_micro_meter_10: Vec<ShaderParams>,
                            param_data_scale_micro_meter_100: Vec<ShaderParams>,
                            param_data_scale_milli_meter_1: Vec<ShaderParams>,
                            param_data_scale_milli_meter_10: Vec<ShaderParams>,
                            param_data_scale_milli_meter_100: Vec<ShaderParams>,
                            param_data_scale_meter_1: Vec<ShaderParams>,
                            param_data_scale_meter_10: Vec<ShaderParams>,
                            param_data_scale_meter_100: Vec<ShaderParams>,
                            param_data_scale_kilo_meter_1: Vec<ShaderParams>,
                            param_data_scale_kilo_meter_10: Vec<ShaderParams>,
                            param_data_scale_kilo_meter_100: Vec<ShaderParams>,
                            param_data_scale_mega_meter_1: Vec<ShaderParams>,
                            param_data_scale_mega_meter_10: Vec<ShaderParams>,
                            param_data_scale_mega_meter_100: Vec<ShaderParams>,
                            param_data_scale_giga_meter_1: Vec<ShaderParams>,
                            param_data_scale_giga_meter_10: Vec<ShaderParams>,
                            param_data_scale_giga_meter_100: Vec<ShaderParams>,
                            param_data_scale_tera_meter_1: Vec<ShaderParams>,
                            param_data_scale_tera_meter_10: Vec<ShaderParams>,
                            param_data_scale_tera_meter_100: Vec<ShaderParams>,
                            param_data_scale_peta_meter_1: Vec<ShaderParams>,
                            param_data_scale_peta_meter_10: Vec<ShaderParams>,
                            param_data_scale_peta_meter_100: Vec<ShaderParams>,
                            param_data_scale_exa_meter_1: Vec<ShaderParams>,
                            param_data_scale_exa_meter_10: Vec<ShaderParams>,
                            param_data_scale_exa_meter_100: Vec<ShaderParams>,
                            param_data_scale_zetta_meter_1: Vec<ShaderParams>,
                            param_data_scale_zetta_meter_10: Vec<ShaderParams>,
                            param_data_scale_zetta_meter_100: Vec<ShaderParams>,
                            param_data_scale_yotta_meter_1: Vec<ShaderParams>,
                            param_data_scale_yotta_meter_10: Vec<ShaderParams>,
                            param_data_scale_yotta_meter_100: Vec<ShaderParams>,
                            param_data_scale_ronna_meter_1: Vec<ShaderParams>,
                            param_data_scale_ronna_meter_10: Vec<ShaderParams>,
                            param_data_scale_ronna_meter_100: Vec<ShaderParams>,
                            param_data_scale_quetta_meter_1: Vec<ShaderParams>,
                            param_data_scale_quetta_meter_10: Vec<ShaderParams>,
                            param_data_scale_quetta_meter_100: Vec<ShaderParams>,
                            param_data_scale_quetta_meter_1000: Vec<ShaderParams>,
                            param_data_scale_quetta_meter_10000: Vec<ShaderParams>,
                            param_data_scale_quetta_meter_100000: Vec<ShaderParams>,
                        }
                        struct Output {
                            params_scale_quecto_meter_000001: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_00001: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_0001: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_001: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_01: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_1: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_10: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_100: PreparedRenderExecutorInput,
                            params_scale_ronto_meter_1: PreparedRenderExecutorInput,
                            params_scale_ronto_meter_10: PreparedRenderExecutorInput,
                            params_scale_ronto_meter_100: PreparedRenderExecutorInput,
                            params_scale_yocto_meter_1: PreparedRenderExecutorInput,
                            params_scale_yocto_meter_10: PreparedRenderExecutorInput,
                            params_scale_yocto_meter_100: PreparedRenderExecutorInput,
                            params_scale_zepto_meter_1: PreparedRenderExecutorInput,
                            params_scale_zepto_meter_10: PreparedRenderExecutorInput,
                            params_scale_zepto_meter_100: PreparedRenderExecutorInput,
                            params_scale_atto_meter_1: PreparedRenderExecutorInput,
                            params_scale_atto_meter_10: PreparedRenderExecutorInput,
                            params_scale_atto_meter_100: PreparedRenderExecutorInput,
                            params_scale_femto_meter_1: PreparedRenderExecutorInput,
                            params_scale_femto_meter_10: PreparedRenderExecutorInput,
                            params_scale_femto_meter_100: PreparedRenderExecutorInput,
                            params_scale_pico_meter_1: PreparedRenderExecutorInput,
                            params_scale_pico_meter_10: PreparedRenderExecutorInput,
                            params_scale_pico_meter_100: PreparedRenderExecutorInput,
                            params_scale_nano_meter_1: PreparedRenderExecutorInput,
                            params_scale_nano_meter_10: PreparedRenderExecutorInput,
                            params_scale_nano_meter_100: PreparedRenderExecutorInput,
                            params_scale_micro_meter_1: PreparedRenderExecutorInput,
                            params_scale_micro_meter_10: PreparedRenderExecutorInput,
                            params_scale_micro_meter_100: PreparedRenderExecutorInput,
                            params_scale_milli_meter_1: PreparedRenderExecutorInput,
                            params_scale_milli_meter_10: PreparedRenderExecutorInput,
                            params_scale_milli_meter_100: PreparedRenderExecutorInput,
                            params_scale_meter_1: PreparedRenderExecutorInput,
                            params_scale_meter_10: PreparedRenderExecutorInput,
                            params_scale_meter_100: PreparedRenderExecutorInput,
                            params_scale_kilo_meter_1: PreparedRenderExecutorInput,
                            params_scale_kilo_meter_10: PreparedRenderExecutorInput,
                            params_scale_kilo_meter_100: PreparedRenderExecutorInput,
                            params_scale_mega_meter_1: PreparedRenderExecutorInput,
                            params_scale_mega_meter_10: PreparedRenderExecutorInput,
                            params_scale_mega_meter_100: PreparedRenderExecutorInput,
                            params_scale_giga_meter_1: PreparedRenderExecutorInput,
                            params_scale_giga_meter_10: PreparedRenderExecutorInput,
                            params_scale_giga_meter_100: PreparedRenderExecutorInput,
                            params_scale_tera_meter_1: PreparedRenderExecutorInput,
                            params_scale_tera_meter_10: PreparedRenderExecutorInput,
                            params_scale_tera_meter_100: PreparedRenderExecutorInput,
                            params_scale_peta_meter_1: PreparedRenderExecutorInput,
                            params_scale_peta_meter_10: PreparedRenderExecutorInput,
                            params_scale_peta_meter_100: PreparedRenderExecutorInput,
                            params_scale_exa_meter_1: PreparedRenderExecutorInput,
                            params_scale_exa_meter_10: PreparedRenderExecutorInput,
                            params_scale_exa_meter_100: PreparedRenderExecutorInput,
                            params_scale_zetta_meter_1: PreparedRenderExecutorInput,
                            params_scale_zetta_meter_10: PreparedRenderExecutorInput,
                            params_scale_zetta_meter_100: PreparedRenderExecutorInput,
                            params_scale_yotta_meter_1: PreparedRenderExecutorInput,
                            params_scale_yotta_meter_10: PreparedRenderExecutorInput,
                            params_scale_yotta_meter_100: PreparedRenderExecutorInput,
                            params_scale_ronna_meter_1: PreparedRenderExecutorInput,
                            params_scale_ronna_meter_10: PreparedRenderExecutorInput,
                            params_scale_ronna_meter_100: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_1: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_10: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_100: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_1000: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_10000: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_100000: PreparedRenderExecutorInput,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Output {
                            let shader_name = input.shader_name;
                            let texture_size = input.texture_size;
                            let render_device = main_access.render_device;
                            let mut images = main_access.images;
                            let shader_registry = main_access.shader_registry;

                            let pipeline_id = *shader_registry.pipelines.get(shader_name)
                                .unwrap_or_else(|| unreachable!("Pipeline for shader '{}' not found", shader_name));
                            let bind_group_layout = shader_registry.bind_group_layouts.get(shader_name)
                                .unwrap_or_else(|| unreachable!("BindGroupLayout for shader '{}' not found", shader_name))
                                .clone();

                            let mut texture_handles_scale_quecto_meter_000001 = Vec::new();
                            let mut param_buffers_scale_quecto_meter_000001 = Vec::new();
                            for param in &input.param_data_scale_quecto_meter_000001 {
                                texture_handles_scale_quecto_meter_000001.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quecto_meter_000001.push(buffer);
                            }
                            let mut texture_handles_scale_quecto_meter_00001 = Vec::new();
                            let mut param_buffers_scale_quecto_meter_00001 = Vec::new();
                            for param in &input.param_data_scale_quecto_meter_00001 {
                                texture_handles_scale_quecto_meter_00001.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quecto_meter_00001.push(buffer);
                            }
                            let mut texture_handles_scale_quecto_meter_0001 = Vec::new();
                            let mut param_buffers_scale_quecto_meter_0001 = Vec::new();
                            for param in &input.param_data_scale_quecto_meter_0001 {
                                texture_handles_scale_quecto_meter_0001.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quecto_meter_0001.push(buffer);
                            }
                            let mut texture_handles_scale_quecto_meter_001 = Vec::new();
                            let mut param_buffers_scale_quecto_meter_001 = Vec::new();
                            for param in &input.param_data_scale_quecto_meter_001 {
                                texture_handles_scale_quecto_meter_001.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quecto_meter_001.push(buffer);
                            }
                            let mut texture_handles_scale_quecto_meter_01 = Vec::new();
                            let mut param_buffers_scale_quecto_meter_01 = Vec::new();
                            for param in &input.param_data_scale_quecto_meter_01 {
                                texture_handles_scale_quecto_meter_01.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quecto_meter_01.push(buffer);
                            }
                            let mut texture_handles_scale_quecto_meter_1 = Vec::new();
                            let mut param_buffers_scale_quecto_meter_1 = Vec::new();
                            for param in &input.param_data_scale_quecto_meter_1 {
                                texture_handles_scale_quecto_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quecto_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_quecto_meter_10 = Vec::new();
                            let mut param_buffers_scale_quecto_meter_10 = Vec::new();
                            for param in &input.param_data_scale_quecto_meter_10 {
                                texture_handles_scale_quecto_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quecto_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_quecto_meter_100 = Vec::new();
                            let mut param_buffers_scale_quecto_meter_100 = Vec::new();
                            for param in &input.param_data_scale_quecto_meter_100 {
                                texture_handles_scale_quecto_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quecto_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_ronto_meter_1 = Vec::new();
                            let mut param_buffers_scale_ronto_meter_1 = Vec::new();
                            for param in &input.param_data_scale_ronto_meter_1 {
                                texture_handles_scale_ronto_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_ronto_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_ronto_meter_10 = Vec::new();
                            let mut param_buffers_scale_ronto_meter_10 = Vec::new();
                            for param in &input.param_data_scale_ronto_meter_10 {
                                texture_handles_scale_ronto_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_ronto_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_ronto_meter_100 = Vec::new();
                            let mut param_buffers_scale_ronto_meter_100 = Vec::new();
                            for param in &input.param_data_scale_ronto_meter_100 {
                                texture_handles_scale_ronto_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_ronto_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_yocto_meter_1 = Vec::new();
                            let mut param_buffers_scale_yocto_meter_1 = Vec::new();
                            for param in &input.param_data_scale_yocto_meter_1 {
                                texture_handles_scale_yocto_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_yocto_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_yocto_meter_10 = Vec::new();
                            let mut param_buffers_scale_yocto_meter_10 = Vec::new();
                            for param in &input.param_data_scale_yocto_meter_10 {
                                texture_handles_scale_yocto_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_yocto_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_yocto_meter_100 = Vec::new();
                            let mut param_buffers_scale_yocto_meter_100 = Vec::new();
                            for param in &input.param_data_scale_yocto_meter_100 {
                                texture_handles_scale_yocto_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_yocto_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_zepto_meter_1 = Vec::new();
                            let mut param_buffers_scale_zepto_meter_1 = Vec::new();
                            for param in &input.param_data_scale_zepto_meter_1 {
                                texture_handles_scale_zepto_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_zepto_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_zepto_meter_10 = Vec::new();
                            let mut param_buffers_scale_zepto_meter_10 = Vec::new();
                            for param in &input.param_data_scale_zepto_meter_10 {
                                texture_handles_scale_zepto_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_zepto_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_zepto_meter_100 = Vec::new();
                            let mut param_buffers_scale_zepto_meter_100 = Vec::new();
                            for param in &input.param_data_scale_zepto_meter_100 {
                                texture_handles_scale_zepto_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_zepto_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_atto_meter_1 = Vec::new();
                            let mut param_buffers_scale_atto_meter_1 = Vec::new();
                            for param in &input.param_data_scale_atto_meter_1 {
                                texture_handles_scale_atto_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_atto_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_atto_meter_10 = Vec::new();
                            let mut param_buffers_scale_atto_meter_10 = Vec::new();
                            for param in &input.param_data_scale_atto_meter_10 {
                                texture_handles_scale_atto_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_atto_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_atto_meter_100 = Vec::new();
                            let mut param_buffers_scale_atto_meter_100 = Vec::new();
                            for param in &input.param_data_scale_atto_meter_100 {
                                texture_handles_scale_atto_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_atto_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_femto_meter_1 = Vec::new();
                            let mut param_buffers_scale_femto_meter_1 = Vec::new();
                            for param in &input.param_data_scale_femto_meter_1 {
                                texture_handles_scale_femto_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_femto_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_femto_meter_10 = Vec::new();
                            let mut param_buffers_scale_femto_meter_10 = Vec::new();
                            for param in &input.param_data_scale_femto_meter_10 {
                                texture_handles_scale_femto_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_femto_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_femto_meter_100 = Vec::new();
                            let mut param_buffers_scale_femto_meter_100 = Vec::new();
                            for param in &input.param_data_scale_femto_meter_100 {
                                texture_handles_scale_femto_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_femto_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_pico_meter_1 = Vec::new();
                            let mut param_buffers_scale_pico_meter_1 = Vec::new();
                            for param in &input.param_data_scale_pico_meter_1 {
                                texture_handles_scale_pico_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_pico_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_pico_meter_10 = Vec::new();
                            let mut param_buffers_scale_pico_meter_10 = Vec::new();
                            for param in &input.param_data_scale_pico_meter_10 {
                                texture_handles_scale_pico_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_pico_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_pico_meter_100 = Vec::new();
                            let mut param_buffers_scale_pico_meter_100 = Vec::new();
                            for param in &input.param_data_scale_pico_meter_100 {
                                texture_handles_scale_pico_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_pico_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_nano_meter_1 = Vec::new();
                            let mut param_buffers_scale_nano_meter_1 = Vec::new();
                            for param in &input.param_data_scale_nano_meter_1 {
                                texture_handles_scale_nano_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_nano_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_nano_meter_10 = Vec::new();
                            let mut param_buffers_scale_nano_meter_10 = Vec::new();
                            for param in &input.param_data_scale_nano_meter_10 {
                                texture_handles_scale_nano_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_nano_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_nano_meter_100 = Vec::new();
                            let mut param_buffers_scale_nano_meter_100 = Vec::new();
                            for param in &input.param_data_scale_nano_meter_100 {
                                texture_handles_scale_nano_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_nano_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_micro_meter_1 = Vec::new();
                            let mut param_buffers_scale_micro_meter_1 = Vec::new();
                            for param in &input.param_data_scale_micro_meter_1 {
                                texture_handles_scale_micro_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_micro_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_micro_meter_10 = Vec::new();
                            let mut param_buffers_scale_micro_meter_10 = Vec::new();
                            for param in &input.param_data_scale_micro_meter_10 {
                                texture_handles_scale_micro_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_micro_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_micro_meter_100 = Vec::new();
                            let mut param_buffers_scale_micro_meter_100 = Vec::new();
                            for param in &input.param_data_scale_micro_meter_100 {
                                texture_handles_scale_micro_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_micro_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_milli_meter_1 = Vec::new();
                            let mut param_buffers_scale_milli_meter_1 = Vec::new();
                            for param in &input.param_data_scale_milli_meter_1 {
                                texture_handles_scale_milli_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_milli_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_milli_meter_10 = Vec::new();
                            let mut param_buffers_scale_milli_meter_10 = Vec::new();
                            for param in &input.param_data_scale_milli_meter_10 {
                                texture_handles_scale_milli_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_milli_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_milli_meter_100 = Vec::new();
                            let mut param_buffers_scale_milli_meter_100 = Vec::new();
                            for param in &input.param_data_scale_milli_meter_100 {
                                texture_handles_scale_milli_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_milli_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_meter_1 = Vec::new();
                            let mut param_buffers_scale_meter_1 = Vec::new();
                            for param in &input.param_data_scale_meter_1 {
                                texture_handles_scale_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_meter_10 = Vec::new();
                            let mut param_buffers_scale_meter_10 = Vec::new();
                            for param in &input.param_data_scale_meter_10 {
                                texture_handles_scale_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_meter_100 = Vec::new();
                            let mut param_buffers_scale_meter_100 = Vec::new();
                            for param in &input.param_data_scale_meter_100 {
                                texture_handles_scale_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_kilo_meter_1 = Vec::new();
                            let mut param_buffers_scale_kilo_meter_1 = Vec::new();
                            for param in &input.param_data_scale_kilo_meter_1 {
                                texture_handles_scale_kilo_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_kilo_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_kilo_meter_10 = Vec::new();
                            let mut param_buffers_scale_kilo_meter_10 = Vec::new();
                            for param in &input.param_data_scale_kilo_meter_10 {
                                texture_handles_scale_kilo_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_kilo_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_kilo_meter_100 = Vec::new();
                            let mut param_buffers_scale_kilo_meter_100 = Vec::new();
                            for param in &input.param_data_scale_kilo_meter_100 {
                                texture_handles_scale_kilo_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_kilo_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_mega_meter_1 = Vec::new();
                            let mut param_buffers_scale_mega_meter_1 = Vec::new();
                            for param in &input.param_data_scale_mega_meter_1 {
                                texture_handles_scale_mega_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_mega_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_mega_meter_10 = Vec::new();
                            let mut param_buffers_scale_mega_meter_10 = Vec::new();
                            for param in &input.param_data_scale_mega_meter_10 {
                                texture_handles_scale_mega_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_mega_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_mega_meter_100 = Vec::new();
                            let mut param_buffers_scale_mega_meter_100 = Vec::new();
                            for param in &input.param_data_scale_mega_meter_100 {
                                texture_handles_scale_mega_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_mega_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_giga_meter_1 = Vec::new();
                            let mut param_buffers_scale_giga_meter_1 = Vec::new();
                            for param in &input.param_data_scale_giga_meter_1 {
                                texture_handles_scale_giga_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_giga_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_giga_meter_10 = Vec::new();
                            let mut param_buffers_scale_giga_meter_10 = Vec::new();
                            for param in &input.param_data_scale_giga_meter_10 {
                                texture_handles_scale_giga_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_giga_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_giga_meter_100 = Vec::new();
                            let mut param_buffers_scale_giga_meter_100 = Vec::new();
                            for param in &input.param_data_scale_giga_meter_100 {
                                texture_handles_scale_giga_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_giga_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_tera_meter_1 = Vec::new();
                            let mut param_buffers_scale_tera_meter_1 = Vec::new();
                            for param in &input.param_data_scale_tera_meter_1 {
                                texture_handles_scale_tera_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_tera_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_tera_meter_10 = Vec::new();
                            let mut param_buffers_scale_tera_meter_10 = Vec::new();
                            for param in &input.param_data_scale_tera_meter_10 {
                                texture_handles_scale_tera_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_tera_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_tera_meter_100 = Vec::new();
                            let mut param_buffers_scale_tera_meter_100 = Vec::new();
                            for param in &input.param_data_scale_tera_meter_100 {
                                texture_handles_scale_tera_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_tera_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_peta_meter_1 = Vec::new();
                            let mut param_buffers_scale_peta_meter_1 = Vec::new();
                            for param in &input.param_data_scale_peta_meter_1 {
                                texture_handles_scale_peta_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_peta_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_peta_meter_10 = Vec::new();
                            let mut param_buffers_scale_peta_meter_10 = Vec::new();
                            for param in &input.param_data_scale_peta_meter_10 {
                                texture_handles_scale_peta_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_peta_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_peta_meter_100 = Vec::new();
                            let mut param_buffers_scale_peta_meter_100 = Vec::new();
                            for param in &input.param_data_scale_peta_meter_100 {
                                texture_handles_scale_peta_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_peta_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_exa_meter_1 = Vec::new();
                            let mut param_buffers_scale_exa_meter_1 = Vec::new();
                            for param in &input.param_data_scale_exa_meter_1 {
                                texture_handles_scale_exa_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_exa_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_exa_meter_10 = Vec::new();
                            let mut param_buffers_scale_exa_meter_10 = Vec::new();
                            for param in &input.param_data_scale_exa_meter_10 {
                                texture_handles_scale_exa_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_exa_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_exa_meter_100 = Vec::new();
                            let mut param_buffers_scale_exa_meter_100 = Vec::new();
                            for param in &input.param_data_scale_exa_meter_100 {
                                texture_handles_scale_exa_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_exa_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_zetta_meter_1 = Vec::new();
                            let mut param_buffers_scale_zetta_meter_1 = Vec::new();
                            for param in &input.param_data_scale_zetta_meter_1 {
                                texture_handles_scale_zetta_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_zetta_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_zetta_meter_10 = Vec::new();
                            let mut param_buffers_scale_zetta_meter_10 = Vec::new();
                            for param in &input.param_data_scale_zetta_meter_10 {
                                texture_handles_scale_zetta_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_zetta_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_zetta_meter_100 = Vec::new();
                            let mut param_buffers_scale_zetta_meter_100 = Vec::new();
                            for param in &input.param_data_scale_zetta_meter_100 {
                                texture_handles_scale_zetta_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_zetta_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_yotta_meter_1 = Vec::new();
                            let mut param_buffers_scale_yotta_meter_1 = Vec::new();
                            for param in &input.param_data_scale_yotta_meter_1 {
                                texture_handles_scale_yotta_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_yotta_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_yotta_meter_10 = Vec::new();
                            let mut param_buffers_scale_yotta_meter_10 = Vec::new();
                            for param in &input.param_data_scale_yotta_meter_10 {
                                texture_handles_scale_yotta_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_yotta_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_yotta_meter_100 = Vec::new();
                            let mut param_buffers_scale_yotta_meter_100 = Vec::new();
                            for param in &input.param_data_scale_yotta_meter_100 {
                                texture_handles_scale_yotta_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_yotta_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_ronna_meter_1 = Vec::new();
                            let mut param_buffers_scale_ronna_meter_1 = Vec::new();
                            for param in &input.param_data_scale_ronna_meter_1 {
                                texture_handles_scale_ronna_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_ronna_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_ronna_meter_10 = Vec::new();
                            let mut param_buffers_scale_ronna_meter_10 = Vec::new();
                            for param in &input.param_data_scale_ronna_meter_10 {
                                texture_handles_scale_ronna_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_ronna_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_ronna_meter_100 = Vec::new();
                            let mut param_buffers_scale_ronna_meter_100 = Vec::new();
                            for param in &input.param_data_scale_ronna_meter_100 {
                                texture_handles_scale_ronna_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_ronna_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_quetta_meter_1 = Vec::new();
                            let mut param_buffers_scale_quetta_meter_1 = Vec::new();
                            for param in &input.param_data_scale_quetta_meter_1 {
                                texture_handles_scale_quetta_meter_1.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quetta_meter_1.push(buffer);
                            }
                            let mut texture_handles_scale_quetta_meter_10 = Vec::new();
                            let mut param_buffers_scale_quetta_meter_10 = Vec::new();
                            for param in &input.param_data_scale_quetta_meter_10 {
                                texture_handles_scale_quetta_meter_10.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quetta_meter_10.push(buffer);
                            }
                            let mut texture_handles_scale_quetta_meter_100 = Vec::new();
                            let mut param_buffers_scale_quetta_meter_100 = Vec::new();
                            for param in &input.param_data_scale_quetta_meter_100 {
                                texture_handles_scale_quetta_meter_100.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quetta_meter_100.push(buffer);
                            }
                            let mut texture_handles_scale_quetta_meter_1000 = Vec::new();
                            let mut param_buffers_scale_quetta_meter_1000 = Vec::new();
                            for param in &input.param_data_scale_quetta_meter_1000 {
                                texture_handles_scale_quetta_meter_1000.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quetta_meter_1000.push(buffer);
                            }
                            let mut texture_handles_scale_quetta_meter_10000 = Vec::new();
                            let mut param_buffers_scale_quetta_meter_10000 = Vec::new();
                            for param in &input.param_data_scale_quetta_meter_10000 {
                                texture_handles_scale_quetta_meter_10000.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quetta_meter_10000.push(buffer);
                            }
                            let mut texture_handles_scale_quetta_meter_100000 = Vec::new();
                            let mut param_buffers_scale_quetta_meter_100000 = Vec::new();
                            for param in &input.param_data_scale_quetta_meter_100000 {
                                texture_handles_scale_quetta_meter_100000.push(images.add(new_chunk_texture(texture_size)));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers_scale_quetta_meter_100000.push(buffer);
                            }

                            Output {
                                params_scale_quecto_meter_000001: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quecto_meter_000001,
                                    param_buffers: param_buffers_scale_quecto_meter_000001,
                                },
                                params_scale_quecto_meter_00001: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quecto_meter_00001,
                                    param_buffers: param_buffers_scale_quecto_meter_00001,
                                },
                                params_scale_quecto_meter_0001: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quecto_meter_0001,
                                    param_buffers: param_buffers_scale_quecto_meter_0001,
                                },
                                params_scale_quecto_meter_001: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quecto_meter_001,
                                    param_buffers: param_buffers_scale_quecto_meter_001,
                                },
                                params_scale_quecto_meter_01: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quecto_meter_01,
                                    param_buffers: param_buffers_scale_quecto_meter_01,
                                },
                                params_scale_quecto_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quecto_meter_1,
                                    param_buffers: param_buffers_scale_quecto_meter_1,
                                },
                                params_scale_quecto_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quecto_meter_10,
                                    param_buffers: param_buffers_scale_quecto_meter_10,
                                },
                                params_scale_quecto_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quecto_meter_100,
                                    param_buffers: param_buffers_scale_quecto_meter_100,
                                },
                                params_scale_ronto_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_ronto_meter_1,
                                    param_buffers: param_buffers_scale_ronto_meter_1,
                                },
                                params_scale_ronto_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_ronto_meter_10,
                                    param_buffers: param_buffers_scale_ronto_meter_10,
                                },
                                params_scale_ronto_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_ronto_meter_100,
                                    param_buffers: param_buffers_scale_ronto_meter_100,
                                },
                                params_scale_yocto_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_yocto_meter_1,
                                    param_buffers: param_buffers_scale_yocto_meter_1,
                                },
                                params_scale_yocto_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_yocto_meter_10,
                                    param_buffers: param_buffers_scale_yocto_meter_10,
                                },
                                params_scale_yocto_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_yocto_meter_100,
                                    param_buffers: param_buffers_scale_yocto_meter_100,
                                },
                                params_scale_zepto_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_zepto_meter_1,
                                    param_buffers: param_buffers_scale_zepto_meter_1,
                                },
                                params_scale_zepto_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_zepto_meter_10,
                                    param_buffers: param_buffers_scale_zepto_meter_10,
                                },
                                params_scale_zepto_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_zepto_meter_100,
                                    param_buffers: param_buffers_scale_zepto_meter_100,
                                },
                                params_scale_atto_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_atto_meter_1,
                                    param_buffers: param_buffers_scale_atto_meter_1,
                                },
                                params_scale_atto_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_atto_meter_10,
                                    param_buffers: param_buffers_scale_atto_meter_10,
                                },
                                params_scale_atto_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_atto_meter_100,
                                    param_buffers: param_buffers_scale_atto_meter_100,
                                },
                                params_scale_femto_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_femto_meter_1,
                                    param_buffers: param_buffers_scale_femto_meter_1,
                                },
                                params_scale_femto_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_femto_meter_10,
                                    param_buffers: param_buffers_scale_femto_meter_10,
                                },
                                params_scale_femto_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_femto_meter_100,
                                    param_buffers: param_buffers_scale_femto_meter_100,
                                },
                                params_scale_pico_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_pico_meter_1,
                                    param_buffers: param_buffers_scale_pico_meter_1,
                                },
                                params_scale_pico_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_pico_meter_10,
                                    param_buffers: param_buffers_scale_pico_meter_10,
                                },
                                params_scale_pico_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_pico_meter_100,
                                    param_buffers: param_buffers_scale_pico_meter_100,
                                },
                                params_scale_nano_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_nano_meter_1,
                                    param_buffers: param_buffers_scale_nano_meter_1,
                                },
                                params_scale_nano_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_nano_meter_10,
                                    param_buffers: param_buffers_scale_nano_meter_10,
                                },
                                params_scale_nano_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_nano_meter_100,
                                    param_buffers: param_buffers_scale_nano_meter_100,
                                },
                                params_scale_micro_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_micro_meter_1,
                                    param_buffers: param_buffers_scale_micro_meter_1,
                                },
                                params_scale_micro_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_micro_meter_10,
                                    param_buffers: param_buffers_scale_micro_meter_10,
                                },
                                params_scale_micro_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_micro_meter_100,
                                    param_buffers: param_buffers_scale_micro_meter_100,
                                },
                                params_scale_milli_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_milli_meter_1,
                                    param_buffers: param_buffers_scale_milli_meter_1,
                                },
                                params_scale_milli_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_milli_meter_10,
                                    param_buffers: param_buffers_scale_milli_meter_10,
                                },
                                params_scale_milli_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_milli_meter_100,
                                    param_buffers: param_buffers_scale_milli_meter_100,
                                },
                                params_scale_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_meter_1,
                                    param_buffers: param_buffers_scale_meter_1,
                                },
                                params_scale_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_meter_10,
                                    param_buffers: param_buffers_scale_meter_10,
                                },
                                params_scale_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_meter_100,
                                    param_buffers: param_buffers_scale_meter_100,
                                },
                                params_scale_kilo_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_kilo_meter_1,
                                    param_buffers: param_buffers_scale_kilo_meter_1,
                                },
                                params_scale_kilo_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_kilo_meter_10,
                                    param_buffers: param_buffers_scale_kilo_meter_10,
                                },
                                params_scale_kilo_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_kilo_meter_100,
                                    param_buffers: param_buffers_scale_kilo_meter_100,
                                },
                                params_scale_mega_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_mega_meter_1,
                                    param_buffers: param_buffers_scale_mega_meter_1,
                                },
                                params_scale_mega_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_mega_meter_10,
                                    param_buffers: param_buffers_scale_mega_meter_10,
                                },
                                params_scale_mega_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_mega_meter_100,
                                    param_buffers: param_buffers_scale_mega_meter_100,
                                },
                                params_scale_giga_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_giga_meter_1,
                                    param_buffers: param_buffers_scale_giga_meter_1,
                                },
                                params_scale_giga_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_giga_meter_10,
                                    param_buffers: param_buffers_scale_giga_meter_10,
                                },
                                params_scale_giga_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_giga_meter_100,
                                    param_buffers: param_buffers_scale_giga_meter_100,
                                },
                                params_scale_tera_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_tera_meter_1,
                                    param_buffers: param_buffers_scale_tera_meter_1,
                                },
                                params_scale_tera_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_tera_meter_10,
                                    param_buffers: param_buffers_scale_tera_meter_10,
                                },
                                params_scale_tera_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_tera_meter_100,
                                    param_buffers: param_buffers_scale_tera_meter_100,
                                },
                                params_scale_peta_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_peta_meter_1,
                                    param_buffers: param_buffers_scale_peta_meter_1,
                                },
                                params_scale_peta_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_peta_meter_10,
                                    param_buffers: param_buffers_scale_peta_meter_10,
                                },
                                params_scale_peta_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_peta_meter_100,
                                    param_buffers: param_buffers_scale_peta_meter_100,
                                },
                                params_scale_exa_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_exa_meter_1,
                                    param_buffers: param_buffers_scale_exa_meter_1,
                                },
                                params_scale_exa_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_exa_meter_10,
                                    param_buffers: param_buffers_scale_exa_meter_10,
                                },
                                params_scale_exa_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_exa_meter_100,
                                    param_buffers: param_buffers_scale_exa_meter_100,
                                },
                                params_scale_zetta_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_zetta_meter_1,
                                    param_buffers: param_buffers_scale_zetta_meter_1,
                                },
                                params_scale_zetta_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_zetta_meter_10,
                                    param_buffers: param_buffers_scale_zetta_meter_10,
                                },
                                params_scale_zetta_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_zetta_meter_100,
                                    param_buffers: param_buffers_scale_zetta_meter_100,
                                },
                                params_scale_yotta_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_yotta_meter_1,
                                    param_buffers: param_buffers_scale_yotta_meter_1,
                                },
                                params_scale_yotta_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_yotta_meter_10,
                                    param_buffers: param_buffers_scale_yotta_meter_10,
                                },
                                params_scale_yotta_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_yotta_meter_100,
                                    param_buffers: param_buffers_scale_yotta_meter_100,
                                },
                                params_scale_ronna_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_ronna_meter_1,
                                    param_buffers: param_buffers_scale_ronna_meter_1,
                                },
                                params_scale_ronna_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_ronna_meter_10,
                                    param_buffers: param_buffers_scale_ronna_meter_10,
                                },
                                params_scale_ronna_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_ronna_meter_100,
                                    param_buffers: param_buffers_scale_ronna_meter_100,
                                },
                                params_scale_quetta_meter_1: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quetta_meter_1,
                                    param_buffers: param_buffers_scale_quetta_meter_1,
                                },
                                params_scale_quetta_meter_10: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quetta_meter_10,
                                    param_buffers: param_buffers_scale_quetta_meter_10,
                                },
                                params_scale_quetta_meter_100: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quetta_meter_100,
                                    param_buffers: param_buffers_scale_quetta_meter_100,
                                },
                                params_scale_quetta_meter_1000: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quetta_meter_1000,
                                    param_buffers: param_buffers_scale_quetta_meter_1000,
                                },
                                params_scale_quetta_meter_10000: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quetta_meter_10000,
                                    param_buffers: param_buffers_scale_quetta_meter_10000,
                                },
                                params_scale_quetta_meter_100000: PreparedRenderExecutorInput {
                                    shader_name,
                                    pipeline_id,
                                    bind_group_layout: bind_group_layout.clone(),
                                    texture_size,
                                    texture_handles: texture_handles_scale_quetta_meter_100000,
                                    param_buffers: param_buffers_scale_quetta_meter_100000,
                                },
                            }
                        }
                    ]
                }

                GetTextureViews: RenderWhile, run_if_paused: false, run_after_startup_finished: false {
                    core_types: [
                        struct RenderAccess<'w> {
                            gpu_images: Res<'w, RenderAssets<GpuImage>>,
                        }
                        struct Input {
                            params_scale_quecto_meter_000001: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_00001: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_0001: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_001: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_01: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_1: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_10: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_100: PreparedRenderExecutorInput,
                            params_scale_ronto_meter_1: PreparedRenderExecutorInput,
                            params_scale_ronto_meter_10: PreparedRenderExecutorInput,
                            params_scale_ronto_meter_100: PreparedRenderExecutorInput,
                            params_scale_yocto_meter_1: PreparedRenderExecutorInput,
                            params_scale_yocto_meter_10: PreparedRenderExecutorInput,
                            params_scale_yocto_meter_100: PreparedRenderExecutorInput,
                            params_scale_zepto_meter_1: PreparedRenderExecutorInput,
                            params_scale_zepto_meter_10: PreparedRenderExecutorInput,
                            params_scale_zepto_meter_100: PreparedRenderExecutorInput,
                            params_scale_atto_meter_1: PreparedRenderExecutorInput,
                            params_scale_atto_meter_10: PreparedRenderExecutorInput,
                            params_scale_atto_meter_100: PreparedRenderExecutorInput,
                            params_scale_femto_meter_1: PreparedRenderExecutorInput,
                            params_scale_femto_meter_10: PreparedRenderExecutorInput,
                            params_scale_femto_meter_100: PreparedRenderExecutorInput,
                            params_scale_pico_meter_1: PreparedRenderExecutorInput,
                            params_scale_pico_meter_10: PreparedRenderExecutorInput,
                            params_scale_pico_meter_100: PreparedRenderExecutorInput,
                            params_scale_nano_meter_1: PreparedRenderExecutorInput,
                            params_scale_nano_meter_10: PreparedRenderExecutorInput,
                            params_scale_nano_meter_100: PreparedRenderExecutorInput,
                            params_scale_micro_meter_1: PreparedRenderExecutorInput,
                            params_scale_micro_meter_10: PreparedRenderExecutorInput,
                            params_scale_micro_meter_100: PreparedRenderExecutorInput,
                            params_scale_milli_meter_1: PreparedRenderExecutorInput,
                            params_scale_milli_meter_10: PreparedRenderExecutorInput,
                            params_scale_milli_meter_100: PreparedRenderExecutorInput,
                            params_scale_meter_1: PreparedRenderExecutorInput,
                            params_scale_meter_10: PreparedRenderExecutorInput,
                            params_scale_meter_100: PreparedRenderExecutorInput,
                            params_scale_kilo_meter_1: PreparedRenderExecutorInput,
                            params_scale_kilo_meter_10: PreparedRenderExecutorInput,
                            params_scale_kilo_meter_100: PreparedRenderExecutorInput,
                            params_scale_mega_meter_1: PreparedRenderExecutorInput,
                            params_scale_mega_meter_10: PreparedRenderExecutorInput,
                            params_scale_mega_meter_100: PreparedRenderExecutorInput,
                            params_scale_giga_meter_1: PreparedRenderExecutorInput,
                            params_scale_giga_meter_10: PreparedRenderExecutorInput,
                            params_scale_giga_meter_100: PreparedRenderExecutorInput,
                            params_scale_tera_meter_1: PreparedRenderExecutorInput,
                            params_scale_tera_meter_10: PreparedRenderExecutorInput,
                            params_scale_tera_meter_100: PreparedRenderExecutorInput,
                            params_scale_peta_meter_1: PreparedRenderExecutorInput,
                            params_scale_peta_meter_10: PreparedRenderExecutorInput,
                            params_scale_peta_meter_100: PreparedRenderExecutorInput,
                            params_scale_exa_meter_1: PreparedRenderExecutorInput,
                            params_scale_exa_meter_10: PreparedRenderExecutorInput,
                            params_scale_exa_meter_100: PreparedRenderExecutorInput,
                            params_scale_zetta_meter_1: PreparedRenderExecutorInput,
                            params_scale_zetta_meter_10: PreparedRenderExecutorInput,
                            params_scale_zetta_meter_100: PreparedRenderExecutorInput,
                            params_scale_yotta_meter_1: PreparedRenderExecutorInput,
                            params_scale_yotta_meter_10: PreparedRenderExecutorInput,
                            params_scale_yotta_meter_100: PreparedRenderExecutorInput,
                            params_scale_ronna_meter_1: PreparedRenderExecutorInput,
                            params_scale_ronna_meter_10: PreparedRenderExecutorInput,
                            params_scale_ronna_meter_100: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_1: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_10: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_100: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_1000: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_10000: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_100000: PreparedRenderExecutorInput,
                        }
                        struct State {
                            params_scale_quecto_meter_000001: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_00001: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_0001: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_001: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_01: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_1: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_10: PreparedRenderExecutorInput,
                            params_scale_quecto_meter_100: PreparedRenderExecutorInput,
                            params_scale_ronto_meter_1: PreparedRenderExecutorInput,
                            params_scale_ronto_meter_10: PreparedRenderExecutorInput,
                            params_scale_ronto_meter_100: PreparedRenderExecutorInput,
                            params_scale_yocto_meter_1: PreparedRenderExecutorInput,
                            params_scale_yocto_meter_10: PreparedRenderExecutorInput,
                            params_scale_yocto_meter_100: PreparedRenderExecutorInput,
                            params_scale_zepto_meter_1: PreparedRenderExecutorInput,
                            params_scale_zepto_meter_10: PreparedRenderExecutorInput,
                            params_scale_zepto_meter_100: PreparedRenderExecutorInput,
                            params_scale_atto_meter_1: PreparedRenderExecutorInput,
                            params_scale_atto_meter_10: PreparedRenderExecutorInput,
                            params_scale_atto_meter_100: PreparedRenderExecutorInput,
                            params_scale_femto_meter_1: PreparedRenderExecutorInput,
                            params_scale_femto_meter_10: PreparedRenderExecutorInput,
                            params_scale_femto_meter_100: PreparedRenderExecutorInput,
                            params_scale_pico_meter_1: PreparedRenderExecutorInput,
                            params_scale_pico_meter_10: PreparedRenderExecutorInput,
                            params_scale_pico_meter_100: PreparedRenderExecutorInput,
                            params_scale_nano_meter_1: PreparedRenderExecutorInput,
                            params_scale_nano_meter_10: PreparedRenderExecutorInput,
                            params_scale_nano_meter_100: PreparedRenderExecutorInput,
                            params_scale_micro_meter_1: PreparedRenderExecutorInput,
                            params_scale_micro_meter_10: PreparedRenderExecutorInput,
                            params_scale_micro_meter_100: PreparedRenderExecutorInput,
                            params_scale_milli_meter_1: PreparedRenderExecutorInput,
                            params_scale_milli_meter_10: PreparedRenderExecutorInput,
                            params_scale_milli_meter_100: PreparedRenderExecutorInput,
                            params_scale_meter_1: PreparedRenderExecutorInput,
                            params_scale_meter_10: PreparedRenderExecutorInput,
                            params_scale_meter_100: PreparedRenderExecutorInput,
                            params_scale_kilo_meter_1: PreparedRenderExecutorInput,
                            params_scale_kilo_meter_10: PreparedRenderExecutorInput,
                            params_scale_kilo_meter_100: PreparedRenderExecutorInput,
                            params_scale_mega_meter_1: PreparedRenderExecutorInput,
                            params_scale_mega_meter_10: PreparedRenderExecutorInput,
                            params_scale_mega_meter_100: PreparedRenderExecutorInput,
                            params_scale_giga_meter_1: PreparedRenderExecutorInput,
                            params_scale_giga_meter_10: PreparedRenderExecutorInput,
                            params_scale_giga_meter_100: PreparedRenderExecutorInput,
                            params_scale_tera_meter_1: PreparedRenderExecutorInput,
                            params_scale_tera_meter_10: PreparedRenderExecutorInput,
                            params_scale_tera_meter_100: PreparedRenderExecutorInput,
                            params_scale_peta_meter_1: PreparedRenderExecutorInput,
                            params_scale_peta_meter_10: PreparedRenderExecutorInput,
                            params_scale_peta_meter_100: PreparedRenderExecutorInput,
                            params_scale_exa_meter_1: PreparedRenderExecutorInput,
                            params_scale_exa_meter_10: PreparedRenderExecutorInput,
                            params_scale_exa_meter_100: PreparedRenderExecutorInput,
                            params_scale_zetta_meter_1: PreparedRenderExecutorInput,
                            params_scale_zetta_meter_10: PreparedRenderExecutorInput,
                            params_scale_zetta_meter_100: PreparedRenderExecutorInput,
                            params_scale_yotta_meter_1: PreparedRenderExecutorInput,
                            params_scale_yotta_meter_10: PreparedRenderExecutorInput,
                            params_scale_yotta_meter_100: PreparedRenderExecutorInput,
                            params_scale_ronna_meter_1: PreparedRenderExecutorInput,
                            params_scale_ronna_meter_10: PreparedRenderExecutorInput,
                            params_scale_ronna_meter_100: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_1: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_10: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_100: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_1000: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_10000: PreparedRenderExecutorInput,
                            params_scale_quetta_meter_100000: PreparedRenderExecutorInput,
                        }
                        struct Output {
                            shared_render_executor: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_000001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_00001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_0001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_01: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_1: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_10: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_100: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_1: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_10: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_100: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_1: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_10: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_100: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_1: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_10: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_100: ChunkRenderExecutor,
                            render_executor_scale_meter_1: ChunkRenderExecutor,
                            render_executor_scale_meter_10: ChunkRenderExecutor,
                            render_executor_scale_meter_100: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_1: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_10: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_100: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_1: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_10: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_100: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_1: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_10: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_100: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_1: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_10: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_100: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_1: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_10: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_100: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_1: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_10: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_100: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_1000: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_10000: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_100000: ChunkRenderExecutor,
                        }
                    ],
                    core_functions: [
                        fn SetupRenderWhile |input, render_access| -> State {
                            State {
                                params_scale_quecto_meter_000001: input.params_scale_quecto_meter_000001,
                                params_scale_quecto_meter_00001: input.params_scale_quecto_meter_00001,
                                params_scale_quecto_meter_0001: input.params_scale_quecto_meter_0001,
                                params_scale_quecto_meter_001: input.params_scale_quecto_meter_001,
                                params_scale_quecto_meter_01: input.params_scale_quecto_meter_01,
                                params_scale_quecto_meter_1: input.params_scale_quecto_meter_1,
                                params_scale_quecto_meter_10: input.params_scale_quecto_meter_10,
                                params_scale_quecto_meter_100: input.params_scale_quecto_meter_100,
                                params_scale_ronto_meter_1: input.params_scale_ronto_meter_1,
                                params_scale_ronto_meter_10: input.params_scale_ronto_meter_10,
                                params_scale_ronto_meter_100: input.params_scale_ronto_meter_100,
                                params_scale_yocto_meter_1: input.params_scale_yocto_meter_1,
                                params_scale_yocto_meter_10: input.params_scale_yocto_meter_10,
                                params_scale_yocto_meter_100: input.params_scale_yocto_meter_100,
                                params_scale_zepto_meter_1: input.params_scale_zepto_meter_1,
                                params_scale_zepto_meter_10: input.params_scale_zepto_meter_10,
                                params_scale_zepto_meter_100: input.params_scale_zepto_meter_100,
                                params_scale_atto_meter_1: input.params_scale_atto_meter_1,
                                params_scale_atto_meter_10: input.params_scale_atto_meter_10,
                                params_scale_atto_meter_100: input.params_scale_atto_meter_100,
                                params_scale_femto_meter_1: input.params_scale_femto_meter_1,
                                params_scale_femto_meter_10: input.params_scale_femto_meter_10,
                                params_scale_femto_meter_100: input.params_scale_femto_meter_100,
                                params_scale_pico_meter_1: input.params_scale_pico_meter_1,
                                params_scale_pico_meter_10: input.params_scale_pico_meter_10,
                                params_scale_pico_meter_100: input.params_scale_pico_meter_100,
                                params_scale_nano_meter_1: input.params_scale_nano_meter_1,
                                params_scale_nano_meter_10: input.params_scale_nano_meter_10,
                                params_scale_nano_meter_100: input.params_scale_nano_meter_100,
                                params_scale_micro_meter_1: input.params_scale_micro_meter_1,
                                params_scale_micro_meter_10: input.params_scale_micro_meter_10,
                                params_scale_micro_meter_100: input.params_scale_micro_meter_100,
                                params_scale_milli_meter_1: input.params_scale_milli_meter_1,
                                params_scale_milli_meter_10: input.params_scale_milli_meter_10,
                                params_scale_milli_meter_100: input.params_scale_milli_meter_100,
                                params_scale_meter_1: input.params_scale_meter_1,
                                params_scale_meter_10: input.params_scale_meter_10,
                                params_scale_meter_100: input.params_scale_meter_100,
                                params_scale_kilo_meter_1: input.params_scale_kilo_meter_1,
                                params_scale_kilo_meter_10: input.params_scale_kilo_meter_10,
                                params_scale_kilo_meter_100: input.params_scale_kilo_meter_100,
                                params_scale_mega_meter_1: input.params_scale_mega_meter_1,
                                params_scale_mega_meter_10: input.params_scale_mega_meter_10,
                                params_scale_mega_meter_100: input.params_scale_mega_meter_100,
                                params_scale_giga_meter_1: input.params_scale_giga_meter_1,
                                params_scale_giga_meter_10: input.params_scale_giga_meter_10,
                                params_scale_giga_meter_100: input.params_scale_giga_meter_100,
                                params_scale_tera_meter_1: input.params_scale_tera_meter_1,
                                params_scale_tera_meter_10: input.params_scale_tera_meter_10,
                                params_scale_tera_meter_100: input.params_scale_tera_meter_100,
                                params_scale_peta_meter_1: input.params_scale_peta_meter_1,
                                params_scale_peta_meter_10: input.params_scale_peta_meter_10,
                                params_scale_peta_meter_100: input.params_scale_peta_meter_100,
                                params_scale_exa_meter_1: input.params_scale_exa_meter_1,
                                params_scale_exa_meter_10: input.params_scale_exa_meter_10,
                                params_scale_exa_meter_100: input.params_scale_exa_meter_100,
                                params_scale_zetta_meter_1: input.params_scale_zetta_meter_1,
                                params_scale_zetta_meter_10: input.params_scale_zetta_meter_10,
                                params_scale_zetta_meter_100: input.params_scale_zetta_meter_100,
                                params_scale_yotta_meter_1: input.params_scale_yotta_meter_1,
                                params_scale_yotta_meter_10: input.params_scale_yotta_meter_10,
                                params_scale_yotta_meter_100: input.params_scale_yotta_meter_100,
                                params_scale_ronna_meter_1: input.params_scale_ronna_meter_1,
                                params_scale_ronna_meter_10: input.params_scale_ronna_meter_10,
                                params_scale_ronna_meter_100: input.params_scale_ronna_meter_100,
                                params_scale_quetta_meter_1: input.params_scale_quetta_meter_1,
                                params_scale_quetta_meter_10: input.params_scale_quetta_meter_10,
                                params_scale_quetta_meter_100: input.params_scale_quetta_meter_100,
                                params_scale_quetta_meter_1000: input.params_scale_quetta_meter_1000,
                                params_scale_quetta_meter_10000: input.params_scale_quetta_meter_10000,
                                params_scale_quetta_meter_100000: input.params_scale_quetta_meter_100000,
                            }
                        }

                        fn RunRenderWhile |state, render_access| -> Outcome<State, Output> {
                            let gpu_images = render_access.gpu_images;

                            let mut texture_views_scale_quecto_meter_000001 = Vec::with_capacity(state.params_scale_quecto_meter_000001.texture_handles.len());
                            for handle in &state.params_scale_quecto_meter_000001.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quecto_meter_000001.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quecto_meter_00001 = Vec::with_capacity(state.params_scale_quecto_meter_00001.texture_handles.len());
                            for handle in &state.params_scale_quecto_meter_00001.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quecto_meter_00001.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quecto_meter_0001 = Vec::with_capacity(state.params_scale_quecto_meter_0001.texture_handles.len());
                            for handle in &state.params_scale_quecto_meter_0001.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quecto_meter_0001.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quecto_meter_001 = Vec::with_capacity(state.params_scale_quecto_meter_001.texture_handles.len());
                            for handle in &state.params_scale_quecto_meter_001.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quecto_meter_001.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quecto_meter_01 = Vec::with_capacity(state.params_scale_quecto_meter_01.texture_handles.len());
                            for handle in &state.params_scale_quecto_meter_01.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quecto_meter_01.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quecto_meter_1 = Vec::with_capacity(state.params_scale_quecto_meter_1.texture_handles.len());
                            for handle in &state.params_scale_quecto_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quecto_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quecto_meter_10 = Vec::with_capacity(state.params_scale_quecto_meter_10.texture_handles.len());
                            for handle in &state.params_scale_quecto_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quecto_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quecto_meter_100 = Vec::with_capacity(state.params_scale_quecto_meter_100.texture_handles.len());
                            for handle in &state.params_scale_quecto_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quecto_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_ronto_meter_1 = Vec::with_capacity(state.params_scale_ronto_meter_1.texture_handles.len());
                            for handle in &state.params_scale_ronto_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_ronto_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_ronto_meter_10 = Vec::with_capacity(state.params_scale_ronto_meter_10.texture_handles.len());
                            for handle in &state.params_scale_ronto_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_ronto_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_ronto_meter_100 = Vec::with_capacity(state.params_scale_ronto_meter_100.texture_handles.len());
                            for handle in &state.params_scale_ronto_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_ronto_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_yocto_meter_1 = Vec::with_capacity(state.params_scale_yocto_meter_1.texture_handles.len());
                            for handle in &state.params_scale_yocto_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_yocto_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_yocto_meter_10 = Vec::with_capacity(state.params_scale_yocto_meter_10.texture_handles.len());
                            for handle in &state.params_scale_yocto_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_yocto_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_yocto_meter_100 = Vec::with_capacity(state.params_scale_yocto_meter_100.texture_handles.len());
                            for handle in &state.params_scale_yocto_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_yocto_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_zepto_meter_1 = Vec::with_capacity(state.params_scale_zepto_meter_1.texture_handles.len());
                            for handle in &state.params_scale_zepto_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_zepto_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_zepto_meter_10 = Vec::with_capacity(state.params_scale_zepto_meter_10.texture_handles.len());
                            for handle in &state.params_scale_zepto_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_zepto_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_zepto_meter_100 = Vec::with_capacity(state.params_scale_zepto_meter_100.texture_handles.len());
                            for handle in &state.params_scale_zepto_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_zepto_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_atto_meter_1 = Vec::with_capacity(state.params_scale_atto_meter_1.texture_handles.len());
                            for handle in &state.params_scale_atto_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_atto_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_atto_meter_10 = Vec::with_capacity(state.params_scale_atto_meter_10.texture_handles.len());
                            for handle in &state.params_scale_atto_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_atto_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_atto_meter_100 = Vec::with_capacity(state.params_scale_atto_meter_100.texture_handles.len());
                            for handle in &state.params_scale_atto_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_atto_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_femto_meter_1 = Vec::with_capacity(state.params_scale_femto_meter_1.texture_handles.len());
                            for handle in &state.params_scale_femto_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_femto_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_femto_meter_10 = Vec::with_capacity(state.params_scale_femto_meter_10.texture_handles.len());
                            for handle in &state.params_scale_femto_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_femto_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_femto_meter_100 = Vec::with_capacity(state.params_scale_femto_meter_100.texture_handles.len());
                            for handle in &state.params_scale_femto_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_femto_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_pico_meter_1 = Vec::with_capacity(state.params_scale_pico_meter_1.texture_handles.len());
                            for handle in &state.params_scale_pico_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_pico_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_pico_meter_10 = Vec::with_capacity(state.params_scale_pico_meter_10.texture_handles.len());
                            for handle in &state.params_scale_pico_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_pico_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_pico_meter_100 = Vec::with_capacity(state.params_scale_pico_meter_100.texture_handles.len());
                            for handle in &state.params_scale_pico_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_pico_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_nano_meter_1 = Vec::with_capacity(state.params_scale_nano_meter_1.texture_handles.len());
                            for handle in &state.params_scale_nano_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_nano_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_nano_meter_10 = Vec::with_capacity(state.params_scale_nano_meter_10.texture_handles.len());
                            for handle in &state.params_scale_nano_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_nano_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_nano_meter_100 = Vec::with_capacity(state.params_scale_nano_meter_100.texture_handles.len());
                            for handle in &state.params_scale_nano_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_nano_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_micro_meter_1 = Vec::with_capacity(state.params_scale_micro_meter_1.texture_handles.len());
                            for handle in &state.params_scale_micro_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_micro_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_micro_meter_10 = Vec::with_capacity(state.params_scale_micro_meter_10.texture_handles.len());
                            for handle in &state.params_scale_micro_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_micro_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_micro_meter_100 = Vec::with_capacity(state.params_scale_micro_meter_100.texture_handles.len());
                            for handle in &state.params_scale_micro_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_micro_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_milli_meter_1 = Vec::with_capacity(state.params_scale_milli_meter_1.texture_handles.len());
                            for handle in &state.params_scale_milli_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_milli_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_milli_meter_10 = Vec::with_capacity(state.params_scale_milli_meter_10.texture_handles.len());
                            for handle in &state.params_scale_milli_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_milli_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_milli_meter_100 = Vec::with_capacity(state.params_scale_milli_meter_100.texture_handles.len());
                            for handle in &state.params_scale_milli_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_milli_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_meter_1 = Vec::with_capacity(state.params_scale_meter_1.texture_handles.len());
                            for handle in &state.params_scale_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_meter_10 = Vec::with_capacity(state.params_scale_meter_10.texture_handles.len());
                            for handle in &state.params_scale_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_meter_100 = Vec::with_capacity(state.params_scale_meter_100.texture_handles.len());
                            for handle in &state.params_scale_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_kilo_meter_1 = Vec::with_capacity(state.params_scale_kilo_meter_1.texture_handles.len());
                            for handle in &state.params_scale_kilo_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_kilo_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_kilo_meter_10 = Vec::with_capacity(state.params_scale_kilo_meter_10.texture_handles.len());
                            for handle in &state.params_scale_kilo_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_kilo_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_kilo_meter_100 = Vec::with_capacity(state.params_scale_kilo_meter_100.texture_handles.len());
                            for handle in &state.params_scale_kilo_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_kilo_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_mega_meter_1 = Vec::with_capacity(state.params_scale_mega_meter_1.texture_handles.len());
                            for handle in &state.params_scale_mega_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_mega_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_mega_meter_10 = Vec::with_capacity(state.params_scale_mega_meter_10.texture_handles.len());
                            for handle in &state.params_scale_mega_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_mega_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_mega_meter_100 = Vec::with_capacity(state.params_scale_mega_meter_100.texture_handles.len());
                            for handle in &state.params_scale_mega_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_mega_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_giga_meter_1 = Vec::with_capacity(state.params_scale_giga_meter_1.texture_handles.len());
                            for handle in &state.params_scale_giga_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_giga_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_giga_meter_10 = Vec::with_capacity(state.params_scale_giga_meter_10.texture_handles.len());
                            for handle in &state.params_scale_giga_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_giga_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_giga_meter_100 = Vec::with_capacity(state.params_scale_giga_meter_100.texture_handles.len());
                            for handle in &state.params_scale_giga_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_giga_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_tera_meter_1 = Vec::with_capacity(state.params_scale_tera_meter_1.texture_handles.len());
                            for handle in &state.params_scale_tera_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_tera_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_tera_meter_10 = Vec::with_capacity(state.params_scale_tera_meter_10.texture_handles.len());
                            for handle in &state.params_scale_tera_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_tera_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_tera_meter_100 = Vec::with_capacity(state.params_scale_tera_meter_100.texture_handles.len());
                            for handle in &state.params_scale_tera_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_tera_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_peta_meter_1 = Vec::with_capacity(state.params_scale_peta_meter_1.texture_handles.len());
                            for handle in &state.params_scale_peta_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_peta_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_peta_meter_10 = Vec::with_capacity(state.params_scale_peta_meter_10.texture_handles.len());
                            for handle in &state.params_scale_peta_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_peta_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_peta_meter_100 = Vec::with_capacity(state.params_scale_peta_meter_100.texture_handles.len());
                            for handle in &state.params_scale_peta_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_peta_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_exa_meter_1 = Vec::with_capacity(state.params_scale_exa_meter_1.texture_handles.len());
                            for handle in &state.params_scale_exa_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_exa_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_exa_meter_10 = Vec::with_capacity(state.params_scale_exa_meter_10.texture_handles.len());
                            for handle in &state.params_scale_exa_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_exa_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_exa_meter_100 = Vec::with_capacity(state.params_scale_exa_meter_100.texture_handles.len());
                            for handle in &state.params_scale_exa_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_exa_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_zetta_meter_1 = Vec::with_capacity(state.params_scale_zetta_meter_1.texture_handles.len());
                            for handle in &state.params_scale_zetta_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_zetta_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_zetta_meter_10 = Vec::with_capacity(state.params_scale_zetta_meter_10.texture_handles.len());
                            for handle in &state.params_scale_zetta_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_zetta_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_zetta_meter_100 = Vec::with_capacity(state.params_scale_zetta_meter_100.texture_handles.len());
                            for handle in &state.params_scale_zetta_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_zetta_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_yotta_meter_1 = Vec::with_capacity(state.params_scale_yotta_meter_1.texture_handles.len());
                            for handle in &state.params_scale_yotta_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_yotta_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_yotta_meter_10 = Vec::with_capacity(state.params_scale_yotta_meter_10.texture_handles.len());
                            for handle in &state.params_scale_yotta_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_yotta_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_yotta_meter_100 = Vec::with_capacity(state.params_scale_yotta_meter_100.texture_handles.len());
                            for handle in &state.params_scale_yotta_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_yotta_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_ronna_meter_1 = Vec::with_capacity(state.params_scale_ronna_meter_1.texture_handles.len());
                            for handle in &state.params_scale_ronna_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_ronna_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_ronna_meter_10 = Vec::with_capacity(state.params_scale_ronna_meter_10.texture_handles.len());
                            for handle in &state.params_scale_ronna_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_ronna_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_ronna_meter_100 = Vec::with_capacity(state.params_scale_ronna_meter_100.texture_handles.len());
                            for handle in &state.params_scale_ronna_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_ronna_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quetta_meter_1 = Vec::with_capacity(state.params_scale_quetta_meter_1.texture_handles.len());
                            for handle in &state.params_scale_quetta_meter_1.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quetta_meter_1.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quetta_meter_10 = Vec::with_capacity(state.params_scale_quetta_meter_10.texture_handles.len());
                            for handle in &state.params_scale_quetta_meter_10.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quetta_meter_10.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quetta_meter_100 = Vec::with_capacity(state.params_scale_quetta_meter_100.texture_handles.len());
                            for handle in &state.params_scale_quetta_meter_100.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quetta_meter_100.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quetta_meter_1000 = Vec::with_capacity(state.params_scale_quetta_meter_1000.texture_handles.len());
                            for handle in &state.params_scale_quetta_meter_1000.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quetta_meter_1000.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quetta_meter_10000 = Vec::with_capacity(state.params_scale_quetta_meter_10000.texture_handles.len());
                            for handle in &state.params_scale_quetta_meter_10000.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quetta_meter_10000.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }
                            let mut texture_views_scale_quetta_meter_100000 = Vec::with_capacity(state.params_scale_quetta_meter_100000.texture_handles.len());
                            for handle in &state.params_scale_quetta_meter_100000.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views_scale_quetta_meter_100000.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }

                            Done(Output {
                                shared_render_executor: ChunkRenderExecutor {
                                    shader_name: state.params_scale_meter_1.shader_name,
                                    pipeline_id: state.params_scale_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_meter_1.bind_group_layout.clone(),
                                    texture_size: state.params_scale_meter_1.texture_size,
                                    texture_handles: Vec::new(),
                                    param_buffers: Vec::new(),
                                    texture_views: Vec::new(),
                                    receiver: None
                                },
                                render_executor_scale_quecto_meter_000001: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quecto_meter_000001.shader_name,
                                    pipeline_id: state.params_scale_quecto_meter_000001.pipeline_id,
                                    bind_group_layout: state.params_scale_quecto_meter_000001.bind_group_layout,
                                    texture_size: state.params_scale_quecto_meter_000001.texture_size,
                                    texture_handles: state.params_scale_quecto_meter_000001.texture_handles,
                                    param_buffers: state.params_scale_quecto_meter_000001.param_buffers,
                                    texture_views: texture_views_scale_quecto_meter_000001,
                                    receiver: None
                                },
                                render_executor_scale_quecto_meter_00001: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quecto_meter_00001.shader_name,
                                    pipeline_id: state.params_scale_quecto_meter_00001.pipeline_id,
                                    bind_group_layout: state.params_scale_quecto_meter_00001.bind_group_layout,
                                    texture_size: state.params_scale_quecto_meter_00001.texture_size,
                                    texture_handles: state.params_scale_quecto_meter_00001.texture_handles,
                                    param_buffers: state.params_scale_quecto_meter_00001.param_buffers,
                                    texture_views: texture_views_scale_quecto_meter_00001,
                                    receiver: None
                                },
                                render_executor_scale_quecto_meter_0001: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quecto_meter_0001.shader_name,
                                    pipeline_id: state.params_scale_quecto_meter_0001.pipeline_id,
                                    bind_group_layout: state.params_scale_quecto_meter_0001.bind_group_layout,
                                    texture_size: state.params_scale_quecto_meter_0001.texture_size,
                                    texture_handles: state.params_scale_quecto_meter_0001.texture_handles,
                                    param_buffers: state.params_scale_quecto_meter_0001.param_buffers,
                                    texture_views: texture_views_scale_quecto_meter_0001,
                                    receiver: None
                                },
                                render_executor_scale_quecto_meter_001: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quecto_meter_001.shader_name,
                                    pipeline_id: state.params_scale_quecto_meter_001.pipeline_id,
                                    bind_group_layout: state.params_scale_quecto_meter_001.bind_group_layout,
                                    texture_size: state.params_scale_quecto_meter_001.texture_size,
                                    texture_handles: state.params_scale_quecto_meter_001.texture_handles,
                                    param_buffers: state.params_scale_quecto_meter_001.param_buffers,
                                    texture_views: texture_views_scale_quecto_meter_001,
                                    receiver: None
                                },
                                render_executor_scale_quecto_meter_01: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quecto_meter_01.shader_name,
                                    pipeline_id: state.params_scale_quecto_meter_01.pipeline_id,
                                    bind_group_layout: state.params_scale_quecto_meter_01.bind_group_layout,
                                    texture_size: state.params_scale_quecto_meter_01.texture_size,
                                    texture_handles: state.params_scale_quecto_meter_01.texture_handles,
                                    param_buffers: state.params_scale_quecto_meter_01.param_buffers,
                                    texture_views: texture_views_scale_quecto_meter_01,
                                    receiver: None
                                },
                                render_executor_scale_quecto_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quecto_meter_1.shader_name,
                                    pipeline_id: state.params_scale_quecto_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_quecto_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_quecto_meter_1.texture_size,
                                    texture_handles: state.params_scale_quecto_meter_1.texture_handles,
                                    param_buffers: state.params_scale_quecto_meter_1.param_buffers,
                                    texture_views: texture_views_scale_quecto_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_quecto_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quecto_meter_10.shader_name,
                                    pipeline_id: state.params_scale_quecto_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_quecto_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_quecto_meter_10.texture_size,
                                    texture_handles: state.params_scale_quecto_meter_10.texture_handles,
                                    param_buffers: state.params_scale_quecto_meter_10.param_buffers,
                                    texture_views: texture_views_scale_quecto_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_quecto_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quecto_meter_100.shader_name,
                                    pipeline_id: state.params_scale_quecto_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_quecto_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_quecto_meter_100.texture_size,
                                    texture_handles: state.params_scale_quecto_meter_100.texture_handles,
                                    param_buffers: state.params_scale_quecto_meter_100.param_buffers,
                                    texture_views: texture_views_scale_quecto_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_ronto_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_ronto_meter_1.shader_name,
                                    pipeline_id: state.params_scale_ronto_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_ronto_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_ronto_meter_1.texture_size,
                                    texture_handles: state.params_scale_ronto_meter_1.texture_handles,
                                    param_buffers: state.params_scale_ronto_meter_1.param_buffers,
                                    texture_views: texture_views_scale_ronto_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_ronto_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_ronto_meter_10.shader_name,
                                    pipeline_id: state.params_scale_ronto_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_ronto_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_ronto_meter_10.texture_size,
                                    texture_handles: state.params_scale_ronto_meter_10.texture_handles,
                                    param_buffers: state.params_scale_ronto_meter_10.param_buffers,
                                    texture_views: texture_views_scale_ronto_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_ronto_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_ronto_meter_100.shader_name,
                                    pipeline_id: state.params_scale_ronto_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_ronto_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_ronto_meter_100.texture_size,
                                    texture_handles: state.params_scale_ronto_meter_100.texture_handles,
                                    param_buffers: state.params_scale_ronto_meter_100.param_buffers,
                                    texture_views: texture_views_scale_ronto_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_yocto_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_yocto_meter_1.shader_name,
                                    pipeline_id: state.params_scale_yocto_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_yocto_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_yocto_meter_1.texture_size,
                                    texture_handles: state.params_scale_yocto_meter_1.texture_handles,
                                    param_buffers: state.params_scale_yocto_meter_1.param_buffers,
                                    texture_views: texture_views_scale_yocto_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_yocto_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_yocto_meter_10.shader_name,
                                    pipeline_id: state.params_scale_yocto_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_yocto_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_yocto_meter_10.texture_size,
                                    texture_handles: state.params_scale_yocto_meter_10.texture_handles,
                                    param_buffers: state.params_scale_yocto_meter_10.param_buffers,
                                    texture_views: texture_views_scale_yocto_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_yocto_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_yocto_meter_100.shader_name,
                                    pipeline_id: state.params_scale_yocto_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_yocto_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_yocto_meter_100.texture_size,
                                    texture_handles: state.params_scale_yocto_meter_100.texture_handles,
                                    param_buffers: state.params_scale_yocto_meter_100.param_buffers,
                                    texture_views: texture_views_scale_yocto_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_zepto_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_zepto_meter_1.shader_name,
                                    pipeline_id: state.params_scale_zepto_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_zepto_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_zepto_meter_1.texture_size,
                                    texture_handles: state.params_scale_zepto_meter_1.texture_handles,
                                    param_buffers: state.params_scale_zepto_meter_1.param_buffers,
                                    texture_views: texture_views_scale_zepto_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_zepto_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_zepto_meter_10.shader_name,
                                    pipeline_id: state.params_scale_zepto_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_zepto_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_zepto_meter_10.texture_size,
                                    texture_handles: state.params_scale_zepto_meter_10.texture_handles,
                                    param_buffers: state.params_scale_zepto_meter_10.param_buffers,
                                    texture_views: texture_views_scale_zepto_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_zepto_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_zepto_meter_100.shader_name,
                                    pipeline_id: state.params_scale_zepto_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_zepto_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_zepto_meter_100.texture_size,
                                    texture_handles: state.params_scale_zepto_meter_100.texture_handles,
                                    param_buffers: state.params_scale_zepto_meter_100.param_buffers,
                                    texture_views: texture_views_scale_zepto_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_atto_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_atto_meter_1.shader_name,
                                    pipeline_id: state.params_scale_atto_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_atto_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_atto_meter_1.texture_size,
                                    texture_handles: state.params_scale_atto_meter_1.texture_handles,
                                    param_buffers: state.params_scale_atto_meter_1.param_buffers,
                                    texture_views: texture_views_scale_atto_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_atto_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_atto_meter_10.shader_name,
                                    pipeline_id: state.params_scale_atto_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_atto_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_atto_meter_10.texture_size,
                                    texture_handles: state.params_scale_atto_meter_10.texture_handles,
                                    param_buffers: state.params_scale_atto_meter_10.param_buffers,
                                    texture_views: texture_views_scale_atto_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_atto_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_atto_meter_100.shader_name,
                                    pipeline_id: state.params_scale_atto_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_atto_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_atto_meter_100.texture_size,
                                    texture_handles: state.params_scale_atto_meter_100.texture_handles,
                                    param_buffers: state.params_scale_atto_meter_100.param_buffers,
                                    texture_views: texture_views_scale_atto_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_femto_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_femto_meter_1.shader_name,
                                    pipeline_id: state.params_scale_femto_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_femto_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_femto_meter_1.texture_size,
                                    texture_handles: state.params_scale_femto_meter_1.texture_handles,
                                    param_buffers: state.params_scale_femto_meter_1.param_buffers,
                                    texture_views: texture_views_scale_femto_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_femto_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_femto_meter_10.shader_name,
                                    pipeline_id: state.params_scale_femto_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_femto_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_femto_meter_10.texture_size,
                                    texture_handles: state.params_scale_femto_meter_10.texture_handles,
                                    param_buffers: state.params_scale_femto_meter_10.param_buffers,
                                    texture_views: texture_views_scale_femto_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_femto_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_femto_meter_100.shader_name,
                                    pipeline_id: state.params_scale_femto_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_femto_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_femto_meter_100.texture_size,
                                    texture_handles: state.params_scale_femto_meter_100.texture_handles,
                                    param_buffers: state.params_scale_femto_meter_100.param_buffers,
                                    texture_views: texture_views_scale_femto_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_pico_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_pico_meter_1.shader_name,
                                    pipeline_id: state.params_scale_pico_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_pico_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_pico_meter_1.texture_size,
                                    texture_handles: state.params_scale_pico_meter_1.texture_handles,
                                    param_buffers: state.params_scale_pico_meter_1.param_buffers,
                                    texture_views: texture_views_scale_pico_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_pico_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_pico_meter_10.shader_name,
                                    pipeline_id: state.params_scale_pico_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_pico_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_pico_meter_10.texture_size,
                                    texture_handles: state.params_scale_pico_meter_10.texture_handles,
                                    param_buffers: state.params_scale_pico_meter_10.param_buffers,
                                    texture_views: texture_views_scale_pico_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_pico_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_pico_meter_100.shader_name,
                                    pipeline_id: state.params_scale_pico_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_pico_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_pico_meter_100.texture_size,
                                    texture_handles: state.params_scale_pico_meter_100.texture_handles,
                                    param_buffers: state.params_scale_pico_meter_100.param_buffers,
                                    texture_views: texture_views_scale_pico_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_nano_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_nano_meter_1.shader_name,
                                    pipeline_id: state.params_scale_nano_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_nano_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_nano_meter_1.texture_size,
                                    texture_handles: state.params_scale_nano_meter_1.texture_handles,
                                    param_buffers: state.params_scale_nano_meter_1.param_buffers,
                                    texture_views: texture_views_scale_nano_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_nano_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_nano_meter_10.shader_name,
                                    pipeline_id: state.params_scale_nano_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_nano_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_nano_meter_10.texture_size,
                                    texture_handles: state.params_scale_nano_meter_10.texture_handles,
                                    param_buffers: state.params_scale_nano_meter_10.param_buffers,
                                    texture_views: texture_views_scale_nano_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_nano_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_nano_meter_100.shader_name,
                                    pipeline_id: state.params_scale_nano_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_nano_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_nano_meter_100.texture_size,
                                    texture_handles: state.params_scale_nano_meter_100.texture_handles,
                                    param_buffers: state.params_scale_nano_meter_100.param_buffers,
                                    texture_views: texture_views_scale_nano_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_micro_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_micro_meter_1.shader_name,
                                    pipeline_id: state.params_scale_micro_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_micro_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_micro_meter_1.texture_size,
                                    texture_handles: state.params_scale_micro_meter_1.texture_handles,
                                    param_buffers: state.params_scale_micro_meter_1.param_buffers,
                                    texture_views: texture_views_scale_micro_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_micro_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_micro_meter_10.shader_name,
                                    pipeline_id: state.params_scale_micro_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_micro_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_micro_meter_10.texture_size,
                                    texture_handles: state.params_scale_micro_meter_10.texture_handles,
                                    param_buffers: state.params_scale_micro_meter_10.param_buffers,
                                    texture_views: texture_views_scale_micro_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_micro_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_micro_meter_100.shader_name,
                                    pipeline_id: state.params_scale_micro_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_micro_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_micro_meter_100.texture_size,
                                    texture_handles: state.params_scale_micro_meter_100.texture_handles,
                                    param_buffers: state.params_scale_micro_meter_100.param_buffers,
                                    texture_views: texture_views_scale_micro_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_milli_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_milli_meter_1.shader_name,
                                    pipeline_id: state.params_scale_milli_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_milli_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_milli_meter_1.texture_size,
                                    texture_handles: state.params_scale_milli_meter_1.texture_handles,
                                    param_buffers: state.params_scale_milli_meter_1.param_buffers,
                                    texture_views: texture_views_scale_milli_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_milli_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_milli_meter_10.shader_name,
                                    pipeline_id: state.params_scale_milli_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_milli_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_milli_meter_10.texture_size,
                                    texture_handles: state.params_scale_milli_meter_10.texture_handles,
                                    param_buffers: state.params_scale_milli_meter_10.param_buffers,
                                    texture_views: texture_views_scale_milli_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_milli_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_milli_meter_100.shader_name,
                                    pipeline_id: state.params_scale_milli_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_milli_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_milli_meter_100.texture_size,
                                    texture_handles: state.params_scale_milli_meter_100.texture_handles,
                                    param_buffers: state.params_scale_milli_meter_100.param_buffers,
                                    texture_views: texture_views_scale_milli_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_meter_1.shader_name,
                                    pipeline_id: state.params_scale_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_meter_1.texture_size,
                                    texture_handles: state.params_scale_meter_1.texture_handles,
                                    param_buffers: state.params_scale_meter_1.param_buffers,
                                    texture_views: texture_views_scale_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_meter_10.shader_name,
                                    pipeline_id: state.params_scale_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_meter_10.texture_size,
                                    texture_handles: state.params_scale_meter_10.texture_handles,
                                    param_buffers: state.params_scale_meter_10.param_buffers,
                                    texture_views: texture_views_scale_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_meter_100.shader_name,
                                    pipeline_id: state.params_scale_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_meter_100.texture_size,
                                    texture_handles: state.params_scale_meter_100.texture_handles,
                                    param_buffers: state.params_scale_meter_100.param_buffers,
                                    texture_views: texture_views_scale_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_kilo_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_kilo_meter_1.shader_name,
                                    pipeline_id: state.params_scale_kilo_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_kilo_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_kilo_meter_1.texture_size,
                                    texture_handles: state.params_scale_kilo_meter_1.texture_handles,
                                    param_buffers: state.params_scale_kilo_meter_1.param_buffers,
                                    texture_views: texture_views_scale_kilo_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_kilo_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_kilo_meter_10.shader_name,
                                    pipeline_id: state.params_scale_kilo_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_kilo_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_kilo_meter_10.texture_size,
                                    texture_handles: state.params_scale_kilo_meter_10.texture_handles,
                                    param_buffers: state.params_scale_kilo_meter_10.param_buffers,
                                    texture_views: texture_views_scale_kilo_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_kilo_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_kilo_meter_100.shader_name,
                                    pipeline_id: state.params_scale_kilo_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_kilo_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_kilo_meter_100.texture_size,
                                    texture_handles: state.params_scale_kilo_meter_100.texture_handles,
                                    param_buffers: state.params_scale_kilo_meter_100.param_buffers,
                                    texture_views: texture_views_scale_kilo_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_mega_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_mega_meter_1.shader_name,
                                    pipeline_id: state.params_scale_mega_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_mega_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_mega_meter_1.texture_size,
                                    texture_handles: state.params_scale_mega_meter_1.texture_handles,
                                    param_buffers: state.params_scale_mega_meter_1.param_buffers,
                                    texture_views: texture_views_scale_mega_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_mega_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_mega_meter_10.shader_name,
                                    pipeline_id: state.params_scale_mega_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_mega_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_mega_meter_10.texture_size,
                                    texture_handles: state.params_scale_mega_meter_10.texture_handles,
                                    param_buffers: state.params_scale_mega_meter_10.param_buffers,
                                    texture_views: texture_views_scale_mega_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_mega_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_mega_meter_100.shader_name,
                                    pipeline_id: state.params_scale_mega_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_mega_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_mega_meter_100.texture_size,
                                    texture_handles: state.params_scale_mega_meter_100.texture_handles,
                                    param_buffers: state.params_scale_mega_meter_100.param_buffers,
                                    texture_views: texture_views_scale_mega_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_giga_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_giga_meter_1.shader_name,
                                    pipeline_id: state.params_scale_giga_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_giga_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_giga_meter_1.texture_size,
                                    texture_handles: state.params_scale_giga_meter_1.texture_handles,
                                    param_buffers: state.params_scale_giga_meter_1.param_buffers,
                                    texture_views: texture_views_scale_giga_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_giga_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_giga_meter_10.shader_name,
                                    pipeline_id: state.params_scale_giga_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_giga_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_giga_meter_10.texture_size,
                                    texture_handles: state.params_scale_giga_meter_10.texture_handles,
                                    param_buffers: state.params_scale_giga_meter_10.param_buffers,
                                    texture_views: texture_views_scale_giga_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_giga_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_giga_meter_100.shader_name,
                                    pipeline_id: state.params_scale_giga_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_giga_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_giga_meter_100.texture_size,
                                    texture_handles: state.params_scale_giga_meter_100.texture_handles,
                                    param_buffers: state.params_scale_giga_meter_100.param_buffers,
                                    texture_views: texture_views_scale_giga_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_tera_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_tera_meter_1.shader_name,
                                    pipeline_id: state.params_scale_tera_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_tera_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_tera_meter_1.texture_size,
                                    texture_handles: state.params_scale_tera_meter_1.texture_handles,
                                    param_buffers: state.params_scale_tera_meter_1.param_buffers,
                                    texture_views: texture_views_scale_tera_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_tera_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_tera_meter_10.shader_name,
                                    pipeline_id: state.params_scale_tera_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_tera_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_tera_meter_10.texture_size,
                                    texture_handles: state.params_scale_tera_meter_10.texture_handles,
                                    param_buffers: state.params_scale_tera_meter_10.param_buffers,
                                    texture_views: texture_views_scale_tera_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_tera_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_tera_meter_100.shader_name,
                                    pipeline_id: state.params_scale_tera_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_tera_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_tera_meter_100.texture_size,
                                    texture_handles: state.params_scale_tera_meter_100.texture_handles,
                                    param_buffers: state.params_scale_tera_meter_100.param_buffers,
                                    texture_views: texture_views_scale_tera_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_peta_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_peta_meter_1.shader_name,
                                    pipeline_id: state.params_scale_peta_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_peta_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_peta_meter_1.texture_size,
                                    texture_handles: state.params_scale_peta_meter_1.texture_handles,
                                    param_buffers: state.params_scale_peta_meter_1.param_buffers,
                                    texture_views: texture_views_scale_peta_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_peta_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_peta_meter_10.shader_name,
                                    pipeline_id: state.params_scale_peta_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_peta_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_peta_meter_10.texture_size,
                                    texture_handles: state.params_scale_peta_meter_10.texture_handles,
                                    param_buffers: state.params_scale_peta_meter_10.param_buffers,
                                    texture_views: texture_views_scale_peta_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_peta_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_peta_meter_100.shader_name,
                                    pipeline_id: state.params_scale_peta_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_peta_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_peta_meter_100.texture_size,
                                    texture_handles: state.params_scale_peta_meter_100.texture_handles,
                                    param_buffers: state.params_scale_peta_meter_100.param_buffers,
                                    texture_views: texture_views_scale_peta_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_exa_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_exa_meter_1.shader_name,
                                    pipeline_id: state.params_scale_exa_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_exa_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_exa_meter_1.texture_size,
                                    texture_handles: state.params_scale_exa_meter_1.texture_handles,
                                    param_buffers: state.params_scale_exa_meter_1.param_buffers,
                                    texture_views: texture_views_scale_exa_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_exa_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_exa_meter_10.shader_name,
                                    pipeline_id: state.params_scale_exa_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_exa_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_exa_meter_10.texture_size,
                                    texture_handles: state.params_scale_exa_meter_10.texture_handles,
                                    param_buffers: state.params_scale_exa_meter_10.param_buffers,
                                    texture_views: texture_views_scale_exa_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_exa_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_exa_meter_100.shader_name,
                                    pipeline_id: state.params_scale_exa_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_exa_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_exa_meter_100.texture_size,
                                    texture_handles: state.params_scale_exa_meter_100.texture_handles,
                                    param_buffers: state.params_scale_exa_meter_100.param_buffers,
                                    texture_views: texture_views_scale_exa_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_zetta_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_zetta_meter_1.shader_name,
                                    pipeline_id: state.params_scale_zetta_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_zetta_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_zetta_meter_1.texture_size,
                                    texture_handles: state.params_scale_zetta_meter_1.texture_handles,
                                    param_buffers: state.params_scale_zetta_meter_1.param_buffers,
                                    texture_views: texture_views_scale_zetta_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_zetta_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_zetta_meter_10.shader_name,
                                    pipeline_id: state.params_scale_zetta_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_zetta_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_zetta_meter_10.texture_size,
                                    texture_handles: state.params_scale_zetta_meter_10.texture_handles,
                                    param_buffers: state.params_scale_zetta_meter_10.param_buffers,
                                    texture_views: texture_views_scale_zetta_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_zetta_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_zetta_meter_100.shader_name,
                                    pipeline_id: state.params_scale_zetta_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_zetta_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_zetta_meter_100.texture_size,
                                    texture_handles: state.params_scale_zetta_meter_100.texture_handles,
                                    param_buffers: state.params_scale_zetta_meter_100.param_buffers,
                                    texture_views: texture_views_scale_zetta_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_yotta_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_yotta_meter_1.shader_name,
                                    pipeline_id: state.params_scale_yotta_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_yotta_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_yotta_meter_1.texture_size,
                                    texture_handles: state.params_scale_yotta_meter_1.texture_handles,
                                    param_buffers: state.params_scale_yotta_meter_1.param_buffers,
                                    texture_views: texture_views_scale_yotta_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_yotta_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_yotta_meter_10.shader_name,
                                    pipeline_id: state.params_scale_yotta_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_yotta_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_yotta_meter_10.texture_size,
                                    texture_handles: state.params_scale_yotta_meter_10.texture_handles,
                                    param_buffers: state.params_scale_yotta_meter_10.param_buffers,
                                    texture_views: texture_views_scale_yotta_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_yotta_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_yotta_meter_100.shader_name,
                                    pipeline_id: state.params_scale_yotta_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_yotta_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_yotta_meter_100.texture_size,
                                    texture_handles: state.params_scale_yotta_meter_100.texture_handles,
                                    param_buffers: state.params_scale_yotta_meter_100.param_buffers,
                                    texture_views: texture_views_scale_yotta_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_ronna_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_ronna_meter_1.shader_name,
                                    pipeline_id: state.params_scale_ronna_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_ronna_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_ronna_meter_1.texture_size,
                                    texture_handles: state.params_scale_ronna_meter_1.texture_handles,
                                    param_buffers: state.params_scale_ronna_meter_1.param_buffers,
                                    texture_views: texture_views_scale_ronna_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_ronna_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_ronna_meter_10.shader_name,
                                    pipeline_id: state.params_scale_ronna_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_ronna_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_ronna_meter_10.texture_size,
                                    texture_handles: state.params_scale_ronna_meter_10.texture_handles,
                                    param_buffers: state.params_scale_ronna_meter_10.param_buffers,
                                    texture_views: texture_views_scale_ronna_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_ronna_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_ronna_meter_100.shader_name,
                                    pipeline_id: state.params_scale_ronna_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_ronna_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_ronna_meter_100.texture_size,
                                    texture_handles: state.params_scale_ronna_meter_100.texture_handles,
                                    param_buffers: state.params_scale_ronna_meter_100.param_buffers,
                                    texture_views: texture_views_scale_ronna_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_quetta_meter_1: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quetta_meter_1.shader_name,
                                    pipeline_id: state.params_scale_quetta_meter_1.pipeline_id,
                                    bind_group_layout: state.params_scale_quetta_meter_1.bind_group_layout,
                                    texture_size: state.params_scale_quetta_meter_1.texture_size,
                                    texture_handles: state.params_scale_quetta_meter_1.texture_handles,
                                    param_buffers: state.params_scale_quetta_meter_1.param_buffers,
                                    texture_views: texture_views_scale_quetta_meter_1,
                                    receiver: None
                                },
                                render_executor_scale_quetta_meter_10: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quetta_meter_10.shader_name,
                                    pipeline_id: state.params_scale_quetta_meter_10.pipeline_id,
                                    bind_group_layout: state.params_scale_quetta_meter_10.bind_group_layout,
                                    texture_size: state.params_scale_quetta_meter_10.texture_size,
                                    texture_handles: state.params_scale_quetta_meter_10.texture_handles,
                                    param_buffers: state.params_scale_quetta_meter_10.param_buffers,
                                    texture_views: texture_views_scale_quetta_meter_10,
                                    receiver: None
                                },
                                render_executor_scale_quetta_meter_100: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quetta_meter_100.shader_name,
                                    pipeline_id: state.params_scale_quetta_meter_100.pipeline_id,
                                    bind_group_layout: state.params_scale_quetta_meter_100.bind_group_layout,
                                    texture_size: state.params_scale_quetta_meter_100.texture_size,
                                    texture_handles: state.params_scale_quetta_meter_100.texture_handles,
                                    param_buffers: state.params_scale_quetta_meter_100.param_buffers,
                                    texture_views: texture_views_scale_quetta_meter_100,
                                    receiver: None
                                },
                                render_executor_scale_quetta_meter_1000: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quetta_meter_1000.shader_name,
                                    pipeline_id: state.params_scale_quetta_meter_1000.pipeline_id,
                                    bind_group_layout: state.params_scale_quetta_meter_1000.bind_group_layout,
                                    texture_size: state.params_scale_quetta_meter_1000.texture_size,
                                    texture_handles: state.params_scale_quetta_meter_1000.texture_handles,
                                    param_buffers: state.params_scale_quetta_meter_1000.param_buffers,
                                    texture_views: texture_views_scale_quetta_meter_1000,
                                    receiver: None
                                },
                                render_executor_scale_quetta_meter_10000: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quetta_meter_10000.shader_name,
                                    pipeline_id: state.params_scale_quetta_meter_10000.pipeline_id,
                                    bind_group_layout: state.params_scale_quetta_meter_10000.bind_group_layout,
                                    texture_size: state.params_scale_quetta_meter_10000.texture_size,
                                    texture_handles: state.params_scale_quetta_meter_10000.texture_handles,
                                    param_buffers: state.params_scale_quetta_meter_10000.param_buffers,
                                    texture_views: texture_views_scale_quetta_meter_10000,
                                    receiver: None
                                },
                                render_executor_scale_quetta_meter_100000: ChunkRenderExecutor {
                                    shader_name: state.params_scale_quetta_meter_100000.shader_name,
                                    pipeline_id: state.params_scale_quetta_meter_100000.pipeline_id,
                                    bind_group_layout: state.params_scale_quetta_meter_100000.bind_group_layout,
                                    texture_size: state.params_scale_quetta_meter_100000.texture_size,
                                    texture_handles: state.params_scale_quetta_meter_100000.texture_handles,
                                    param_buffers: state.params_scale_quetta_meter_100000.param_buffers,
                                    texture_views: texture_views_scale_quetta_meter_100000,
                                    receiver: None
                                },
                            })
                        }
                    ]
                }

                DispatchRenderTextures: Render, run_if_paused: false, run_after_startup_finished: false {
                    core_types: [
                        struct RenderAccess<'w> {
                            render_device: Res<'w, RenderDevice>,
                            queue: Res<'w, RenderQueue>,
                            pipeline_cache: Res<'w, PipelineCache>,
                        }
                        struct Input {
                            shared_render_executor: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_000001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_00001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_0001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_01: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_1: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_10: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_100: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_1: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_10: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_100: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_1: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_10: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_100: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_1: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_10: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_100: ChunkRenderExecutor,
                            render_executor_scale_meter_1: ChunkRenderExecutor,
                            render_executor_scale_meter_10: ChunkRenderExecutor,
                            render_executor_scale_meter_100: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_1: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_10: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_100: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_1: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_10: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_100: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_1: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_10: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_100: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_1: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_10: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_100: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_1: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_10: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_100: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_1: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_10: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_100: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_1000: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_10000: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_100000: ChunkRenderExecutor,
                        }
                        struct Output {
                            shared_render_executor: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_000001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_00001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_0001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_01: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_1: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_10: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_100: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_1: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_10: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_100: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_1: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_10: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_100: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_1: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_10: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_100: ChunkRenderExecutor,
                            render_executor_scale_meter_1: ChunkRenderExecutor,
                            render_executor_scale_meter_10: ChunkRenderExecutor,
                            render_executor_scale_meter_100: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_1: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_10: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_100: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_1: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_10: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_100: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_1: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_10: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_100: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_1: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_10: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_100: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_1: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_10: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_100: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_1: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_10: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_100: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_1000: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_10000: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_100000: ChunkRenderExecutor,
                        }
                    ],
                    core_functions: [
                        fn RunRender |input, render_access| -> Output {
                            let mut shared_render_executor = input.shared_render_executor;
                            let render_executor_scale_quecto_meter_000001 = &input.render_executor_scale_quecto_meter_000001;
                            let render_executor_scale_quecto_meter_00001 = &input.render_executor_scale_quecto_meter_00001;
                            let render_executor_scale_quecto_meter_0001 = &input.render_executor_scale_quecto_meter_0001;
                            let render_executor_scale_quecto_meter_001 = &input.render_executor_scale_quecto_meter_001;
                            let render_executor_scale_quecto_meter_01 = &input.render_executor_scale_quecto_meter_01;
                            let render_executor_scale_quecto_meter_1 = &input.render_executor_scale_quecto_meter_1;
                            let render_executor_scale_quecto_meter_10 = &input.render_executor_scale_quecto_meter_10;
                            let render_executor_scale_quecto_meter_100 = &input.render_executor_scale_quecto_meter_100;
                            let render_executor_scale_ronto_meter_1 = &input.render_executor_scale_ronto_meter_1;
                            let render_executor_scale_ronto_meter_10 = &input.render_executor_scale_ronto_meter_10;
                            let render_executor_scale_ronto_meter_100 = &input.render_executor_scale_ronto_meter_100;
                            let render_executor_scale_yocto_meter_1 = &input.render_executor_scale_yocto_meter_1;
                            let render_executor_scale_yocto_meter_10 = &input.render_executor_scale_yocto_meter_10;
                            let render_executor_scale_yocto_meter_100 = &input.render_executor_scale_yocto_meter_100;
                            let render_executor_scale_zepto_meter_1 = &input.render_executor_scale_zepto_meter_1;
                            let render_executor_scale_zepto_meter_10 = &input.render_executor_scale_zepto_meter_10;
                            let render_executor_scale_zepto_meter_100 = &input.render_executor_scale_zepto_meter_100;
                            let render_executor_scale_atto_meter_1 = &input.render_executor_scale_atto_meter_1;
                            let render_executor_scale_atto_meter_10 = &input.render_executor_scale_atto_meter_10;
                            let render_executor_scale_atto_meter_100 = &input.render_executor_scale_atto_meter_100;
                            let render_executor_scale_femto_meter_1 = &input.render_executor_scale_femto_meter_1;
                            let render_executor_scale_femto_meter_10 = &input.render_executor_scale_femto_meter_10;
                            let render_executor_scale_femto_meter_100 = &input.render_executor_scale_femto_meter_100;
                            let render_executor_scale_pico_meter_1 = &input.render_executor_scale_pico_meter_1;
                            let render_executor_scale_pico_meter_10 = &input.render_executor_scale_pico_meter_10;
                            let render_executor_scale_pico_meter_100 = &input.render_executor_scale_pico_meter_100;
                            let render_executor_scale_nano_meter_1 = &input.render_executor_scale_nano_meter_1;
                            let render_executor_scale_nano_meter_10 = &input.render_executor_scale_nano_meter_10;
                            let render_executor_scale_nano_meter_100 = &input.render_executor_scale_nano_meter_100;
                            let render_executor_scale_micro_meter_1 = &input.render_executor_scale_micro_meter_1;
                            let render_executor_scale_micro_meter_10 = &input.render_executor_scale_micro_meter_10;
                            let render_executor_scale_micro_meter_100 = &input.render_executor_scale_micro_meter_100;
                            let render_executor_scale_milli_meter_1 = &input.render_executor_scale_milli_meter_1;
                            let render_executor_scale_milli_meter_10 = &input.render_executor_scale_milli_meter_10;
                            let render_executor_scale_milli_meter_100 = &input.render_executor_scale_milli_meter_100;
                            let render_executor_scale_meter_1 = &input.render_executor_scale_meter_1;
                            let render_executor_scale_meter_10 = &input.render_executor_scale_meter_10;
                            let render_executor_scale_meter_100 = &input.render_executor_scale_meter_100;
                            let render_executor_scale_kilo_meter_1 = &input.render_executor_scale_kilo_meter_1;
                            let render_executor_scale_kilo_meter_10 = &input.render_executor_scale_kilo_meter_10;
                            let render_executor_scale_kilo_meter_100 = &input.render_executor_scale_kilo_meter_100;
                            let render_executor_scale_mega_meter_1 = &input.render_executor_scale_mega_meter_1;
                            let render_executor_scale_mega_meter_10 = &input.render_executor_scale_mega_meter_10;
                            let render_executor_scale_mega_meter_100 = &input.render_executor_scale_mega_meter_100;
                            let render_executor_scale_giga_meter_1 = &input.render_executor_scale_giga_meter_1;
                            let render_executor_scale_giga_meter_10 = &input.render_executor_scale_giga_meter_10;
                            let render_executor_scale_giga_meter_100 = &input.render_executor_scale_giga_meter_100;
                            let render_executor_scale_tera_meter_1 = &input.render_executor_scale_tera_meter_1;
                            let render_executor_scale_tera_meter_10 = &input.render_executor_scale_tera_meter_10;
                            let render_executor_scale_tera_meter_100 = &input.render_executor_scale_tera_meter_100;
                            let render_executor_scale_peta_meter_1 = &input.render_executor_scale_peta_meter_1;
                            let render_executor_scale_peta_meter_10 = &input.render_executor_scale_peta_meter_10;
                            let render_executor_scale_peta_meter_100 = &input.render_executor_scale_peta_meter_100;
                            let render_executor_scale_exa_meter_1 = &input.render_executor_scale_exa_meter_1;
                            let render_executor_scale_exa_meter_10 = &input.render_executor_scale_exa_meter_10;
                            let render_executor_scale_exa_meter_100 = &input.render_executor_scale_exa_meter_100;
                            let render_executor_scale_zetta_meter_1 = &input.render_executor_scale_zetta_meter_1;
                            let render_executor_scale_zetta_meter_10 = &input.render_executor_scale_zetta_meter_10;
                            let render_executor_scale_zetta_meter_100 = &input.render_executor_scale_zetta_meter_100;
                            let render_executor_scale_yotta_meter_1 = &input.render_executor_scale_yotta_meter_1;
                            let render_executor_scale_yotta_meter_10 = &input.render_executor_scale_yotta_meter_10;
                            let render_executor_scale_yotta_meter_100 = &input.render_executor_scale_yotta_meter_100;
                            let render_executor_scale_ronna_meter_1 = &input.render_executor_scale_ronna_meter_1;
                            let render_executor_scale_ronna_meter_10 = &input.render_executor_scale_ronna_meter_10;
                            let render_executor_scale_ronna_meter_100 = &input.render_executor_scale_ronna_meter_100;
                            let render_executor_scale_quetta_meter_1 = &input.render_executor_scale_quetta_meter_1;
                            let render_executor_scale_quetta_meter_10 = &input.render_executor_scale_quetta_meter_10;
                            let render_executor_scale_quetta_meter_100 = &input.render_executor_scale_quetta_meter_100;
                            let render_executor_scale_quetta_meter_1000 = &input.render_executor_scale_quetta_meter_1000;
                            let render_executor_scale_quetta_meter_10000 = &input.render_executor_scale_quetta_meter_10000;
                            let render_executor_scale_quetta_meter_100000 = &input.render_executor_scale_quetta_meter_100000;

                            let pipeline = render_access.pipeline_cache
                                .get_compute_pipeline(shared_render_executor.pipeline_id)
                                .expect("Pipeline not ready");

                            let mut encoder = render_access.render_device.create_command_encoder(&CommandEncoderDescriptor {
                                label: Some("DispatchRenderTextures Encoder"),
                            });

                            let big_loop_iter_scale_quecto_meter_000001 = render_executor_scale_quecto_meter_000001.texture_views.iter().zip(&render_executor_scale_quecto_meter_000001.param_buffers).zip(&render_executor_scale_quecto_meter_000001.texture_handles);
                            let big_loop_iter_scale_quecto_meter_00001 = render_executor_scale_quecto_meter_00001.texture_views.iter().zip(&render_executor_scale_quecto_meter_00001.param_buffers).zip(&render_executor_scale_quecto_meter_00001.texture_handles);
                            let big_loop_iter_scale_quecto_meter_0001 = render_executor_scale_quecto_meter_0001.texture_views.iter().zip(&render_executor_scale_quecto_meter_0001.param_buffers).zip(&render_executor_scale_quecto_meter_0001.texture_handles);
                            let big_loop_iter_scale_quecto_meter_001 = render_executor_scale_quecto_meter_001.texture_views.iter().zip(&render_executor_scale_quecto_meter_001.param_buffers).zip(&render_executor_scale_quecto_meter_001.texture_handles);
                            let big_loop_iter_scale_quecto_meter_01 = render_executor_scale_quecto_meter_01.texture_views.iter().zip(&render_executor_scale_quecto_meter_01.param_buffers).zip(&render_executor_scale_quecto_meter_01.texture_handles);
                            let big_loop_iter_scale_quecto_meter_1 = render_executor_scale_quecto_meter_1.texture_views.iter().zip(&render_executor_scale_quecto_meter_1.param_buffers).zip(&render_executor_scale_quecto_meter_1.texture_handles);
                            let big_loop_iter_scale_quecto_meter_10 = render_executor_scale_quecto_meter_10.texture_views.iter().zip(&render_executor_scale_quecto_meter_10.param_buffers).zip(&render_executor_scale_quecto_meter_10.texture_handles);
                            let big_loop_iter_scale_quecto_meter_100 = render_executor_scale_quecto_meter_100.texture_views.iter().zip(&render_executor_scale_quecto_meter_100.param_buffers).zip(&render_executor_scale_quecto_meter_100.texture_handles);
                            let big_loop_iter_scale_ronto_meter_1 = render_executor_scale_ronto_meter_1.texture_views.iter().zip(&render_executor_scale_ronto_meter_1.param_buffers).zip(&render_executor_scale_ronto_meter_1.texture_handles);
                            let big_loop_iter_scale_ronto_meter_10 = render_executor_scale_ronto_meter_10.texture_views.iter().zip(&render_executor_scale_ronto_meter_10.param_buffers).zip(&render_executor_scale_ronto_meter_10.texture_handles);
                            let big_loop_iter_scale_ronto_meter_100 = render_executor_scale_ronto_meter_100.texture_views.iter().zip(&render_executor_scale_ronto_meter_100.param_buffers).zip(&render_executor_scale_ronto_meter_100.texture_handles);
                            let big_loop_iter_scale_yocto_meter_1 = render_executor_scale_yocto_meter_1.texture_views.iter().zip(&render_executor_scale_yocto_meter_1.param_buffers).zip(&render_executor_scale_yocto_meter_1.texture_handles);
                            let big_loop_iter_scale_yocto_meter_10 = render_executor_scale_yocto_meter_10.texture_views.iter().zip(&render_executor_scale_yocto_meter_10.param_buffers).zip(&render_executor_scale_yocto_meter_10.texture_handles);
                            let big_loop_iter_scale_yocto_meter_100 = render_executor_scale_yocto_meter_100.texture_views.iter().zip(&render_executor_scale_yocto_meter_100.param_buffers).zip(&render_executor_scale_yocto_meter_100.texture_handles);
                            let big_loop_iter_scale_zepto_meter_1 = render_executor_scale_zepto_meter_1.texture_views.iter().zip(&render_executor_scale_zepto_meter_1.param_buffers).zip(&render_executor_scale_zepto_meter_1.texture_handles);
                            let big_loop_iter_scale_zepto_meter_10 = render_executor_scale_zepto_meter_10.texture_views.iter().zip(&render_executor_scale_zepto_meter_10.param_buffers).zip(&render_executor_scale_zepto_meter_10.texture_handles);
                            let big_loop_iter_scale_zepto_meter_100 = render_executor_scale_zepto_meter_100.texture_views.iter().zip(&render_executor_scale_zepto_meter_100.param_buffers).zip(&render_executor_scale_zepto_meter_100.texture_handles);
                            let big_loop_iter_scale_atto_meter_1 = render_executor_scale_atto_meter_1.texture_views.iter().zip(&render_executor_scale_atto_meter_1.param_buffers).zip(&render_executor_scale_atto_meter_1.texture_handles);
                            let big_loop_iter_scale_atto_meter_10 = render_executor_scale_atto_meter_10.texture_views.iter().zip(&render_executor_scale_atto_meter_10.param_buffers).zip(&render_executor_scale_atto_meter_10.texture_handles);
                            let big_loop_iter_scale_atto_meter_100 = render_executor_scale_atto_meter_100.texture_views.iter().zip(&render_executor_scale_atto_meter_100.param_buffers).zip(&render_executor_scale_atto_meter_100.texture_handles);
                            let big_loop_iter_scale_femto_meter_1 = render_executor_scale_femto_meter_1.texture_views.iter().zip(&render_executor_scale_femto_meter_1.param_buffers).zip(&render_executor_scale_femto_meter_1.texture_handles);
                            let big_loop_iter_scale_femto_meter_10 = render_executor_scale_femto_meter_10.texture_views.iter().zip(&render_executor_scale_femto_meter_10.param_buffers).zip(&render_executor_scale_femto_meter_10.texture_handles);
                            let big_loop_iter_scale_femto_meter_100 = render_executor_scale_femto_meter_100.texture_views.iter().zip(&render_executor_scale_femto_meter_100.param_buffers).zip(&render_executor_scale_femto_meter_100.texture_handles);
                            let big_loop_iter_scale_pico_meter_1 = render_executor_scale_pico_meter_1.texture_views.iter().zip(&render_executor_scale_pico_meter_1.param_buffers).zip(&render_executor_scale_pico_meter_1.texture_handles);
                            let big_loop_iter_scale_pico_meter_10 = render_executor_scale_pico_meter_10.texture_views.iter().zip(&render_executor_scale_pico_meter_10.param_buffers).zip(&render_executor_scale_pico_meter_10.texture_handles);
                            let big_loop_iter_scale_pico_meter_100 = render_executor_scale_pico_meter_100.texture_views.iter().zip(&render_executor_scale_pico_meter_100.param_buffers).zip(&render_executor_scale_pico_meter_100.texture_handles);
                            let big_loop_iter_scale_nano_meter_1 = render_executor_scale_nano_meter_1.texture_views.iter().zip(&render_executor_scale_nano_meter_1.param_buffers).zip(&render_executor_scale_nano_meter_1.texture_handles);
                            let big_loop_iter_scale_nano_meter_10 = render_executor_scale_nano_meter_10.texture_views.iter().zip(&render_executor_scale_nano_meter_10.param_buffers).zip(&render_executor_scale_nano_meter_10.texture_handles);
                            let big_loop_iter_scale_nano_meter_100 = render_executor_scale_nano_meter_100.texture_views.iter().zip(&render_executor_scale_nano_meter_100.param_buffers).zip(&render_executor_scale_nano_meter_100.texture_handles);
                            let big_loop_iter_scale_micro_meter_1 = render_executor_scale_micro_meter_1.texture_views.iter().zip(&render_executor_scale_micro_meter_1.param_buffers).zip(&render_executor_scale_micro_meter_1.texture_handles);
                            let big_loop_iter_scale_micro_meter_10 = render_executor_scale_micro_meter_10.texture_views.iter().zip(&render_executor_scale_micro_meter_10.param_buffers).zip(&render_executor_scale_micro_meter_10.texture_handles);
                            let big_loop_iter_scale_micro_meter_100 = render_executor_scale_micro_meter_100.texture_views.iter().zip(&render_executor_scale_micro_meter_100.param_buffers).zip(&render_executor_scale_micro_meter_100.texture_handles);
                            let big_loop_iter_scale_milli_meter_1 = render_executor_scale_milli_meter_1.texture_views.iter().zip(&render_executor_scale_milli_meter_1.param_buffers).zip(&render_executor_scale_milli_meter_1.texture_handles);
                            let big_loop_iter_scale_milli_meter_10 = render_executor_scale_milli_meter_10.texture_views.iter().zip(&render_executor_scale_milli_meter_10.param_buffers).zip(&render_executor_scale_milli_meter_10.texture_handles);
                            let big_loop_iter_scale_milli_meter_100 = render_executor_scale_milli_meter_100.texture_views.iter().zip(&render_executor_scale_milli_meter_100.param_buffers).zip(&render_executor_scale_milli_meter_100.texture_handles);
                            let big_loop_iter_scale_meter_1 = render_executor_scale_meter_1.texture_views.iter().zip(&render_executor_scale_meter_1.param_buffers).zip(&render_executor_scale_meter_1.texture_handles);
                            let big_loop_iter_scale_meter_10 = render_executor_scale_meter_10.texture_views.iter().zip(&render_executor_scale_meter_10.param_buffers).zip(&render_executor_scale_meter_10.texture_handles);
                            let big_loop_iter_scale_meter_100 = render_executor_scale_meter_100.texture_views.iter().zip(&render_executor_scale_meter_100.param_buffers).zip(&render_executor_scale_meter_100.texture_handles);
                            let big_loop_iter_scale_kilo_meter_1 = render_executor_scale_kilo_meter_1.texture_views.iter().zip(&render_executor_scale_kilo_meter_1.param_buffers).zip(&render_executor_scale_kilo_meter_1.texture_handles);
                            let big_loop_iter_scale_kilo_meter_10 = render_executor_scale_kilo_meter_10.texture_views.iter().zip(&render_executor_scale_kilo_meter_10.param_buffers).zip(&render_executor_scale_kilo_meter_10.texture_handles);
                            let big_loop_iter_scale_kilo_meter_100 = render_executor_scale_kilo_meter_100.texture_views.iter().zip(&render_executor_scale_kilo_meter_100.param_buffers).zip(&render_executor_scale_kilo_meter_100.texture_handles);
                            let big_loop_iter_scale_mega_meter_1 = render_executor_scale_mega_meter_1.texture_views.iter().zip(&render_executor_scale_mega_meter_1.param_buffers).zip(&render_executor_scale_mega_meter_1.texture_handles);
                            let big_loop_iter_scale_mega_meter_10 = render_executor_scale_mega_meter_10.texture_views.iter().zip(&render_executor_scale_mega_meter_10.param_buffers).zip(&render_executor_scale_mega_meter_10.texture_handles);
                            let big_loop_iter_scale_mega_meter_100 = render_executor_scale_mega_meter_100.texture_views.iter().zip(&render_executor_scale_mega_meter_100.param_buffers).zip(&render_executor_scale_mega_meter_100.texture_handles);
                            let big_loop_iter_scale_giga_meter_1 = render_executor_scale_giga_meter_1.texture_views.iter().zip(&render_executor_scale_giga_meter_1.param_buffers).zip(&render_executor_scale_giga_meter_1.texture_handles);
                            let big_loop_iter_scale_giga_meter_10 = render_executor_scale_giga_meter_10.texture_views.iter().zip(&render_executor_scale_giga_meter_10.param_buffers).zip(&render_executor_scale_giga_meter_10.texture_handles);
                            let big_loop_iter_scale_giga_meter_100 = render_executor_scale_giga_meter_100.texture_views.iter().zip(&render_executor_scale_giga_meter_100.param_buffers).zip(&render_executor_scale_giga_meter_100.texture_handles);
                            let big_loop_iter_scale_tera_meter_1 = render_executor_scale_tera_meter_1.texture_views.iter().zip(&render_executor_scale_tera_meter_1.param_buffers).zip(&render_executor_scale_tera_meter_1.texture_handles);
                            let big_loop_iter_scale_tera_meter_10 = render_executor_scale_tera_meter_10.texture_views.iter().zip(&render_executor_scale_tera_meter_10.param_buffers).zip(&render_executor_scale_tera_meter_10.texture_handles);
                            let big_loop_iter_scale_tera_meter_100 = render_executor_scale_tera_meter_100.texture_views.iter().zip(&render_executor_scale_tera_meter_100.param_buffers).zip(&render_executor_scale_tera_meter_100.texture_handles);
                            let big_loop_iter_scale_peta_meter_1 = render_executor_scale_peta_meter_1.texture_views.iter().zip(&render_executor_scale_peta_meter_1.param_buffers).zip(&render_executor_scale_peta_meter_1.texture_handles);
                            let big_loop_iter_scale_peta_meter_10 = render_executor_scale_peta_meter_10.texture_views.iter().zip(&render_executor_scale_peta_meter_10.param_buffers).zip(&render_executor_scale_peta_meter_10.texture_handles);
                            let big_loop_iter_scale_peta_meter_100 = render_executor_scale_peta_meter_100.texture_views.iter().zip(&render_executor_scale_peta_meter_100.param_buffers).zip(&render_executor_scale_peta_meter_100.texture_handles);
                            let big_loop_iter_scale_exa_meter_1 = render_executor_scale_exa_meter_1.texture_views.iter().zip(&render_executor_scale_exa_meter_1.param_buffers).zip(&render_executor_scale_exa_meter_1.texture_handles);
                            let big_loop_iter_scale_exa_meter_10 = render_executor_scale_exa_meter_10.texture_views.iter().zip(&render_executor_scale_exa_meter_10.param_buffers).zip(&render_executor_scale_exa_meter_10.texture_handles);
                            let big_loop_iter_scale_exa_meter_100 = render_executor_scale_exa_meter_100.texture_views.iter().zip(&render_executor_scale_exa_meter_100.param_buffers).zip(&render_executor_scale_exa_meter_100.texture_handles);
                            let big_loop_iter_scale_zetta_meter_1 = render_executor_scale_zetta_meter_1.texture_views.iter().zip(&render_executor_scale_zetta_meter_1.param_buffers).zip(&render_executor_scale_zetta_meter_1.texture_handles);
                            let big_loop_iter_scale_zetta_meter_10 = render_executor_scale_zetta_meter_10.texture_views.iter().zip(&render_executor_scale_zetta_meter_10.param_buffers).zip(&render_executor_scale_zetta_meter_10.texture_handles);
                            let big_loop_iter_scale_zetta_meter_100 = render_executor_scale_zetta_meter_100.texture_views.iter().zip(&render_executor_scale_zetta_meter_100.param_buffers).zip(&render_executor_scale_zetta_meter_100.texture_handles);
                            let big_loop_iter_scale_yotta_meter_1 = render_executor_scale_yotta_meter_1.texture_views.iter().zip(&render_executor_scale_yotta_meter_1.param_buffers).zip(&render_executor_scale_yotta_meter_1.texture_handles);
                            let big_loop_iter_scale_yotta_meter_10 = render_executor_scale_yotta_meter_10.texture_views.iter().zip(&render_executor_scale_yotta_meter_10.param_buffers).zip(&render_executor_scale_yotta_meter_10.texture_handles);
                            let big_loop_iter_scale_yotta_meter_100 = render_executor_scale_yotta_meter_100.texture_views.iter().zip(&render_executor_scale_yotta_meter_100.param_buffers).zip(&render_executor_scale_yotta_meter_100.texture_handles);
                            let big_loop_iter_scale_ronna_meter_1 = render_executor_scale_ronna_meter_1.texture_views.iter().zip(&render_executor_scale_ronna_meter_1.param_buffers).zip(&render_executor_scale_ronna_meter_1.texture_handles);
                            let big_loop_iter_scale_ronna_meter_10 = render_executor_scale_ronna_meter_10.texture_views.iter().zip(&render_executor_scale_ronna_meter_10.param_buffers).zip(&render_executor_scale_ronna_meter_10.texture_handles);
                            let big_loop_iter_scale_ronna_meter_100 = render_executor_scale_ronna_meter_100.texture_views.iter().zip(&render_executor_scale_ronna_meter_100.param_buffers).zip(&render_executor_scale_ronna_meter_100.texture_handles);
                            let big_loop_iter_scale_quetta_meter_1 = render_executor_scale_quetta_meter_1.texture_views.iter().zip(&render_executor_scale_quetta_meter_1.param_buffers).zip(&render_executor_scale_quetta_meter_1.texture_handles);
                            let big_loop_iter_scale_quetta_meter_10 = render_executor_scale_quetta_meter_10.texture_views.iter().zip(&render_executor_scale_quetta_meter_10.param_buffers).zip(&render_executor_scale_quetta_meter_10.texture_handles);
                            let big_loop_iter_scale_quetta_meter_100 = render_executor_scale_quetta_meter_100.texture_views.iter().zip(&render_executor_scale_quetta_meter_100.param_buffers).zip(&render_executor_scale_quetta_meter_100.texture_handles);
                            let big_loop_iter_scale_quetta_meter_1000 = render_executor_scale_quetta_meter_1000.texture_views.iter().zip(&render_executor_scale_quetta_meter_1000.param_buffers).zip(&render_executor_scale_quetta_meter_1000.texture_handles);
                            let big_loop_iter_scale_quetta_meter_10000 = render_executor_scale_quetta_meter_10000.texture_views.iter().zip(&render_executor_scale_quetta_meter_10000.param_buffers).zip(&render_executor_scale_quetta_meter_10000.texture_handles);
                            let big_loop_iter_scale_quetta_meter_100000 = render_executor_scale_quetta_meter_100000.texture_views.iter().zip(&render_executor_scale_quetta_meter_100000.param_buffers).zip(&render_executor_scale_quetta_meter_100000.texture_handles);

                            let ultra_massive_chungus_iter = vec![
                                big_loop_iter_scale_quecto_meter_000001,
                                big_loop_iter_scale_quecto_meter_00001,
                                big_loop_iter_scale_quecto_meter_0001,
                                big_loop_iter_scale_quecto_meter_001,
                                big_loop_iter_scale_quecto_meter_01,
                                big_loop_iter_scale_quecto_meter_1,
                                big_loop_iter_scale_quecto_meter_10,
                                big_loop_iter_scale_quecto_meter_100,
                                big_loop_iter_scale_ronto_meter_1,
                                big_loop_iter_scale_ronto_meter_10,
                                big_loop_iter_scale_ronto_meter_100,
                                big_loop_iter_scale_yocto_meter_1,
                                big_loop_iter_scale_yocto_meter_10,
                                big_loop_iter_scale_yocto_meter_100,
                                big_loop_iter_scale_zepto_meter_1,
                                big_loop_iter_scale_zepto_meter_10,
                                big_loop_iter_scale_zepto_meter_100,
                                big_loop_iter_scale_atto_meter_1,
                                big_loop_iter_scale_atto_meter_10,
                                big_loop_iter_scale_atto_meter_100,
                                big_loop_iter_scale_femto_meter_1,
                                big_loop_iter_scale_femto_meter_10,
                                big_loop_iter_scale_femto_meter_100,
                                big_loop_iter_scale_pico_meter_1,
                                big_loop_iter_scale_pico_meter_10,
                                big_loop_iter_scale_pico_meter_100,
                                big_loop_iter_scale_nano_meter_1,
                                big_loop_iter_scale_nano_meter_10,
                                big_loop_iter_scale_nano_meter_100,
                                big_loop_iter_scale_micro_meter_1,
                                big_loop_iter_scale_micro_meter_10,
                                big_loop_iter_scale_micro_meter_100,
                                big_loop_iter_scale_milli_meter_1,
                                big_loop_iter_scale_milli_meter_10,
                                big_loop_iter_scale_milli_meter_100,
                                big_loop_iter_scale_meter_1,
                                big_loop_iter_scale_meter_10,
                                big_loop_iter_scale_meter_100,
                                big_loop_iter_scale_kilo_meter_1,
                                big_loop_iter_scale_kilo_meter_10,
                                big_loop_iter_scale_kilo_meter_100,
                                big_loop_iter_scale_mega_meter_1,
                                big_loop_iter_scale_mega_meter_10,
                                big_loop_iter_scale_mega_meter_100,
                                big_loop_iter_scale_giga_meter_1,
                                big_loop_iter_scale_giga_meter_10,
                                big_loop_iter_scale_giga_meter_100,
                                big_loop_iter_scale_tera_meter_1,
                                big_loop_iter_scale_tera_meter_10,
                                big_loop_iter_scale_tera_meter_100,
                                big_loop_iter_scale_peta_meter_1,
                                big_loop_iter_scale_peta_meter_10,
                                big_loop_iter_scale_peta_meter_100,
                                big_loop_iter_scale_exa_meter_1,
                                big_loop_iter_scale_exa_meter_10,
                                big_loop_iter_scale_exa_meter_100,
                                big_loop_iter_scale_zetta_meter_1,
                                big_loop_iter_scale_zetta_meter_10,
                                big_loop_iter_scale_zetta_meter_100,
                                big_loop_iter_scale_yotta_meter_1,
                                big_loop_iter_scale_yotta_meter_10,
                                big_loop_iter_scale_yotta_meter_100,
                                big_loop_iter_scale_ronna_meter_1,
                                big_loop_iter_scale_ronna_meter_10,
                                big_loop_iter_scale_ronna_meter_100,
                                big_loop_iter_scale_quetta_meter_1,
                                big_loop_iter_scale_quetta_meter_10,
                                big_loop_iter_scale_quetta_meter_100,
                                big_loop_iter_scale_quetta_meter_1000,
                                big_loop_iter_scale_quetta_meter_10000,
                                big_loop_iter_scale_quetta_meter_100000,
                            ];

                            let ultra_massive_chungus_iter = ultra_massive_chungus_iter.into_iter().flatten();

                            for ((view, buffer), _handle) in ultra_massive_chungus_iter {
                                let bind_group = render_access.render_device.create_bind_group(
                                    Some("ChunkRender BindGroup"),
                                    &shared_render_executor.bind_group_layout,
                                    &[
                                        BindGroupEntry {
                                            binding: 0,
                                            resource: BindingResource::TextureView(view),
                                        },
                                        BindGroupEntry {
                                            binding: 1,
                                            resource: buffer.as_entire_binding(),
                                        },
                                    ],
                                );

                                let size = shared_render_executor.texture_size;
                                let (width, height) = (size, size);
                                let workgroup_x = CONFIG().get::<u32>("gpu/texture_generator/workgroup_size_x");
                                let workgroup_y = CONFIG().get::<u32>("gpu/texture_generator/workgroup_size_y");

                                let dispatch_x = width.div_ceil(workgroup_x);
                                let dispatch_y = height.div_ceil(workgroup_y);

                                let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                                    label: Some("ChunkRender ComputePass"),
                                    timestamp_writes: None,
                                });

                                pass.set_pipeline(pipeline);
                                pass.set_bind_group(0, &bind_group, &[]);
                                pass.dispatch_workgroups(dispatch_x, dispatch_y, 1);
                            }

                            render_access.queue.submit(Some(encoder.finish()));

                            let (sender, receiver) = crossbeam_channel::unbounded();
                            render_access.queue.on_submitted_work_done(move || {
                                let _ = sender.send(());
                            });

                            shared_render_executor.receiver = Some(receiver);

                            Output {
                                shared_render_executor,
                                render_executor_scale_quecto_meter_000001: render_executor_scale_quecto_meter_000001.clone(),
                                render_executor_scale_quecto_meter_00001: render_executor_scale_quecto_meter_00001.clone(),
                                render_executor_scale_quecto_meter_0001: render_executor_scale_quecto_meter_0001.clone(),
                                render_executor_scale_quecto_meter_001: render_executor_scale_quecto_meter_001.clone(),
                                render_executor_scale_quecto_meter_01: render_executor_scale_quecto_meter_01.clone(),
                                render_executor_scale_quecto_meter_1: render_executor_scale_quecto_meter_1.clone(),
                                render_executor_scale_quecto_meter_10: render_executor_scale_quecto_meter_10.clone(),
                                render_executor_scale_quecto_meter_100: render_executor_scale_quecto_meter_100.clone(),
                                render_executor_scale_ronto_meter_1: render_executor_scale_ronto_meter_1.clone(),
                                render_executor_scale_ronto_meter_10: render_executor_scale_ronto_meter_10.clone(),
                                render_executor_scale_ronto_meter_100: render_executor_scale_ronto_meter_100.clone(),
                                render_executor_scale_yocto_meter_1: render_executor_scale_yocto_meter_1.clone(),
                                render_executor_scale_yocto_meter_10: render_executor_scale_yocto_meter_10.clone(),
                                render_executor_scale_yocto_meter_100: render_executor_scale_yocto_meter_100.clone(),
                                render_executor_scale_zepto_meter_1: render_executor_scale_zepto_meter_1.clone(),
                                render_executor_scale_zepto_meter_10: render_executor_scale_zepto_meter_10.clone(),
                                render_executor_scale_zepto_meter_100: render_executor_scale_zepto_meter_100.clone(),
                                render_executor_scale_atto_meter_1: render_executor_scale_atto_meter_1.clone(),
                                render_executor_scale_atto_meter_10: render_executor_scale_atto_meter_10.clone(),
                                render_executor_scale_atto_meter_100: render_executor_scale_atto_meter_100.clone(),
                                render_executor_scale_femto_meter_1: render_executor_scale_femto_meter_1.clone(),
                                render_executor_scale_femto_meter_10: render_executor_scale_femto_meter_10.clone(),
                                render_executor_scale_femto_meter_100: render_executor_scale_femto_meter_100.clone(),
                                render_executor_scale_pico_meter_1: render_executor_scale_pico_meter_1.clone(),
                                render_executor_scale_pico_meter_10: render_executor_scale_pico_meter_10.clone(),
                                render_executor_scale_pico_meter_100: render_executor_scale_pico_meter_100.clone(),
                                render_executor_scale_nano_meter_1: render_executor_scale_nano_meter_1.clone(),
                                render_executor_scale_nano_meter_10: render_executor_scale_nano_meter_10.clone(),
                                render_executor_scale_nano_meter_100: render_executor_scale_nano_meter_100.clone(),
                                render_executor_scale_micro_meter_1: render_executor_scale_micro_meter_1.clone(),
                                render_executor_scale_micro_meter_10: render_executor_scale_micro_meter_10.clone(),
                                render_executor_scale_micro_meter_100: render_executor_scale_micro_meter_100.clone(),
                                render_executor_scale_milli_meter_1: render_executor_scale_milli_meter_1.clone(),
                                render_executor_scale_milli_meter_10: render_executor_scale_milli_meter_10.clone(),
                                render_executor_scale_milli_meter_100: render_executor_scale_milli_meter_100.clone(),
                                render_executor_scale_meter_1: render_executor_scale_meter_1.clone(),
                                render_executor_scale_meter_10: render_executor_scale_meter_10.clone(),
                                render_executor_scale_meter_100: render_executor_scale_meter_100.clone(),
                                render_executor_scale_kilo_meter_1: render_executor_scale_kilo_meter_1.clone(),
                                render_executor_scale_kilo_meter_10: render_executor_scale_kilo_meter_10.clone(),
                                render_executor_scale_kilo_meter_100: render_executor_scale_kilo_meter_100.clone(),
                                render_executor_scale_mega_meter_1: render_executor_scale_mega_meter_1.clone(),
                                render_executor_scale_mega_meter_10: render_executor_scale_mega_meter_10.clone(),
                                render_executor_scale_mega_meter_100: render_executor_scale_mega_meter_100.clone(),
                                render_executor_scale_giga_meter_1: render_executor_scale_giga_meter_1.clone(),
                                render_executor_scale_giga_meter_10: render_executor_scale_giga_meter_10.clone(),
                                render_executor_scale_giga_meter_100: render_executor_scale_giga_meter_100.clone(),
                                render_executor_scale_tera_meter_1: render_executor_scale_tera_meter_1.clone(),
                                render_executor_scale_tera_meter_10: render_executor_scale_tera_meter_10.clone(),
                                render_executor_scale_tera_meter_100: render_executor_scale_tera_meter_100.clone(),
                                render_executor_scale_peta_meter_1: render_executor_scale_peta_meter_1.clone(),
                                render_executor_scale_peta_meter_10: render_executor_scale_peta_meter_10.clone(),
                                render_executor_scale_peta_meter_100: render_executor_scale_peta_meter_100.clone(),
                                render_executor_scale_exa_meter_1: render_executor_scale_exa_meter_1.clone(),
                                render_executor_scale_exa_meter_10: render_executor_scale_exa_meter_10.clone(),
                                render_executor_scale_exa_meter_100: render_executor_scale_exa_meter_100.clone(),
                                render_executor_scale_zetta_meter_1: render_executor_scale_zetta_meter_1.clone(),
                                render_executor_scale_zetta_meter_10: render_executor_scale_zetta_meter_10.clone(),
                                render_executor_scale_zetta_meter_100: render_executor_scale_zetta_meter_100.clone(),
                                render_executor_scale_yotta_meter_1: render_executor_scale_yotta_meter_1.clone(),
                                render_executor_scale_yotta_meter_10: render_executor_scale_yotta_meter_10.clone(),
                                render_executor_scale_yotta_meter_100: render_executor_scale_yotta_meter_100.clone(),
                                render_executor_scale_ronna_meter_1: render_executor_scale_ronna_meter_1.clone(),
                                render_executor_scale_ronna_meter_10: render_executor_scale_ronna_meter_10.clone(),
                                render_executor_scale_ronna_meter_100: render_executor_scale_ronna_meter_100.clone(),
                                render_executor_scale_quetta_meter_1: render_executor_scale_quetta_meter_1.clone(),
                                render_executor_scale_quetta_meter_10: render_executor_scale_quetta_meter_10.clone(),
                                render_executor_scale_quetta_meter_100: render_executor_scale_quetta_meter_100.clone(),
                                render_executor_scale_quetta_meter_1000: render_executor_scale_quetta_meter_1000.clone(),
                                render_executor_scale_quetta_meter_10000: render_executor_scale_quetta_meter_10000.clone(),
                                render_executor_scale_quetta_meter_100000: render_executor_scale_quetta_meter_100000.clone(),
                            }
                        }
                    ]
                }

                WaitForTexturesReady: EcsWhile, run_if_paused: false, run_after_startup_finished: false {
                    core_types: [
                        struct MainAccess {}

                        struct Input {
                            shared_render_executor: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_000001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_00001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_0001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_01: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_1: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_10: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_100: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_1: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_10: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_100: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_1: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_10: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_100: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_1: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_10: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_100: ChunkRenderExecutor,
                            render_executor_scale_meter_1: ChunkRenderExecutor,
                            render_executor_scale_meter_10: ChunkRenderExecutor,
                            render_executor_scale_meter_100: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_1: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_10: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_100: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_1: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_10: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_100: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_1: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_10: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_100: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_1: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_10: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_100: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_1: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_10: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_100: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_1: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_10: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_100: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_1000: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_10000: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_100000: ChunkRenderExecutor,
                        }

                        struct State {
                            shared_render_executor: Progress<ChunkRenderExecutor, ChunkRenderExecutor>,
                            render_executor_scale_quecto_meter_000001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_00001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_0001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_01: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_1: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_10: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_100: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_1: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_10: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_100: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_1: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_10: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_100: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_1: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_10: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_100: ChunkRenderExecutor,
                            render_executor_scale_meter_1: ChunkRenderExecutor,
                            render_executor_scale_meter_10: ChunkRenderExecutor,
                            render_executor_scale_meter_100: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_1: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_10: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_100: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_1: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_10: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_100: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_1: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_10: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_100: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_1: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_10: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_100: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_1: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_10: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_100: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_1: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_10: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_100: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_1000: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_10000: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_100000: ChunkRenderExecutor,
                        }

                        struct Output {
                            render_executor_scale_quecto_meter_000001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_00001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_0001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_001: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_01: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_quecto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_ronto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_yocto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_zepto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_atto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_1: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_10: ChunkRenderExecutor,
                            render_executor_scale_femto_meter_100: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_1: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_10: ChunkRenderExecutor,
                            render_executor_scale_pico_meter_100: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_1: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_10: ChunkRenderExecutor,
                            render_executor_scale_nano_meter_100: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_1: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_10: ChunkRenderExecutor,
                            render_executor_scale_micro_meter_100: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_1: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_10: ChunkRenderExecutor,
                            render_executor_scale_milli_meter_100: ChunkRenderExecutor,
                            render_executor_scale_meter_1: ChunkRenderExecutor,
                            render_executor_scale_meter_10: ChunkRenderExecutor,
                            render_executor_scale_meter_100: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_1: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_10: ChunkRenderExecutor,
                            render_executor_scale_kilo_meter_100: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_1: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_10: ChunkRenderExecutor,
                            render_executor_scale_mega_meter_100: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_1: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_10: ChunkRenderExecutor,
                            render_executor_scale_giga_meter_100: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_1: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_10: ChunkRenderExecutor,
                            render_executor_scale_tera_meter_100: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_peta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_1: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_10: ChunkRenderExecutor,
                            render_executor_scale_exa_meter_100: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_zetta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_yotta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_1: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_10: ChunkRenderExecutor,
                            render_executor_scale_ronna_meter_100: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_1: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_10: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_100: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_1000: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_10000: ChunkRenderExecutor,
                            render_executor_scale_quetta_meter_100000: ChunkRenderExecutor,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> State {
                            State {
                                shared_render_executor: Progress::Unfinished(input.shared_render_executor),
                                render_executor_scale_quecto_meter_000001: input.render_executor_scale_quecto_meter_000001,
                                render_executor_scale_quecto_meter_00001: input.render_executor_scale_quecto_meter_00001,
                                render_executor_scale_quecto_meter_0001: input.render_executor_scale_quecto_meter_0001,
                                render_executor_scale_quecto_meter_001: input.render_executor_scale_quecto_meter_001,
                                render_executor_scale_quecto_meter_01: input.render_executor_scale_quecto_meter_01,
                                render_executor_scale_quecto_meter_1: input.render_executor_scale_quecto_meter_1,
                                render_executor_scale_quecto_meter_10: input.render_executor_scale_quecto_meter_10,
                                render_executor_scale_quecto_meter_100: input.render_executor_scale_quecto_meter_100,
                                render_executor_scale_ronto_meter_1: input.render_executor_scale_ronto_meter_1,
                                render_executor_scale_ronto_meter_10: input.render_executor_scale_ronto_meter_10,
                                render_executor_scale_ronto_meter_100: input.render_executor_scale_ronto_meter_100,
                                render_executor_scale_yocto_meter_1: input.render_executor_scale_yocto_meter_1,
                                render_executor_scale_yocto_meter_10: input.render_executor_scale_yocto_meter_10,
                                render_executor_scale_yocto_meter_100: input.render_executor_scale_yocto_meter_100,
                                render_executor_scale_zepto_meter_1: input.render_executor_scale_zepto_meter_1,
                                render_executor_scale_zepto_meter_10: input.render_executor_scale_zepto_meter_10,
                                render_executor_scale_zepto_meter_100: input.render_executor_scale_zepto_meter_100,
                                render_executor_scale_atto_meter_1: input.render_executor_scale_atto_meter_1,
                                render_executor_scale_atto_meter_10: input.render_executor_scale_atto_meter_10,
                                render_executor_scale_atto_meter_100: input.render_executor_scale_atto_meter_100,
                                render_executor_scale_femto_meter_1: input.render_executor_scale_femto_meter_1,
                                render_executor_scale_femto_meter_10: input.render_executor_scale_femto_meter_10,
                                render_executor_scale_femto_meter_100: input.render_executor_scale_femto_meter_100,
                                render_executor_scale_pico_meter_1: input.render_executor_scale_pico_meter_1,
                                render_executor_scale_pico_meter_10: input.render_executor_scale_pico_meter_10,
                                render_executor_scale_pico_meter_100: input.render_executor_scale_pico_meter_100,
                                render_executor_scale_nano_meter_1: input.render_executor_scale_nano_meter_1,
                                render_executor_scale_nano_meter_10: input.render_executor_scale_nano_meter_10,
                                render_executor_scale_nano_meter_100: input.render_executor_scale_nano_meter_100,
                                render_executor_scale_micro_meter_1: input.render_executor_scale_micro_meter_1,
                                render_executor_scale_micro_meter_10: input.render_executor_scale_micro_meter_10,
                                render_executor_scale_micro_meter_100: input.render_executor_scale_micro_meter_100,
                                render_executor_scale_milli_meter_1: input.render_executor_scale_milli_meter_1,
                                render_executor_scale_milli_meter_10: input.render_executor_scale_milli_meter_10,
                                render_executor_scale_milli_meter_100: input.render_executor_scale_milli_meter_100,
                                render_executor_scale_meter_1: input.render_executor_scale_meter_1,
                                render_executor_scale_meter_10: input.render_executor_scale_meter_10,
                                render_executor_scale_meter_100: input.render_executor_scale_meter_100,
                                render_executor_scale_kilo_meter_1: input.render_executor_scale_kilo_meter_1,
                                render_executor_scale_kilo_meter_10: input.render_executor_scale_kilo_meter_10,
                                render_executor_scale_kilo_meter_100: input.render_executor_scale_kilo_meter_100,
                                render_executor_scale_mega_meter_1: input.render_executor_scale_mega_meter_1,
                                render_executor_scale_mega_meter_10: input.render_executor_scale_mega_meter_10,
                                render_executor_scale_mega_meter_100: input.render_executor_scale_mega_meter_100,
                                render_executor_scale_giga_meter_1: input.render_executor_scale_giga_meter_1,
                                render_executor_scale_giga_meter_10: input.render_executor_scale_giga_meter_10,
                                render_executor_scale_giga_meter_100: input.render_executor_scale_giga_meter_100,
                                render_executor_scale_tera_meter_1: input.render_executor_scale_tera_meter_1,
                                render_executor_scale_tera_meter_10: input.render_executor_scale_tera_meter_10,
                                render_executor_scale_tera_meter_100: input.render_executor_scale_tera_meter_100,
                                render_executor_scale_peta_meter_1: input.render_executor_scale_peta_meter_1,
                                render_executor_scale_peta_meter_10: input.render_executor_scale_peta_meter_10,
                                render_executor_scale_peta_meter_100: input.render_executor_scale_peta_meter_100,
                                render_executor_scale_exa_meter_1: input.render_executor_scale_exa_meter_1,
                                render_executor_scale_exa_meter_10: input.render_executor_scale_exa_meter_10,
                                render_executor_scale_exa_meter_100: input.render_executor_scale_exa_meter_100,
                                render_executor_scale_zetta_meter_1: input.render_executor_scale_zetta_meter_1,
                                render_executor_scale_zetta_meter_10: input.render_executor_scale_zetta_meter_10,
                                render_executor_scale_zetta_meter_100: input.render_executor_scale_zetta_meter_100,
                                render_executor_scale_yotta_meter_1: input.render_executor_scale_yotta_meter_1,
                                render_executor_scale_yotta_meter_10: input.render_executor_scale_yotta_meter_10,
                                render_executor_scale_yotta_meter_100: input.render_executor_scale_yotta_meter_100,
                                render_executor_scale_ronna_meter_1: input.render_executor_scale_ronna_meter_1,
                                render_executor_scale_ronna_meter_10: input.render_executor_scale_ronna_meter_10,
                                render_executor_scale_ronna_meter_100: input.render_executor_scale_ronna_meter_100,
                                render_executor_scale_quetta_meter_1: input.render_executor_scale_quetta_meter_1,
                                render_executor_scale_quetta_meter_10: input.render_executor_scale_quetta_meter_10,
                                render_executor_scale_quetta_meter_100: input.render_executor_scale_quetta_meter_100,
                                render_executor_scale_quetta_meter_1000: input.render_executor_scale_quetta_meter_1000,
                                render_executor_scale_quetta_meter_10000: input.render_executor_scale_quetta_meter_10000,
                                render_executor_scale_quetta_meter_100000: input.render_executor_scale_quetta_meter_100000,
                            }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, Output> {
                            let progress = match state.shared_render_executor.unfinished_as_ref().unwrap().receiver.as_ref().expect("Render shared_render_executor did not include a GPU completion receiver").try_recv() {
                                Ok(_) => Progress::Finished(state),
                                Err(crossbeam_channel::TryRecvError::Empty) => Progress::Unfinished(state),
                                Err(e) => panic!("Render texture GPU dispatch failed: {}", e),
                            };
                            
                            match progress {
                                Progress::Finished(state) => Done(Output {
                                    render_executor_scale_quecto_meter_000001: state.render_executor_scale_quecto_meter_000001,
                                    render_executor_scale_quecto_meter_00001: state.render_executor_scale_quecto_meter_00001,
                                    render_executor_scale_quecto_meter_0001: state.render_executor_scale_quecto_meter_0001,
                                    render_executor_scale_quecto_meter_001: state.render_executor_scale_quecto_meter_001,
                                    render_executor_scale_quecto_meter_01: state.render_executor_scale_quecto_meter_01,
                                    render_executor_scale_quecto_meter_1: state.render_executor_scale_quecto_meter_1,
                                    render_executor_scale_quecto_meter_10: state.render_executor_scale_quecto_meter_10,
                                    render_executor_scale_quecto_meter_100: state.render_executor_scale_quecto_meter_100,
                                    render_executor_scale_ronto_meter_1: state.render_executor_scale_ronto_meter_1,
                                    render_executor_scale_ronto_meter_10: state.render_executor_scale_ronto_meter_10,
                                    render_executor_scale_ronto_meter_100: state.render_executor_scale_ronto_meter_100,
                                    render_executor_scale_yocto_meter_1: state.render_executor_scale_yocto_meter_1,
                                    render_executor_scale_yocto_meter_10: state.render_executor_scale_yocto_meter_10,
                                    render_executor_scale_yocto_meter_100: state.render_executor_scale_yocto_meter_100,
                                    render_executor_scale_zepto_meter_1: state.render_executor_scale_zepto_meter_1,
                                    render_executor_scale_zepto_meter_10: state.render_executor_scale_zepto_meter_10,
                                    render_executor_scale_zepto_meter_100: state.render_executor_scale_zepto_meter_100,
                                    render_executor_scale_atto_meter_1: state.render_executor_scale_atto_meter_1,
                                    render_executor_scale_atto_meter_10: state.render_executor_scale_atto_meter_10,
                                    render_executor_scale_atto_meter_100: state.render_executor_scale_atto_meter_100,
                                    render_executor_scale_femto_meter_1: state.render_executor_scale_femto_meter_1,
                                    render_executor_scale_femto_meter_10: state.render_executor_scale_femto_meter_10,
                                    render_executor_scale_femto_meter_100: state.render_executor_scale_femto_meter_100,
                                    render_executor_scale_pico_meter_1: state.render_executor_scale_pico_meter_1,
                                    render_executor_scale_pico_meter_10: state.render_executor_scale_pico_meter_10,
                                    render_executor_scale_pico_meter_100: state.render_executor_scale_pico_meter_100,
                                    render_executor_scale_nano_meter_1: state.render_executor_scale_nano_meter_1,
                                    render_executor_scale_nano_meter_10: state.render_executor_scale_nano_meter_10,
                                    render_executor_scale_nano_meter_100: state.render_executor_scale_nano_meter_100,
                                    render_executor_scale_micro_meter_1: state.render_executor_scale_micro_meter_1,
                                    render_executor_scale_micro_meter_10: state.render_executor_scale_micro_meter_10,
                                    render_executor_scale_micro_meter_100: state.render_executor_scale_micro_meter_100,
                                    render_executor_scale_milli_meter_1: state.render_executor_scale_milli_meter_1,
                                    render_executor_scale_milli_meter_10: state.render_executor_scale_milli_meter_10,
                                    render_executor_scale_milli_meter_100: state.render_executor_scale_milli_meter_100,
                                    render_executor_scale_meter_1: state.render_executor_scale_meter_1,
                                    render_executor_scale_meter_10: state.render_executor_scale_meter_10,
                                    render_executor_scale_meter_100: state.render_executor_scale_meter_100,
                                    render_executor_scale_kilo_meter_1: state.render_executor_scale_kilo_meter_1,
                                    render_executor_scale_kilo_meter_10: state.render_executor_scale_kilo_meter_10,
                                    render_executor_scale_kilo_meter_100: state.render_executor_scale_kilo_meter_100,
                                    render_executor_scale_mega_meter_1: state.render_executor_scale_mega_meter_1,
                                    render_executor_scale_mega_meter_10: state.render_executor_scale_mega_meter_10,
                                    render_executor_scale_mega_meter_100: state.render_executor_scale_mega_meter_100,
                                    render_executor_scale_giga_meter_1: state.render_executor_scale_giga_meter_1,
                                    render_executor_scale_giga_meter_10: state.render_executor_scale_giga_meter_10,
                                    render_executor_scale_giga_meter_100: state.render_executor_scale_giga_meter_100,
                                    render_executor_scale_tera_meter_1: state.render_executor_scale_tera_meter_1,
                                    render_executor_scale_tera_meter_10: state.render_executor_scale_tera_meter_10,
                                    render_executor_scale_tera_meter_100: state.render_executor_scale_tera_meter_100,
                                    render_executor_scale_peta_meter_1: state.render_executor_scale_peta_meter_1,
                                    render_executor_scale_peta_meter_10: state.render_executor_scale_peta_meter_10,
                                    render_executor_scale_peta_meter_100: state.render_executor_scale_peta_meter_100,
                                    render_executor_scale_exa_meter_1: state.render_executor_scale_exa_meter_1,
                                    render_executor_scale_exa_meter_10: state.render_executor_scale_exa_meter_10,
                                    render_executor_scale_exa_meter_100: state.render_executor_scale_exa_meter_100,
                                    render_executor_scale_zetta_meter_1: state.render_executor_scale_zetta_meter_1,
                                    render_executor_scale_zetta_meter_10: state.render_executor_scale_zetta_meter_10,
                                    render_executor_scale_zetta_meter_100: state.render_executor_scale_zetta_meter_100,
                                    render_executor_scale_yotta_meter_1: state.render_executor_scale_yotta_meter_1,
                                    render_executor_scale_yotta_meter_10: state.render_executor_scale_yotta_meter_10,
                                    render_executor_scale_yotta_meter_100: state.render_executor_scale_yotta_meter_100,
                                    render_executor_scale_ronna_meter_1: state.render_executor_scale_ronna_meter_1,
                                    render_executor_scale_ronna_meter_10: state.render_executor_scale_ronna_meter_10,
                                    render_executor_scale_ronna_meter_100: state.render_executor_scale_ronna_meter_100,
                                    render_executor_scale_quetta_meter_1: state.render_executor_scale_quetta_meter_1,
                                    render_executor_scale_quetta_meter_10: state.render_executor_scale_quetta_meter_10,
                                    render_executor_scale_quetta_meter_100: state.render_executor_scale_quetta_meter_100,
                                    render_executor_scale_quetta_meter_1000: state.render_executor_scale_quetta_meter_1000,
                                    render_executor_scale_quetta_meter_10000: state.render_executor_scale_quetta_meter_10000,
                                    render_executor_scale_quetta_meter_100000: state.render_executor_scale_quetta_meter_100000,
                                }),
                                Progress::Unfinished(state) => Wait(State {
                                    shared_render_executor: state.shared_render_executor,
                                    render_executor_scale_quecto_meter_000001: state.render_executor_scale_quecto_meter_000001,
                                    render_executor_scale_quecto_meter_00001: state.render_executor_scale_quecto_meter_00001,
                                    render_executor_scale_quecto_meter_0001: state.render_executor_scale_quecto_meter_0001,
                                    render_executor_scale_quecto_meter_001: state.render_executor_scale_quecto_meter_001,
                                    render_executor_scale_quecto_meter_01: state.render_executor_scale_quecto_meter_01,
                                    render_executor_scale_quecto_meter_1: state.render_executor_scale_quecto_meter_1,
                                    render_executor_scale_quecto_meter_10: state.render_executor_scale_quecto_meter_10,
                                    render_executor_scale_quecto_meter_100: state.render_executor_scale_quecto_meter_100,
                                    render_executor_scale_ronto_meter_1: state.render_executor_scale_ronto_meter_1,
                                    render_executor_scale_ronto_meter_10: state.render_executor_scale_ronto_meter_10,
                                    render_executor_scale_ronto_meter_100: state.render_executor_scale_ronto_meter_100,
                                    render_executor_scale_yocto_meter_1: state.render_executor_scale_yocto_meter_1,
                                    render_executor_scale_yocto_meter_10: state.render_executor_scale_yocto_meter_10,
                                    render_executor_scale_yocto_meter_100: state.render_executor_scale_yocto_meter_100,
                                    render_executor_scale_zepto_meter_1: state.render_executor_scale_zepto_meter_1,
                                    render_executor_scale_zepto_meter_10: state.render_executor_scale_zepto_meter_10,
                                    render_executor_scale_zepto_meter_100: state.render_executor_scale_zepto_meter_100,
                                    render_executor_scale_atto_meter_1: state.render_executor_scale_atto_meter_1,
                                    render_executor_scale_atto_meter_10: state.render_executor_scale_atto_meter_10,
                                    render_executor_scale_atto_meter_100: state.render_executor_scale_atto_meter_100,
                                    render_executor_scale_femto_meter_1: state.render_executor_scale_femto_meter_1,
                                    render_executor_scale_femto_meter_10: state.render_executor_scale_femto_meter_10,
                                    render_executor_scale_femto_meter_100: state.render_executor_scale_femto_meter_100,
                                    render_executor_scale_pico_meter_1: state.render_executor_scale_pico_meter_1,
                                    render_executor_scale_pico_meter_10: state.render_executor_scale_pico_meter_10,
                                    render_executor_scale_pico_meter_100: state.render_executor_scale_pico_meter_100,
                                    render_executor_scale_nano_meter_1: state.render_executor_scale_nano_meter_1,
                                    render_executor_scale_nano_meter_10: state.render_executor_scale_nano_meter_10,
                                    render_executor_scale_nano_meter_100: state.render_executor_scale_nano_meter_100,
                                    render_executor_scale_micro_meter_1: state.render_executor_scale_micro_meter_1,
                                    render_executor_scale_micro_meter_10: state.render_executor_scale_micro_meter_10,
                                    render_executor_scale_micro_meter_100: state.render_executor_scale_micro_meter_100,
                                    render_executor_scale_milli_meter_1: state.render_executor_scale_milli_meter_1,
                                    render_executor_scale_milli_meter_10: state.render_executor_scale_milli_meter_10,
                                    render_executor_scale_milli_meter_100: state.render_executor_scale_milli_meter_100,
                                    render_executor_scale_meter_1: state.render_executor_scale_meter_1,
                                    render_executor_scale_meter_10: state.render_executor_scale_meter_10,
                                    render_executor_scale_meter_100: state.render_executor_scale_meter_100,
                                    render_executor_scale_kilo_meter_1: state.render_executor_scale_kilo_meter_1,
                                    render_executor_scale_kilo_meter_10: state.render_executor_scale_kilo_meter_10,
                                    render_executor_scale_kilo_meter_100: state.render_executor_scale_kilo_meter_100,
                                    render_executor_scale_mega_meter_1: state.render_executor_scale_mega_meter_1,
                                    render_executor_scale_mega_meter_10: state.render_executor_scale_mega_meter_10,
                                    render_executor_scale_mega_meter_100: state.render_executor_scale_mega_meter_100,
                                    render_executor_scale_giga_meter_1: state.render_executor_scale_giga_meter_1,
                                    render_executor_scale_giga_meter_10: state.render_executor_scale_giga_meter_10,
                                    render_executor_scale_giga_meter_100: state.render_executor_scale_giga_meter_100,
                                    render_executor_scale_tera_meter_1: state.render_executor_scale_tera_meter_1,
                                    render_executor_scale_tera_meter_10: state.render_executor_scale_tera_meter_10,
                                    render_executor_scale_tera_meter_100: state.render_executor_scale_tera_meter_100,
                                    render_executor_scale_peta_meter_1: state.render_executor_scale_peta_meter_1,
                                    render_executor_scale_peta_meter_10: state.render_executor_scale_peta_meter_10,
                                    render_executor_scale_peta_meter_100: state.render_executor_scale_peta_meter_100,
                                    render_executor_scale_exa_meter_1: state.render_executor_scale_exa_meter_1,
                                    render_executor_scale_exa_meter_10: state.render_executor_scale_exa_meter_10,
                                    render_executor_scale_exa_meter_100: state.render_executor_scale_exa_meter_100,
                                    render_executor_scale_zetta_meter_1: state.render_executor_scale_zetta_meter_1,
                                    render_executor_scale_zetta_meter_10: state.render_executor_scale_zetta_meter_10,
                                    render_executor_scale_zetta_meter_100: state.render_executor_scale_zetta_meter_100,
                                    render_executor_scale_yotta_meter_1: state.render_executor_scale_yotta_meter_1,
                                    render_executor_scale_yotta_meter_10: state.render_executor_scale_yotta_meter_10,
                                    render_executor_scale_yotta_meter_100: state.render_executor_scale_yotta_meter_100,
                                    render_executor_scale_ronna_meter_1: state.render_executor_scale_ronna_meter_1,
                                    render_executor_scale_ronna_meter_10: state.render_executor_scale_ronna_meter_10,
                                    render_executor_scale_ronna_meter_100: state.render_executor_scale_ronna_meter_100,
                                    render_executor_scale_quetta_meter_1: state.render_executor_scale_quetta_meter_1,
                                    render_executor_scale_quetta_meter_10: state.render_executor_scale_quetta_meter_10,
                                    render_executor_scale_quetta_meter_100: state.render_executor_scale_quetta_meter_100,
                                    render_executor_scale_quetta_meter_1000: state.render_executor_scale_quetta_meter_1000,
                                    render_executor_scale_quetta_meter_10000: state.render_executor_scale_quetta_meter_10000,
                                    render_executor_scale_quetta_meter_100000: state.render_executor_scale_quetta_meter_100000,
                                })
                            }
                        }
                    ]
                }
            ]
        }
    ]
}
