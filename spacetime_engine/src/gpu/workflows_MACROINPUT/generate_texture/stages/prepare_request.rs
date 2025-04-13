crate::workflow_stage_util!("PrepareRequest");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct MainAccess<'w> {
        render_device: Res<'w, RenderDevice>,
        images: ResMut<'w, Assets<Image>>,
        shader_registry: Res<'w, ShaderRegistry>,
    }
    pub struct Input {
        shader_name: &'static str,
        texture_size: usize,
        param_data: Vec<f32>,
    }
    pub struct Output {
        request: GeneratorRequest<GeneratorParams>,
    }
    pub enum Error {
        GeneratorNotFound {
            shader_name: &'static str,
        },
    }
}

pub mod core_functions {
    crate::workflow_stage_core_functions_util!();
    crate::workflow_stage_core_function_util!(run_ecs);

    pub fn run_ecs_inner(input: Input, main_access: MainAccess) -> Result<Output, Error> {
        let shader_name = input.shader_name;
        let texture_size = input.texture_size;
        let param_data = input.param_data;

        let render_device = main_access.render_device;
        let mut images = main_access.images;
        let shader_registry = main_access.shader_registry;

        if !shader_registry.shaders.contains_key(shader_name) {
            return Err(Error::GeneratorNotFound { shader_name })
        }

        let pipeline_id = *shader_registry.pipelines.get(shader_name).unwrap();
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
