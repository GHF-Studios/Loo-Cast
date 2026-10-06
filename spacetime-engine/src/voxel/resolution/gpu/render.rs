//! Render-world allocation, shader admission, dispatch and completion reporting.

use super::*;

pub(super) fn init_gpu_terrain_pipeline(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
) {
    let density_layout = BindGroupLayoutDescriptor::new(
        "voxel GPU density",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                uniform_buffer::<GpuTerrainDispatch>(false),
                storage_buffer::<Vec<f32>>(false),
            ),
        ),
    );
    let topology_layout = BindGroupLayoutDescriptor::new(
        "voxel GPU Transvoxel",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                uniform_buffer::<GpuTerrainDispatch>(false),
                storage_buffer_read_only::<Vec<u32>>(false),
                storage_buffer_read_only::<Vec<f32>>(false),
                storage_buffer::<Vec<f32>>(false),
                storage_buffer::<Vec<u32>>(false),
            ),
        ),
    );

    let density_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("voxel GPU density".into()),
        layout: vec![density_layout.clone()],
        shader: GPU_TERRAIN_DENSITY_SHADER.clone(),
        ..default()
    });
    let topology_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("voxel GPU Transvoxel".into()),
        layout: vec![topology_layout.clone()],
        shader: GPU_TERRAIN_TOPOLOGY_SHADER.clone(),
        ..default()
    });

    commands.insert_resource(GpuTerrainPipeline {
        density_layout,
        density_pipeline,
        topology_layout,
        topology_pipeline,
    });
}

fn pack_transvoxel_tables() -> Vec<u32> {
    use transvoxel_data::{
        regular_cell_data::{REGULAR_CELL_CLASS, REGULAR_CELL_DATA, REGULAR_VERTEX_DATA},
        transition_cell_data::{
            TRANSITION_CELL_CLASS, TRANSITION_CELL_DATA, TRANSITION_VERTEX_DATA,
        },
    };

    let mut out = Vec::with_capacity(12_312);
    out.extend(REGULAR_CELL_CLASS.into_iter().map(u32::from));
    out.extend(
        REGULAR_CELL_DATA
            .into_iter()
            .map(|data| u32::from((data.get_vertex_count() << 4) | data.get_triangle_count())),
    );
    for data in REGULAR_CELL_DATA {
        out.extend(data.vertex_index.into_iter().map(u32::from));
    }
    for row in REGULAR_VERTEX_DATA {
        out.extend(row.into_iter().map(u32::from));
    }

    out.extend(TRANSITION_CELL_CLASS.into_iter().map(u32::from));
    out.extend(
        TRANSITION_CELL_DATA
            .into_iter()
            .map(|data| u32::from((data.get_vertex_count() << 4) | data.get_triangle_count())),
    );
    for data in TRANSITION_CELL_DATA {
        out.extend(data.vertex_index.into_iter().map(u32::from));
    }
    for row in TRANSITION_VERTEX_DATA {
        out.extend(row.into_iter().map(u32::from));
    }

    debug_assert_eq!(out.len(), 12_312);
    out
}

pub(super) fn init_gpu_terrain_buffers(
    mut commands: Commands,
    render_device: Res<bevy::render::renderer::RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    let mut tables = StorageBuffer::from(pack_transvoxel_tables());
    tables.set_label(Some("canonical Transvoxel lookup tables"));
    tables.write_buffer(&render_device, &render_queue);

    let mut scratch = StorageBuffer::from(vec![0.0_f32; 7665]);
    scratch.set_label(Some("GPU terrain density scratch"));
    scratch.write_buffer(&render_device, &render_queue);

    commands.insert_resource(GpuTransvoxelTables(tables));
    commands.insert_resource(GpuTerrainScratch(scratch));
}

pub(super) fn prepare_gpu_terrain_builds(
    blocks: Query<&GpuTerrainBuild>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<GpuTerrainPipeline>,
    mut state: ResMut<GpuTerrainBuildState>,
    mut failure_reported: Local<bool>,
) {
    state.pending.clear();
    state.active_meshes.clear();
    for block in &blocks {
        state.active_meshes.insert(block.mesh.id());
    }
    state.extracted_blocks = state.active_meshes.len();

    // Completed components are removed on the main world after publication.
    // Their build IDs cannot retain render-world bookkeeping indefinitely.
    let GpuTerrainBuildState {
        processed,
        active_meshes,
        ..
    } = &mut *state;
    processed.retain(|id, _| active_meshes.contains(id));

    let mut failed = None;
    let density_ready = match pipeline_cache.get_compute_pipeline_state(pipeline.density_pipeline) {
        CachedPipelineState::Ok(_) => true,
        CachedPipelineState::Err(ShaderCacheError::ShaderNotLoaded(_)) => false,
        CachedPipelineState::Err(err) => {
            failed = Some(("density", err));
            false
        }
        _ => false,
    };
    let topology_ready = match pipeline_cache.get_compute_pipeline_state(pipeline.topology_pipeline)
    {
        CachedPipelineState::Ok(_) => true,
        CachedPipelineState::Err(ShaderCacheError::ShaderNotLoaded(_)) => false,
        CachedPipelineState::Err(err) => {
            failed = Some(("Transvoxel topology", err));
            false
        }
        _ => false,
    };

    if let Some((stage, err)) = failed {
        if !*failure_reported {
            error!(
                stage,
                ?err,
                "GPU binary terrain compute pipeline failed; binary terrain presentation cannot advance"
            );
            *failure_reported = true;
        }
        return;
    }

    if !density_ready || !topology_ready {
        return;
    }
    *failure_reported = false;

    for block in &blocks {
        let id = block.mesh.id();
        if state.processed.get(&id) == Some(&block.build_id) {
            continue;
        }
        state.pending.push(block.clone());
    }
}

#[cfg(feature = "profiling-tracy")]
fn emit_gpu_terrain_pressure(
    mesh_allocator: &MeshAllocator,
    extracted_blocks: usize,
    pending_blocks: usize,
    dispatched_blocks: usize,
    transition_faces: u64,
    density_sample_calls: u64,
    fine_band_visits: u64,
    processed_records: usize,
) {
    let Some(client) = tracy_client::Client::running() else {
        return;
    };

    client.plot(
        tracy_client::plot_name!("GPU terrain/extracted blocks"),
        extracted_blocks as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/pending blocks"),
        pending_blocks as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/dispatched blocks/frame"),
        dispatched_blocks as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/transition faces/frame"),
        transition_faces as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/density samples/frame"),
        density_sample_calls as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/fine-band visits/frame"),
        fine_band_visits as f64,
    );

    // These MeshAllocator values cover the renderer's shared mesh slab domain,
    // which is useful for spotting terrain-driven allocator pressure.
    client.plot(
        tracy_client::plot_name!("GPU terrain/mesh slab bytes"),
        mesh_allocator.slabs_size() as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/mesh slab count"),
        mesh_allocator.slab_count() as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/index allocations"),
        mesh_allocator.index_allocation_count() as f64,
    );

    // Lower bound only: transition-capable shells carry more fixed indices.
    client.plot(
        tracy_client::plot_name!("GPU terrain/min fixed draw indices/frame"),
        extracted_blocks.saturating_mul(REGULAR_MAX_INDICES) as f64,
    );
    client.plot(
        tracy_client::plot_name!("GPU terrain/processed records"),
        processed_records as f64,
    );
}

pub(super) fn execute_gpu_terrain_builds(
    mut render_context: RenderContext,
    mesh_allocator: Res<MeshAllocator>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<GpuTerrainPipeline>,
    tables: Res<GpuTransvoxelTables>,
    scratch: Res<GpuTerrainScratch>,
    render_queue: Res<RenderQueue>,
    completion_sink: Res<GpuTerrainBuildCompletionSink>,
    mut state: ResMut<GpuTerrainBuildState>,
) {
    let Some(density_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.density_pipeline)
    else {
        return;
    };
    let Some(topology_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.topology_pipeline)
    else {
        return;
    };
    let Some(table_buffer) = tables.0.buffer() else {
        return;
    };
    let Some(scratch_buffer) = scratch.0.buffer() else {
        return;
    };

    // RenderDiagnosticsPlugin is automatically present in Bevy Tracy builds.
    // These become actual GPU timestamp zones in Tracy.
    let diagnostics = render_context.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();

    let mut pending = std::mem::take(&mut state.pending);
    state.completed_batch.clear();
    #[cfg(feature = "profiling-tracy")]
    let pending_count = pending.len();
    let mut dispatched = 0usize;
    let mut transition_faces = 0u64;
    let mut density_sample_calls = 0u64;
    let mut fine_band_visits = 0u64;

    for block in pending.drain(..) {
        let mesh_id = block.mesh.id();
        let Some(vertex_slice) = mesh_allocator.mesh_vertex_slice(&mesh_id) else {
            continue;
        };
        let Some(index_slice) = mesh_allocator.mesh_index_slice(&mesh_id) else {
            continue;
        };

        let block_transition_faces = block.descriptor.meta.y.count_ones() as u64;

        // 9^3 regular samples. Each transition face evaluates its 17^2
        // density lattice once; tangential gradients reuse it and only the
        // normal derivative performs two additional complete SDF samples.
        let block_density_calls = 729u64 + block_transition_faces.saturating_mul(289u64 * 3u64);

        transition_faces = transition_faces.saturating_add(block_transition_faces);
        density_sample_calls = density_sample_calls.saturating_add(block_density_calls);
        fine_band_visits = fine_band_visits
            .saturating_add(block_density_calls.saturating_mul(u64::from(block.descriptor.meta.w)));

        let dispatch = GpuTerrainDispatch {
            ranges: UVec4::new(
                vertex_slice.range.start * GPU_VERTEX_FLOATS,
                vertex_slice.range.end * GPU_VERTEX_FLOATS,
                index_slice.range.start,
                index_slice.range.end,
            ),
            descriptor: block.descriptor,
        };
        let mut uniform = UniformBuffer::from(dispatch);
        uniform.write_buffer(render_context.render_device(), &render_queue);

        let density_bind_group = render_context.render_device().create_bind_group(
            Some("voxel GPU density block"),
            &pipeline_cache.get_bind_group_layout(&pipeline.density_layout),
            &BindGroupEntries::sequential((&uniform, scratch_buffer.as_entire_buffer_binding())),
        );

        {
            let mut pass =
                render_context
                    .command_encoder()
                    .begin_compute_pass(&ComputePassDescriptor {
                        label: Some("voxel GPU density"),
                        ..default()
                    });
            let gpu_span = diagnostics.time_span(&mut pass, "voxel GPU terrain/density");
            pass.set_pipeline(density_pipeline);
            pass.set_bind_group(0, &density_bind_group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
            gpu_span.end(&mut pass);
        }

        let topology_bind_group = render_context.render_device().create_bind_group(
            Some("voxel GPU Transvoxel block"),
            &pipeline_cache.get_bind_group_layout(&pipeline.topology_layout),
            &BindGroupEntries::sequential((
                &uniform,
                table_buffer.as_entire_buffer_binding(),
                scratch_buffer.as_entire_buffer_binding(),
                vertex_slice.buffer.as_entire_buffer_binding(),
                index_slice.buffer.as_entire_buffer_binding(),
            )),
        );

        {
            let mut pass =
                render_context
                    .command_encoder()
                    .begin_compute_pass(&ComputePassDescriptor {
                        label: Some("voxel GPU Transvoxel"),
                        ..default()
                    });
            let gpu_span = diagnostics.time_span(&mut pass, "voxel GPU terrain/topology");
            pass.set_pipeline(topology_pipeline);
            pass.set_bind_group(0, &topology_bind_group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
            gpu_span.end(&mut pass);
        }

        dispatched = dispatched.saturating_add(1);
        state.processed.insert(mesh_id, block.build_id);
        state.completed_batch.push(block.build_id);
    }

    // Preserve allocation capacity across render frames and publish the whole
    // completion batch under one synchronization acquisition.
    state.pending = pending;
    if !state.completed_batch.is_empty() {
        completion_sink.0.extend(state.completed_batch.drain(..));
    }

    #[cfg(feature = "profiling-tracy")]
    emit_gpu_terrain_pressure(
        &mesh_allocator,
        state.extracted_blocks,
        pending_count,
        dispatched,
        transition_faces,
        density_sample_calls,
        fine_band_visits,
        state.processed.len(),
    );
}
