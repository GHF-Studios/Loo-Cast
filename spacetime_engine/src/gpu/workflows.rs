use crate::workflow::{resources::WorkflowTypeModuleRegistry, target::WorkflowTypeModule};

pub fn initialize_workflow_type_module(workflow_type_module_registry: &mut WorkflowTypeModuleRegistry) {
    workflow_type_module_registry.register(
        WorkflowTypeModule {
            name: "GPU".to_owned(),
            workflow_types: vec![
                setup_texture_generator::create_workflow_type(),
                generate_texture::create_workflow_type(),
            ],
        },
    );
}

pub mod setup_texture_generator {
    use bevy::{prelude::*, render::renderer::RenderDevice};
    use bevy::ecs::system::SystemState;
    use bevy::render::render_resource::*;

    use crate::workflow::stage::*;
    use crate::workflow::types::RawWorkflowData;
    use crate::gpu::resources::ShaderPipelineRegistry;
    use crate::workflow::{stage::WorkflowStage, stage_io::{WorkflowIO, InputState, OutputState}, types::WorkflowType};

    pub struct Input(pub SetupPipelineInput);

    pub struct SetupPipelineInput {
        pub shader_name: &'static str,
        pub shader_path: String,
    }

    pub struct Output(pub Result<(), String>);

    pub fn create_workflow_type() -> WorkflowType {
        WorkflowType {
            name: "SetupTextureGenerator".to_owned(),
            primary_validation: Box::new(|io: WorkflowIO<InputState>| -> Result<WorkflowIO<InputState>, String> {
                let (workflow_input, _) = io.get_input::<Input>();
                let stage_input = workflow_input.0;

                Ok(WorkflowIO::new_input(RawWorkflowData::new(stage_input)))
            }),
            secondary_validation: Box::new(|io: WorkflowIO<InputState>, _world: &mut World| -> Result<WorkflowIO<InputState>, String> {
                Ok(io)
            }),
            stages: vec![
                WorkflowStage::Ecs(WorkflowStageEcs {
                    name: "SetupPhase1".to_owned(),
                    function: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> WorkflowIO<OutputState> {
                        let (input, io) = io.get_input::<SetupPipelineInput>();
                        let shader_name = input.shader_name;
                        let shader_path = input.shader_path.clone();

                        let mut system_state: SystemState<(
                            ResMut<Assets<Shader>>,
                            Res<ShaderPipelineRegistry>,
                        )> = SystemState::new(world);
                        let (
                            mut shader_assets, 
                            shader_pipeline_registry,
                        ) = system_state.get_mut(world);

                        let shader_source = match std::fs::read_to_string(&shader_path) {
                            Ok(source) => source,
                            Err(e) => {
                                return io.set_output(RawWorkflowData::new(Output(Err(format!("Failed to read shader: {}", e)))))
                            },
                        };

                        let shader = Shader::from_wgsl(shader_source, shader_path.clone());
                        let shader_handle = shader_assets.add(shader);

                        if shader_pipeline_registry.bind_group_layouts.contains_key(shader_name) {
                            // TODO: Remove these damn panics and unreachables and replace them with proper per-stage error handling and proper IO handling of errors
                            unreachable!("Failed to setup texture generator: Shader '{}' already registered", shader_name)
                        }

                        debug!("Executed ecs stage: SetupTextureGenerator::SetupPhase1");
                        io.set_output(RawWorkflowData::new((input, shader_handle)))
                    })
                }),
                WorkflowStage::Render(WorkflowStageRender {
                    name: "SetupPhase2".to_owned(),
                    function: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> WorkflowIO<OutputState> {
                        let ((input, shader_handle), io) = io.get_input::<(SetupPipelineInput, Handle<Shader>)>();
                        let shader_name = input.shader_name;

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
                                // Example: A storage buffer at binding 0
                                BindGroupLayoutEntry {
                                    binding: 0,
                                    visibility: ShaderStages::COMPUTE,
                                    ty: BindingType::StorageTexture {
                                        access: StorageTextureAccess::WriteOnly,
                                        format: TextureFormat::Rgba8Unorm,
                                        view_dimension: TextureViewDimension::D2,
                                    },
                                    count: None,
                                }
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

                        debug!("Executed render stage: SetupTextureGenerator::SetupPhase2");
                        io.set_output(RawWorkflowData::new((input, shader_handle, bind_group_layout, pipeline_id)))
                    })
                }),
                WorkflowStage::Ecs(WorkflowStageEcs {
                    name: "SetupPhase3".to_owned(),
                    function: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> WorkflowIO<OutputState> {
                        let ((
                            input, 
                            shader_handle, 
                            bind_group_layout, 
                            pipeline_id
                        ), io) = io.get_input::<(
                            SetupPipelineInput, 
                            Handle<Shader>, 
                            BindGroupLayout, 
                            CachedComputePipelineId
                        )>();

                        let shader_name = input.shader_name;

                        let mut shader_pipeline_registry = SystemState::<ResMut<ShaderPipelineRegistry>>::new(world).get_mut(world);

                        shader_pipeline_registry.shaders.insert(shader_name.to_string(), shader_handle);
                        shader_pipeline_registry.pipelines.insert(shader_name.to_string(), pipeline_id);
                        shader_pipeline_registry.bind_group_layouts.insert(shader_name.to_string(), bind_group_layout);

                        debug!("Executed ecs stage: SetupTextureGenerator::SetupPhase3");
                        io.set_output(RawWorkflowData::new(Output(Ok(()))))
                    })
                }),
            ]
        }
    }
}

pub mod generate_texture {
    use bevy::{prelude::*, render::{render_asset::RenderAssetUsages, renderer::{RenderDevice, RenderQueue}}};
    use bevy::ecs::system::SystemState;
    use bevy::render::render_resource::*;
    use crossbeam_channel::{unbounded, Receiver};
    use crate::{gpu::resources::ShaderPipelineRegistry, workflow::{stage::{WorkflowStageRender, WorkflowStageRenderWhile, WorkflowStageWhileOutcome}, types::RawWorkflowData}};
    use crate::workflow::{stage::{WorkflowStage, WorkflowStageEcsWhile, WorkflowStageEcs}, 
        stage_io::{WorkflowIO, InputState, OutputState}, types::WorkflowType};

    /// Input to the workflow
    pub struct Input(pub GenerateTextureInput);

    /// Data needed for texture generation
    pub struct GenerateTextureInput {
        pub shader_name: String,
        pub texture_size: usize,
    }

    /// Data passed from ECS to EcsWhile (waiting for pipeline)
    pub struct PreparedPipeline {
        pub pipeline_id: CachedComputePipelineId,
        pub texture: Handle<Image>,
        pub status_buffer: Buffer,
        pub readback_buffer: Buffer,
    }

    /// Data passed after pipeline is ready, before dispatch
    pub struct DispatchData {
        pub pipeline_id: CachedComputePipelineId,
        pub bind_group_layout: BindGroupLayout,
        pub texture: Handle<Image>,
        pub status_buffer: Buffer,
        pub readback_buffer: Buffer,
    }

    /// Data passed after dispatching compute
    pub struct ComputePending {
        pub texture: Handle<Image>,
        pub status_buffer: Buffer,
        pub readback_buffer: Buffer,
    }

    /// Output (final texture handle)
    pub struct Output(pub Result<Handle<Image>, String>);

    /// Temporary resource to track mapping completion
    #[derive(Resource)]
    struct BufferMappingReceiver(Receiver<()>);

    pub fn create_workflow_type() -> WorkflowType {
        WorkflowType {
            name: "GenerateTexture".to_owned(),
            primary_validation: Box::new(|io: WorkflowIO<InputState>| -> Result<WorkflowIO<InputState>, String> {
                let (workflow_input, _) = io.get_input::<Input>();
                let stage_input = workflow_input.0;

                Ok(WorkflowIO::new_input(RawWorkflowData::new(stage_input)))
            }),
            secondary_validation: Box::new(|io: WorkflowIO<InputState>, _world: &mut World| -> Result<WorkflowIO<InputState>, String> {
                Ok(io)
            }),
            stages: vec![
                // **1. ECS Stage: Prepare Compute Resources**
                WorkflowStage::Ecs(WorkflowStageEcs {
                    name: "PrepareCompute".to_owned(),
                    function: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> WorkflowIO<OutputState> {
                        debug!("Active Workflow Stage: Gpu::GenerateTexture::PrepareCompute");
                        let (input, io) = io.get_input::<GenerateTextureInput>();
                        let shader_name = input.shader_name.clone();
                        let texture_size = input.texture_size;

                        let mut system_state: SystemState<(
                            Res<RenderDevice>,
                            ResMut<Assets<Image>>,
                            Res<ShaderPipelineRegistry>,
                        )> = SystemState::new(world);

                        let (render_device, mut images, shader_pipeline_registry) = system_state.get_mut(world);

                        let pipeline_id = match shader_pipeline_registry.pipelines.get(&shader_name) {
                            Some(&id) => { 
                                id 
                            },
                            None => {
                                // TODO: Bake fallability into the workflow type, instead of this rude abrupt panic
                                unreachable!("Failed to generate texture: Pipeline not found for shader: {}", shader_name)
                            },
                        };

                        let texture = Image::new_fill(
                            Extent3d {
                                width: texture_size as u32,
                                height: texture_size as u32,
                                depth_or_array_layers: 1,
                            },
                            TextureDimension::D2,
                            &[0, 0, 0, 0],
                            TextureFormat::Rgba8Unorm,
                            RenderAssetUsages::MAIN_WORLD
                        );

                        let texture_handle = images.add(texture);

                        // Create a status buffer (only STORAGE, used inside shader)
                        let status_buffer = render_device.create_buffer(&BufferDescriptor {
                            label: Some("Compute Status Buffer"),
                            size: std::mem::size_of::<u32>() as u64,
                            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC, // 🚀 Fix: No MAP_READ
                            mapped_at_creation: false,
                        });

                        // Create a readback buffer (only COPY_DST | MAP_READ, used by CPU)
                        let readback_buffer = render_device.create_buffer(&BufferDescriptor {
                            label: Some("Readback Buffer"),
                            size: std::mem::size_of::<u32>() as u64,
                            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ, // 🚀 Fix: Separated readback buffer
                            mapped_at_creation: false,
                        });

                        io.set_output(RawWorkflowData::new(PreparedPipeline {
                            pipeline_id,
                            texture: texture_handle,
                            status_buffer,
                            readback_buffer
                        }))
                    }),
                }),

                // **2. RenderWhile Stage: Wait for Pipeline Compilation**
                WorkflowStage::RenderWhile(WorkflowStageRenderWhile {
                    name: "WaitForPipeline".to_owned(),
                    // TODO: Maybe instead of WorkflowStageEcsWhileOutcome use a future and handle the ecs while stage async-ly somehow??? 
                    function: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> WorkflowStageWhileOutcome {
                        debug!("Active Workflow Stage: Gpu::GenerateTexture::WaitForPipeline");
                        let input = io.get_input_ref::<PreparedPipeline>();
                        let pipeline_id = input.pipeline_id;
                
                        let mut system_state: SystemState<Res<PipelineCache>> = SystemState::new(world);
                        let pipeline_cache = system_state.get(world);
                
                        match pipeline_cache.get_compute_pipeline_state(pipeline_id) {
                            CachedPipelineState::Queued => {
                                WorkflowStageWhileOutcome::Waiting(io)
                            },
                            CachedPipelineState::Creating(_) => {
                                WorkflowStageWhileOutcome::Waiting(io)
                            },
                            CachedPipelineState::Ok(pipeline) => {
                                let (input, io) = io.get_input::<PreparedPipeline>();
                                let compute_pipeline = match pipeline {
                                    // TODO: Bake fallability into the workflow type, instead of this rude abrupt panic
                                    Pipeline::RenderPipeline(_) => unreachable!("Failed to generate texture: Expected a compute pipeline"),
                                    Pipeline::ComputePipeline(compute_pipeline) => compute_pipeline
                                };
                                WorkflowStageWhileOutcome::Completed(io.set_output(RawWorkflowData::new(DispatchData {
                                    pipeline_id,
                                    bind_group_layout: compute_pipeline.get_bind_group_layout(0).into(),
                                    texture: input.texture.clone(),
                                    status_buffer: input.status_buffer,
                                    readback_buffer: input.readback_buffer,
                                })))
                            },
                            CachedPipelineState::Err(e) => {
                                // TODO: Bake fallability into the workflow type, instead of this rude abrupt panic
                                unreachable!("Failed to generate texture: Failed to create pipeline: {}", e);
                            },
                        }
                    }),
                }),

                // NEW: **3. ECS Stage: Dispatch Compute Work**
                WorkflowStage::Render(WorkflowStageRender {
                    name: "DispatchCompute".to_owned(),
                    function: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> WorkflowIO<OutputState> {
                        debug!("Active Workflow Stage: Gpu::GenerateTexture::DispatchCompute");
                        let (input, io) = io.get_input::<DispatchData>();
                        let pipeline_id = input.pipeline_id;
                        let bind_group_layout = input.bind_group_layout.clone();
                        let texture = input.texture;
                        let status_buffer = input.status_buffer;
                        let readback_buffer = input.readback_buffer;

                        let mut system_state: SystemState<(
                            Res<RenderDevice>,
                            Res<RenderQueue>,
                            Res<PipelineCache>,
                        )> = SystemState::new(world);

                        let (render_device, queue, pipeline_cache) = system_state.get_mut(world);

                        let pipeline = pipeline_cache.get_compute_pipeline(pipeline_id).unwrap();

                        let bind_group = render_device.create_bind_group(
                            Some("Compute Bind Group"),
                            &bind_group_layout,
                            &[
                                BindGroupEntry {
                                    binding: 0,
                                    resource: status_buffer.as_entire_binding(),
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

                        io.set_output(RawWorkflowData::new(ComputePending { texture, status_buffer, readback_buffer }))
                    }),
                }),

                // **4. EcsWhile Stage: Wait for Compute Execution (Polling)**
                WorkflowStage::EcsWhile(WorkflowStageEcsWhile {
                    name: "WaitForCompute".to_owned(),
                    function: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> WorkflowStageWhileOutcome {
                        debug!("Active Workflow Stage: Gpu::GenerateTexture::WaitForCompute");
                        let input = io.get_input_ref::<ComputePending>();
                        let readback_buffer = &input.readback_buffer;
                        
                        let mapping_receiver = SystemState::<Option<ResMut<BufferMappingReceiver>>>::new(world).get_mut(world);
                        if let Some(receiver) = mapping_receiver {
                            if receiver.0.try_recv().is_ok() {
                                let (input, io) = io.get_input::<ComputePending>();
                                world.remove_resource::<BufferMappingReceiver>(); // Cleanup
                                return WorkflowStageWhileOutcome::Completed(io.set_output(RawWorkflowData::new(Output(Ok(input.texture)))));
                            }
                        } else {
                            let (sender, receiver) = unbounded();
                            world.insert_resource(BufferMappingReceiver(receiver));
                
                            let render_device = SystemState::<Res<RenderDevice>>::new(world).get_mut(world);
                            render_device.map_buffer(
                                &readback_buffer.slice(..),
                                MapMode::Read,
                                move |result| {
                                    if result.is_ok() {
                                        let _ = sender.send(());
                                    }
                                },
                            );
                        }
                        
                        WorkflowStageWhileOutcome::Waiting(io)
                    }),
                }),
            ],
        }
    }
}


