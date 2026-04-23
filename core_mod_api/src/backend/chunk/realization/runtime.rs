use super::field::{
    ChunkRealizationRecord, SerializableGridCoord, canonical_grid_coord, chunk_file_path, density_field_signature, generate_chunk_realization_record,
    load_chunk_realization_record, sample_root_native_position, save_chunk_realization_record,
};
use super::meshing::build_chunk_mesh;
use super::projection::{
    ChunkInstanceCullCandidate, chunk_center_in_active_native_space, chunk_instance_scale_in_active_native_space, select_visible_chunk_instance_entities,
};
use crate::bevy::prelude::*;
use crate::bevy_rapier3d::prelude::Collider;
use crate::config::statics::CONFIG;
use crate::player::components::Player;
use crate::render::components::{MainCamera, WorldPresentationRoot};
use crate::usf::chunk::realization::output_channels::{
    ChunkRealizationAudioEmitter, ChunkRealizationInteractionTrigger, ChunkRealizationParticleEmitter, ChunkRealizationSimulationService, OutputChannelPayload,
    OutputChannelRegistry,
};
use crate::usf::chunk::components::{Chunk, ChunkLoader};
use crate::usf::mod_packs::UsfActiveModPack;
use crate::usf::phenomenon::{PhenomenonDefinitionRegistry, PhenomenonOutputFieldSpec};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use crate::usf::zone::ZoneDensityProfile;
use crate::usf::zone::ZoneTypeId;
use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;
use crate::workflow::functions::{WorkflowTimeoutControlDecision, handle_composite_workflow_return_now, run_workflow_ioe_with_timeout_control};
use crate::workflow::types::WorkflowTimeoutMode;
use core_engine_macros::{composite_workflow, composite_workflow_return};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tokio::task::JoinHandle;

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfChunkRealizationRuntimeSettings {
    pub enabled: bool,
    pub enable_instance_culling: bool,
    pub intent_grace_frames: u32,
    pub world_seed: u64,
    pub sample_step: u16,
    pub iso_level: u8,
    pub reconcile_batch_size: usize,
    pub reconcile_commit_budget: usize,
    pub reconcile_build_workers: usize,
    pub persistence_dir: String,
}
impl Default for UsfChunkRealizationRuntimeSettings {
    fn default() -> Self {
        Self {
            enabled: CONFIG().get::<bool>("usf/chunk/realization/enabled"),
            enable_instance_culling: CONFIG().get::<bool>("usf/chunk/realization/enable_instance_culling"),
            intent_grace_frames: CONFIG().get::<u32>("usf/chunk/realization/intent_grace_frames"),
            world_seed: CONFIG().get::<u64>("usf/chunk/realization/world_seed"),
            sample_step: CONFIG().get::<u16>("usf/chunk/realization/sample_step"),
            iso_level: CONFIG().get::<u8>("usf/chunk/realization/iso_level"),
            reconcile_batch_size: CONFIG().get::<usize>("usf/chunk/realization/reconcile_batch_size"),
            reconcile_commit_budget: CONFIG().get::<usize>("usf/chunk/realization/reconcile_commit_budget"),
            reconcile_build_workers: CONFIG().get::<usize>("usf/chunk/realization/reconcile_build_workers"),
            persistence_dir: CONFIG().get::<String>("usf/chunk/realization/persistence_dir"),
        }
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ChunkRealizationInstance {
    pub chunk_seed: u64,
    pub sample_step: u16,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct ChunkRealizationIntent {
    pub zone_type: ZoneTypeId,
    pub zone_density_profile: ZoneDensityProfile,
    pub zone_density_signature: u64,
    pub phenomenon_script_id: String,
    pub selected_model_id: String,
    pub output_field_spec: PhenomenonOutputFieldSpec,
    pub channel_payloads: HashMap<String, OutputChannelPayload>,
    pub chunk_store_key: String,
}

#[derive(Resource, Debug, Default)]
pub struct ChunkRealizationCache {
    pub records: HashMap<GridVec, ChunkRealizationRecord>,
}

#[derive(Resource)]
pub struct ChunkRealizationReconcileWorkflowState {
    pub handle: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    queued_entities: VecDeque<Entity>,
    queued_lookup: HashSet<Entity>,
    in_flight_entities: HashSet<Entity>,
}
impl Default for ChunkRealizationReconcileWorkflowState {
    fn default() -> Self {
        Self {
            handle: None,
            queued_entities: VecDeque::new(),
            queued_lookup: HashSet::new(),
            in_flight_entities: HashSet::new(),
        }
    }
}
impl ChunkRealizationReconcileWorkflowState {
    fn queue(&mut self, entity: Entity) -> bool {
        if self.in_flight_entities.contains(&entity) || self.queued_lookup.contains(&entity) {
            return false;
        }
        self.queued_entities.push_back(entity);
        self.queued_lookup.insert(entity);
        true
    }

    fn clear(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
        self.queued_entities.clear();
        self.queued_lookup.clear();
        self.in_flight_entities.clear();
    }

    fn clear_in_flight(&mut self) {
        self.in_flight_entities.clear();
    }

    fn drain_batch(&mut self, batch_size: usize) -> Vec<Entity> {
        let mut batch = Vec::with_capacity(batch_size);
        while batch.len() < batch_size {
            let Some(entity) = self.queued_entities.pop_front() else {
                break;
            };
            self.queued_lookup.remove(&entity);
            batch.push(entity);
        }
        batch
    }

    fn mark_in_flight(&mut self, entities: &[Entity]) {
        self.in_flight_entities.extend(entities.iter().copied());
    }

    fn prune_stale(&mut self, chunk_instance_query: &Query<Option<&ChunkRealizationInstance>, With<Chunk>>) {
        self.queued_entities.retain(|entity| matches!(chunk_instance_query.get(*entity), Ok(None)));
        self.queued_lookup = self.queued_entities.iter().copied().collect::<HashSet<_>>();
        self.in_flight_entities.retain(|entity| matches!(chunk_instance_query.get(*entity), Ok(None)));
    }

    fn queued_len(&self) -> usize {
        self.queued_entities.len()
    }

    fn in_flight_len(&self) -> usize {
        self.in_flight_entities.len()
    }
}

pub(crate) fn validate_chunk_realization_channel_registrations_system(
    settings: Option<Res<UsfChunkRealizationRuntimeSettings>>,
    channel_registry: Option<Res<OutputChannelRegistry>>,
    phenomenon_definitions: Option<Res<PhenomenonDefinitionRegistry>>,
) {
    let Some(settings) = settings else {
        return;
    };
    if !settings.enabled {
        return;
    }
    let Some(channel_registry) = channel_registry else {
        panic!("USF runtime channel-registration validation failed: missing OutputChannelRegistry resource.");
    };
    let ensure_channel = |channel: &str, reason: &str| {
        if !channel_registry.has_registration(channel) {
            panic!(
                "USF runtime channel-registration validation failed: {} requires output channel '{}', but no execution registration is present.",
                reason, channel
            );
        }
    };

    let Some(phenomenon_definitions) = phenomenon_definitions else {
        panic!("USF runtime channel-registration validation failed: missing PhenomenonDefinitionRegistry resource.");
    };
    if phenomenon_definitions.any_model_declares_output_density_field() {
        ensure_channel("mesh", "at least one phenomenon model defines mesh density output");
    }
    if phenomenon_definitions.any_model_declares_output_material_profile() {
        ensure_channel("material", "at least one phenomenon model defines material profile output");
    }
    if phenomenon_definitions.any_model_declares_output_collider() {
        ensure_channel("collider", "at least one phenomenon model enables collider output");
    }
    if phenomenon_definitions.any_model_declares_output_audio_emitter() {
        ensure_channel("audio", "at least one phenomenon model defines audio output");
    }
    if phenomenon_definitions.any_model_declares_output_particle_emitter() {
        ensure_channel("particles", "at least one phenomenon model defines particle output");
    }
    if phenomenon_definitions.any_model_declares_output_interaction_trigger() {
        ensure_channel("trigger", "at least one phenomenon model defines interaction trigger");
    }
    if phenomenon_definitions.any_model_declares_simulation_service() {
        ensure_channel("simulation_service", "at least one phenomenon model defines simulation service");
    }
}

#[derive(Default)]
pub(crate) struct ChunkRealizationReconcileQueueProbe {
    window_start: Option<Instant>,
    calls: u32,
    pending_seen: u64,
    enqueued: u64,
    queue_len_total: u64,
    in_flight_len_total: u64,
}
impl ChunkRealizationReconcileQueueProbe {
    fn observe(&mut self, pending_seen: usize, enqueued: usize, queue_len: usize, in_flight_len: usize) {
        if !usf_hotpath_probe_enabled() {
            return;
        }

        let now = Instant::now();
        let window_start = self.window_start.get_or_insert(now);
        self.calls += 1;
        self.pending_seen += pending_seen as u64;
        self.enqueued += enqueued as u64;
        self.queue_len_total += queue_len as u64;
        self.in_flight_len_total += in_flight_len as u64;
        if now.duration_since(*window_start) < Duration::from_secs(1) {
            return;
        }

        let calls = self.calls.max(1) as f64;
        warn!(
            "USF hotpath probe [chunk_realization_reconcile_queue]: calls={}, pending_seen={}, enqueued={}, avg_queue_len={:.1}, avg_in_flight_len={:.1}",
            self.calls,
            self.pending_seen,
            self.enqueued,
            (self.queue_len_total as f64) / calls,
            (self.in_flight_len_total as f64) / calls
        );

        self.window_start = Some(now);
        self.calls = 0;
        self.pending_seen = 0;
        self.enqueued = 0;
        self.queue_len_total = 0;
        self.in_flight_len_total = 0;
    }
}

#[derive(Default)]
pub(crate) struct ChunkRealizationReconcileWorkflowProbe {
    window_start: Option<Instant>,
    calls: u32,
    waiting_for_handle_calls: u32,
    completed_batches: u32,
    drained_entities: u64,
    spawned_task_batches: u32,
    spawned_tasks: u64,
    empty_drains: u32,
    queue_len_total: u64,
    in_flight_len_total: u64,
}
impl ChunkRealizationReconcileWorkflowProbe {
    fn observe(
        &mut self,
        waiting_for_handle: bool,
        completed_batch: bool,
        drained_entities: usize,
        spawned_tasks: usize,
        queue_len: usize,
        in_flight_len: usize,
    ) {
        if !usf_hotpath_probe_enabled() {
            return;
        }

        let now = Instant::now();
        let window_start = self.window_start.get_or_insert(now);
        self.calls += 1;
        if waiting_for_handle {
            self.waiting_for_handle_calls += 1;
        }
        if completed_batch {
            self.completed_batches += 1;
        }
        self.drained_entities += drained_entities as u64;
        if drained_entities == 0 {
            self.empty_drains += 1;
        }
        if spawned_tasks > 0 {
            self.spawned_task_batches += 1;
            self.spawned_tasks += spawned_tasks as u64;
        }
        self.queue_len_total += queue_len as u64;
        self.in_flight_len_total += in_flight_len as u64;
        if now.duration_since(*window_start) < Duration::from_secs(1) {
            return;
        }

        let calls = self.calls.max(1) as f64;
        warn!(
            "USF hotpath probe [chunk_realization_reconcile_workflow]: calls={}, waiting_calls={}, completed_batches={}, drained_entities={}, spawned_task_batches={}, spawned_tasks={}, empty_drains={}, avg_queue_len={:.1}, avg_in_flight_len={:.1}",
            self.calls,
            self.waiting_for_handle_calls,
            self.completed_batches,
            self.drained_entities,
            self.spawned_task_batches,
            self.spawned_tasks,
            self.empty_drains,
            (self.queue_len_total as f64) / calls,
            (self.in_flight_len_total as f64) / calls
        );

        self.window_start = Some(now);
        self.calls = 0;
        self.waiting_for_handle_calls = 0;
        self.completed_batches = 0;
        self.drained_entities = 0;
        self.spawned_task_batches = 0;
        self.spawned_tasks = 0;
        self.empty_drains = 0;
        self.queue_len_total = 0;
        self.in_flight_len_total = 0;
    }
}

fn usf_hotpath_probe_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("LOOCAST_USF_HOTPATH_PROBE")
            .map(|raw| matches!(raw.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
            .unwrap_or(false)
    })
}

#[derive(Debug, Clone)]
pub struct ChunkRealizationIntentSnapshot {
    pub chunk_entity: Entity,
    pub chunk_coord: GridVec,
    pub chunk_scale: Scale,
    pub canonical_coord: GridVec,
    pub zone_type: ZoneTypeId,
    pub zone_density_profile: ZoneDensityProfile,
    pub zone_density_signature: u64,
    pub phenomenon_script_id: String,
    pub selected_model_id: String,
    pub output_field_spec: PhenomenonOutputFieldSpec,
    pub channel_payloads: HashMap<String, OutputChannelPayload>,
    pub chunk_store_key: String,
}

#[derive(Debug)]
pub struct ChunkRealizationResolvedArtifact {
    pub chunk_entity: Entity,
    pub chunk_coord: GridVec,
    pub canonical_coord: GridVec,
    pub record: ChunkRealizationRecord,
    pub channel_payloads: HashMap<String, OutputChannelPayload>,
    pub mesh: Option<Mesh>,
}

pub(crate) fn run_if_chunk_realization_runtime_enabled(
    settings: Option<Res<UsfChunkRealizationRuntimeSettings>>,
    active_modpack: Option<Res<UsfActiveModPack>>,
) -> bool {
    let Some(settings) = settings else {
        return false;
    };
    let Some(_active_modpack) = active_modpack else {
        return false;
    };
    settings.enabled
}

pub(crate) fn queue_chunk_realization_reconcile_requests_system(
    settings: Res<UsfChunkRealizationRuntimeSettings>,
    pending_chunks: Query<Entity, (With<Chunk>, With<ChunkRealizationIntent>, Without<ChunkRealizationInstance>)>,
    mut reconcile_state: ResMut<ChunkRealizationReconcileWorkflowState>,
    mut probe: Local<ChunkRealizationReconcileQueueProbe>,
) {
    if !settings.enabled {
        return;
    }

    let mut pending_seen = 0usize;
    let mut enqueued = 0usize;
    for entity in pending_chunks.iter() {
        pending_seen += 1;
        if reconcile_state.queue(entity) {
            enqueued += 1;
        }
    }
    probe.observe(pending_seen, enqueued, reconcile_state.queued_len(), reconcile_state.in_flight_len());
}

pub(crate) fn run_chunk_realization_reconcile_workflow_system(
    settings: Res<UsfChunkRealizationRuntimeSettings>,
    chunk_instance_query: Query<Option<&ChunkRealizationInstance>, With<Chunk>>,
    chunk_query: Query<(&Chunk, Option<&ChunkRealizationInstance>, Option<&ChunkRealizationIntent>)>,
    mut reconcile_state: ResMut<ChunkRealizationReconcileWorkflowState>,
    mut probe: Local<ChunkRealizationReconcileWorkflowProbe>,
) {
    if !settings.enabled {
        reconcile_state.clear();
        return;
    }

    if reconcile_state.handle.as_ref().is_some_and(|handle| !handle.is_finished()) {
        probe.observe(true, false, 0, 0, reconcile_state.queued_len(), reconcile_state.in_flight_len());
        return;
    }

    let mut completed_batch = false;
    if let Some(handle) = reconcile_state.handle.take() {
        handle_composite_workflow_return_now(handle, |_ctx| {
            composite_workflow_return!();
        });
        reconcile_state.clear_in_flight();
        completed_batch = true;
    }

    reconcile_state.prune_stale(&chunk_instance_query);
    let batch_size = settings.reconcile_batch_size.max(1);
    let batch_entities = reconcile_state.drain_batch(batch_size);
    if batch_entities.is_empty() {
        probe.observe(false, completed_batch, 0, 0, reconcile_state.queued_len(), reconcile_state.in_flight_len());
        return;
    }
    let drained_entities = batch_entities.len();

    let mut in_flight_entities = Vec::<Entity>::new();
    let mut tasks = Vec::<ChunkRealizationIntentSnapshot>::new();

    for entity in batch_entities {
        let Ok((chunk, maybe_instance, intent)) = chunk_query.get(entity) else {
            // Chunk disappeared between queueing and reconcile dispatch.
            continue;
        };
        if maybe_instance.is_some() {
            continue;
        }
        let Some(intent) = intent.cloned() else {
            // Intent authority can lag one or more frames; retry this chunk later.
            reconcile_state.queue(entity);
            continue;
        };
        let chunk_scale = chunk.coord.scale;
        let canonical_coord = canonical_grid_coord(&chunk.coord);

        in_flight_entities.push(entity);
        tasks.push(ChunkRealizationIntentSnapshot {
            chunk_entity: entity,
            chunk_coord: chunk.coord.clone(),
            chunk_scale,
            canonical_coord,
            zone_type: intent.zone_type,
            zone_density_profile: intent.zone_density_profile,
            zone_density_signature: intent.zone_density_signature,
            phenomenon_script_id: intent.phenomenon_script_id,
            selected_model_id: intent.selected_model_id,
            output_field_spec: intent.output_field_spec,
            channel_payloads: intent.channel_payloads,
            chunk_store_key: intent.chunk_store_key,
        });
    }

    if tasks.is_empty() {
        probe.observe(
            false,
            completed_batch,
            drained_entities,
            0,
            reconcile_state.queued_len(),
            reconcile_state.in_flight_len(),
        );
        return;
    }

    let spawned_tasks = in_flight_entities.len();
    reconcile_state.mark_in_flight(&in_flight_entities);

    let reconcile_async_input = crate::usf::chunk::realization::reconcile_workflow::AsyncInput {
        settings: settings.clone(),
        tasks,
        build_workers: settings.reconcile_build_workers.max(1),
        commit_budget: settings.reconcile_commit_budget.max(1),
    };

    let handle = composite_workflow!(
        ReconcileChunkRealizationArtifactsBatch,
        move in reconcile_async_input: crate::usf::chunk::realization::reconcile_workflow::AsyncInput,
    {
        let _ = run_workflow_ioe_with_timeout_control::<crate::usf::chunk::workflows::usf_chunk::reconcile_chunk_realization_artifacts::TypeIOE, _>(
            Duration::from_secs_f64(5.0),
            WorkflowTimeoutMode::VirtualTime,
            crate::usf::chunk::workflows::usf_chunk::reconcile_chunk_realization_artifacts::stages::resolve_intents::core_types::Input {
                inner: crate::usf::chunk::realization::reconcile_workflow::Input {
                    inner: reconcile_async_input,
                },
            },
            |ctx| chunk_realization_reconcile_timeout_decision(ctx.module_name, ctx.workflow_name, ctx.timeout_count),
        )
        .await;
    });
    reconcile_state.handle = Some(handle);
    probe.observe(
        false,
        completed_batch,
        drained_entities,
        spawned_tasks,
        reconcile_state.queued_len(),
        reconcile_state.in_flight_len(),
    );
}

fn chunk_realization_reconcile_timeout_decision(
    module_name: &'static str,
    workflow_name: &'static str,
    timeout_count: usize,
) -> WorkflowTimeoutControlDecision {
    if timeout_count <= 2 {
        warn!(
            "Chunk realization reconcile timeout request: {}::{}, timeout_count={}, decision=Retry",
            module_name, workflow_name, timeout_count
        );
        return WorkflowTimeoutControlDecision::Retry;
    }

    warn!(
        "Chunk realization reconcile timeout escalation: {}::{}, timeout_count={}, decision=Abort",
        module_name, workflow_name, timeout_count
    );
    WorkflowTimeoutControlDecision::Abort
}

pub(crate) fn resolve_chunk_realization_artifact(
    settings: &UsfChunkRealizationRuntimeSettings,
    task: ChunkRealizationIntentSnapshot,
) -> ChunkRealizationResolvedArtifact {
    let chunk_file = chunk_file_path(
        &settings.persistence_dir,
        settings.world_seed,
        task.chunk_scale,
        &task.canonical_coord,
        task.chunk_store_key.as_str(),
    );
    let expected_coord = SerializableGridCoord::from_grid(&task.canonical_coord);
    let expected_density_field_signature = density_field_signature(task.output_field_spec);

    let mut record = load_chunk_realization_record(&chunk_file).filter(|loaded| {
        loaded.schema_version == 3
            && loaded.world_seed == settings.world_seed
            && loaded.active_scale_index == task.chunk_scale.index_from_top()
            && loaded.chunk_coord == expected_coord
            && loaded.zone_type.eq_ignore_ascii_case(&task.zone_type.0)
            && loaded.zone_density_signature == task.zone_density_signature
            && loaded.density_field_signature == expected_density_field_signature
            && loaded.phenomenon_script_id.eq_ignore_ascii_case(task.phenomenon_script_id.as_str())
            && loaded.selected_model_id.eq_ignore_ascii_case(task.selected_model_id.as_str())
            && loaded.sample_step == settings.sample_step
            && loaded.iso_level == settings.iso_level
            && loaded.cache_authority == "runtime_cache"
    });

    if record.is_none() {
        let generated = generate_chunk_realization_record(
            settings.world_seed,
            settings.sample_step,
            settings.iso_level,
            task.chunk_scale,
            &task.canonical_coord,
            &task.zone_type,
            task.zone_density_profile,
            task.zone_density_signature,
            task.phenomenon_script_id.as_str(),
            task.selected_model_id.as_str(),
            task.output_field_spec,
        );
        if let Err(error) = save_chunk_realization_record(&chunk_file, &generated) {
            warn!("USF runtime persistence write failed for {:?}: {}", chunk_file, error);
        }
        record = Some(generated);
    }

    let record = record.expect("USF runtime chunk record should exist after generate/load");
    let mesh_requested = matches!(task.channel_payloads.get("mesh"), Some(OutputChannelPayload::Mesh));
    let mesh = if mesh_requested { build_chunk_mesh(&record) } else { None };

    ChunkRealizationResolvedArtifact {
        chunk_entity: task.chunk_entity,
        chunk_coord: task.chunk_coord,
        canonical_coord: task.canonical_coord,
        record,
        channel_payloads: task.channel_payloads,
        mesh,
    }
}

pub(crate) fn sync_chunk_realization_instance_transforms_system(
    settings: Res<UsfChunkRealizationRuntimeSettings>,
    mut params: ParamSet<(
        Query<(&ChunkLoader, &Transform), With<Player>>,
        Query<&Transform, (With<MainCamera>, Without<Player>, Without<ChunkRealizationInstance>)>,
        Single<&Transform, (With<WorldPresentationRoot>, Without<Player>, Without<ChunkRealizationInstance>)>,
        Query<
            (Entity, &Chunk, &mut Transform, &mut Visibility),
            (
                With<ChunkRealizationInstance>,
                Without<Player>,
                Without<MainCamera>,
                Without<WorldPresentationRoot>,
            ),
        >,
    )>,
) {
    if !settings.enabled {
        return;
    }

    let (active_scale_index, player_root_native) = {
        let player_loader_query = params.p0();
        let Ok((chunk_loader, player_transform)) = player_loader_query.single() else {
            return;
        };
        let player_coord = canonical_grid_coord(&chunk_loader.coord);
        let player_root_native = sample_root_native_position(&player_coord, player_transform.translation);
        (chunk_loader.scale.index_from_top() as i16, player_root_native)
    };

    let camera_transform = params.p1().single().ok().copied();
    let (root_scale, root_rotation, root_translation) = {
        let root_transform = *params.p2();
        (root_transform.scale.x, root_transform.rotation, root_transform.translation)
    };
    let mut chunk_query = params.p3();

    if !settings.enable_instance_culling {
        for (_entity, chunk, mut transform, mut visibility) in chunk_query.iter_mut() {
            transform.translation = chunk_center_in_active_native_space(&chunk.coord, player_root_native, active_scale_index);
            transform.rotation = Quat::IDENTITY;
            transform.scale = Vec3::splat(chunk_instance_scale_in_active_native_space(chunk.coord.scale, active_scale_index));
            *visibility = Visibility::Visible;
        }
        return;
    }

    let Some(camera_transform) = camera_transform else {
        for (_entity, chunk, mut transform, mut visibility) in chunk_query.iter_mut() {
            transform.translation = chunk_center_in_active_native_space(&chunk.coord, player_root_native, active_scale_index);
            transform.rotation = Quat::IDENTITY;
            transform.scale = Vec3::splat(chunk_instance_scale_in_active_native_space(chunk.coord.scale, active_scale_index));
            *visibility = Visibility::Visible;
        }
        return;
    };

    let camera_pos = camera_transform.translation;
    let camera_forward = (-(camera_transform.rotation * Vec3::Z)).normalize_or_zero();
    let load_radius = CONFIG().get::<u32>("chunk_loader/load_radius") as usize;
    let side = load_radius.saturating_mul(2).saturating_add(1);
    let active_budget = side.saturating_mul(side).clamp(16, 144);
    let mut cull_candidates = Vec::new();

    for (entity, chunk, mut transform, mut visibility) in chunk_query.iter_mut() {
        transform.translation = chunk_center_in_active_native_space(&chunk.coord, player_root_native, active_scale_index);
        transform.rotation = Quat::IDENTITY;
        transform.scale = Vec3::splat(chunk_instance_scale_in_active_native_space(chunk.coord.scale, active_scale_index));
        *visibility = Visibility::Hidden;

        let relative_scale = chunk.coord.scale.index_from_top() as i16 - active_scale_index;
        if relative_scale > 0 {
            continue;
        }

        let chunk_world_pos = root_translation + (root_rotation * (transform.translation * root_scale));
        let to_chunk = chunk_world_pos - camera_pos;
        let distance_sq = to_chunk.length_squared();
        let front_dot = if distance_sq <= f32::EPSILON {
            1.0
        } else {
            to_chunk.normalize_or_zero().dot(camera_forward)
        };
        cull_candidates.push(ChunkInstanceCullCandidate {
            entity,
            coarse_depth: (-relative_scale).max(0) as u8,
            distance_sq,
            front_dot,
        });
    }

    let visible_entities = select_visible_chunk_instance_entities(&cull_candidates, active_budget);
    for (entity, _chunk, _transform, mut visibility) in chunk_query.iter_mut() {
        *visibility = if visible_entities.contains(&entity) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub(crate) fn bind_chunk_realization_instances_to_world_presentation_root_system(
    mut commands: Commands,
    root_query: Single<Entity, With<WorldPresentationRoot>>,
    chunk_query: Query<(Entity, Option<&ChildOf>), (With<ChunkRealizationInstance>, Without<Player>)>,
) {
    let root = *root_query;
    for (entity, child_of) in chunk_query.iter() {
        if child_of.is_some_and(|relation| relation.parent() == root) {
            continue;
        }
        commands.entity(entity).insert(ChildOf(root));
    }
}

pub(crate) fn clear_unbound_chunk_realization_instances_system(
    mut commands: Commands,
    instance_query: Query<(Entity, Option<&ChunkRealizationIntent>), With<ChunkRealizationInstance>>,
) {
    for (entity, intent) in instance_query.iter() {
        if intent.is_some() {
            continue;
        }
        commands.entity(entity).remove::<ChunkRealizationInstance>();
        commands.entity(entity).remove::<Mesh3d>();
        commands.entity(entity).remove::<MeshMaterial3d<StandardMaterial>>();
        commands.entity(entity).remove::<Collider>();
        commands.entity(entity).remove::<ChunkRealizationAudioEmitter>();
        commands.entity(entity).remove::<ChunkRealizationParticleEmitter>();
        commands.entity(entity).remove::<ChunkRealizationInteractionTrigger>();
        commands.entity(entity).remove::<ChunkRealizationSimulationService>();
    }
}

pub(crate) fn prune_chunk_realization_cache_system(
    settings: Res<UsfChunkRealizationRuntimeSettings>,
    loaded_chunks: Query<&Chunk>,
    mut chunk_store: ResMut<ChunkRealizationCache>,
) {
    if !settings.enabled {
        chunk_store.records.clear();
        return;
    }

    let loaded = loaded_chunks.iter().map(|chunk| canonical_grid_coord(&chunk.coord)).collect::<HashSet<_>>();
    chunk_store.records.retain(|coord, _| loaded.contains(coord));
}
