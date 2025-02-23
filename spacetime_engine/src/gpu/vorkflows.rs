use workflow::prelude::*;

workflow_mod! {
    name: "Gpu", 
    workflows: [
        SetupTextureGenerator {
            user_types: [],
            user_functions: [],
            stages: [
                SetupPhase1 {
                    core_types: [
                        struct Input { 
                            shader_name: &'static str, 
                            shader_path: String 
                        },
                        struct Output {
                            shader_name: &'static str, 
                            shader_handle: Handle<Shader>,
                        },
                        enum Error {
                            ShaderAlreadyRegistered { 
                                shader_name: &'static str 
                            },
                            FailedToReadShader { 
                                shader_name: &'static str, 
                                error: std::io::Error 
                            }
                        },
                    ],
                    core_functions: [
                        fn RunEcs |input, world| {
                            let shader_name = input.shader_name;
                            let shader_path = input.shader_path;
        
                            let mut system_state: SystemState<(
                                ResMut<Assets<Shader>>,
                                Res<ShaderRegistry>,
                            )> = SystemState::new(world);
                            let (mut shader_assets, shader_registry) = system_state.get_mut(world);
        
                            if shader_registry.shaders.contains_key(shader_name) {
                                return Err(Error::ShaderAlreadyRegistered { shader_name })
                            }
        
                            let shader_source = std::fs::read_to_string(&shader_path)
                                .map_err(|e| Error::FailedToReadShader { shader_name, error: e })?;
                            
                            let shader = Shader::from_wgsl(shader_source, shader_path.clone());
                            let shader_handle = shader_assets.add(shader);
        
                            Ok(Output { shader_name, shader_handle })
                        },
                    ],
                },
    
                SetupPhase2 {
                    core_types: [
                        struct Input {
                            shader_name: &'static str, 
                            shader_handle: Handle<Shader>
                        },
                        struct State {
                            shader_name: &'static str, 
                            shader_handle: Handle<Shader>,
                            bind_group_layout: BindGroupLayout, 
                            pipeline_id: CachedComputePipelineId,
                        },
                        struct Output { 
                            shader_name: &'static str, 
                            shader_handle: Handle<Shader>, 
                            pipeline_id: CachedComputePipelineId,
                            bind_group_layout: BindGroupLayout, 
                        },
                        enum Error {
                            ExpectedComputePipelineGotRenderPipeline {
                                shader_name: String,
                                pipeline_id: CachedComputePipelineId,
                            },
                            FailedToCreatePipeline {
                                shader_name: String,
                                err: PipelineCacheError,
                            }
                        },
                    ],
                    core_functions: [
                        fn SetupRenderWhile |input, world| {
                            let shader_name = input.shader_name;
                            let shader_handle = input.shader_handle;
    
                            let mut system_state: SystemState<(
                                Res<RenderDevice>,
                                Res<PipelineCache>,
                                Res<RenderAssets<GpuImage>>
                            )> = SystemState::new(world);
                            let (render_device, pipeline_cache, gpu_images) = system_state.get(world);
    
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
                                            ty: BufferBindingType::Storage { read_only: false },
                                            has_dynamic_offset: false,
                                            min_binding_size: None,
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
                            });

                            Ok(State { shader_name, shader_handle, bind_group_layout, pipeline_id })
                        },
                        fn RunRenderWhile |state, world| {
                            match pipeline_cache.get_compute_pipeline_state(pipeline_id) {
                                CachedPipelineState::Queued | CachedPipelineState::Creating(_) => {
                                    Ok(Wait(state))
                                },
                                CachedPipelineState::Err(err) => {
                                    Err(Error::FailedToCreatePipeline { 
                                        shader_name, 
                                        err 
                                    })
                                },
                                CachedPipelineState::Ok(pipeline) => {
                                    match pipeline {
                                        Pipeline::RenderPipeline(_) => Err(Error::ExpectedComputePipelineGotRenderPipeline {
                                            shader_name: state.shader_name,
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
                        },
                    ],
                },
    
                SetupPhase3 {
                    core_types: [
                        struct Input { 
                            shader_name: &'static str, 
                            shader_handle: Handle<Shader>, 
                            pipeline_id: CachedComputePipelineId,
                            bind_group_layout: BindGroupLayout, 
                        },
                    ],
                    core_functions: [
                        fn RunEcs |input, world| {
                            let shader_name = input.shader_name;
                            let shader_handle = input.shader_handle;
                            let bind_group_layout = input.bind_group_layout;
                            let pipeline_id = input.pipeline_id;
        
                            let mut shader_registry = SystemState::<ResMut<ShaderRegistry>>::new(world).get_mut(world);
                            
                            shader_registry.shaders.insert(shader_name.to_string(), shader_handle);
                            shader_registry.pipelines.insert(shader_name.to_string(), pipeline_id);
                            shader_registry.bind_group_layouts.insert(shader_name.to_string(), bind_group_layout);
                        },
                    ],
                }
            ],
        },

        GenerateTexture {
            user_types: [
                struct GeneratorRequest<T> {
                    inner: T
                }
                
                struct GeneratorParams {
                    shader_name: &'static str,
                    pipeline_id: CachedComputePipelineId,
                    bind_group_layout: BindGroupLayout,
                    texture_handle: Handle<Image>,
                    param_data: Vec<f32>,
                }
                impl GeneratorRequest<GeneratorParams> {
                    pub fn new(
                        shader_name: &'static str,
                        pipeline_id: CachedComputePipelineId,
                        bind_group_layout: BindGroupLayout,
                        texture_handle: Handle<Image>,
                        param_data: Vec<f32>,
                    ) -> Self {
                        Self {
                            inner: GeneratorParams {
                                shader_name,
                                pipeline_id,
                                bind_group_layout,
                                texture,
                                param_data,
                            }
                        }
                    }

                    pub fn set_texture_view(self, texture_view: TextureView) -> Self<PreparedGenerator> {
                        Self {
                            inner: PreparedGenerator {
                                shader_name: self.inner.shader_name,
                                pipeline_id: self.inner.pipeline_id,
                                bind_group_layout: self.inner.bind_group_layout,
                                texture_handle: self.inner.texture_handle,
                                texture_view,
                                param_buffer: self.inner.param_buffer,
                            }
                        }
                    }
                }
                
                struct PreparedGenerator {
                    shader_name: &'static str,
                    pipeline_id: CachedComputePipelineId,
                    bind_group_layout: BindGroupLayout,
                    texture_handle: Handle<Image>,
                    texture_view: TextureView,
                    param_buffer: Buffer,
                }
                impl GeneratorRequest<PreparedGenerator> {
                    pub fn track_dispatch(self, texture_handle: Handle<Image>, receiver: Receiver<()>) -> Self<DispatchedCompute> {
                        Self {
                            inner: DispatchedCompute {
                                shader_name: self.inner.shader_name,
                                texture,
                                receiver,
                            }
                        }
                    }
                }
                
                struct DispatchedCompute {
                    shader_name: &'static str,
                    texture_handle: Handle<Image>,
                    receiver: Receiver<()>
                }
                impl GeneratorRequest<DispatchedCompute> {
                    pub fn consume(self) -> (&'static str, Handle<Image>) {
                        (self.inner.shader_name, self.inner.texture_handle)
                    }
                }
            ],
            user_functions: [],
            stages: [
                PrepareRequest {
                    core_types: [
                        struct Input {
                            shader_name: &'static str,
                            texture_size: usize,
                            param_data: Vec<f32>,
                        },
                        struct Output {
                            request: GeneratorRequest<GeneratorParams>,
                        },
                        enum Error {
                            GeneratorNotFound {
                                shader_name: &'static str,
                            },
                        },
                    ],
                    core_functions: [
                        fn RunEcs |input, world| {
                            let shader_name = input.shader_name;
                            let texture_size = input.texture_size;
                            let param_data = input.param_data;

                            let mut system_state: SystemState<(
                                ResMut<Assets<Image>>,
                                Res<ShaderRegistry>,
                            )> = SystemState::new(world);
                            let (mut images, shader_registry) = system_state.get_mut(world);

                            if !shader_registry.shaders.get(shader_name) {
                                return Err(Error::GeneratorNotFound { shader_name })
                            }

                            let pipeline_id = shader_registry.pipelines.get(shader_name).unwrap();
                            let bind_group_layout = shader_registry.bind_group_layouts.get(shader_name).unwrap();

                            let texture = Image {
                                texture_descriptor: TextureDescriptor {
                                    label: Some("Compute Shader Outputput Texture"),
                                    size: Extent3d {
                                        width: texture_size as u32,
                                        height: texture_size as u32,
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
                                data: vec![0; texture_size * texture_size * 4],
                                ..Default::default()
                            };
                            let texture_handle = images.add(texture);

                            let request = GeneratorRequest<GeneratorParams>::new(
                                shader_name, 
                                pipeline_id, 
                                bind_group_layout, 
                                texture_handle, 
                                param_data
                            );

                            Ok(Output { request })
                        },
                    ],
                },
    
                GetTextureView {
                    core_types: [
                        struct Input {
                            request: GeneratorRequest<GeneratorParams>,
                        },
                        struct State {
                            request: GeneratorRequest<GeneratorParams>,
                        },
                        struct Output {
                            request: GeneratorRequest<PreparedGenerator>,
                        },
                    ],
                    core_functions: [
                        fn SetupRenderWhile |input, world| {
                            State { request: input.request }
                        },
                        fn RunRenderWhile |state, world| {
                            let gpu_images = SystemState::<Res<RenderAssets<GpuImage>>>::new(world).get(world);
                
                            if let Some(gpu_image) = gpu_images.get(&state.request.inner.texture_handle) {
                                let texture_view = gpu_image.texture_view.clone();
                
                                let prepared_request = state.request.set_texture_view(texture_view);
                                Done(Output { request: prepared_request })
                            } else {
                                Wait(state)
                            }
                        },
                    ],
                },
    
                DispatchCompute {
                    core_types: [
                        struct Input {
                            request: GeneratorRequest<PreparedGenerator>,
                        },
                        struct Output {
                            request: GeneratorRequest<DispatchedCompute>,
                        },
                    ],
                    core_functions: [
                        fn RunRender |input, world| {
                            let prepared = &input.request.inner;
                            let shader_name = prepared.shader_name;
                            let texture_view = &prepared.texture_view;
                            let bind_group_layout = &prepared.bind_group_layout;
                            let param_buffer = &prepared.param_buffer;
                
                            let mut system_state: SystemState<(
                                Res<RenderDevice>,
                                Res<RenderQueue>,
                                Res<PipelineCache>,
                            )> = SystemState::new(world);
                            let (render_device, queue, pipeline_cache) = system_state.get_mut(world);
                
                            let pipeline = pipeline_cache.get_compute_pipeline(prepared.pipeline_id)
                                .expect("Compute pipeline not found");
                
                            let bind_group = render_device.create_bind_group(
                                Some("Compute Bind Group"),
                                bind_group_layout,
                                &[
                                    BindGroupEntry {
                                        binding: 0,
                                        resource: BindingResource::TextureView(texture_view),
                                    },
                                    BindGroupEntry {
                                        binding: 1,
                                        resource: param_buffer.as_entire_binding(),
                                    },
                                ],
                            );
                
                            let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor { label: None });
                            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor { label: None, timestamp_writes: None });
                
                            compute_pass.set_pipeline(pipeline);
                            compute_pass.set_bind_group(0, &bind_group, &[]);
                            compute_pass.dispatch_workgroups(8, 8, 1);
                            drop(compute_pass);
                
                            queue.submit(Some(encoder.finish()));
                
                            let (sender, receiver) = unbounded();
                            queue.on_submitted_work_done(move || {
                                let _ = sender.send(());
                            });
                
                            let dispatched_request = input.request.track_dispatch(prepared.texture_handle.clone(), receiver);
                            Output { request: dispatched_request }
                        },
                    ],
                },
    
                WaitForCompute {
                    core_types: [
                        struct Input {
                            request: GeneratorRequest<DispatchedCompute>,
                        },
                        struct State {
                            request: GeneratorRequest<DispatchedCompute>,
                        },
                        struct Output {
                            shader_name: &'static str,
                            texture_handle: Handle<Image>,
                        },
                        enum Error {
                            ComputePassReceiverDisconnected {
                                shader_name: &'static str,
                            },
                        },
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, world| {
                            Ok(State { request: input.request })
                        },
                        fn RunEcsWhile |state, world| {
                            let receiver = &state.request.inner.receiver;
                
                            match receiver.try_recv() {
                                Ok(_) => {
                                    let (shader_name, texture_handle) = state.request.consume();
                                    Ok(Done(Output { shader_name, texture_handle }))
                                },
                                Err(crossbeam_channel::TryRecvError::Empty) => {
                                    Ok(Wait(state))
                                },
                                Err(crossbeam_channel::TryRecvError::Disconnected) => {
                                    Err(Error::ComputePassReceiverDisconnected { shader_name: state.request.inner.shader_name })
                                },
                            }
                        },
                    ],
                },
            ],
        },
    ],
}
