use super::surface_field::{
    PersistedChunkRecord, SerializableGridCoord, canonical_grid_coord, chunk_file_path, color_from_seed, density_field_signature, generate_chunk_record,
    load_chunk_record, sample_root_native_position, save_chunk_record,
};
use super::surface_meshing::build_chunk_mesh;
use super::surface_projection::{
    ChunkVisualCullCandidate, chunk_center_in_active_native_space, chunk_visual_scale_in_active_native_space, select_visible_chunk_visual_entities,
};
use crate::bevy::prelude::*;
use crate::bevy_rapier3d::prelude::{Collider, ComputedColliderShape};
use crate::chunk::components::{Chunk, ChunkLoader};
use crate::config::statics::CONFIG;
use crate::player::components::Player;
use crate::render::components::{MainCamera, WorldPresentationRoot};
use crate::usf::content::{UsfActiveModpack, UsfExecutionPlan};
use crate::usf::definition::ZoneTypeId;
use crate::usf::phenomenon::{MetricSurfaceDebugFieldDefinition, PhenomenonDefinitionRegistry, PhenomenonKind};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use crate::usf::world::UsfWorld;
use crate::usf::zlm::ZlmRegistry;
use crate::usf::zone::{ZoneBehaviorRegistry, ZoneDensityProfile};
use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;
use crate::workflow::functions::{WorkflowTimeoutControlDecision, handle_composite_workflow_return_now, run_workflow_ioe_with_timeout_control};
use crate::workflow::types::WorkflowTimeoutMode;
use core_mod_macros::{composite_workflow, composite_workflow_return};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Duration;
use tokio::task::JoinHandle;

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfChunkSurfaceRuntimeSettings {
    pub enabled: bool,
    pub world_seed: u64,
    pub sample_step: u16,
    pub iso_level: u8,
    pub hydration_batch_size: usize,
    pub hydration_commit_budget: usize,
    pub hydration_build_workers: usize,
    pub persistence_dir: String,
}
impl Default for UsfChunkSurfaceRuntimeSettings {
    fn default() -> Self {
        Self {
            enabled: CONFIG().get::<bool>("usf/runtime/chunk_surface/enabled"),
            world_seed: CONFIG().get::<u64>("usf/runtime/chunk_surface/world_seed"),
            sample_step: CONFIG().get::<u16>("usf/runtime/chunk_surface/sample_step"),
            iso_level: CONFIG().get::<u8>("usf/runtime/chunk_surface/iso_level"),
            hydration_batch_size: CONFIG().get::<usize>("usf/runtime/chunk_surface/hydration_batch_size"),
            hydration_commit_budget: CONFIG().get::<usize>("usf/runtime/chunk_surface/hydration_commit_budget"),
            hydration_build_workers: CONFIG().get::<usize>("usf/runtime/chunk_surface/hydration_build_workers"),
            persistence_dir: CONFIG().get::<String>("usf/runtime/chunk_surface/persistence_dir"),
        }
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct UsfChunkSurfaceVisual {
    pub chunk_seed: u64,
    pub sample_step: u16,
}

#[derive(Resource, Debug, Default)]
pub struct UsfChunkSurfaceStore {
    pub records: HashMap<GridVec, PersistedChunkRecord>,
}

#[derive(Resource)]
pub struct UsfChunkSurfaceHydrationWorkflowState {
    pub handle: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    queued_entities: VecDeque<Entity>,
    queued_lookup: HashSet<Entity>,
    in_flight_entities: HashSet<Entity>,
}
impl Default for UsfChunkSurfaceHydrationWorkflowState {
    fn default() -> Self {
        Self {
            handle: None,
            queued_entities: VecDeque::new(),
            queued_lookup: HashSet::new(),
            in_flight_entities: HashSet::new(),
        }
    }
}
impl UsfChunkSurfaceHydrationWorkflowState {
    fn queue(&mut self, entity: Entity) {
        if self.in_flight_entities.contains(&entity) || self.queued_lookup.contains(&entity) {
            return;
        }
        self.queued_entities.push_back(entity);
        self.queued_lookup.insert(entity);
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

    fn prune_stale(&mut self, chunk_visual_query: &Query<Option<&UsfChunkSurfaceVisual>, With<Chunk>>) {
        self.queued_entities.retain(|entity| matches!(chunk_visual_query.get(*entity), Ok(None)));
        self.queued_lookup = self.queued_entities.iter().copied().collect::<HashSet<_>>();
        self.in_flight_entities.retain(|entity| matches!(chunk_visual_query.get(*entity), Ok(None)));
    }
}

#[derive(Debug, Clone)]
pub struct ChunkSurfaceHydrationTask {
    pub chunk_entity: Entity,
    pub chunk_coord: GridVec,
    pub chunk_scale: Scale,
    pub canonical_coord: GridVec,
    pub zone_type: ZoneTypeId,
    pub zone_density_profile: ZoneDensityProfile,
    pub zone_density_signature: u64,
    pub phenomenon_script_id: String,
    pub metric_surface_debug_field: MetricSurfaceDebugFieldDefinition,
    pub chunk_store_key: String,
}

#[derive(Debug)]
pub struct ChunkSurfaceHydrationArtifact {
    pub chunk_entity: Entity,
    pub chunk_coord: GridVec,
    pub canonical_coord: GridVec,
    pub record: PersistedChunkRecord,
    pub mesh: Option<Mesh>,
}

pub(crate) fn run_if_chunk_surface_runtime_enabled(
    settings: Option<Res<UsfChunkSurfaceRuntimeSettings>>,
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

pub(crate) fn queue_chunk_surface_hydration_requests_system(
    settings: Res<UsfChunkSurfaceRuntimeSettings>,
    added_chunks: Query<Entity, (Added<Chunk>, Without<UsfChunkSurfaceVisual>)>,
    mut hydration_state: ResMut<UsfChunkSurfaceHydrationWorkflowState>,
) {
    if !settings.enabled {
        return;
    }

    for entity in added_chunks.iter() {
        hydration_state.queue(entity);
    }
}

pub(crate) fn run_chunk_surface_hydration_workflow_system(
    settings: Res<UsfChunkSurfaceRuntimeSettings>,
    active_modpack: Res<UsfActiveModpack>,
    phenomenon_definitions: Res<PhenomenonDefinitionRegistry>,
    execution_plan: Res<UsfExecutionPlan>,
    mut usf_world: ResMut<UsfWorld>,
    zlm_registry: Res<ZlmRegistry>,
    zone_behavior_registry: Res<ZoneBehaviorRegistry>,
    chunk_visual_query: Query<Option<&UsfChunkSurfaceVisual>, With<Chunk>>,
    chunk_query: Query<(&Chunk, Option<&UsfChunkSurfaceVisual>)>,
    mut hydration_state: ResMut<UsfChunkSurfaceHydrationWorkflowState>,
) {
    if !settings.enabled {
        hydration_state.clear();
        return;
    }

    if hydration_state.handle.as_ref().is_some_and(|handle| !handle.is_finished()) {
        return;
    }

    if let Some(handle) = hydration_state.handle.take() {
        handle_composite_workflow_return_now(handle, |_ctx| {
            composite_workflow_return!();
        });
        hydration_state.clear_in_flight();
    }

    hydration_state.prune_stale(&chunk_visual_query);
    let batch_size = settings.hydration_batch_size.max(1);
    let batch_entities = hydration_state.drain_batch(batch_size);
    if batch_entities.is_empty() {
        return;
    }

    let mut in_flight_entities = Vec::<Entity>::new();
    let mut tasks = Vec::<ChunkSurfaceHydrationTask>::new();

    for entity in batch_entities {
        let Ok((chunk, maybe_visual)) = chunk_query.get(entity) else {
            continue;
        };
        if maybe_visual.is_some() {
            continue;
        }

        let chunk_scale = chunk.coord.scale;
        let Some(route) = execution_plan.route_for_scale(chunk_scale) else {
            warn!(
                "USF mod runtime chunk-surface hydration skipped: missing execution route for scale index {}",
                chunk_scale.index_from_top()
            );
            continue;
        };
        let canonical_coord = canonical_grid_coord(&chunk.coord);
        let zone_type = {
            let Some(chunk_sample) = usf_world.sample_chunk(&canonical_coord, &active_modpack, &zlm_registry) else {
                warn!(
                    "USF mod runtime chunk-surface hydration skipped: missing world sampling contracts for chunk {:?} at scale index {}",
                    chunk.coord,
                    chunk_scale.index_from_top()
                );
                continue;
            };
            chunk_sample.zone_type
        };
        let zone_density_profile = zone_behavior_registry.density_profile_for_zone(&zone_type).unwrap_or_else(|| {
            panic!(
                "USF mod runtime chunk-surface hydration failed: missing zone density profile for zone '{}'.",
                zone_type.0
            )
        });
        let supports = zone_behavior_registry.supports_for_zone(&zone_type).unwrap_or_else(|| {
            panic!(
                "USF mod runtime chunk-surface hydration failed: missing supported phenomena for zone '{}'.",
                zone_type.0
            )
        });
        let Some(selected_support) = supports.first() else {
            continue;
        };
        let phenomenon_script_id = selected_support.phenomenon_id.clone();
        let phenomenon_kind = phenomenon_definitions
            .kind_for(phenomenon_script_id.as_str())
            .unwrap_or_else(|| panic!("USF mod runtime chunk-surface hydration failed: unknown phenomenon '{}'.", phenomenon_script_id));
        if phenomenon_kind != PhenomenonKind::MetricSurfaceDebug {
            panic!(
                "USF mod runtime chunk-surface hydration failed: phenomenon '{}' has unsupported kind '{:?}'. \
                 Expected '{:?}' for metric-surface meshing.",
                phenomenon_script_id,
                phenomenon_kind,
                PhenomenonKind::MetricSurfaceDebug
            );
        }
        let metric_surface_debug_field = phenomenon_definitions
            .metric_surface_debug_for_scale(phenomenon_script_id.as_str(), chunk_scale)
            .unwrap_or_else(|| {
                panic!(
                    "USF mod runtime chunk-surface hydration failed: phenomenon '{}' is missing metric_surface_debug field definition for scale {}.",
                    phenomenon_script_id,
                    chunk_scale.index_from_top()
                )
            });
        let zone_density_signature = zone_density_profile.signature();
        let chunk_store_key = route.chunk_store_key.as_str();

        in_flight_entities.push(entity);
        tasks.push(ChunkSurfaceHydrationTask {
            chunk_entity: entity,
            chunk_coord: chunk.coord.clone(),
            chunk_scale,
            canonical_coord,
            zone_type,
            zone_density_profile,
            zone_density_signature,
            phenomenon_script_id,
            metric_surface_debug_field,
            chunk_store_key: chunk_store_key.to_string(),
        });
    }

    if tasks.is_empty() {
        return;
    }

    hydration_state.mark_in_flight(&in_flight_entities);

    let hydrate_async_input = crate::chunk::workflows::external::hydrate_chunk_surface_visuals::AsyncInput {
        settings: settings.clone(),
        tasks,
        build_workers: settings.hydration_build_workers.max(1),
        commit_budget: settings.hydration_commit_budget.max(1),
    };

    let handle = composite_workflow!(
        HydrateChunkSurfaceVisualsBatch,
        move in hydrate_async_input: crate::chunk::workflows::external::hydrate_chunk_surface_visuals::AsyncInput,
    {
        let _ = run_workflow_ioe_with_timeout_control::<crate::chunk::workflows::chunk::hydrate_chunk_surface_visuals::TypeIOE, _>(
            Duration::from_secs_f64(5.0),
            WorkflowTimeoutMode::VirtualTime,
            crate::chunk::workflows::chunk::hydrate_chunk_surface_visuals::stages::build_artifacts::core_types::Input {
                inner: crate::chunk::workflows::external::hydrate_chunk_surface_visuals::Input {
                    inner: hydrate_async_input,
                },
            },
            |ctx| chunk_surface_hydration_timeout_decision(ctx.module_name, ctx.workflow_name, ctx.timeout_count),
        )
        .await;
    });
    hydration_state.handle = Some(handle);
}

fn chunk_surface_hydration_timeout_decision(module_name: &'static str, workflow_name: &'static str, timeout_count: usize) -> WorkflowTimeoutControlDecision {
    if timeout_count == 1 {
        warn!(
            "Chunk-surface hydration timeout request: {}::{}, timeout_count={}, decision=Retry",
            module_name, workflow_name, timeout_count
        );
        return WorkflowTimeoutControlDecision::Retry;
    }

    warn!(
        "Chunk-surface hydration timeout escalation: {}::{}, timeout_count={}, decision=Panic",
        module_name, workflow_name, timeout_count
    );
    WorkflowTimeoutControlDecision::Panic
}

pub(crate) fn prepare_chunk_surface_hydration_artifact(
    settings: &UsfChunkSurfaceRuntimeSettings,
    task: ChunkSurfaceHydrationTask,
) -> ChunkSurfaceHydrationArtifact {
    let chunk_file = chunk_file_path(
        &settings.persistence_dir,
        settings.world_seed,
        task.chunk_scale,
        &task.canonical_coord,
        task.chunk_store_key.as_str(),
    );
    let expected_coord = SerializableGridCoord::from_grid(&task.canonical_coord);
    let expected_density_field_signature = density_field_signature(task.metric_surface_debug_field);

    let mut record = load_chunk_record(&chunk_file).filter(|loaded| {
        loaded.world_seed == settings.world_seed
            && loaded.active_scale_index == task.chunk_scale.index_from_top()
            && loaded.chunk_coord == expected_coord
            && loaded.zone_type.eq_ignore_ascii_case(&task.zone_type.0)
            && loaded.zone_density_signature == task.zone_density_signature
            && loaded.density_field_signature == expected_density_field_signature
            && loaded.phenomenon_script_id.eq_ignore_ascii_case(task.phenomenon_script_id.as_str())
            && loaded.sample_step == settings.sample_step
            && loaded.iso_level == settings.iso_level
    });

    if record.is_none() {
        let generated = generate_chunk_record(
            settings.world_seed,
            settings.sample_step,
            settings.iso_level,
            task.chunk_scale,
            &task.canonical_coord,
            &task.zone_type,
            task.zone_density_profile,
            task.zone_density_signature,
            task.phenomenon_script_id.as_str(),
            task.metric_surface_debug_field,
        );
        if let Err(error) = save_chunk_record(&chunk_file, &generated) {
            warn!("USF mod runtime persistence write failed for {:?}: {}", chunk_file, error);
        }
        record = Some(generated);
    }

    let record = record.expect("USF mod runtime chunk record should exist after generate/load");
    let mesh = build_chunk_mesh(&record);

    ChunkSurfaceHydrationArtifact {
        chunk_entity: task.chunk_entity,
        chunk_coord: task.chunk_coord,
        canonical_coord: task.canonical_coord,
        record,
        mesh,
    }
}

pub(crate) fn apply_chunk_surface_hydration_artifact(
    artifact: ChunkSurfaceHydrationArtifact,
    commands: &mut Commands,
    chunk_store: &mut UsfChunkSurfaceStore,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let ChunkSurfaceHydrationArtifact {
        chunk_entity,
        chunk_coord,
        canonical_coord,
        record,
        mesh,
    } = artifact;

    if let Some(mesh) = mesh {
        let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::default());
        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(StandardMaterial {
            base_color: color_from_seed(record.chunk_seed),
            perceptual_roughness: 0.9,
            metallic: 0.0,
            ..Default::default()
        });
        let mut entity_commands = commands.entity(chunk_entity);
        entity_commands.insert((Mesh3d(mesh_handle), MeshMaterial3d(material_handle), Visibility::Visible));
        if let Some(collider) = collider {
            entity_commands.insert(collider);
        } else {
            warn!(
                "USF mod runtime collider build failed for chunk {:?}; mesh will render without collision.",
                chunk_coord
            );
            entity_commands.remove::<Collider>();
        }
    } else {
        commands.entity(chunk_entity).remove::<Collider>();
    }

    commands.entity(chunk_entity).insert(UsfChunkSurfaceVisual {
        chunk_seed: record.chunk_seed,
        sample_step: record.sample_step,
    });

    chunk_store.records.insert(canonical_coord, record);
}

pub(crate) fn sync_chunk_surface_visual_transforms_system(
    settings: Res<UsfChunkSurfaceRuntimeSettings>,
    mut params: ParamSet<(
        Query<(&ChunkLoader, &Transform), With<Player>>,
        Query<&Transform, (With<MainCamera>, Without<Player>, Without<UsfChunkSurfaceVisual>)>,
        Single<&Transform, (With<WorldPresentationRoot>, Without<Player>, Without<UsfChunkSurfaceVisual>)>,
        Query<
            (Entity, &Chunk, &mut Transform, &mut Visibility),
            (
                With<UsfChunkSurfaceVisual>,
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

    let Some(camera_transform) = camera_transform else {
        for (_entity, chunk, mut transform, mut visibility) in chunk_query.iter_mut() {
            transform.translation = chunk_center_in_active_native_space(&chunk.coord, player_root_native, active_scale_index);
            transform.rotation = Quat::IDENTITY;
            transform.scale = Vec3::splat(chunk_visual_scale_in_active_native_space(chunk.coord.scale, active_scale_index));
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
        transform.scale = Vec3::splat(chunk_visual_scale_in_active_native_space(chunk.coord.scale, active_scale_index));
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
        cull_candidates.push(ChunkVisualCullCandidate {
            entity,
            coarse_depth: (-relative_scale).max(0) as u8,
            distance_sq,
            front_dot,
        });
    }

    let visible_entities = select_visible_chunk_visual_entities(&cull_candidates, active_budget);
    for (entity, _chunk, _transform, mut visibility) in chunk_query.iter_mut() {
        *visibility = if visible_entities.contains(&entity) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub(crate) fn bind_chunk_surface_visuals_to_world_presentation_root_system(
    mut commands: Commands,
    root_query: Single<Entity, With<WorldPresentationRoot>>,
    chunk_query: Query<(Entity, Option<&ChildOf>), (With<UsfChunkSurfaceVisual>, Without<Player>)>,
) {
    let root = *root_query;
    for (entity, child_of) in chunk_query.iter() {
        if child_of.is_some_and(|relation| relation.parent() == root) {
            continue;
        }
        commands.entity(entity).insert(ChildOf(root));
    }
}

pub(crate) fn prune_chunk_surface_store_system(
    settings: Res<UsfChunkSurfaceRuntimeSettings>,
    loaded_chunks: Query<&Chunk>,
    mut chunk_store: ResMut<UsfChunkSurfaceStore>,
) {
    if !settings.enabled {
        chunk_store.records.clear();
        return;
    }

    let loaded = loaded_chunks.iter().map(|chunk| canonical_grid_coord(&chunk.coord)).collect::<HashSet<_>>();
    chunk_store.records.retain(|coord, _| loaded.contains(coord));
}
