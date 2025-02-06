use bevy::prelude::*;

use crate::action::{resources::ActionTypeModuleRegistry, target::ActionTypeModule};

// TODO: Create macro to define actions and their types in a more streamlined and natural way
// TODO: Instead of an Action Target Type, we should register an Action Module Type, and integrate that change everywhere that's related

pub fn initialize_action_type_module(action_type_module_registry: &mut ResMut<ActionTypeModuleRegistry>) {
    action_type_module_registry.register(
        ActionTypeModule {
            name: "GPU".to_owned(),
            action_types: vec![
                generate_texture::create_action_type(),
            ],
        },
    );
}

pub mod setup_texture_generator {
    use bevy::{prelude::*, render::renderer::RenderDevice};
    use bevy::ecs::system::SystemState;
    use bevy::render::render_resource::*;

    use crate::gpu::resources::ShaderPipelineRegistry;
    use crate::{action::{stage::{ActionStage, ActionStageAsync, ActionStageEcs}, stage_io::{ActionIO, InputState, OutputState}, types::ActionType}, chunk::{components::ChunkComponent, functions::chunk_pos_to_world, resources::ChunkManager}, config::statics::CONFIG};

    pub struct Input(pub SetupPipelineInput);

    pub struct SetupPipelineInput {
        pub shader_name: &'static str,
        pub shader_path: String,
    }

    pub struct Output(pub Result<(), String>);

    pub fn create_action_type() -> ActionType {
        ActionType {
            name: "SetupTextureGenerator".to_owned(),
            validation: Box::new(|io: ActionIO<InputState>, world: &mut World| -> Result<ActionIO<OutputState>, String> {
                let (action_input, io) = io.get_input::<Input>();
                let stage_input = action_input.0;
                Ok(io.set_output(stage_input))
            }),
            stages: vec![
                ActionStage::Ecs(ActionStageEcs {
                    name: "SetupPipeline".to_owned(),
                    function: Box::new(|io: ActionIO<InputState>, world: &mut World| -> ActionIO<OutputState> {
                        let (input, io) = io.get_input::<SetupPipelineInput>();
                        let shader_name = input.shader_name.clone();
                        let shader_path = input.shader_path.clone();

                        let mut system_state: SystemState<(
                            Res<RenderDevice>,
                            ResMut<Assets<Shader>>,
                            Res<PipelineCache>,
                            ResMut<ShaderPipelineRegistry>
                        )> = SystemState::new(world);
                        let (render_device, mut shader_assets, pipeline_cache, mut shader_pipeline_registry) = system_state.get_mut(world);

                        // Read shader source from file
                        let shader_source = match std::fs::read_to_string(&shader_path) {
                            Ok(source) => source,
                            Err(e) => return io.set_output(Output(Err(format!("Failed to read shader: {}", e)))),
                        };

                        // Create shader
                        let shader = Shader::from_wgsl(shader_source, shader_path.clone());
                        let shader_handle = shader_assets.add(shader);

                        // Create Bind Group Layout
                        let bind_group_layout = render_device.create_bind_group_layout(
                            Some("Compute Bind Group Layout"),
                            &[
                                // Example: A storage buffer at binding 0
                                BindGroupLayoutEntry {
                                    binding: 0,
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


                        // Request pipeline creation in Bevy's PipelineCache
                        let pipeline_id = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                            label: Some(format!("Pipeline for {}",shader_name).into()),
                            layout: vec![bind_group_layout],
                            shader: shader_handle.clone(),
                            shader_defs: vec![],
                            entry_point: "main".into(),
                            push_constant_ranges: vec![PushConstantRange {
                                stages: ShaderStages::COMPUTE,
                                range: 0..4, // Example: A single 4-byte (u32) push constant
                            }],
                        });

                        // Store shader & pipeline handles
                        shader_pipeline_registry.shaders.insert(shader_name.to_string(), shader_handle);
                        shader_pipeline_registry.pipelines.insert(shader_name.to_string(), pipeline_id);
                        
                        io.set_output(Output(Ok(())))
                    }),
                }),
            ],
        }
    }
}

pub mod generate_texture {
    use bevy::{prelude::*, render::{render_asset::RenderAssetUsages, renderer::{RenderDevice, RenderQueue}}};
    use bevy::ecs::system::SystemState;
    use bevy::render::render_resource::*;

    use crate::{action::stage::ActionStageWhileEcs, gpu::resources::ShaderPipelineRegistry};
    use crate::action::{stage::{ActionStage, ActionStageAsync, ActionStageEcs}, 
        stage_io::{ActionIO, InputState, OutputState}, types::ActionType};

    /// Input to the action
    pub struct Input(pub GenerateTextureInput);

    /// Data needed for texture generation
    pub struct GenerateTextureInput {
        pub shader_name: String,
        pub texture_size: usize,
    }

    /// Data passed from ECS to Async (waiting for pipeline)
    pub struct PreparedPipeline {
        pub pipeline_id: CachedComputePipelineId,
        pub texture: Handle<Image>,
    }

    /// Data passed after pipeline is ready, before dispatch
    pub struct DispatchData {
        pub pipeline_id: CachedComputePipelineId,
        pub bind_group_layout: BindGroupLayout,
        pub texture: Handle<Image>,
    }

    /// Data passed after dispatching compute
    pub struct ComputePending {
        pub texture: Handle<Image>,
        pub fence: Buffer,
    }

    /// Output (final texture handle)
    pub struct Output(pub Result<Handle<Image>, String>);

    pub fn create_action_type() -> ActionType {
        ActionType {
            name: "GenerateTexture".to_owned(),
            validation: Box::new(|io: ActionIO<InputState>, world: &mut World| -> Result<ActionIO<OutputState>, String> {
                let (action_input, io) = io.get_input::<Input>();
                let stage_input = action_input.0;
                Ok(io.set_output(stage_input))
            }),
            stages: vec![
                // **1. ECS Stage: Prepare Compute Resources**
                ActionStage::Ecs(ActionStageEcs {
                    name: "PrepareCompute".to_owned(),
                    function: Box::new(|io: ActionIO<InputState>, world: &mut World| -> ActionIO<OutputState> {
                        let (input, io) = io.get_input::<GenerateTextureInput>();
                        let shader_name = input.shader_name.clone();
                        let texture_size = input.texture_size;

                        let mut system_state: SystemState<(
                            Res<RenderDevice>,
                            ResMut<Assets<Image>>,
                            Res<PipelineCache>,
                            Res<ShaderPipelineRegistry>,
                        )> = SystemState::new(world);

                        let (render_device, mut images, pipeline_cache, shader_registry) = system_state.get_mut(world);

                        let pipeline_id = match shader_registry.pipelines.get(&shader_name) {
                            Some(&id) => id,
                            None => return io.set_output(Output(Err(format!("Pipeline not found for shader: {}", shader_name)))),
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
                        io.set_output(PreparedPipeline {
                            pipeline_id,
                            texture: texture_handle,
                        })
                    }),
                }),

                // **2. Async Stage: Wait for Pipeline Compilation**
                ActionStage::WhileEcs(ActionStageWhileEcs {
                    name: "WaitForPipeline".to_owned(),
                    function: Box::new(|io: ActionIO<InputState>, world: &mut World| -> Result<ActionIO<InputState>, ActionIO<OutputState>> {
                        let input = io.get_input_ref::<PreparedPipeline>();
                        let pipeline_id = input.pipeline_id;
                
                        let mut system_state: SystemState<Res<PipelineCache>> = SystemState::new(world);
                        let pipeline_cache = system_state.get(world);
                
                        if let Some(_) = pipeline_cache.get_compute_pipeline(pipeline_id) {
                            let (input, io) = io.get_input::<PreparedPipeline>();
                            Err(io.set_output(DispatchData {
                                pipeline_id,
                                bind_group_layout: pipeline_cache.get_compute_pipeline(pipeline_id).unwrap().get_bind_group_layout(0).into(),
                                texture: input.texture.clone(),
                            }))
                        } else {
                            Ok(io)
                        }
                    }),
                }),

                // **3. ECS Stage: Dispatch Compute Work**
                ActionStage::Ecs(ActionStageEcs {
                    name: "DispatchCompute".to_owned(),
                    function: Box::new(|io: ActionIO<InputState>, world: &mut World| -> ActionIO<OutputState> {
                        let (input, io) = io.get_input::<DispatchData>();
                        let pipeline_id = input.pipeline_id;
                        let bind_group_layout = input.bind_group_layout.clone();
                        let texture = input.texture;

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
                            &[], // TODO: Add entries here
                        );

                        let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor { label: None });
                        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor { label: None, timestamp_writes: None });

                        compute_pass.set_pipeline(&pipeline);
                        compute_pass.set_bind_group(0, &bind_group, &[]);
                        compute_pass.dispatch_workgroups(8, 8, 1);

                        drop(compute_pass);

                        // Create GPU Fence
                        let fence = render_device.create_buffer(&BufferDescriptor {
                            label: Some("Compute Fence"),
                            size: std::mem::size_of::<u32>() as u64,
                            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
                            mapped_at_creation: false,
                        });

                        encoder.copy_buffer_to_buffer(
                            &pipeline_cache.create_empty_buffer(4),
                            0,
                            &fence,
                            0,
                            4,
                        );

                        queue.submit(Some(encoder.finish()));

                        io.set_output(ComputePending { texture, fence })
                    }),
                }),

                // **4. Async Stage: Wait for Compute Execution**
                ActionStage::Async(ActionStageAsync {
                    name: "WaitForCompute".to_owned(),
                    function: Box::new(|io: ActionIO<InputState>| Box::pin(async move {
                        let (input, io) = io.get_input::<ComputePending>();
                        let texture = input.texture;
                        let fence = input.fence;

                        // Wait for fence completion
                        while !fence.map_async(wgpu::MapMode::Read).await {
                            bevy::tasks::future::yield_now().await;
                        }

                        io.set_output(Output(Ok(texture)))
                    })),
                }),

                // **5. ECS Stage: Cleanup & Return Output**
                ActionStage::Ecs(ActionStageEcs {
                    name: "FinalizeCompute".to_owned(),
                    function: Box::new(|io: ActionIO<InputState>, world: &mut World| -> ActionIO<OutputState> {
                        let (input, io) = io.get_input::<Output>();
                        io.set_output(input)
                    }),
                }),
            ],
        }
    }
}

