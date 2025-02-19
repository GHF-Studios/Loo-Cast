use workflow::prelude::*;

workflow_module!(
    name: "Gpu", 
    workflows: [
        workflow!(
            name: "SetupTextureGenerator", 
            stages: [
                SetupPhase1: Ecs,
                SetupPhase2: Render,
                SetupPhase3: Ecs,
            ],
            flow_graph: [
                _ => SetupPhase1,
                SetupPhase1 => Result<SetupPhase2>,
                SetupPhase2 => SetupPhase3,
                SetupPhase3 => _
            ],
            functions: [
                fn SetupPhase1 {
                    signature: {
                        struct Input {
                            shader_name: &'static str,
                            shader_path: String
                        },
                        struct Output {
                            shader_name: &'static str,
                            shader_handle: Handle<Shader>
                        },
                        enum Error {
                            ShaderAlreadyRegistered {
                                shader_name: &'static str,
                            },
                            FailedToReadShader {
                                shader_name: &'static str,
                                error: std::io::Error
                            }
                        }
                    },
                    code_block: {
                        let mut system_state: SystemState<(
                            ResMut<Assets<Shader>>,
                            Res<ShaderRegistry>,
                        )> = SystemState::new(world);
                        let (
                            mut shader_assets, 
                            shader_registry,
                        ) = system_state.get_mut(world);

                        if shader_registry.shaders.contains_key(shader_name) {
                            err!(ShaderAlreadyRegistered { shader_name });
                        }

                        let shader_source = match std::fs::read_to_string(&shader_path) {
                            Ok(source) => source,
                            Err(err) => err!(FailedToReadShader { shader_name, error: err }),
                        };
                        let shader = Shader::from_wgsl(shader_source, shader_path.clone());
                        let shader_handle = world.resource_mut::<Assets<Shader>>().add(shader);

                        ok!(shader_handle)
                    },
                },
                fn SetupPhase2 {
                    signature: {
                        struct Input {
                            shader_name: &'static str,
                            shader_handle: Handle<Shader>,
                        },
                        struct Output {
                            shader_name: &'static str,
                            shader_handle: Handle<Shader>,
                            bind_group_layout: BindGroupLayout, 
                            pipeline_id: CachedComputePipelineId,
                        }
                    },
                    code_block: {
                        let mut system_state: SystemState<(
                            Res<RenderDevice>,
                            Res<PipelineCache>,
                        )> = SystemState::new(world);
                        let (
                            render_device, 
                            pipeline_cache, 
                        ) = system_state.get(world);

                        let bind_group_layout = render_device.create_bind_group_layout(
                            Some("Compute Bind Group Layout"),
                            &[
                                // Storage Texture
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
                                // Parameter Buffer
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
                            label: Some(format!("Pipeline for {}",shader_name).into()),
                            layout: vec![bind_group_layout.clone()],
                            shader: shader_handle.clone(),
                            shader_defs: vec![],
                            entry_point: "main".into(),
                            push_constant_ranges: vec![PushConstantRange {
                                stages: ShaderStages::COMPUTE,
                                range: 0..4, // Example: A single 4-byte (u32) push constant
                            }],
                        });

                        ok!(shader_name, shader_handle, bind_group_layout, pipeline_id)
                    }
                },
                fn SetupPhase3 {
                    signature: {
                        struct Input {
                            shader_name: &'static str,
                            shader_handle: Handle<Shader>,
                            bind_group_layout: BindGroupLayout, 
                            pipeline_id: CachedComputePipelineId,
                        }
                    },
                    code_block: {
                        let mut shader_registry = SystemState::<ResMut<ShaderRegistry>>::new(world).get_mut(world);

                        shader_registry.shaders.insert(shader_name.to_string(), shader_handle);
                        shader_registry.pipelines.insert(shader_name.to_string(), pipeline_id);
                        shader_registry.bind_group_layouts.insert(shader_name.to_string(), bind_group_layout);

                        ok!()
                    }
                },
            ],
        ),
    ],
);
