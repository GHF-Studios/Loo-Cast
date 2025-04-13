crate::workflow_stage_util!("GetTextureView");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct RenderAccess<'w> {
        gpu_images: Res<'w, RenderAssets<GpuImage>>,
    }
    pub struct Input {
        request: GeneratorRequest<GeneratorParams>,
    }
    pub struct State {
        request: GeneratorRequest<GeneratorParams>,
    }
    pub struct Output {
        request: GeneratorRequest<PreparedGenerator>,
    }
}

pub mod core_functions {
    crate::workflow_stage_core_functions_util!();
    crate::workflow_stage_core_function_util!(setup_render_while);
    crate::workflow_stage_core_function_util!(run_render_while);

    pub fn setup_render_while_inner(input: Input, render_access: RenderAccess) -> State {
        State { request: input.request }
    }
    pub fn run_render_while_inner(state: State, render_access: RenderAccess) -> Outcome<State, Output> {
        let gpu_images = render_access.gpu_images;

        if let Some(gpu_image) = gpu_images.get(&state.request.inner.texture_handle) {
            let texture_view = gpu_image.texture_view.clone();

            let prepared_request = state.request.set_texture_view(texture_view);
            Done(Output { request: prepared_request })
        } else {
            Wait(state)
        }
    }
}
