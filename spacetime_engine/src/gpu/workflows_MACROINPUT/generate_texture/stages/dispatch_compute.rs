crate::workflow_stage_util!("DispatchCompute");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct RenderAccess<'w> {
        render_device: Res<'w, RenderDevice>,
        queue: Res<'w, RenderQueue>,
        pipeline_cache: Res<'w, PipelineCache>,
    }
    pub struct Input {
        request: GeneratorRequest<PreparedGenerator>,
    }
    pub struct Output {
        request: GeneratorRequest<DispatchedCompute>,
    }
}

pub mod core_functions {
    crate::workflow_stage_core_functions_util!();
    crate::workflow_stage_core_function_util!(run_render);

    pub fn run_render_inner(input: Input, render_access: RenderAccess) -> Output {
        let prepared = &input.request.inner;
        let pipeline_id = prepared.pipeline_id;
        let bind_group_layout = &prepared.bind_group_layout;
        let texture_handle = prepared.texture_handle.clone();
        let texture_view = &prepared.texture_view;
        let param_buffer = &prepared.param_buffer;

        let render_device = render_access.render_device;
        let queue = render_access.queue;
        let pipeline_cache = render_access.pipeline_cache;

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
