crate::workflow_stage_util!("SetupPhase3");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct MainAccess<'w> {
        shader_registry: ResMut<'w, ShaderRegistry>,
    }
    pub struct Input {
        shader_name: &'static str,
        shader_handle: Handle<Shader>,
        pipeline_id: CachedComputePipelineId,
        bind_group_layout: BindGroupLayout,
    }
}

pub mod core_functions {
    crate::workflow_stage_core_functions_util!();
    crate::workflow_stage_core_function_util!(run_ecs);

    pub fn run_ecs_inner(input: Input, main_access: MainAccess) {
        let shader_name = input.shader_name;
        let shader_handle = input.shader_handle;
        let bind_group_layout = input.bind_group_layout;
        let pipeline_id = input.pipeline_id;

        let mut shader_registry = main_access.shader_registry;

        shader_registry.shaders.insert(shader_name.to_string(), shader_handle);
        shader_registry.pipelines.insert(shader_name.to_string(), pipeline_id);
        shader_registry.bind_group_layouts.insert(shader_name.to_string(), bind_group_layout);
    }
}
