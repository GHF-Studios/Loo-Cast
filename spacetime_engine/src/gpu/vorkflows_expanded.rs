mod gpu_vorkflows {
    pub const NAME: &str = "Gpu";
    pub mod setup_texture_generator {
        pub const NAME: &str = "SetupTextureGenerator";
        pub mod vorkflow_imports {
            // Automatic imports
            pub use super::user_types::*;
            pub use super::user_functions::*;
            
            pub use crate::vorkflow::types::{Outcome, Outcome::Wait, Outcome::Done};

            // User imports
            pub use crate::gpu::resources::ShaderRegistry;

            pub use bevy::prelude::{World, Handle, Shader, Res, ResMut, Assets};
            pub use bevy::ecs::system::SystemState;
            pub use bevy::render::render_resource::{
                BindGroupLayout, CachedComputePipelineId, 
                PipelineCache, BindGroupLayoutEntry, ShaderStages, 
                BindingType, StorageTextureAccess, TextureFormat, 
                TextureViewDimension, BufferBindingType, PushConstantRange, 
                CachedPipelineState, Pipeline, ComputePipelineDescriptor
            };
            pub use bevy::render::render_asset::RenderAssets;
            pub use bevy::render::texture::GpuImage;
            pub use bevy::render::renderer::RenderDevice;
        }
        pub mod user_types {
            use super::vorkflow_imports::*;
        }
        pub mod user_functions {
            use super::vorkflow_imports::*;
        }
        pub mod stages {
            pub mod setup_phase_1 {
                pub mod core_types {
                    pub use super::super::super::vorkflow_imports::*;

                    pub struct Input { 
                        pub shader_name: &'static str, 
                        pub shader_path: String 
                    }
                    pub struct Output {
                        pub shader_name: &'static str, 
                        pub shader_handle: Handle<Shader>,
                    }
                    pub enum Error {
                        ShaderAlreadyRegistered { 
                            shader_name: &'static str 
                        },
                        FailedToReadShader { 
                            shader_name: &'static str, 
                            error: std::io::Error 
                        }
                    }
                }
                pub mod core_functions {
                    use super::core_types::*;

                    pub fn run_ecs(input: Input, world: &mut World) -> Result<Output, Error> {
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
                    }
                }
            }
            pub mod setup_phase_2 {
                pub mod core_types {
                    pub use super::super::super::vorkflow_imports::*;

                    pub struct Input {
                        pub shader_name: &'static str, 
                        pub shader_handle: Handle<Shader>
                    }
                    pub struct State {
                        pub shader_name: &'static str, 
                        pub shader_handle: Handle<Shader>,
                        pub bind_group_layout: BindGroupLayout, 
                        pub pipeline_id: CachedComputePipelineId,
                    }
                    pub struct Output { 
                        pub shader_name: &'static str, 
                        pub shader_handle: Handle<Shader>, 
                        pub pipeline_id: CachedComputePipelineId,
                        pub bind_group_layout: BindGroupLayout, 
                    }
                    pub enum Error {
                        ExpectedComputePipelineGotRenderPipeline {
                            shader_name: &'static str,
                            pipeline_id: CachedComputePipelineId,
                        },
                        FailedToCreatePipeline {
                            shader_name: &'static str,
                            pipeline_cache_err: String,
                        }
                    }
                }
                pub mod core_functions {
                    use super::core_types::*;

                    pub fn setup_render_while(input: Input, world: &mut World) -> Result<State, Error> {
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
                    }

                    pub fn run_render_while(state: State, world: &mut World) -> Result<Outcome<State, Output>, Error> {
                        let shader_name = state.shader_name;
                        let shader_handle = state.shader_handle.clone();
                        let bind_group_layout = state.bind_group_layout.clone();
                        let pipeline_id = state.pipeline_id.clone();

                        let pipeline_cache = SystemState::<Res::<PipelineCache>>::new(world).get(world);

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
                    }
                }
            }
            pub mod setup_phase_3 {
                pub mod core_types {
                    pub use super::super::super::vorkflow_imports::*;

                    pub struct Input { 
                        pub shader_name: &'static str, 
                        pub shader_handle: Handle<Shader>, 
                        pub pipeline_id: CachedComputePipelineId,
                        pub bind_group_layout: BindGroupLayout, 
                    }
                }
                pub mod core_functions {
                    use super::core_types::*;

                    pub fn run_ecs(input: Input, world: &mut World) {
                        let shader_name = input.shader_name;
                        let shader_handle = input.shader_handle;
                        let bind_group_layout = input.bind_group_layout;
                        let pipeline_id = input.pipeline_id;
    
                        let mut shader_registry = SystemState::<ResMut<ShaderRegistry>>::new(world).get_mut(world);
                        
                        shader_registry.shaders.insert(shader_name.to_string(), shader_handle);
                        shader_registry.pipelines.insert(shader_name.to_string(), pipeline_id);
                        shader_registry.bind_group_layouts.insert(shader_name.to_string(), bind_group_layout);
                    }
                }
            }
        }
    }
    pub mod generate_texture {
        pub const NAME: &str = "GenerateTexture";
        pub mod vorkflow_imports {
            // Automatic imports
            pub use super::user_types::*;
            pub use super::user_functions::*;
            
            pub use crate::vorkflow::types::{Outcome, Outcome::Wait, Outcome::Done};

            // User imports
            pub use crate::gpu::resources::ShaderRegistry;

            pub use bevy::prelude::{Handle, World, Res, ResMut, Assets, Image};
            pub use bevy::render::render_resource::{
                CachedComputePipelineId, BindGroupLayout, 
                Buffer, TextureView, TextureDescriptor, Extent3d, 
                TextureDimension, TextureFormat, TextureUsages, 
                BufferInitDescriptor, BufferUsages, CommandEncoderDescriptor,
                ComputePassDescriptor
            };
            pub use bevy::ecs::system::SystemState;
            pub use bevy::render::render_asset::RenderAssets;
            pub use bevy::render::texture::GpuImage;
            pub use bevy::render::renderer::{RenderDevice, RenderQueue};
            pub use bevy::render::render_resource::{PipelineCache, BindGroupEntry, BindingResource};

            pub use crossbeam_channel::Receiver;
        }
        pub mod user_types {
            use super::vorkflow_imports::*;

            pub struct GeneratorRequest<T> {
                pub inner: T
            }
            
            pub struct GeneratorParams {
                pub shader_name: &'static str,
                pub pipeline_id: CachedComputePipelineId,
                pub bind_group_layout: BindGroupLayout,
                pub texture_handle: Handle<Image>,
                pub param_buffer: Buffer,
            }
            impl GeneratorRequest<GeneratorParams> {
                pub fn new(
                    shader_name: &'static str,
                    pipeline_id: CachedComputePipelineId,
                    bind_group_layout: BindGroupLayout,
                    texture_handle: Handle<Image>,
                    param_buffer: Buffer,
                ) -> Self {
                    Self {
                        inner: GeneratorParams {
                            shader_name,
                            pipeline_id,
                            bind_group_layout,
                            texture_handle,
                            param_buffer,
                        }
                    }
                }

                pub fn set_texture_view(self, texture_view: TextureView) -> GeneratorRequest<PreparedGenerator> {
                    GeneratorRequest {
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
            
            pub struct PreparedGenerator {
                pub shader_name: &'static str,
                pub pipeline_id: CachedComputePipelineId,
                pub bind_group_layout: BindGroupLayout,
                pub texture_handle: Handle<Image>,
                pub texture_view: TextureView,
                pub param_buffer: Buffer,
            }
            impl GeneratorRequest<PreparedGenerator> {
                pub fn track_dispatch(self, texture_handle: Handle<Image>, receiver: Receiver<()>) -> GeneratorRequest<DispatchedCompute> {
                    GeneratorRequest {
                        inner: DispatchedCompute {
                            shader_name: self.inner.shader_name,
                            texture_handle,
                            receiver,
                        }
                    }
                }
            }
            
            pub struct DispatchedCompute {
                pub shader_name: &'static str,
                pub texture_handle: Handle<Image>,
                pub receiver: Receiver<()>
            }
            impl GeneratorRequest<DispatchedCompute> {
                pub fn consume(self) -> (&'static str, Handle<Image>) {
                    (self.inner.shader_name, self.inner.texture_handle)
                }
            }
        }
        pub mod user_functions {
            use super::vorkflow_imports::*;
        }
        pub mod stages {
            pub mod prepare_request {
                pub mod core_types {
                    pub use super::super::super::vorkflow_imports::*;

                    pub struct Input {
                        pub shader_name: &'static str,
                        pub texture_size: usize,
                        pub param_data: Vec<f32>,
                    }
                    pub struct Output {
                        pub request: GeneratorRequest<GeneratorParams>,
                    }
                    pub enum Error {
                        GeneratorNotFound {
                            shader_name: &'static str,
                        },
                    }
                }
                pub mod core_functions {
                    use super::core_types::*;

                    pub fn run_ecs(input: Input, world: &mut World) -> Result<Output, Error> {
                        let shader_name = input.shader_name;
                        let texture_size = input.texture_size;
                        let param_data = input.param_data;

                        let mut system_state: SystemState<(
                            Res<RenderDevice>,
                            ResMut<Assets<Image>>,
                            Res<ShaderRegistry>,
                        )> = SystemState::new(world);
                        let (render_device, mut images, shader_registry) = system_state.get_mut(world);

                        if shader_registry.shaders.get(shader_name).is_none() {
                            return Err(Error::GeneratorNotFound { shader_name })
                        }

                        let pipeline_id = shader_registry.pipelines.get(shader_name).unwrap().clone();
                        let bind_group_layout = shader_registry.bind_group_layouts.get(shader_name).unwrap().clone();

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

                        let param_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                            label: Some("Parameter Buffer"),
                            contents: bytemuck::cast_slice(&param_data),
                            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                        });

                        let request = GeneratorRequest::new(
                            shader_name, 
                            pipeline_id, 
                            bind_group_layout, 
                            texture_handle, 
                            param_buffer
                        );

                        Ok(Output { request })
                    }
                }
            }
            pub mod get_texture_view {
                pub mod core_types {
                    pub use super::super::super::vorkflow_imports::*;

                    pub struct Input {
                        pub request: GeneratorRequest<GeneratorParams>,
                    }
                    pub struct State {
                        pub request: GeneratorRequest<GeneratorParams>,
                    }
                    pub struct Output {
                        pub request: GeneratorRequest<PreparedGenerator>,
                    }
                }
                pub mod core_functions {
                    use super::core_types::*;

                    pub fn setup_render_while(input: Input, _world: &mut World) -> State {
                        State { request: input.request }
                    }

                    pub fn run_render_while(state: State, world: &mut World) -> Outcome<State, Output> {
                        let gpu_images = SystemState::<Res<RenderAssets<GpuImage>>>::new(world).get(world);
            
                        if let Some(gpu_image) = gpu_images.get(&state.request.inner.texture_handle) {
                            let texture_view = gpu_image.texture_view.clone();
            
                            let prepared_request = state.request.set_texture_view(texture_view);
                            Done(Output { request: prepared_request })
                        } else {
                            Wait(state)
                        }
                    }
                }
            }
            pub mod dispatch_compute {
                pub mod core_types {
                    pub use super::super::super::vorkflow_imports::*;

                    pub struct Input {
                        pub request: GeneratorRequest<PreparedGenerator>,
                    }
                    pub struct Output {
                        pub request: GeneratorRequest<DispatchedCompute>,
                    }
                }
                pub mod core_functions {
                    use super::core_types::*;

                    pub fn run_render(input: Input, world: &mut World) -> Output {
                        let prepared = &input.request.inner;
                        let pipeline_id = prepared.pipeline_id.clone();
                        let bind_group_layout = &prepared.bind_group_layout;
                        let texture_handle = prepared.texture_handle.clone();
                        let texture_view = &prepared.texture_view;
                        let param_buffer = &prepared.param_buffer;
            
                        let mut system_state: SystemState<(
                            Res<RenderDevice>,
                            Res<RenderQueue>,
                            Res<PipelineCache>,
                        )> = SystemState::new(world);
                        let (render_device, queue, pipeline_cache) = system_state.get_mut(world);
            
                        let pipeline = pipeline_cache.get_compute_pipeline(pipeline_id)
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
            
                        let (sender, receiver) = crossbeam_channel::unbounded();
                        queue.on_submitted_work_done(move || {
                            let _ = sender.send(());
                        });
                        
                        let dispatched_request = input.request.track_dispatch(texture_handle, receiver);
                        Output { request: dispatched_request }
                    }
                }
            }
            pub mod wait_for_compute {
                pub mod core_types {
                    pub use super::super::super::vorkflow_imports::*;

                    pub struct Input {
                        pub request: GeneratorRequest<DispatchedCompute>,
                    }
                    pub struct State {
                        pub request: GeneratorRequest<DispatchedCompute>,
                    }
                    pub struct Output {
                        pub shader_name: &'static str,
                        pub texture_handle: Handle<Image>,
                    }
                    pub enum Error {
                        ComputePassReceiverDisconnected {
                            shader_name: &'static str,
                        },
                    }
                }
                pub mod core_functions {
                    use super::core_types::*;

                    pub fn setup_ecs_while(input: Input, _world: &mut World) -> Result<State, Error> {
                        Ok(State { request: input.request })
                    }

                    pub fn run_ecs_while(state: State, world: &mut World) -> Result<Outcome<State, Output>, Error> {
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
                    }
                }
            }
        }
    }
}