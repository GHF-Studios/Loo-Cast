use workflow::prelude::*;

workflow_mod! {
    name: "Gpu", 
    workflows: [
        SetupTextureGenerator {
            user_types: [],
            user_functions: [],
            stages: [
                SetupPhase1: Ecs {
                    core_types: [
                        struct In { 
                            shader_name: &'static str, 
                            shader_path: String 
                        },
                        struct Out {
                            shader_name: &'static str, 
                            shader_handle: Handle<Shader>,
                        },
                        enum Err {
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
                        fn Run |&mut input, world| {
                            let shader_name = input.shader_name;
                            let shader_path = input.shader_path;
        
                            let mut system_state: SystemState<(
                                ResMut<Assets<Shader>>,
                                Res<ShaderRegistry>,
                            )> = SystemState::new(world);
                            let (mut shader_assets, shader_registry) = system_state.get_mut(world);
        
                            if shader_registry.shaders.contains_key(shader_name) {
                                return Err::ShaderAlreadyRegistered { shader_name }
                            }
        
                            let shader_source = std::fs::read_to_string(&shader_path)
                                .map_err(|e| Err::FailedToReadShader { shader_name, error: e })?;
                            
                            let shader = Shader::from_wgsl(shader_source, shader_path.clone());
                            let shader_handle = shader_assets.add(shader);
        
                            Out { shader_name, shader_handle }
                        },
                    ],
                },
    
                SetupPhase2: RenderWhile {
                    core_types: [
                        struct In {
                            shader_name: &'static str, 
                            shader_handle: Handle<Shader>
                        },
                        struct State {
                            shader_name: &'static str, 
                            shader_handle: Handle<Shader>,
                            bind_group_layout: BindGroupLayout, 
                            pipeline_id: CachedComputePipelineId ,
                        }
                        struct Out { 
                            shader_name: &'static str, 
                            shader_handle: Handle<Shader>, 
                            pipeline_id: CachedComputePipelineId 
                            bind_group_layout: BindGroupLayout, 
                        },
                        enum Err {
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
                        fn Setup |input, world| {
                            let shader_name = input.shader_name;
                            let shader_handle = input.shader_handle;
    
                            let mut system_state: SystemState<(
                                Res<RenderDevice>,
                                Res<PipelineCache>,
                                Res<RenderAssets<GpuImage>>
                            )> = SystemState::new(world);
                            let (render_device, pipeline_cache, gpu_images) = system_state.get(world);
    
                            let bind_group_layout = render_device.create_bind_group_layout(
                                Some("Compute Bind Group Layout"),
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
                                label: Some(format!("Pipeline for {}", shader_name).into()),
                                layout: vec![bind_group_layout.clone()],
                                shader: shader_handle.clone(),
                                shader_defs: vec![],
                                entry_point: "main".into(),
                                push_constant_ranges: vec![PushConstantRange {
                                    stages: ShaderStages::COMPUTE,
                                    range: 0..4,
                                }],
                            });

                            State { shader_name, shader_handle, bind_group_layout, pipeline_id }
                        },
                        fn Run |state, world| {
                            match pipeline_cache.get_compute_pipeline_state(pipeline_id) {
                                CachedPipelineState::Queued | CachedPipelineState::Creating(_) => {
                                    Wait {}
                                },
                                CachedPipelineState::Err(err) => {
                                    Err::FailedToCreatePipeline { 
                                        shader_name, 
                                        err 
                                    }
                                },
                                CachedPipelineState::Ok(pipeline) => {
                                    match pipeline {
                                        Pipeline::RenderPipeline(_) => Err::ExpectedComputePipelineGotRenderPipeline {
                                            shader_name: state.shader_name,
                                            pipeline_id: state.pipeline_id
                                        },
                                        Pipeline::ComputePipeline(_) => Out { 
                                            shader_name, 
                                            shader_handle, 
                                            pipeline_id, 
                                            bind_group_layout
                                        }
                                    }
                                },
                            }
                        },
                    ],
                },
    
                SetupPhase3: Ecs {
                    core_types: [
                        struct In { 
                            shader_name: &'static str, 
                            shader_handle: Handle<Shader>, 
                            pipeline_id: CachedComputePipelineId 
                            bind_group_layout: BindGroupLayout, 
                        },
                    ],
                    core_functions: [
                        fn Run |input, world| {
                            let shader_name = input.shader_name;
                            let shader_handle = input.shader_handle;
                            let bind_group_layout = input.bind_group_layout;
                            let pipeline_id = input.pipeline_id;
        
                            let mut shader_registry = SystemState::<ResMut<ShaderRegistry>>::new(world).get_mut(world);
                            
                            shader_registry.shaders.insert(shader_name.to_string(), shader_handle);
                            shader_registry.pipelines.insert(shader_name.to_string(), pipeline_id);
                            shader_registry.bind_group_layouts.insert(shader_name.to_string(), bind_group_layout);
        
                            Out {}
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
                    pub fn on_entry(
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

                    pub fn on_await_texture_view(self, texture_view: TextureView) -> Self<PreparedGenerator> {
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
                    pub fn on_dispatch_compute(self, texture_handle: Handle<Image>, receiver: Receiver<()>) -> Self<DispatchedCompute> {
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
                    pub fn on_exit(self) -> (&'static str, Handle<Image>) {
                        (self.inner.shader_name, self.inner.texture_handle)
                    }
                }
            ],
            user_functions: [],
            stages: [
                PrepareRequest: Ecs {
                    core_types: [
                        struct In {
                            shader_name: &'static str,
                            texture_size: usize,
                            param_data: Vec<f32>,
                        },
                        struct Out {
                            request: GeneratorRequest<GeneratorParams>,
                        },
                        enum Err {
                            GeneratorNotFound {
                                shader_name: &'static str,
                            },
                        },
                    ],
                    core_functions: [
                        fn Run |input, world| {
                            let shader_name = input.shader_name;
                            let texture_size = input.texture_size;
                            let param_data = input.param_data;

                            let mut system_state: SystemState<(
                                ResMut<Assets<Image>>,
                                Res<ShaderRegistry>,
                            )> = SystemState::new(world);
                            let (mut images, shader_registry) = system_state.get_mut(world);

                            if !shader_registry.shaders.get(shader_name) {
                                return Err::GeneratorNotFound { shader_name }
                            }

                            let pipeline_id = shader_registry.pipelines.get(shader_name).unwrap();
                            let bind_group_layout = shader_registry.pipelines.get(shader_name).unwrap();

                            let texture = Image {
                                texture_descriptor: TextureDescriptor {
                                    label: Some("Compute Shader Output Texture"),
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

                            Out { request }
                        },
                    ],
                },
    
                WaitForTextureView: RenderWhile {
                    core_types: [
                        struct In {
                            request: GeneratorRequest<GeneratorParams>,
                        },
                        struct Out {
                            request: GeneratorRequest<PreparedGenerator>,
                        },
                    ],
                    core_functions: [
                        fn Run |input, world| {
                        },
                    ],
                },
    
                DispatchCompute: Render {
                    core_types: [
                        struct In {
                            request: GeneratorRequest<PreparedGenerator>,
                        },
                        struct Out {
                            request: GeneratorRequest<DispatchedCompute>,
                        },
                    ],
                    core_functions: [
                        fn Run |input, world| {
                        },
                    ],
                },
    
                WaitForCompute: EcsWhile {
                    core_types: [
                        struct In {
                            request: GeneratorRequest<DispatchedCompute>,
                        },
                        struct Out {
                            shader_name: &'static str,
                            texture_handle: Handle<Image>,
                        },
                        enum Err {
                            ComputePassReceiverDisconnected {
                                shader_name: &'static str,
                            },
                        },
                    ],
                    core_functions: [
                        fn Run |input, world| {
                        },
                    ],
                },
            ],
        },
    ],
}
