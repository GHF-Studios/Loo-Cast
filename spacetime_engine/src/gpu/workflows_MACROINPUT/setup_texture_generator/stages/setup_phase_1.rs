crate::workflow_stage_util!("SetupPhase1");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct MainAccess<'w> {
        shader_assets: ResMut<'w, Assets<Shader>>,
        shader_registry: Res<'w, ShaderRegistry>,
    }
    pub struct Input {
        shader_name: &'static str,
        shader_path: String
    }
    pub struct Output {
        shader_name: &'static str,
        shader_handle: Handle<Shader>,
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
    crate::workflow_stage_core_functions_util!();
    crate::workflow_stage_core_function_util!(run_ecs);

    pub fn run_ecs_inner(input: Input, main_access: MainAccess) -> Result<Output, Error> {
        let shader_name = input.shader_name;
        let shader_path = &input.shader_path;

        let mut shader_assets = main_access.shader_assets;
        let shader_registry = main_access.shader_registry;

        if shader_registry.shaders.contains_key(shader_name) {
            return Err(Error::ShaderAlreadyRegistered { shader_name })
        }

        let shader_source = std::fs::read_to_string(shader_path)
            .map_err(|e| Error::FailedToReadShader { shader_name, error: e })?;

        let shader = Shader::from_wgsl(shader_source, shader_path.clone());
        let shader_handle = shader_assets.add(shader);

        Ok(Output { shader_name, shader_handle })
    }
}
