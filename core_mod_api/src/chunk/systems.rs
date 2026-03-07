use crate::bevy::prelude::*;
use core_mod_macros::{composite_workflow, composite_workflow_return};
use std::collections::HashSet;
use std::time::Duration;

use crate::chunk::components::ChunkLoader;
use crate::chunk::enums::ZoomState;
use crate::chunk::messages::ChunkBatchLifecycleMessage;
use crate::chunk::resources::{
    ChunkActionWorkflowState, ChunkBatchPlanResult, ChunkBatchTracker, ChunkLoadGate, ChunkLoadGateState, ChunkManager,
};
use crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput;
use crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput;
use crate::config::statics::CONFIG;
use crate::core::protocol::{AppOrchestrationSignal, AppOrchestrationState, OrchestrationPressure};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use crate::workflow::functions::{
    handle_composite_workflow_return_now, run_workflow_io_with_timeout_control, run_workflow_ioe_with_timeout_control, WorkflowTimeoutControlDecision,
};
use crate::workflow::resources::WorkflowTimeoutSignalReceiver;
use crate::workflow::types::WorkflowTimeoutMode;

use super::resources::ChunkRenderHandles;

pub(crate) struct ChunkDetectionOutput {
    pub spawn_chunk_inputs: Vec<SpawnChunkInput>,
    pub despawn_chunk_inputs: Vec<DespawnChunkInput>,
    pub boundary_spawn_targets: HashSet<GridVec>,
    pub boundary_despawn_targets: HashSet<GridVec>,
}

#[tracing::instrument(skip_all)]
pub(crate) fn chunk_startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let quad = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));
    let light_material: Handle<ColorMaterial> = materials.add(ColorMaterial::from_color(Color::srgb(0.75, 0.75, 0.75)));
    let dark_material = materials.add(ColorMaterial::from_color(Color::srgb(0.25, 0.25, 0.25)));

    commands.insert_resource(ChunkRenderHandles {
        quad,
        light_material,
        dark_material,
    });
}

#[tracing::instrument(skip_all)]
pub(crate) fn chunk_zoom_cooldown_system(time: Res<Time<Virtual>>, mut timer: Local<f32>, mut query: Query<&mut ChunkLoader>) {
    if *timer > 0.0 {
        *timer -= time.delta_secs();
        if *timer < 0.0 {
            *timer = 0.0;
        }
    }

    for mut chunk_loader in query.iter_mut() {
        if chunk_loader.zoom_state != ZoomState::None && *timer == 0.0 {
            chunk_loader.zoom_state = ZoomState::None;
            *timer = CONFIG().get::<f32>("chunk_loader/zoom_cooldown_secs");
        }
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn chunk_timeout_signal_system(
    mut chunk_load_gate: ResMut<ChunkLoadGate>,
    timeout_signal_receiver: Option<Res<WorkflowTimeoutSignalReceiver>>,
) {
    let Some(timeout_signal_receiver) = timeout_signal_receiver else {
        return;
    };

    while let Ok(signal) = timeout_signal_receiver.0.try_recv() {
        let changed = chunk_load_gate.lock_by_timeout(signal.module_name, signal.workflow_name, signal.timeout_count);
        if changed {
            warn!(
                "ChunkLoadGate locked due to workflow timeout request: {}::{}, timeout_count={}",
                signal.module_name, signal.workflow_name, signal.timeout_count
            );
        }
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn sync_chunk_orchestration_state_system(
    chunk_load_gate: Option<Res<ChunkLoadGate>>,
    chunk_batch_tracker: Option<Res<ChunkBatchTracker>>,
    mut app_orchestration_state: Option<ResMut<AppOrchestrationState>>,
    mut app_orchestration_signal_writer: MessageWriter<AppOrchestrationSignal>,
) {
    let Some(mut app_orchestration_state) = app_orchestration_state else {
        return;
    };
    let Some(chunk_load_gate) = chunk_load_gate else {
        return;
    };

    let next_pressure = match chunk_load_gate.state {
        ChunkLoadGateState::Open => OrchestrationPressure::Open,
        ChunkLoadGateState::LockedByInFlightBoundary => OrchestrationPressure::BoundaryOverlap,
        ChunkLoadGateState::LockedByTimeout => OrchestrationPressure::TimeoutRecovery,
    };

    let active_batches = chunk_batch_tracker
        .as_ref()
        .map(|tracker| tracker.is_batch_running() as u64 + tracker.is_batch_planned() as u64)
        .unwrap_or_default();
    let active_retries = chunk_load_gate.lock_info.map(|info| info.timeout_count as u64).unwrap_or_default();

    if app_orchestration_state.pressure != next_pressure {
        app_orchestration_state.pressure = next_pressure;
        app_orchestration_signal_writer.write(AppOrchestrationSignal::PressureChanged {
            pressure: next_pressure,
            source: "chunk_load_gate".to_string(),
            details: format!(
                "gate_state={:?}, active_batches={}, timeout_retries={}",
                chunk_load_gate.state, active_batches, active_retries
            ),
        });
    }

    app_orchestration_state.active_batches = active_batches;
    app_orchestration_state.active_retries = active_retries;
}

fn chunk_workflow_timeout_decision(
    module_name: &'static str,
    workflow_name: &'static str,
    timeout_count: usize,
) -> WorkflowTimeoutControlDecision {
    if timeout_count == 1 {
        warn!(
            "Chunk workflow timeout request: {}::{}, timeout_count={}, decision=Retry",
            module_name, workflow_name, timeout_count
        );
        return WorkflowTimeoutControlDecision::Retry;
    }

    warn!(
        "Chunk workflow timeout escalation: {}::{}, timeout_count={}, decision=Panic",
        module_name, workflow_name, timeout_count
    );
    WorkflowTimeoutControlDecision::Panic
}

#[tracing::instrument(skip_all)]
pub(crate) fn chunk_detection_system(
    chunk_loader_query: Query<&ChunkLoader>,
    chunk_manager: Res<ChunkManager>,
) -> ChunkDetectionOutput {
    let chunk_loader = match chunk_loader_query.single() {
        Ok(data) => data,
        Err(_) => {
            // warn!("No ChunkLoader found in the world; skipping chunk detection. Despawning all chunks.");
            return ChunkDetectionOutput {
                spawn_chunk_inputs: Vec::new(),
                despawn_chunk_inputs: chunk_manager.chunks.iter().cloned().map(DespawnChunkInput::new).collect(),
                boundary_spawn_targets: HashSet::new(),
                boundary_despawn_targets: HashSet::new(),
            };
        }
    };

    let mut spawn_chunk_inputs = Vec::new();
    let mut despawn_chunk_inputs = Vec::new();
    let radius = chunk_manager.load_radius;
    let current_chunks = chunk_manager.chunks.clone();
    let mut chunk_loader_grid_coord_cursor = &chunk_loader.coord;
    let mut target_chunk_cone = Vec::new();

    // warn!("Starting Chunk Detection with current Chunks: {:?}", current_chunks);

    loop {
        let current_scale = chunk_loader_grid_coord_cursor.scale;
        let scale_diff = current_scale as i8 - chunk_loader.coord.scale as i8;
        if scale_diff > Scale::MAX_DIFF_SCALE_EXP {
            break;
        }

        // warn!("Chunk Detection at scale: {:?}", current_scale);
        let coords_in_radius = chunk_loader_grid_coord_cursor
            .query_grid_radius(radius)
            .into_iter()
            .collect::<HashSet<GridVec>>();
        // warn!("Detected Chunks: {:?}", coords_in_radius);
        target_chunk_cone.push((chunk_loader_grid_coord_cursor.clone(), coords_in_radius));

        if current_scale == Scale::MAX {
            break;
        }

        let Some(parent) = chunk_loader_grid_coord_cursor.parent.as_ref() else {
            break;
        };
        chunk_loader_grid_coord_cursor = parent;
    }

    target_chunk_cone.reverse();

    // for (chunk_loader_grid_coord, target_chunks) in &target_chunk_cone {
    //     warn!("Target Chunks at scale: {:?} => {{{}}} {:?}", chunk_loader_grid_coord.scale, target_chunks.len(), target_chunks);
    // }

    let mut required_target_chunks: HashSet<GridVec> = HashSet::new();
    for (_coord, target_chunks) in &target_chunk_cone {
        required_target_chunks.extend(target_chunks.iter().cloned());
    }

    let preload_enabled = CONFIG().get::<bool>("chunk_loader/preload_enabled");
    let preload_scale_steps = CONFIG().get::<u8>("chunk_loader/preload_scale_steps") as usize;
    let preload_radius = CONFIG().get::<u32>("chunk_loader/preload_radius");
    let preload_translation_bias_threshold = CONFIG().get::<f32>("chunk_loader/preload_translation_bias_threshold");

    let mut preload_target_chunks: HashSet<GridVec> = HashSet::new();
    if preload_enabled {
        // Predict near-border directional movement at the current scale.
        let mut directional_delta = IVec2::ZERO;
        let local_x = chunk_loader.usf_transform.translation.x.local as f32;
        let local_y = chunk_loader.usf_transform.translation.y.local as f32;
        if local_x >= preload_translation_bias_threshold {
            directional_delta.x = 1;
        } else if local_x <= -preload_translation_bias_threshold {
            directional_delta.x = -1;
        }
        if local_y >= preload_translation_bias_threshold {
            directional_delta.y = 1;
        } else if local_y <= -preload_translation_bias_threshold {
            directional_delta.y = -1;
        }
        if directional_delta != IVec2::ZERO {
            let directional_steps = preload_scale_steps.max(1);
            let directional_radius = radius.saturating_add(preload_radius.max(1));

            for step in 1..=directional_steps {
                let directional_center = chunk_loader.coord.clone() + (directional_delta * step as i32);
                preload_target_chunks.extend(directional_center.query_grid_radius(directional_radius).into_iter());
            }
        }
    }

    preload_target_chunks.retain(|coord| !required_target_chunks.contains(coord));

    let mut final_target_chunks = required_target_chunks.clone();
    final_target_chunks.extend(preload_target_chunks.iter().cloned());

    let chunks_to_load = final_target_chunks.difference(&current_chunks).cloned();
    let chunks_to_unload = current_chunks.difference(&final_target_chunks).cloned();
    let boundary_spawn_targets: HashSet<GridVec> = required_target_chunks.difference(&current_chunks).cloned().collect();
    let boundary_despawn_targets: HashSet<GridVec> = HashSet::new();

    for chunk in chunks_to_load {
        spawn_chunk_inputs.push(SpawnChunkInput {
            grid_coord: chunk,
            metric_texture: Handle::default(),
        });
    }

    for chunk in chunks_to_unload {
        despawn_chunk_inputs.push(DespawnChunkInput { grid_coord: chunk });
    }

    // if !despawn_chunk_inputs.is_empty() {
    //     error!(
    //         "DespawnChunk-Inputs detected: {:?}",
    //         despawn_chunk_inputs.iter().map(|input| input.grid_coord.clone()).collect::<Vec<_>>()
    //     );
    // }

    ChunkDetectionOutput {
        spawn_chunk_inputs,
        despawn_chunk_inputs,
        boundary_spawn_targets,
        boundary_despawn_targets,
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn chunk_management_system(
    In(detection_output): In<ChunkDetectionOutput>,
    chunk_loader_query: Query<&ChunkLoader>,
    mut workflow_state: ResMut<ChunkActionWorkflowState>,
    mut chunk_load_gate: ResMut<ChunkLoadGate>,
    mut chunk_batch_tracker: ResMut<ChunkBatchTracker>,
    mut chunk_batch_lifecycle_writer: MessageWriter<ChunkBatchLifecycleMessage>,
) {
    let ChunkDetectionOutput {
        spawn_chunk_inputs,
        despawn_chunk_inputs,
        boundary_spawn_targets,
        boundary_despawn_targets,
    } = detection_output;

    let current_view_scale = chunk_loader_query
        .single()
        .map(|chunk_loader| chunk_loader.coord.scale as i32)
        .unwrap_or_default();
    let current_view_coord = chunk_loader_query
        .single()
        .map(|chunk_loader| chunk_loader.coord.clone())
        .unwrap_or_default();
    let incoming_spawn_targets: HashSet<GridVec> = spawn_chunk_inputs.iter().map(|input| input.grid_coord.clone()).collect();
    let incoming_despawn_targets: HashSet<GridVec> = despawn_chunk_inputs.iter().map(|input| input.grid_coord.clone()).collect();
    let has_boundary_request = !boundary_spawn_targets.is_empty() || !boundary_despawn_targets.is_empty();
    if has_boundary_request {
        chunk_load_gate.boundary_quiet_frames = 0;
    }

    match chunk_batch_tracker.sync_plan(&incoming_spawn_targets, &incoming_despawn_targets) {
        ChunkBatchPlanResult::Unchanged => {}
        ChunkBatchPlanResult::Planned(batch) => {
            warn!(
                "Chunk batch planned: batch_id={}, spawn={}, despawn={}",
                batch.id,
                batch.spawn_count(),
                batch.despawn_count()
            );
            chunk_batch_lifecycle_writer.write(ChunkBatchLifecycleMessage::Planned { batch });
        }
        ChunkBatchPlanResult::Replanned { previous, planned } => {
            warn!(
                "Chunk batch cancelled: batch_id={}, reason=Replanned, spawn={}, despawn={}",
                previous.id,
                previous.spawn_count(),
                previous.despawn_count()
            );
            chunk_batch_lifecycle_writer.write(ChunkBatchLifecycleMessage::Cancelled {
                batch_id: previous.id,
                reason: crate::chunk::resources::ChunkBatchCancellationReason::Replanned,
                spawn_count: previous.spawn_count(),
                despawn_count: previous.despawn_count(),
            });
            warn!(
                "Chunk batch planned: batch_id={}, spawn={}, despawn={}",
                planned.id,
                planned.spawn_count(),
                planned.despawn_count()
            );
            chunk_batch_lifecycle_writer.write(ChunkBatchLifecycleMessage::Planned { batch: planned });
        }
        ChunkBatchPlanResult::Cleared { previous, reason } => {
            warn!(
                "Chunk batch cancelled: batch_id={}, reason={}, spawn={}, despawn={}",
                previous.id,
                reason.as_str(),
                previous.spawn_count(),
                previous.despawn_count()
            );
            chunk_batch_lifecycle_writer.write(ChunkBatchLifecycleMessage::Cancelled {
                batch_id: previous.id,
                reason,
                spawn_count: previous.spawn_count(),
                despawn_count: previous.despawn_count(),
            });
        }
    }

    // Step 1: If workflows are running, wait for all to complete
    if let Some(handles) = &mut workflow_state.handles {
        let spawn_done = handles.spawn.as_ref().is_none_or(|h| h.is_finished());
        let despawn_done = handles.despawn.as_ref().is_none_or(|h| h.is_finished());

        if !spawn_done || !despawn_done {
            if has_boundary_request && workflow_state.has_new_boundary_request(&boundary_spawn_targets, &boundary_despawn_targets) {
                let changed = chunk_load_gate.lock_by_in_flight_boundary();
                if changed {
                    warn!(
                        "ChunkLoadGate locked due to boundary request while previous chunk batch is still running (incoming spawn={}, despawn={}, active spawn={}, despawn={})",
                        incoming_spawn_targets.len(),
                        incoming_despawn_targets.len(),
                        workflow_state.in_flight_spawn_targets.len(),
                        workflow_state.in_flight_despawn_targets.len()
                    );
                }
            }
            //warn!(
            //    "Waiting for chunk action workflows to finish... spawn_done: {}, despawn_done: {}",
            //    spawn_done, despawn_done
            //);
            return;
        }

        // Cleanup finished handles
        if let Some(handle) = handles.spawn.take() {
            handle_composite_workflow_return_now(handle, |_ctx| {
                composite_workflow_return!();

                warn!("Finished composite workflow 'SpawnChunks'");
            });
        }
        if let Some(handle) = handles.despawn.take() {
            handle_composite_workflow_return_now(handle, |_ctx| {
                composite_workflow_return!();

                warn!("Finished composite workflow 'DespawnChunks'");
            });
        }

        workflow_state.handles = None;
        workflow_state.clear_in_flight_targets();

        if let Some(finished_batch) = chunk_batch_tracker.finish_running() {
            warn!(
                "Chunk batch finished: batch_id={}, spawn={}, despawn={}",
                finished_batch.id,
                finished_batch.spawn_count(),
                finished_batch.despawn_count()
            );
            chunk_batch_lifecycle_writer.write(ChunkBatchLifecycleMessage::Finished {
                batch: finished_batch,
            });
        }
    }

    if spawn_chunk_inputs.is_empty() && despawn_chunk_inputs.is_empty() {
        if chunk_load_gate.is_locked() && workflow_state.is_idle() {
            if chunk_load_gate.state == ChunkLoadGateState::LockedByInFlightBoundary {
                // Require a short quiet period before unlocking to avoid lock-state chatter
                // around boundary completion edges.
                const REQUIRED_QUIET_FRAMES: u8 = 2;
                chunk_load_gate.boundary_quiet_frames = chunk_load_gate.boundary_quiet_frames.saturating_add(1);
                if chunk_load_gate.boundary_quiet_frames < REQUIRED_QUIET_FRAMES {
                    return;
                }
            }
            let lock_info = chunk_load_gate.lock_info;
            if chunk_load_gate.unlock() {
                if let Some(info) = lock_info {
                    warn!(
                        "ChunkLoadGate unlocked after workload recovery (triggered by {}::{}, timeout_count={})",
                        info.module_name, info.workflow_name, info.timeout_count
                    );
                } else {
                    warn!("ChunkLoadGate unlocked after workload recovery");
                }
            }
        }
        return;
    }

    if !spawn_chunk_inputs.is_empty() || !despawn_chunk_inputs.is_empty() {
        // warn!(
        //     "Chunk Detection: To Load: {:?}, To Unload: {:?}",
        //     spawn_chunk_inputs.iter().map(|input| input.grid_coord.clone()).collect::<Vec<_>>(),
        //     despawn_chunk_inputs.iter().map(|input| input.grid_coord.clone()).collect::<Vec<_>>()
        // );
    }

    if let Some(started_batch) = chunk_batch_tracker.start_planned_or_direct(&incoming_spawn_targets, &incoming_despawn_targets) {
        warn!(
            "Chunk batch started: batch_id={}, spawn={}, despawn={}",
            started_batch.id,
            started_batch.spawn_count(),
            started_batch.despawn_count()
        );
        chunk_batch_lifecycle_writer.write(ChunkBatchLifecycleMessage::Started {
            batch: started_batch,
        });
    }

    // Step 2: Build & launch composite workflows
    let spawn_handle = if !spawn_chunk_inputs.is_empty() {
        let param_data = spawn_chunk_inputs
            .iter()
            .map(|input| {
                let coord = input.grid_coord.clone();
                let coord_native_logical = coord.clone().to_native_logical(current_view_coord.clone());
                let coord_relative_chunk_units = IVec2::new(
                    (coord_native_logical.x / 1000.0).round() as i32,
                    (coord_native_logical.y / 1000.0).round() as i32,
                );

                crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                    chunk_pos: [coord_relative_chunk_units.x, coord_relative_chunk_units.y],
                    chunk_size: 1000,
                    chunk_scale: coord.scale as i32,
                    current_view_scale,
                    _padding0: 0,
                    _padding1: [0, 0, 0, 0],
                }
            })
            .collect::<Vec<_>>();

        Some(composite_workflow!(
            SpawnChunks,
            move in spawn_chunk_inputs: Vec<SpawnChunkInput>,
            move in param_data: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
        {
            warn!("Running composite workflow 'SpawnChunks'");

            let shader_name = CONFIG().get::<&'static str>("chunk/texture_generator_shader");

            let generate_output = run_workflow_io_with_timeout_control::<crate::gpu::workflows::gpu::generate_chunk_textures::TypeIO, _>(
                Duration::from_secs_f64(1.0),
                WorkflowTimeoutMode::VirtualTime,
                crate::gpu::workflows::gpu::generate_chunk_textures::stages::prepare_render_executor::core_types::Input {
                    shader_name,
                    param_data,
                },
                |ctx| chunk_workflow_timeout_decision(ctx.module_name, ctx.workflow_name, ctx.timeout_count),
            )
            .await
            .unwrap_or_else(|control_error| {
                panic!(
                    "Chunk workflow control error while running Gpu::GenerateChunkTextures: {:?}",
                    control_error
                )
            });

            let spawn_inputs_with_textures = spawn_chunk_inputs
                .into_iter()
                .zip(generate_output.render_executor.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();

            warn!("Finished GenerateChunkTextures workflow execution");

            let _ = run_workflow_ioe_with_timeout_control::<crate::chunk::workflows::chunk::spawn_chunks::TypeIOE, _>(
                Duration::from_secs_f64(1.0),
                WorkflowTimeoutMode::VirtualTime,
                crate::chunk::workflows::chunk::spawn_chunks::stages::validate_and_spawn_and_wait::core_types::Input {
                    inner: crate::chunk::workflows::external::spawn_chunks::Input {
                        inputs: spawn_inputs_with_textures,
                    },
                },
                |ctx| chunk_workflow_timeout_decision(ctx.module_name, ctx.workflow_name, ctx.timeout_count),
            )
            .await;
        }))
    } else {
        None
    };

    let despawn_handle = if !despawn_chunk_inputs.is_empty() {
        Some(composite_workflow!(
            DespawnChunks,
            move in despawn_chunk_inputs: Vec<DespawnChunkInput>,
        {
            warn!("Running composite workflow 'DespawnChunks'");

            let _ = run_workflow_ioe_with_timeout_control::<crate::chunk::workflows::chunk::despawn_chunks::TypeIOE, _>(
                Duration::from_secs_f64(1.0),
                WorkflowTimeoutMode::VirtualTime,
                crate::chunk::workflows::chunk::despawn_chunks::stages::find_and_despawn_and_wait::core_types::Input {
                    inner: crate::chunk::workflows::external::despawn_chunks::Input {
                        inputs: despawn_chunk_inputs,
                    },
                },
                |ctx| chunk_workflow_timeout_decision(ctx.module_name, ctx.workflow_name, ctx.timeout_count),
            )
            .await;
        }))
    } else {
        None
    };

    workflow_state.set_in_flight_targets(boundary_spawn_targets, boundary_despawn_targets);
    workflow_state.handles = Some(crate::chunk::types::ChunkActionWorkflowHandles {
        spawn: spawn_handle,
        despawn: despawn_handle,
    });
}
