crate::workflow_stage_util!("SetupPhase2");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct RenderAccess<'w> {
        render_device: Res<'w, RenderDevice>,
        pipeline_cache: Res<'w, PipelineCache>,
    }
    pub struct Input {
        shader_name: &'static str,
        shader_handle: Handle<Shader>
    }
    pub struct State {
        shader_name: &'static str,
        shader_handle: Handle<Shader>,
        bind_group_layout: BindGroupLayout,
        pipeline_id: CachedComputePipelineId,
    }
    pub struct Output {
        shader_name: &'static str,
        shader_handle: Handle<Shader>,
        pipeline_id: CachedComputePipelineId,
        bind_group_layout: BindGroupLayout,
    }
    pub enum Error {
        ExpectedComputePipelineGotRenderPipeline {
            shader_name: String,
            pipeline_id: CachedComputePipelineId,
        },
        FailedToCreatePipeline {
            shader_name: &'static str,
            pipeline_cache_err: String,
        }
    }
}

pub mod core_functions {
    crate::workflow_stage_core_functions_util!();
    crate::workflow_stage_core_function_util!(setup_render_while);
    crate::workflow_stage_core_function_util!(run_render_while);

    pub fn setup_render_while_inner(input: Input, render_access: RenderAccess) -> Result<State, Error> {
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
    pub fn run_render_while_inner(state: State, render_access: RenderAccess) -> Result<Outcome<State, Output>, Error> {
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
}
