use spacetime_engine_macros::define_workflow_mod;

pub mod gpu {
    pub const NAME: &str = stringify!("Gpu");
    pub fn register_workflow_type_module(
        workflow_type_module_registry: &mut crate::workflow::resources::WorkflowTypeModuleRegistry,
    ) {
        workflow_type_module_registry.register(crate::workflow::types::WorkflowTypeModule {
            name: stringify!("Gpu"),
            workflow_types: vec![
                setup_texture_generator::TypeIE::create_workflow(),
                generate_texture::TypeIOE::create_workflow(),
            ],
        });
    }
    pub mod setup_texture_generator {
        pub const NAME: &str = stringify!("SetupTextureGenerator");
        pub async fn run(
            input: <TypeIE as crate::workflow::traits::WorkflowTypeIE>::Input,
        ) -> Result<(), <TypeIE as crate::workflow::traits::WorkflowTypeIE>::Error> {
            crate::workflow::functions::run_workflow_ie::<TypeIE>(input).await
        }
        #[derive(std::fmt::Debug, thiserror::Error)]
        pub enum Error {
            SetupPhase1Error(self::stages::setup_phase1::core_types::Error),
            SetupPhase2Error(self::stages::setup_phase2::core_types::Error),
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        pub struct TypeIE;

        impl crate::workflow::traits::WorkflowTypeIE for TypeIE {
            type Input = self::stages::setup_phase1::core_types::Input;
            type Error = Error;
            const MODULE_NAME: &'static str = super::NAME;
            const WORKFLOW_NAME: &'static str = self::NAME;
        }
        impl TypeIE {
            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                crate::workflow::types::WorkflowType {
                    name: self::NAME,
                    stages: vec![
                        crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                            index: 0,
                            name: stringify!("SetupPhase1"),
                            signature: crate::workflow::stage::StageSignature::IOE,
                            run_ecs: Box::new(self::stages::setup_phase1::core_functions::run_ecs)
                                as Box<
                                    dyn FnMut(
                                            Option<Box<dyn std::any::Any + Send + Sync>>,
                                            &mut bevy::prelude::World,
                                        )
                                            -> Option<Box<dyn std::any::Any + Send + Sync>>
                                        + Send
                                        + Sync,
                                >,
                            handle_ecs_run_response: Box::new(
                                |module_name: &'static str,
                                 workflow_name: &'static str,
                                 response: Option<Box<dyn std::any::Any + Send + Sync>>,
                                 completion_sender: crossbeam_channel::Sender<
                                    crate::workflow::events::StageCompletionEvent,
                                >,
                                 failure_sender: Option<
                                    crossbeam_channel::Sender<
                                        crate::workflow::events::StageFailureEvent,
                                    >,
                                >| {
                                    Box::new(move |stage: crate::workflow::stage::StageEcs| {
                                        let response = response.expect(
                                            "Ecs stages with output and error must have a response",
                                        );
                                        let result_data:Result<crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase1::core_types::Output,crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase1::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                                        match result_data {
                                            Ok(output) => {
                                                let output:crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase2::core_types::Input = unsafe {
                                            std::mem::transmute(output)
                                        };
                                                let output = Some(Box::new(output)
                                                    as Box<dyn std::any::Any + Send + Sync>);
                                                if let Err(send_err) = completion_sender.send(
                                                    crate::workflow::events::StageCompletionEvent {
                                                        ty: crate::workflow::stage::StageType::Ecs,
                                                        module_name,
                                                        workflow_name,
                                                        current_stage: 0,
                                                        stage_return:
                                                            crate::workflow::stage::Stage::Ecs(
                                                                stage,
                                                            ),
                                                        stage_output: output,
                                                    },
                                                ) {
                                                    unreachable!("Ecs response handler error: Completion event send error: {}",send_err);
                                                }
                                            }
                                            Err(error) => {
                                                let error = crate::gpu::workflows::gpu::setup_texture_generator::Error::SetupPhase1Error(error);
                                                let error = Some(Box::new(error)
                                                    as Box<dyn std::any::Any + Send + Sync>);
                                                let failure_sender = match failure_sender {
                                                    Some(failure_sender) => failure_sender,
                                                    None => {
                                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                                    }
                                                };
                                                if let Err(send_err) = failure_sender.send(
                                                    crate::workflow::events::StageFailureEvent {
                                                        ty: crate::workflow::stage::StageType::Ecs,
                                                        module_name,
                                                        workflow_name,
                                                        current_stage: 0,
                                                        stage_return:
                                                            crate::workflow::stage::Stage::Ecs(
                                                                stage,
                                                            ),
                                                        stage_error: error,
                                                    },
                                                ) {
                                                    unreachable!("Ecs response handler error: Failure event send error: {}",send_err);
                                                }
                                            }
                                        }
                                    })
                                },
                            ),
                            completion_sender:
                                crate::workflow::channels::get_stage_completion_sender().clone(),
                            failure_sender: Some(
                                crate::workflow::channels::get_stage_failure_sender().clone(),
                            ),
                        }),
                        crate::workflow::stage::Stage::RenderWhile(
                            crate::workflow::stage::StageRenderWhile {
                                index: 1,
                                name: stringify!("SetupPhase2"),
                                signature: crate::workflow::stage::StageSignature::IOE,
                                setup_render_while: Box::new(
                                    self::stages::setup_phase2::core_functions::setup_render_while,
                                )
                                    as Box<
                                        dyn FnMut(
                                                Option<Box<dyn std::any::Any + Send + Sync>>,
                                                &mut bevy::prelude::World,
                                            )
                                                -> Option<Box<dyn std::any::Any + Send + Sync>>
                                            + Send
                                            + Sync,
                                    >,
                                run_render_while: Box::new(
                                    self::stages::setup_phase2::core_functions::run_render_while,
                                )
                                    as Box<
                                        dyn FnMut(
                                                Option<Box<dyn std::any::Any + Send + Sync>>,
                                                &mut bevy::prelude::World,
                                            )
                                                -> Box<dyn std::any::Any + Send + Sync>
                                            + Send
                                            + Sync,
                                    >,
                                handle_render_while_setup_response: Box::new(
                                    |module_name: &'static str,
                                     workflow_name: &'static str,
                                     response: Option<Box<dyn std::any::Any + Send + Sync>>,
                                     setup_sender: crossbeam_channel::Sender<
                                        crate::workflow::events::StageSetupEvent,
                                    >,
                                     failure_sender: Option<
                                        crossbeam_channel::Sender<
                                            crate::workflow::events::StageFailureEvent,
                                        >,
                                    >| {
                                        Box::new(move|stage:crate::workflow::stage::StageRenderWhile|{
                                let response = response.expect("RenderWhile stages with state and error must have a response");
                                let result:Result<crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase2::core_types::State,crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase2::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                                match result {
                                    Ok(state) => {
                                        let state = Some(Box::new(state)as Box<dyn std::any::Any+Send+Sync>);
                                        if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                                            ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_state:state,
                                        }){
                                            unreachable!("RenderWhile response handler error: Setup event send error: {}",send_err);
                                        }
                                    }Err(error) => {
                                        let error = crate::gpu::workflows::gpu::setup_texture_generator::Error::SetupPhase2Error(error);
                                        let error = Some(Box::new(error)as Box<dyn std::any::Any+Send+Sync>);
                                        let failure_sender = match failure_sender {
                                            Some(failure_sender) => failure_sender,None => {
                                                unreachable!("RenderWhile response handler error: Failure event send error: No failure sender provided");
                                            }
                                        };
                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_error:error,
                                        }){
                                            unreachable!("RenderWhile response handler error: Failure event send error: {}",send_err);
                                        }
                                    }
                                }
                            })
                                    },
                                ),
                                handle_render_while_run_response: Box::new(
                                    |module_name: &'static str,
                                     workflow_name: &'static str,
                                     response: Option<Box<dyn std::any::Any + Send + Sync>>,
                                     wait_sender: crossbeam_channel::Sender<
                                        crate::workflow::events::StageWaitEvent,
                                    >,
                                     completion_sender: crossbeam_channel::Sender<
                                        crate::workflow::events::StageCompletionEvent,
                                    >,
                                     failure_sender: Option<
                                        crossbeam_channel::Sender<
                                            crate::workflow::events::StageFailureEvent,
                                        >,
                                    >| {
                                        Box::new(move|stage:crate::workflow::stage::StageRenderWhile|{
                                let response = response.expect("RenderWhile stages with output and error must have a response");
                                let outcome_result:Result<crate::workflow::types::Outcome<crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase2::core_types::State,crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase2::core_types::Output> ,crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase2::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                                match outcome_result {
                                    Ok(outcome) => {
                                        match outcome {
                                            crate::workflow::types::Outcome::Wait(state) => {
                                                let state = Some(Box::new(state)as Box<dyn std::any::Any+Send+Sync>);
                                                if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                    ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_state:state,
                                                }){
                                                    unreachable!("RenderWhile response handler error: Wait event send error: {}",send_err);
                                                }
                                            },crate::workflow::types::Outcome::Done(output) => {
                                                let output:crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase3::core_types::Input = unsafe {
                                                    std::mem::transmute(output)
                                                };
                                                let output = Some(Box::new(output)as Box<dyn std::any::Any+Send+Sync>);
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                    ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_output:output,
                                                }){
                                                    unreachable!("RenderWhile response handler error: Completion event send error: {}",send_err);
                                                }
                                            }
                                        }
                                    }Err(error) => {
                                        let error = crate::gpu::workflows::gpu::setup_texture_generator::Error::SetupPhase2Error(error);
                                        let error = Some(Box::new(error)as Box<dyn std::any::Any+Send+Sync>);
                                        let failure_sender = match failure_sender {
                                            Some(failure_sender) => failure_sender,None => {
                                                unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                            }
                                        };
                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_error:error,
                                        }){
                                            unreachable!("RenderWhile response handler error: Failure event send error: {}",send_err);
                                        }
                                    }
                                }
                            })
                                    },
                                ),
                                setup_sender: crate::workflow::channels::get_stage_setup_sender()
                                    .clone(),
                                wait_sender: crate::workflow::channels::get_stage_wait_sender()
                                    .clone(),
                                completion_sender:
                                    crate::workflow::channels::get_stage_completion_sender().clone(),
                                failure_sender: Some(
                                    crate::workflow::channels::get_stage_failure_sender().clone(),
                                ),
                            },
                        ),
                        crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                            index: 2,
                            name: stringify!("SetupPhase3"),
                            signature: crate::workflow::stage::StageSignature::I,
                            run_ecs: Box::new(self::stages::setup_phase3::core_functions::run_ecs)
                                as Box<
                                    dyn FnMut(
                                            Option<Box<dyn std::any::Any + Send + Sync>>,
                                            &mut bevy::prelude::World,
                                        )
                                            -> Option<Box<dyn std::any::Any + Send + Sync>>
                                        + Send
                                        + Sync,
                                >,
                            handle_ecs_run_response: Box::new(
                                |_module_name: &'static str,
                                 _workflow_name: &'static str,
                                 _response: Option<Box<dyn std::any::Any + Send + Sync>>,
                                 _completion_sender: crossbeam_channel::Sender<
                                    crate::workflow::events::StageCompletionEvent,
                                >,
                                 _failure_sender: Option<
                                    crossbeam_channel::Sender<
                                        crate::workflow::events::StageFailureEvent,
                                    >,
                                >| {
                                    Box::new(move |_stage: crate::workflow::stage::StageEcs| {
                                        unreachable!(
                                            "Tried to call ecs run response handler placeholder"
                                        );
                                    })
                                },
                            ),
                            completion_sender:
                                crate::workflow::channels::get_stage_completion_sender().clone(),
                            failure_sender: None,
                        }),
                    ],
                }
            }
        }
        pub mod workflow_imports {
            pub use super::user_items::*;
            pub(super) use crate::gpu::resources::ShaderRegistry;
            pub use crate::workflow::types::{Outcome, Outcome::Done, Outcome::Wait};
            pub(super) use bevy::ecs::system::SystemState;
            pub use bevy::prelude::World;
            pub(super) use bevy::prelude::{Assets, Handle, Res, ResMut, Shader};
            pub(super) use bevy::render::render_asset::RenderAssets;
            pub(super) use bevy::render::render_resource::{
                BindGroupLayout, BindGroupLayoutEntry, BindingType, BufferBindingType,
                CachedComputePipelineId, CachedPipelineState, ComputePipelineDescriptor, Pipeline,
                PipelineCache, PushConstantRange, ShaderStages, StorageTextureAccess,
                TextureFormat, TextureViewDimension,
            };
            pub(super) use bevy::render::renderer::RenderDevice;
            pub(super) use bevy::render::texture::GpuImage;
        }
        pub mod user_items {
            use super::workflow_imports::*;
        }
        pub mod stages {
            pub mod setup_phase1 {
                pub const NAME: &str = stringify!("SetupPhase1");
                pub mod core_types {
                    use super::super::super::workflow_imports::*;
                    use thiserror::Error;
                    pub struct Input {
                        pub shader_name: &'static str,
                        pub shader_path: String,
                    }
                    pub struct Output {
                        pub shader_name: &'static str,
                        pub shader_handle: Handle<Shader>,
                    }
                    #[derive(std::fmt::Debug, Error)]
                    pub enum Error {
                        ShaderAlreadyRegistered {
                            shader_name: &'static str,
                        },
                        FailedToReadShader {
                            shader_name: &'static str,
                            error: std::io::Error,
                        },
                    }
                    impl std::fmt::Display for Error {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{:?}", self)
                        }
                    }
                }
                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;
                    pub fn run_ecs(
                        input: Option<Box<dyn std::any::Any + Send + Sync>>,
                        world: &mut World,
                    ) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                        let input = input.unwrap().downcast::<Input>().unwrap();
                        let result = run_ecs_inner(*input, world);
                        Some(Box::new(result))
                    }
                    fn run_ecs_inner(input: Input, world: &mut World) -> Result<Output, Error> {
                        let shader_name = input.shader_name;
                        let shader_path = &input.shader_path;
                        let mut system_state: SystemState<(
                            ResMut<Assets<Shader>>,
                            Res<ShaderRegistry>,
                        )> = SystemState::new(world);
                        let (mut shader_assets, shader_registry) = system_state.get_mut(world);
                        if shader_registry.shaders.contains_key(shader_name) {
                            return Err(Error::ShaderAlreadyRegistered { shader_name });
                        }
                        let shader_source = std::fs::read_to_string(shader_path).map_err(|e| {
                            Error::FailedToReadShader {
                                shader_name,
                                error: e,
                            }
                        })?;
                        let shader = Shader::from_wgsl(shader_source, shader_path.clone());
                        let shader_handle = shader_assets.add(shader);
                        Ok(Output {
                            shader_name,
                            shader_handle,
                        })
                    }
                }
            }
            pub mod setup_phase2 {
                pub const NAME: &str = stringify!("SetupPhase2");
                pub mod core_types {
                    use super::super::super::workflow_imports::*;
                    use thiserror::Error;
                    pub struct Input {
                        pub shader_name: &'static str,
                        pub shader_handle: Handle<Shader>,
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
                    #[derive(std::fmt::Debug, Error)]
                    pub enum Error {
                        ExpectedComputePipelineGotRenderPipeline {
                            shader_name: String,
                            pipeline_id: CachedComputePipelineId,
                        },
                        FailedToCreatePipeline {
                            shader_name: &'static str,
                            pipeline_cache_err: String,
                        },
                    }
                    impl std::fmt::Display for Error {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{:?}", self)
                        }
                    }
                }
                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;
                    pub fn setup_render_while(
                        input: Option<Box<dyn std::any::Any + Send + Sync>>,
                        world: &mut World,
                    ) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                        let input = input.unwrap().downcast::<Input>().unwrap();
                        let result = setup_render_while_inner(*input, world);
                        Some(Box::new(result))
                    }
                    fn setup_render_while_inner(
                        input: Input,
                        world: &mut World,
                    ) -> Result<State, Error> {
                        let shader_name = input.shader_name;
                        let shader_handle = input.shader_handle;
                        let mut system_state: SystemState<(
                            Res<RenderDevice>,
                            Res<PipelineCache>,
                            Res<RenderAssets<GpuImage>>,
                        )> = SystemState::new(world);
                        let (render_device, pipeline_cache, gpu_images) = system_state.get(world);
                        let bind_group_layout = render_device.create_bind_group_layout(
                            None,
                            &[
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
                        let pipeline_id =
                            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
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
                        Ok(State {
                            shader_name,
                            shader_handle,
                            bind_group_layout,
                            pipeline_id,
                        })
                    }
                    pub fn run_render_while(
                        state: Option<Box<dyn std::any::Any + Send + Sync>>,
                        world: &mut World,
                    ) -> Box<dyn std::any::Any + Send + Sync> {
                        let state = state.unwrap().downcast::<State>().unwrap();
                        let outcome_result = run_render_while_inner(*state, world);
                        Box::new(outcome_result)
                    }
                    fn run_render_while_inner(
                        state: State,
                        world: &mut World,
                    ) -> Result<Outcome<State, Output>, Error> {
                        let shader_name = state.shader_name;
                        let shader_handle = state.shader_handle.clone();
                        let bind_group_layout = state.bind_group_layout.clone();
                        let pipeline_id = state.pipeline_id.clone();
                        let pipeline_cache =
                            SystemState::<Res<PipelineCache>>::new(world).get(world);
                        match pipeline_cache.get_compute_pipeline_state(pipeline_id) {
                            CachedPipelineState::Queued | CachedPipelineState::Creating(_) => {
                                Ok(Wait(state))
                            }
                            CachedPipelineState::Err(err) => Err(Error::FailedToCreatePipeline {
                                shader_name,
                                pipeline_cache_err: format!("{}", err),
                            }),
                            CachedPipelineState::Ok(pipeline) => match pipeline {
                                Pipeline::RenderPipeline(_) => {
                                    Err(Error::ExpectedComputePipelineGotRenderPipeline {
                                        shader_name: state.shader_name.to_string(),
                                        pipeline_id: state.pipeline_id,
                                    })
                                }
                                Pipeline::ComputePipeline(_) => Ok(Done(Output {
                                    shader_name,
                                    shader_handle,
                                    pipeline_id,
                                    bind_group_layout,
                                })),
                            },
                        }
                    }
                }
            }
            pub mod setup_phase3 {
                pub const NAME: &str = stringify!("SetupPhase3");
                pub mod core_types {
                    use super::super::super::workflow_imports::*;
                    pub struct Input {
                        pub shader_name: &'static str,
                        pub shader_handle: Handle<Shader>,
                        pub pipeline_id: CachedComputePipelineId,
                        pub bind_group_layout: BindGroupLayout,
                    }
                }
                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;
                    pub fn run_ecs(
                        input: Option<Box<dyn std::any::Any + Send + Sync>>,
                        world: &mut World,
                    ) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                        let input = input.unwrap().downcast::<Input>().unwrap();
                        run_ecs_inner(*input, world);
                        None
                    }
                    fn run_ecs_inner(input: Input, world: &mut World) {
                        let shader_name = input.shader_name;
                        let shader_handle = input.shader_handle;
                        let bind_group_layout = input.bind_group_layout;
                        let pipeline_id = input.pipeline_id;
                        let mut shader_registry =
                            SystemState::<ResMut<ShaderRegistry>>::new(world).get_mut(world);
                        shader_registry
                            .shaders
                            .insert(shader_name.to_string(), shader_handle);
                        shader_registry
                            .pipelines
                            .insert(shader_name.to_string(), pipeline_id);
                        shader_registry
                            .bind_group_layouts
                            .insert(shader_name.to_string(), bind_group_layout);
                    }
                }
            }
        }
    }
    pub mod generate_texture {
        pub const NAME: &str = stringify!("GenerateTexture");
        pub async fn run(
            input: <TypeIOE as crate::workflow::traits::WorkflowTypeIOE>::Input,
        ) -> Result<
            <TypeIOE as crate::workflow::traits::WorkflowTypeIOE>::Output,
            <TypeIOE as crate::workflow::traits::WorkflowTypeIOE>::Error,
        > {
            crate::workflow::functions::run_workflow_ioe::<TypeIOE>(input).await
        }
        #[derive(std::fmt::Debug, thiserror::Error)]
        pub enum Error {
            PrepareRequestError(self::stages::prepare_request::core_types::Error),
            WaitForComputeError(self::stages::wait_for_compute::core_types::Error),
        }
        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        pub struct TypeIOE;

        impl crate::workflow::traits::WorkflowTypeIOE for TypeIOE {
            type Input = self::stages::prepare_request::core_types::Input;
            type Output = self::stages::wait_for_compute::core_types::Output;
            type Error = Error;
            const MODULE_NAME: &'static str = super::NAME;
            const WORKFLOW_NAME: &'static str = self::NAME;
        }
        impl TypeIOE {
            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                crate::workflow::types::WorkflowType {
                    name:self::NAME,stages:vec![crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                        index:0,name:stringify!("PrepareRequest"),signature:crate::workflow::stage::StageSignature::IOE,run_ecs:Box::new(self::stages::prepare_request::core_functions::run_ecs)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> , &mut bevy::prelude::World)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,handle_ecs_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                            Box::new(move|stage:crate::workflow::stage::StageEcs|{
                                let response = response.expect("Ecs stages with output and error must have a response");
                                let result_data:Result<crate::gpu::workflows::gpu::generate_texture::stages::prepare_request::core_types::Output,crate::gpu::workflows::gpu::generate_texture::stages::prepare_request::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                                match result_data {
                                    Ok(output) => {
                                        let output:crate::gpu::workflows::gpu::generate_texture::stages::get_texture_view::core_types::Input = unsafe {
                                            std::mem::transmute(output)
                                        };
                                        let output = Some(Box::new(output)as Box<dyn std::any::Any+Send+Sync>);
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty:crate::workflow::stage::StageType::Ecs,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::Ecs(stage),stage_output:output,
                                        }){
                                            unreachable!("Ecs response handler error: Completion event send error: {}",send_err);
                                        }
                                    }Err(error) => {
                                        let error = crate::gpu::workflows::gpu::generate_texture::Error::PrepareRequestError(error);
                                        let error = Some(Box::new(error)as Box<dyn std::any::Any+Send+Sync>);
                                        let failure_sender = match failure_sender {
                                            Some(failure_sender) => failure_sender,None => {
                                                unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                            }
                                        };
                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty:crate::workflow::stage::StageType::Ecs,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::Ecs(stage),stage_error:error,
                                        }){
                                            unreachable!("Ecs response handler error: Failure event send error: {}",send_err);
                                        }
                                    }
                                }
                            })
                        }),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:Some(crate::workflow::channels::get_stage_failure_sender().clone()),
                    }),crate::workflow::stage::Stage::RenderWhile(crate::workflow::stage::StageRenderWhile {
                        index:1,name:stringify!("GetTextureView"),signature:crate::workflow::stage::StageSignature::IO,setup_render_while:Box::new(self::stages::get_texture_view::core_functions::setup_render_while)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> , &mut bevy::prelude::World)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,run_render_while:Box::new(self::stages::get_texture_view::core_functions::run_render_while)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> , &mut bevy::prelude::World)->Box<dyn std::any::Any+Send+Sync> +Send+Sync> ,handle_render_while_setup_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,setup_sender:crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent> ,_failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                            Box::new(move|stage:crate::workflow::stage::StageRenderWhile|{
                                let response = response.expect("RenderWhile stages with state must have a response");
                                let state:crate::gpu::workflows::gpu::generate_texture::stages::get_texture_view::core_types::State =  *response.downcast().expect("Failed to downcast response result data");
                                let state = Some(Box::new(state)as Box<dyn std::any::Any+Send+Sync>);
                                if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                                    ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_state:state,
                                }){
                                    unreachable!("RenderWhile response handler error: Setup event send error: {}",send_err);
                                }
                            })
                        }),handle_render_while_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,wait_sender:crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,_failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                            Box::new(move|stage:crate::workflow::stage::StageRenderWhile|{
                                let response = response.expect("RenderWhile stages with output must have a response");
                                let outcome:crate::workflow::types::Outcome<crate::gpu::workflows::gpu::generate_texture::stages::get_texture_view::core_types::State,crate::gpu::workflows::gpu::generate_texture::stages::get_texture_view::core_types::Output>  =  *response.downcast().expect("Failed to downcast response outcome data");
                                match outcome {
                                    crate::workflow::types::Outcome::Wait(state) => {
                                        let state = Some(Box::new(state)as Box<dyn std::any::Any+Send+Sync>);
                                        if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                            ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_state:state,
                                        }){
                                            unreachable!("RenderWhile response handler error: Wait event send error: {}",send_err);
                                        }
                                    },crate::workflow::types::Outcome::Done(output) => {
                                        let output:crate::gpu::workflows::gpu::generate_texture::stages::dispatch_compute::core_types::Input = unsafe {
                                            std::mem::transmute(output)
                                        };
                                        let output = Some(Box::new(output)as Box<dyn std::any::Any+Send+Sync>);
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_output:output,
                                        }){
                                            unreachable!("RenderWhile response handler error: Completion event send error: {}",send_err);
                                        }
                                    }
                                }
                            })
                        }),setup_sender:crate::workflow::channels::get_stage_setup_sender().clone(),wait_sender:crate::workflow::channels::get_stage_wait_sender().clone(),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:None,
                    }),crate::workflow::stage::Stage::Render(crate::workflow::stage::StageRender {
                        index:2,name:stringify!("DispatchCompute"),signature:crate::workflow::stage::StageSignature::IO,run_render:Box::new(self::stages::dispatch_compute::core_functions::run_render)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> , &mut bevy::prelude::World)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,handle_render_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,_failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                            Box::new(move|stage:crate::workflow::stage::StageRender|{
                                let response = response.expect("Render stages with output (last stage) must have a response");
                                let output:crate::gpu::workflows::gpu::generate_texture::stages::dispatch_compute::core_types::Output =  *response.downcast().expect("Failed to downcast response output data");
                                let output:crate::gpu::workflows::gpu::generate_texture::stages::wait_for_compute::core_types::Input = unsafe {
                                    std::mem::transmute(output)
                                };
                                let output = Some(Box::new(output)as Box<dyn std::any::Any+Send+Sync>);
                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                    ty:crate::workflow::stage::StageType::Render,module_name,workflow_name,current_stage:2,stage_return:crate::workflow::stage::Stage::Render(stage),stage_output:output,
                                }){
                                    unreachable!("Render response handler error: Completion event send error: {}",send_err);
                                }
                            })
                        }),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:None,
                    }),crate::workflow::stage::Stage::EcsWhile(crate::workflow::stage::StageEcsWhile {
                        index:3,name:stringify!("WaitForCompute"),signature:crate::workflow::stage::StageSignature::IOE,setup_ecs_while:Box::new(self::stages::wait_for_compute::core_functions::setup_ecs_while)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> , &mut bevy::prelude::World)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,run_ecs_while:Box::new(self::stages::wait_for_compute::core_functions::run_ecs_while)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> , &mut bevy::prelude::World)->Box<dyn std::any::Any+Send+Sync> +Send+Sync> ,handle_ecs_while_setup_response:Box::new(|_module_name: &'static str,_workflow_name: &'static str,_response:Option<Box<dyn std::any::Any+Send+Sync>> ,_setup_sender:crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent> ,_failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                            unreachable!("Tried to call ecs while setup response handler placeholder");
                        }),handle_ecs_while_run_response:Box::new(|_module_name: &'static str,_workflow_name: &'static str,_response:Option<Box<dyn std::any::Any+Send+Sync>> ,_wait_sender:crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent> ,_completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,_failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                            Box::new(move|_stage:crate::workflow::stage::StageEcsWhile|{
                                unreachable!("Tried to call ecs while run response handler placeholder");
                            })
                        }),setup_sender:crate::workflow::channels::get_stage_setup_sender().clone(),wait_sender:crate::workflow::channels::get_stage_wait_sender().clone(),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:Some(crate::workflow::channels::get_stage_failure_sender().clone()),
                    })],
                }
            }
        }
        pub mod workflow_imports {
            pub use super::user_items::*;
            pub(super) use crate::gpu::resources::ShaderRegistry;
            pub use crate::workflow::types::{Outcome, Outcome::Done, Outcome::Wait};
            pub(super) use bevy::ecs::system::SystemState;
            pub use bevy::prelude::World;
            pub(super) use bevy::prelude::{Assets, Handle, Image, Res, ResMut};
            pub(super) use bevy::render::render_asset::RenderAssets;
            pub(super) use bevy::render::render_resource::{
                BindGroupEntry, BindingResource, PipelineCache,
            };
            pub(super) use bevy::render::render_resource::{
                BindGroupLayout, Buffer, BufferInitDescriptor, BufferUsages,
                CachedComputePipelineId, CommandEncoderDescriptor, ComputePassDescriptor, Extent3d,
                TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
            };
            pub(super) use bevy::render::renderer::{RenderDevice, RenderQueue};
            pub(super) use bevy::render::texture::GpuImage;
            pub(super) use crossbeam_channel::Receiver;
        }
        pub mod user_items {
            use super::workflow_imports::*;
            pub struct GeneratorRequest<T> {
                pub inner: T,
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
                        },
                    }
                }
                pub fn set_texture_view(
                    self,
                    texture_view: TextureView,
                ) -> GeneratorRequest<PreparedGenerator> {
                    GeneratorRequest {
                        inner: PreparedGenerator {
                            shader_name: self.inner.shader_name,
                            pipeline_id: self.inner.pipeline_id,
                            bind_group_layout: self.inner.bind_group_layout,
                            texture_handle: self.inner.texture_handle,
                            texture_view,
                            param_buffer: self.inner.param_buffer,
                        },
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
                pub fn track_dispatch(
                    self,
                    texture_handle: Handle<Image>,
                    receiver: Receiver<()>,
                ) -> GeneratorRequest<DispatchedCompute> {
                    GeneratorRequest {
                        inner: DispatchedCompute {
                            shader_name: self.inner.shader_name,
                            texture_handle,
                            receiver,
                        },
                    }
                }
            }
            pub struct DispatchedCompute {
                pub shader_name: &'static str,
                pub texture_handle: Handle<Image>,
                pub receiver: Receiver<()>,
            }
            impl GeneratorRequest<DispatchedCompute> {
                pub fn consume(self) -> (&'static str, Handle<Image>) {
                    (self.inner.shader_name, self.inner.texture_handle)
                }
            }
        }
        pub mod stages {
            pub mod prepare_request {
                pub const NAME: &str = stringify!("PrepareRequest");
                pub mod core_types {
                    use super::super::super::workflow_imports::*;
                    use thiserror::Error;
                    pub struct Input {
                        pub shader_name: &'static str,
                        pub texture_size: usize,
                        pub param_data: Vec<f32>,
                    }
                    pub struct Output {
                        pub request: GeneratorRequest<GeneratorParams>,
                    }
                    #[derive(std::fmt::Debug, Error)]
                    pub enum Error {
                        GeneratorNotFound { shader_name: &'static str },
                    }
                    impl std::fmt::Display for Error {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{:?}", self)
                        }
                    }
                }
                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;
                    pub fn run_ecs(
                        input: Option<Box<dyn std::any::Any + Send + Sync>>,
                        world: &mut World,
                    ) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                        let input = input.unwrap().downcast::<Input>().unwrap();
                        let result = run_ecs_inner(*input, world);
                        Some(Box::new(result))
                    }
                    fn run_ecs_inner(input: Input, world: &mut World) -> Result<Output, Error> {
                        let shader_name = input.shader_name;
                        let texture_size = input.texture_size;
                        let param_data = input.param_data;
                        let mut system_state: SystemState<(
                            Res<RenderDevice>,
                            ResMut<Assets<Image>>,
                            Res<ShaderRegistry>,
                        )> = SystemState::new(world);
                        let (render_device, mut images, shader_registry) =
                            system_state.get_mut(world);
                        if shader_registry.shaders.get(shader_name).is_none() {
                            return Err(Error::GeneratorNotFound { shader_name });
                        }
                        let pipeline_id =
                            shader_registry.pipelines.get(shader_name).unwrap().clone();
                        let bind_group_layout = shader_registry
                            .bind_group_layouts
                            .get(shader_name)
                            .unwrap()
                            .clone();
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
                        let param_buffer =
                            render_device.create_buffer_with_data(&BufferInitDescriptor {
                                label: Some("Parameter Buffer"),
                                contents: bytemuck::cast_slice(&param_data),
                                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
                            });
                        let request = GeneratorRequest::new(
                            shader_name,
                            pipeline_id,
                            bind_group_layout,
                            texture_handle,
                            param_buffer,
                        );
                        Ok(Output { request })
                    }
                }
            }
            pub mod get_texture_view {
                pub const NAME: &str = stringify!("GetTextureView");
                pub mod core_types {
                    use super::super::super::workflow_imports::*;
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
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;
                    pub fn setup_render_while(
                        input: Option<Box<dyn std::any::Any + Send + Sync>>,
                        world: &mut World,
                    ) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                        let input = input.unwrap().downcast::<Input>().unwrap();
                        let state = setup_render_while_inner(*input, world);
                        Some(Box::new(state))
                    }
                    fn setup_render_while_inner(input: Input, world: &mut World) -> State {
                        State {
                            request: input.request,
                        }
                    }
                    pub fn run_render_while(
                        state: Option<Box<dyn std::any::Any + Send + Sync>>,
                        world: &mut World,
                    ) -> Box<dyn std::any::Any + Send + Sync> {
                        let state = state.unwrap().downcast::<State>().unwrap();
                        let outcome = run_render_while_inner(*state, world);
                        Box::new(outcome)
                    }
                    fn run_render_while_inner(
                        state: State,
                        world: &mut World,
                    ) -> Outcome<State, Output> {
                        let gpu_images =
                            SystemState::<Res<RenderAssets<GpuImage>>>::new(world).get(world);
                        if let Some(gpu_image) = gpu_images.get(&state.request.inner.texture_handle)
                        {
                            let texture_view = gpu_image.texture_view.clone();
                            let prepared_request = state.request.set_texture_view(texture_view);
                            Done(Output {
                                request: prepared_request,
                            })
                        } else {
                            Wait(state)
                        }
                    }
                }
            }
            pub mod dispatch_compute {
                pub const NAME: &str = stringify!("DispatchCompute");
                pub mod core_types {
                    use super::super::super::workflow_imports::*;
                    pub struct Input {
                        pub request: GeneratorRequest<PreparedGenerator>,
                    }
                    pub struct Output {
                        pub request: GeneratorRequest<DispatchedCompute>,
                    }
                }
                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;
                    pub fn run_render(
                        input: Option<Box<dyn std::any::Any + Send + Sync>>,
                        world: &mut World,
                    ) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                        let input = input.unwrap().downcast::<Input>().unwrap();
                        let output = run_render_inner(*input, world);
                        Some(Box::new(output))
                    }
                    fn run_render_inner(input: Input, world: &mut World) -> Output {
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
                        let pipeline = pipeline_cache
                            .get_compute_pipeline(pipeline_id)
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
                        let mut encoder = render_device
                            .create_command_encoder(&CommandEncoderDescriptor { label: None });
                        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                            label: None,
                            timestamp_writes: None,
                        });
                        compute_pass.set_pipeline(pipeline);
                        compute_pass.set_bind_group(0, &bind_group, &[]);
                        compute_pass.dispatch_workgroups(8, 8, 1);
                        drop(compute_pass);
                        queue.submit(Some(encoder.finish()));
                        let (sender, receiver) = crossbeam_channel::unbounded();
                        queue.on_submitted_work_done(move || {
                            let _ = sender.send(());
                        });
                        let dispatched_request =
                            input.request.track_dispatch(texture_handle, receiver);
                        Output {
                            request: dispatched_request,
                        }
                    }
                }
            }
            pub mod wait_for_compute {
                pub const NAME: &str = stringify!("WaitForCompute");
                pub mod core_types {
                    use super::super::super::workflow_imports::*;
                    use thiserror::Error;
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
                    #[derive(std::fmt::Debug, Error)]
                    pub enum Error {
                        ComputePassReceiverDisconnected { shader_name: &'static str },
                    }
                    impl std::fmt::Display for Error {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{:?}", self)
                        }
                    }
                }
                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;
                    pub fn setup_ecs_while(
                        input: Option<Box<dyn std::any::Any + Send + Sync>>,
                        world: &mut World,
                    ) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                        let input = input.unwrap().downcast::<Input>().unwrap();
                        let result = setup_ecs_while_inner(*input, world);
                        Some(Box::new(result))
                    }
                    fn setup_ecs_while_inner(
                        input: Input,
                        world: &mut World,
                    ) -> Result<State, Error> {
                        Ok(State {
                            request: input.request,
                        })
                    }
                    pub fn run_ecs_while(
                        state: Option<Box<dyn std::any::Any + Send + Sync>>,
                        world: &mut World,
                    ) -> Box<dyn std::any::Any + Send + Sync> {
                        let state = state.unwrap().downcast::<State>().unwrap();
                        let outcome_result = run_ecs_while_inner(*state, world);
                        Box::new(outcome_result)
                    }
                    fn run_ecs_while_inner(
                        state: State,
                        world: &mut World,
                    ) -> Result<Outcome<State, Output>, Error> {
                        let receiver = &state.request.inner.receiver;
                        match receiver.try_recv() {
                            Ok(_) => {
                                let (shader_name, texture_handle) = state.request.consume();
                                Ok(Done(Output {
                                    shader_name,
                                    texture_handle,
                                }))
                            }
                            Err(crossbeam_channel::TryRecvError::Empty) => Ok(Wait(state)),
                            Err(crossbeam_channel::TryRecvError::Disconnected) => {
                                Err(Error::ComputePassReceiverDisconnected {
                                    shader_name: state.request.inner.shader_name,
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}
