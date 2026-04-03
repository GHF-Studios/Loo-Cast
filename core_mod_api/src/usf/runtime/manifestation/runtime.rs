use super::field::{
    CachedChunkManifestationRecord, SerializableGridCoord, canonical_grid_coord, chunk_file_path, density_field_signature,
    generate_chunk_manifestation_cache_record, load_chunk_manifestation_cache_record, sample_root_native_position, save_chunk_manifestation_cache_record,
};
use super::meshing::build_chunk_mesh;
use super::projection::{
    ChunkInstanceCullCandidate, chunk_center_in_active_native_space, chunk_instance_scale_in_active_native_space, select_visible_chunk_instance_entities,
};
use crate::usf::runtime::capability::manifestation::{
    ChunkManifestationInstanceAudioEmitter, ChunkManifestationInstanceInteractionTrigger, ChunkManifestationInstanceParticleEmitter,
};
use crate::bevy::prelude::*;
use crate::bevy_rapier3d::prelude::Collider;
use crate::chunk::components::{Chunk, ChunkLoader};
use crate::config::statics::CONFIG;
use crate::player::components::Player;
use crate::render::components::{MainCamera, WorldPresentationRoot};
use crate::usf::capability::{CapabilityId, UsfCapabilityGraph};
use crate::usf::content::UsfActiveModpack;
use crate::usf::definition::ZoneTypeId;
use crate::usf::phenomenon::{
    InteractionTriggerDefinition, ManifestationAudioEmitterDefinition, ManifestationMaterialProfileDefinition, ManifestationParticleEmitterDefinition,
    PhenomenonDefinitionRegistry, PhenomenonManifestationFieldContract,
};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use crate::usf::zone::ZoneDensityProfile;
use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;
use crate::workflow::functions::{WorkflowTimeoutControlDecision, handle_composite_workflow_return_now, run_workflow_ioe_with_timeout_control};
use crate::workflow::types::WorkflowTimeoutMode;
use core_mod_macros::{composite_workflow, composite_workflow_return};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tokio::task::JoinHandle;

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfChunkManifestationRuntimeSettings {
    pub enabled: bool,
    pub attach_meshes: bool,
    pub enable_instance_culling: bool,
    pub binding_grace_frames: u32,
    pub world_seed: u64,
    pub sample_step: u16,
    pub iso_level: u8,
    pub hydration_batch_size: usize,
    pub hydration_commit_budget: usize,
    pub hydration_build_workers: usize,
    pub persistence_dir: String,
}
impl Default for UsfChunkManifestationRuntimeSettings {
    fn default() -> Self {
        Self {
            enabled: CONFIG().get::<bool>("usf/runtime/chunk_manifestation/enabled"),
            attach_meshes: CONFIG().get::<bool>("usf/runtime/chunk_manifestation/attach_meshes"),
            enable_instance_culling: CONFIG().get::<bool>("usf/runtime/chunk_manifestation/enable_instance_culling"),
            binding_grace_frames: CONFIG().get::<u32>("usf/runtime/chunk_manifestation/binding_grace_frames"),
            world_seed: CONFIG().get::<u64>("usf/runtime/chunk_manifestation/world_seed"),
            sample_step: CONFIG().get::<u16>("usf/runtime/chunk_manifestation/sample_step"),
            iso_level: CONFIG().get::<u8>("usf/runtime/chunk_manifestation/iso_level"),
            hydration_batch_size: CONFIG().get::<usize>("usf/runtime/chunk_manifestation/hydration_batch_size"),
            hydration_commit_budget: CONFIG().get::<usize>("usf/runtime/chunk_manifestation/hydration_commit_budget"),
            hydration_build_workers: CONFIG().get::<usize>("usf/runtime/chunk_manifestation/hydration_build_workers"),
            persistence_dir: CONFIG().get::<String>("usf/runtime/chunk_manifestation/persistence_dir"),
        }
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct UsfChunkManifestationInstance {
    pub chunk_seed: u64,
    pub sample_step: u16,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct ChunkManifestationBinding {
    pub zone_type: ZoneTypeId,
    pub zone_density_profile: ZoneDensityProfile,
    pub zone_density_signature: u64,
    pub phenomenon_script_id: String,
    pub manifestation_field_contract: PhenomenonManifestationFieldContract,
    pub manifestation_material_profile: Option<ManifestationMaterialProfileDefinition>,
    pub manifestation_collider_enabled: bool,
    pub manifestation_audio_emitter: Option<ManifestationAudioEmitterDefinition>,
    pub manifestation_particle_emitter: Option<ManifestationParticleEmitterDefinition>,
    pub interaction_trigger: Option<InteractionTriggerDefinition>,
    pub chunk_store_key: String,
}

#[derive(Resource, Debug, Default)]
pub struct UsfChunkManifestationStore {
    pub records: HashMap<GridVec, CachedChunkManifestationRecord>,
}

#[derive(Resource)]
pub struct UsfChunkManifestationHydrationWorkflowState {
    pub handle: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    queued_entities: VecDeque<Entity>,
    queued_lookup: HashSet<Entity>,
    in_flight_entities: HashSet<Entity>,
}
impl Default for UsfChunkManifestationHydrationWorkflowState {
    fn default() -> Self {
        Self {
            handle: None,
            queued_entities: VecDeque::new(),
            queued_lookup: HashSet::new(),
            in_flight_entities: HashSet::new(),
        }
    }
}
impl UsfChunkManifestationHydrationWorkflowState {
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

    fn prune_stale(&mut self, chunk_instance_query: &Query<Option<&UsfChunkManifestationInstance>, With<Chunk>>) {
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

pub(crate) fn validate_chunk_manifestation_capability_contracts_system(
    settings: Option<Res<UsfChunkManifestationRuntimeSettings>>,
    capability_graph: Option<Res<UsfCapabilityGraph>>,
    phenomenon_definitions: Option<Res<PhenomenonDefinitionRegistry>>,
) {
    let Some(settings) = settings else {
        return;
    };
    if !settings.enabled {
        return;
    }
    let Some(capability_graph) = capability_graph else {
        panic!("USF runtime capability validation failed: missing UsfCapabilityGraph resource.");
    };

    if settings.attach_meshes {
        let capability = CapabilityId::new("presentation.chunk_manifestation.instance_render");
        if !capability_graph.presentation_capabilities.contains(&capability) {
            panic!(
                "USF runtime capability validation failed: attach_meshes=true requires presentation capability '{}'.",
                capability.0
            );
        }
    }
    let Some(phenomenon_definitions) = phenomenon_definitions else {
        panic!("USF runtime capability validation failed: missing PhenomenonDefinitionRegistry resource.");
    };
    if phenomenon_definitions.any_model_uses_manifestation_collider() {
        let capability = CapabilityId::new("simulation.chunk_manifestation.instance_collider");
        if !capability_graph.simulation_capabilities.contains(&capability) {
            panic!(
                "USF runtime capability validation failed: at least one phenomenon model enables manifestation collider and requires simulation capability '{}'.",
                capability.0
            );
        }
    }
    if phenomenon_definitions.any_model_uses_manifestation_audio_emitter() {
        let capability = CapabilityId::new("presentation.chunk_manifestation.instance_audio");
        if !capability_graph.presentation_capabilities.contains(&capability) {
            panic!(
                "USF runtime capability validation failed: at least one phenomenon model defines manifestation audio and requires presentation capability '{}'.",
                capability.0
            );
        }
    }
    if phenomenon_definitions.any_model_uses_manifestation_particle_emitter() {
        let capability = CapabilityId::new("presentation.chunk_manifestation.instance_particles");
        if !capability_graph.presentation_capabilities.contains(&capability) {
            panic!(
                "USF runtime capability validation failed: at least one phenomenon model defines manifestation particles and requires presentation capability '{}'.",
                capability.0
            );
        }
    }
    if phenomenon_definitions.any_model_uses_interaction_trigger() {
        let capability = CapabilityId::new("interaction.chunk_manifestation.instance_trigger");
        if !capability_graph.interaction_capabilities.contains(&capability) {
            panic!(
                "USF runtime capability validation failed: at least one phenomenon model defines interaction trigger and requires interaction capability '{}'.",
                capability.0
            );
        }
    }
}

#[derive(Default)]
pub(crate) struct ManifestationHydrationQueueProbe {
    window_start: Option<Instant>,
    calls: u32,
    pending_seen: u64,
    enqueued: u64,
    queue_len_total: u64,
    in_flight_len_total: u64,
}
impl ManifestationHydrationQueueProbe {
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
            "USF hotpath probe [manifestation_hydration_queue]: calls={}, pending_seen={}, enqueued={}, avg_queue_len={:.1}, avg_in_flight_len={:.1}",
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
pub(crate) struct ManifestationHydrationWorkflowProbe {
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
impl ManifestationHydrationWorkflowProbe {
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
            "USF hotpath probe [manifestation_hydration_workflow]: calls={}, waiting_calls={}, completed_batches={}, drained_entities={}, spawned_task_batches={}, spawned_tasks={}, empty_drains={}, avg_queue_len={:.1}, avg_in_flight_len={:.1}",
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
pub struct ChunkManifestationHydrationTask {
    pub chunk_entity: Entity,
    pub chunk_coord: GridVec,
    pub chunk_scale: Scale,
    pub canonical_coord: GridVec,
    pub zone_type: ZoneTypeId,
    pub zone_density_profile: ZoneDensityProfile,
    pub zone_density_signature: u64,
    pub phenomenon_script_id: String,
    pub manifestation_field_contract: PhenomenonManifestationFieldContract,
    pub manifestation_material_profile: Option<ManifestationMaterialProfileDefinition>,
    pub manifestation_collider_enabled: bool,
    pub manifestation_audio_emitter: Option<ManifestationAudioEmitterDefinition>,
    pub manifestation_particle_emitter: Option<ManifestationParticleEmitterDefinition>,
    pub interaction_trigger: Option<InteractionTriggerDefinition>,
    pub chunk_store_key: String,
}

#[derive(Debug)]
pub struct ChunkManifestationHydrationArtifact {
    pub chunk_entity: Entity,
    pub chunk_coord: GridVec,
    pub canonical_coord: GridVec,
    pub record: CachedChunkManifestationRecord,
    pub manifestation_material_profile: Option<ManifestationMaterialProfileDefinition>,
    pub manifestation_collider_enabled: bool,
    pub manifestation_audio_emitter: Option<ManifestationAudioEmitterDefinition>,
    pub manifestation_particle_emitter: Option<ManifestationParticleEmitterDefinition>,
    pub interaction_trigger: Option<InteractionTriggerDefinition>,
    pub mesh: Option<Mesh>,
}

pub(crate) fn run_if_chunk_manifestation_runtime_enabled(
    settings: Option<Res<UsfChunkManifestationRuntimeSettings>>,
    active_modpack: Option<Res<UsfActiveModpack>>,
) -> bool {
    let Some(settings) = settings else {
        return false;
    };
    let Some(_active_modpack) = active_modpack else {
        return false;
    };
    settings.enabled
}

pub(crate) fn queue_chunk_manifestation_hydration_requests_system(
    settings: Res<UsfChunkManifestationRuntimeSettings>,
    pending_chunks: Query<Entity, (With<Chunk>, With<ChunkManifestationBinding>, Without<UsfChunkManifestationInstance>)>,
    mut hydration_state: ResMut<UsfChunkManifestationHydrationWorkflowState>,
    mut probe: Local<ManifestationHydrationQueueProbe>,
) {
    if !settings.enabled {
        return;
    }

    let mut pending_seen = 0usize;
    let mut enqueued = 0usize;
    for entity in pending_chunks.iter() {
        pending_seen += 1;
        if hydration_state.queue(entity) {
            enqueued += 1;
        }
    }
    probe.observe(pending_seen, enqueued, hydration_state.queued_len(), hydration_state.in_flight_len());
}

pub(crate) fn run_chunk_manifestation_hydration_workflow_system(
    settings: Res<UsfChunkManifestationRuntimeSettings>,
    chunk_instance_query: Query<Option<&UsfChunkManifestationInstance>, With<Chunk>>,
    chunk_query: Query<(&Chunk, Option<&UsfChunkManifestationInstance>, Option<&ChunkManifestationBinding>)>,
    mut hydration_state: ResMut<UsfChunkManifestationHydrationWorkflowState>,
    mut probe: Local<ManifestationHydrationWorkflowProbe>,
) {
    if !settings.enabled {
        hydration_state.clear();
        return;
    }

    if hydration_state.handle.as_ref().is_some_and(|handle| !handle.is_finished()) {
        probe.observe(true, false, 0, 0, hydration_state.queued_len(), hydration_state.in_flight_len());
        return;
    }

    let mut completed_batch = false;
    if let Some(handle) = hydration_state.handle.take() {
        handle_composite_workflow_return_now(handle, |_ctx| {
            composite_workflow_return!();
        });
        hydration_state.clear_in_flight();
        completed_batch = true;
    }

    hydration_state.prune_stale(&chunk_instance_query);
    let batch_size = settings.hydration_batch_size.max(1);
    let batch_entities = hydration_state.drain_batch(batch_size);
    if batch_entities.is_empty() {
        probe.observe(false, completed_batch, 0, 0, hydration_state.queued_len(), hydration_state.in_flight_len());
        return;
    }
    let drained_entities = batch_entities.len();

    let mut in_flight_entities = Vec::<Entity>::new();
    let mut tasks = Vec::<ChunkManifestationHydrationTask>::new();

    for entity in batch_entities {
        let Ok((chunk, maybe_instance, binding)) = chunk_query.get(entity) else {
            // Chunk disappeared between queueing and hydration dispatch.
            continue;
        };
        if maybe_instance.is_some() {
            continue;
        }
        let Some(binding) = binding.cloned() else {
            // Binding authority can lag one or more frames; retry this chunk later.
            hydration_state.queue(entity);
            continue;
        };
        let chunk_scale = chunk.coord.scale;
        let canonical_coord = canonical_grid_coord(&chunk.coord);

        in_flight_entities.push(entity);
        tasks.push(ChunkManifestationHydrationTask {
            chunk_entity: entity,
            chunk_coord: chunk.coord.clone(),
            chunk_scale,
            canonical_coord,
            zone_type: binding.zone_type,
            zone_density_profile: binding.zone_density_profile,
            zone_density_signature: binding.zone_density_signature,
            phenomenon_script_id: binding.phenomenon_script_id,
            manifestation_field_contract: binding.manifestation_field_contract,
            manifestation_material_profile: binding.manifestation_material_profile,
            manifestation_collider_enabled: binding.manifestation_collider_enabled,
            manifestation_audio_emitter: binding.manifestation_audio_emitter,
            manifestation_particle_emitter: binding.manifestation_particle_emitter,
            interaction_trigger: binding.interaction_trigger,
            chunk_store_key: binding.chunk_store_key,
        });
    }

    if tasks.is_empty() {
        probe.observe(
            false,
            completed_batch,
            drained_entities,
            0,
            hydration_state.queued_len(),
            hydration_state.in_flight_len(),
        );
        return;
    }

    let spawned_tasks = in_flight_entities.len();
    hydration_state.mark_in_flight(&in_flight_entities);

    let hydrate_async_input = crate::usf::runtime::manifestation::hydration_workflow::AsyncInput {
        settings: settings.clone(),
        tasks,
        build_workers: settings.hydration_build_workers.max(1),
        commit_budget: settings.hydration_commit_budget.max(1),
    };

    let handle = composite_workflow!(
        HydrateChunkManifestationInstancesBatch,
        move in hydrate_async_input: crate::usf::runtime::manifestation::hydration_workflow::AsyncInput,
    {
        let _ = run_workflow_ioe_with_timeout_control::<crate::chunk::workflows::chunk::hydrate_chunk_manifestation_instances::TypeIOE, _>(
            Duration::from_secs_f64(5.0),
            WorkflowTimeoutMode::VirtualTime,
            crate::chunk::workflows::chunk::hydrate_chunk_manifestation_instances::stages::build_artifacts::core_types::Input {
                inner: crate::usf::runtime::manifestation::hydration_workflow::Input {
                    inner: hydrate_async_input,
                },
            },
            |ctx| chunk_manifestation_hydration_timeout_decision(ctx.module_name, ctx.workflow_name, ctx.timeout_count),
        )
        .await;
    });
    hydration_state.handle = Some(handle);
    probe.observe(
        false,
        completed_batch,
        drained_entities,
        spawned_tasks,
        hydration_state.queued_len(),
        hydration_state.in_flight_len(),
    );
}

fn chunk_manifestation_hydration_timeout_decision(
    module_name: &'static str,
    workflow_name: &'static str,
    timeout_count: usize,
) -> WorkflowTimeoutControlDecision {
    if timeout_count <= 2 {
        warn!(
            "Chunk manifestation hydration timeout request: {}::{}, timeout_count={}, decision=Retry",
            module_name, workflow_name, timeout_count
        );
        return WorkflowTimeoutControlDecision::Retry;
    }

    warn!(
        "Chunk manifestation hydration timeout escalation: {}::{}, timeout_count={}, decision=Abort",
        module_name, workflow_name, timeout_count
    );
    WorkflowTimeoutControlDecision::Abort
}

pub(crate) fn prepare_chunk_manifestation_hydration_artifact(
    settings: &UsfChunkManifestationRuntimeSettings,
    task: ChunkManifestationHydrationTask,
) -> ChunkManifestationHydrationArtifact {
    let chunk_file = chunk_file_path(
        &settings.persistence_dir,
        settings.world_seed,
        task.chunk_scale,
        &task.canonical_coord,
        task.chunk_store_key.as_str(),
    );
    let expected_coord = SerializableGridCoord::from_grid(&task.canonical_coord);
    let expected_density_field_signature = density_field_signature(task.manifestation_field_contract);

    let mut record = load_chunk_manifestation_cache_record(&chunk_file).filter(|loaded| {
        loaded.world_seed == settings.world_seed
            && loaded.active_scale_index == task.chunk_scale.index_from_top()
            && loaded.chunk_coord == expected_coord
            && loaded.zone_type.eq_ignore_ascii_case(&task.zone_type.0)
            && loaded.zone_density_signature == task.zone_density_signature
            && loaded.density_field_signature == expected_density_field_signature
            && loaded.phenomenon_script_id.eq_ignore_ascii_case(task.phenomenon_script_id.as_str())
            && loaded.sample_step == settings.sample_step
            && loaded.iso_level == settings.iso_level
            && loaded.cache_authority == "derived_cache"
    });

    if record.is_none() {
        let generated = generate_chunk_manifestation_cache_record(
            settings.world_seed,
            settings.sample_step,
            settings.iso_level,
            task.chunk_scale,
            &task.canonical_coord,
            &task.zone_type,
            task.zone_density_profile,
            task.zone_density_signature,
            task.phenomenon_script_id.as_str(),
            task.manifestation_field_contract,
        );
        if let Err(error) = save_chunk_manifestation_cache_record(&chunk_file, &generated) {
            warn!("USF runtime persistence write failed for {:?}: {}", chunk_file, error);
        }
        record = Some(generated);
    }

    let record = record.expect("USF runtime chunk record should exist after generate/load");
    let mesh = if settings.attach_meshes { build_chunk_mesh(&record) } else { None };

    ChunkManifestationHydrationArtifact {
        chunk_entity: task.chunk_entity,
        chunk_coord: task.chunk_coord,
        canonical_coord: task.canonical_coord,
        record,
        manifestation_material_profile: task.manifestation_material_profile,
        manifestation_collider_enabled: task.manifestation_collider_enabled,
        manifestation_audio_emitter: task.manifestation_audio_emitter,
        manifestation_particle_emitter: task.manifestation_particle_emitter,
        interaction_trigger: task.interaction_trigger,
        mesh,
    }
}

pub(crate) fn sync_chunk_manifestation_instance_transforms_system(
    settings: Res<UsfChunkManifestationRuntimeSettings>,
    mut params: ParamSet<(
        Query<(&ChunkLoader, &Transform), With<Player>>,
        Query<&Transform, (With<MainCamera>, Without<Player>, Without<UsfChunkManifestationInstance>)>,
        Single<&Transform, (With<WorldPresentationRoot>, Without<Player>, Without<UsfChunkManifestationInstance>)>,
        Query<
            (Entity, &Chunk, &mut Transform, &mut Visibility),
            (
                With<UsfChunkManifestationInstance>,
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

pub(crate) fn bind_chunk_manifestation_instances_to_world_presentation_root_system(
    mut commands: Commands,
    root_query: Single<Entity, With<WorldPresentationRoot>>,
    chunk_query: Query<(Entity, Option<&ChildOf>), (With<UsfChunkManifestationInstance>, Without<Player>)>,
) {
    let root = *root_query;
    for (entity, child_of) in chunk_query.iter() {
        if child_of.is_some_and(|relation| relation.parent() == root) {
            continue;
        }
        commands.entity(entity).insert(ChildOf(root));
    }
}

pub(crate) fn clear_unbound_chunk_manifestation_instances_system(
    mut commands: Commands,
    instance_query: Query<(Entity, Option<&ChunkManifestationBinding>), With<UsfChunkManifestationInstance>>,
) {
    for (entity, binding) in instance_query.iter() {
        if binding.is_some() {
            continue;
        }
        commands.entity(entity).remove::<UsfChunkManifestationInstance>();
        commands.entity(entity).remove::<Mesh3d>();
        commands.entity(entity).remove::<MeshMaterial3d<StandardMaterial>>();
        commands.entity(entity).remove::<Collider>();
        commands.entity(entity).remove::<ChunkManifestationInstanceAudioEmitter>();
        commands.entity(entity).remove::<ChunkManifestationInstanceParticleEmitter>();
        commands.entity(entity).remove::<ChunkManifestationInstanceInteractionTrigger>();
    }
}

pub(crate) fn prune_chunk_manifestation_store_system(
    settings: Res<UsfChunkManifestationRuntimeSettings>,
    loaded_chunks: Query<&Chunk>,
    mut chunk_store: ResMut<UsfChunkManifestationStore>,
) {
    if !settings.enabled {
        chunk_store.records.clear();
        return;
    }

    let loaded = loaded_chunks.iter().map(|chunk| canonical_grid_coord(&chunk.coord)).collect::<HashSet<_>>();
    chunk_store.records.retain(|coord, _| loaded.contains(coord));
}
