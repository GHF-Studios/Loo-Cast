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
                            param_data: Vec<ShaderParams>,
                        }
                        struct Output {
                            params: PreparedRenderExecutorInput,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Output {
                            let shader_name = input.shader_name;
                            let render_device = main_access.render_device;
                            let mut images = main_access.images;
                            let shader_registry = main_access.shader_registry;

                            let pipeline_id = *shader_registry.pipelines.get(shader_name)
                                .unwrap_or_else(|| unreachable!("Pipeline for shader '{}' not found", shader_name));
                            let bind_group_layout = shader_registry.bind_group_layouts.get(shader_name)
                                .unwrap_or_else(|| unreachable!("BindGroupLayout for shader '{}' not found", shader_name))
                                .clone();

                            let mut texture_handles = Vec::new();
                            let mut param_buffers = Vec::new();

                            for param in &input.param_data {
                                let texture = Image {
                                    texture_descriptor: TextureDescriptor {
                                        label: Some("RenderTexture"),
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
                                    data: vec![0; (input.texture_size * input.texture_size * 4) as usize].into(),
                                    ..Default::default()
                                };

                                texture_handles.push(images.add(texture));

                                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                                    label: Some("RenderTextureParamBuffer"),
                                    contents: bytemuck::bytes_of(param),
                                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                                });
                                param_buffers.push(buffer);
                            }

                            Output {
                                params: PreparedRenderExecutorInput {
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
                            params: PreparedRenderExecutorInput,
                        }
                        struct State {
                            params: PreparedRenderExecutorInput,
                        }
                        struct Output {
                            executor: ChunkRenderExecutor,
                        }
                    ],
                    core_functions: [
                        fn SetupRenderWhile |input, render_access| -> State {
                            State { params: input.params }
                        }

                        fn RunRenderWhile |state, render_access| -> Outcome<State, Output> {
                            let gpu_images = render_access.gpu_images;

                            let mut texture_views = Vec::with_capacity(state.params.texture_handles.len());

                            for handle in &state.params.texture_handles {
                                match gpu_images.get(handle) {
                                    Some(gpu_image) => {
                                        texture_views.push(gpu_image.texture_view.clone());
                                    },
                                    None => return Wait(state),
                                }
                            }

                            Done(Output {
                                executor: ChunkRenderExecutor {
                                    shader_name: state.params.shader_name,
                                    pipeline_id: state.params.pipeline_id,
                                    bind_group_layout: state.params.bind_group_layout,
                                    texture_size: state.params.texture_size,
                                    texture_handles: state.params.texture_handles,
                                    param_buffers: state.params.param_buffers,
                                    texture_views,
                                    receiver: None
                                }
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
                            executor: ChunkRenderExecutor,
                        }
                        struct Output {
                            executor: ChunkRenderExecutor,
                        }
                    ],
                    core_functions: [
                        fn RunRender |input, render_access| -> Output {
                            let executor = &input.executor;

                            let pipeline = render_access.pipeline_cache
                                .get_compute_pipeline(executor.pipeline_id)
                                .expect("Pipeline not ready");

                            let mut encoder = render_access.render_device.create_command_encoder(&CommandEncoderDescriptor {
                                label: Some("DispatchRenderTextures Encoder"),
                            });

                            for ((view, buffer), _handle) in executor.texture_views.iter().zip(&executor.param_buffers).zip(&executor.texture_handles) {
                                let bind_group = render_access.render_device.create_bind_group(
                                    Some("ChunkRender BindGroup"),
                                    &executor.bind_group_layout,
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

                                let size = executor.texture_size;
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

                            let mut executor = input.executor.clone();
                            executor.receiver = Some(receiver);

                            Output {
                                executor: executor.clone(),
                            }
                        }
                    ]
                }

                WaitForTexturesReady: EcsWhile, run_if_paused: false, run_after_startup_finished: false {
                    core_types: [
                        struct MainAccess {}
                        struct Input {
                            executor: ChunkRenderExecutor,
                        }
                        struct State {
                            executor: ChunkRenderExecutor,
                        }
                        struct Output {
                            executor: ChunkRenderExecutor,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> State {
                            State { executor: input.executor }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, Output> {
                            match &state.executor.receiver {
                                Some(receiver) => {
                                    match receiver.try_recv() {
                                        Ok(_) => Done(Output { executor: state.executor.clone() }),
                                        Err(crossbeam_channel::TryRecvError::Empty) => Wait(state),
                                        Err(_) => panic!("Render texture GPU dispatch failed"),
                                    }
                                }
                                None => {
                                    unreachable!("Render executor did not include a GPU completion receiver");
                                }
                            }
                        }
                    ]
                }
            ]
        }
    ]
}
