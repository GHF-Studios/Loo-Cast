use crate::bevy::asset::RenderAssetUsages;
use crate::bevy::mesh::Indices;
use crate::bevy::prelude::*;
use crate::bevy::render::render_resource::PrimitiveTopology;
use crate::bevy_rapier3d::prelude::{Collider, ComputedColliderShape};
use crate::chunk::components::{Chunk, ChunkLoader};
#[cfg(not(test))]
use crate::chunk::gpu_density;
use crate::chunk::resources::ChunkManager;
use crate::config::statics::CONFIG;
use crate::player::components::Player;
use crate::render::components::{MainCamera, WorldPresentationRoot};
use crate::usf::content::{PLACEHOLDER_GAMEPLAY_CONTENT_PACKAGE_ID, ScaleContentRegistry, UsfActiveContentProfile, UsfExecutionPlan};
use crate::usf::definition::ZoneTypeId;
use crate::usf::dpt::{DptChunkKey, DptStore};
use crate::usf::phenomenon::{MetricSurfaceDebugFieldDefinition, PhenomenonDefinitionRegistry, PhenomenonKind};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::Scale;
use crate::usf::zlm::ZlmRegistry;
use crate::usf::zone::{ZoneBehaviorRegistry, ZoneDensityProfile};
use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;
use crate::workflow::functions::{WorkflowTimeoutControlDecision, handle_composite_workflow_return_now, run_workflow_ioe_with_timeout_control};
use crate::workflow::types::WorkflowTimeoutMode;
use core_mod_macros::{composite_workflow, composite_workflow_return};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::task::JoinHandle;

const CHUNK_SPAN_UNITS_I64: i64 = 1_000;
const HALF_CHUNK_SPAN_F32: f32 = 500.0;
const ROOT_AXIS_CELL_COUNT: i64 = 10;
const ROOT_AXIS_PERIOD_UNITS: i64 = CHUNK_SPAN_UNITS_I64 * ROOT_AXIS_CELL_COUNT;

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfDemoSettings {
    pub enabled: bool,
    pub world_seed: u64,
    pub sample_step: u16,
    pub iso_level: u8,
    pub hydration_batch_size: usize,
    pub hydration_commit_budget: usize,
    pub hydration_build_workers: usize,
    pub persistence_dir: String,
}
impl Default for UsfDemoSettings {
    fn default() -> Self {
        Self {
            enabled: CONFIG().get::<bool>("usf_demo/enabled"),
            world_seed: CONFIG().get::<u64>("usf_demo/world_seed"),
            sample_step: CONFIG().get::<u16>("usf_demo/sample_step"),
            iso_level: CONFIG().get::<u8>("usf_demo/iso_level"),
            hydration_batch_size: CONFIG().get::<usize>("usf_demo/hydration_batch_size"),
            hydration_commit_budget: CONFIG().get::<usize>("usf_demo/hydration_commit_budget"),
            hydration_build_workers: CONFIG().get::<usize>("usf_demo/hydration_build_workers"),
            persistence_dir: CONFIG().get::<String>("usf_demo/persistence_dir"),
        }
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct UsfDemoChunkVisual {
    pub chunk_seed: u64,
    pub sample_step: u16,
}

#[derive(Resource, Debug, Default)]
pub struct UsfDemoChunkStore {
    pub records: HashMap<GridVec, PersistedChunkRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializableGridCoord {
    pub scale_index: u8,
    pub digits: Vec<[i8; 3]>,
}
impl SerializableGridCoord {
    fn from_grid(coord: &GridVec) -> Self {
        let mut canonical = coord.clone();
        canonical.normalize();
        let digits = canonical
            .to_raw_vec_3d()
            .into_iter()
            .map(|xyz| [xyz.x as i8, xyz.y as i8, xyz.z as i8])
            .collect::<Vec<_>>();
        Self {
            scale_index: canonical.scale.index_from_top(),
            digits,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MixedMetricFieldU8 {
    Uniform(u8),
    Brick(Vec<u8>),
}
impl MixedMetricFieldU8 {
    fn from_values(values: Vec<u8>) -> Self {
        if let Some(first) = values.first().copied() {
            if values.iter().all(|value| *value == first) {
                return Self::Uniform(first);
            }
        }
        Self::Brick(values)
    }

    fn expand(&self, expected_len: usize) -> Option<Vec<u8>> {
        match self {
            MixedMetricFieldU8::Uniform(value) => Some(vec![*value; expected_len]),
            MixedMetricFieldU8::Brick(values) => {
                if values.len() == expected_len {
                    Some(values.clone())
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MixedMetricFieldU32 {
    Uniform(u32),
    Brick(Vec<u32>),
}
impl MixedMetricFieldU32 {
    fn from_values(values: Vec<u32>) -> Self {
        if let Some(first) = values.first().copied() {
            if values.iter().all(|value| *value == first) {
                return Self::Uniform(first);
            }
        }
        Self::Brick(values)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PersistedChunkRecord {
    pub world_seed: u64,
    pub active_scale_index: u8,
    pub chunk_coord: SerializableGridCoord,
    #[serde(default)]
    pub zone_type: String,
    #[serde(default)]
    pub zone_density_signature: u64,
    #[serde(default)]
    pub density_field_signature: u64,
    #[serde(default)]
    pub phenomenon_script_id: String,
    pub chunk_seed: u64,
    pub sample_step: u16,
    pub iso_level: u8,
    pub axis_samples: Vec<u16>,
    pub rho_field: MixedMetricFieldU8,
    pub zone_field: MixedMetricFieldU32,
}

pub(crate) fn sync_chunk_manager_loader_state_system(player_loader_query: Query<&ChunkLoader, With<Player>>, mut chunk_manager: ResMut<ChunkManager>) {
    let Ok(chunk_loader) = player_loader_query.single() else {
        return;
    };

    chunk_manager.active_scale = chunk_loader.scale;
    chunk_manager.loader_origin_grid = chunk_loader.origin_offset.clone();
    chunk_manager.loader_origin_unit = UnitVec::new(chunk_loader.origin_offset.clone(), Vec3::ZERO);
}

#[derive(Resource)]
pub struct UsfDemoHydrationWorkflowState {
    pub handle: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    queued_entities: VecDeque<Entity>,
    queued_lookup: HashSet<Entity>,
    in_flight_entities: HashSet<Entity>,
}
impl Default for UsfDemoHydrationWorkflowState {
    fn default() -> Self {
        Self {
            handle: None,
            queued_entities: VecDeque::new(),
            queued_lookup: HashSet::new(),
            in_flight_entities: HashSet::new(),
        }
    }
}
impl UsfDemoHydrationWorkflowState {
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

    fn prune_stale(&mut self, chunk_visual_query: &Query<Option<&UsfDemoChunkVisual>, With<Chunk>>) {
        self.queued_entities.retain(|entity| matches!(chunk_visual_query.get(*entity), Ok(None)));
        self.queued_lookup = self.queued_entities.iter().copied().collect::<HashSet<_>>();
        self.in_flight_entities.retain(|entity| matches!(chunk_visual_query.get(*entity), Ok(None)));
    }
}

#[derive(Debug, Clone)]
pub struct ChunkDemoHydrationTask {
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
pub struct ChunkDemoHydrationArtifact {
    pub chunk_entity: Entity,
    pub chunk_coord: GridVec,
    pub canonical_coord: GridVec,
    pub record: PersistedChunkRecord,
    pub mesh: Option<Mesh>,
}

pub(crate) fn run_if_placeholder_gameplay_content_enabled(
    settings: Option<Res<UsfDemoSettings>>,
    active_content_profile: Option<Res<UsfActiveContentProfile>>,
) -> bool {
    let Some(settings) = settings else {
        return false;
    };
    let Some(active_content_profile) = active_content_profile else {
        return false;
    };
    settings.enabled && active_content_profile.is_package_enabled(PLACEHOLDER_GAMEPLAY_CONTENT_PACKAGE_ID)
}

pub(crate) fn queue_chunk_demo_hydration_requests_system(
    settings: Res<UsfDemoSettings>,
    added_chunks: Query<Entity, (Added<Chunk>, Without<UsfDemoChunkVisual>)>,
    mut hydration_state: ResMut<UsfDemoHydrationWorkflowState>,
) {
    if !settings.enabled {
        return;
    }

    for entity in added_chunks.iter() {
        hydration_state.queue(entity);
    }
}

pub(crate) fn run_chunk_demo_hydration_workflow_system(
    settings: Res<UsfDemoSettings>,
    active_content_profile: Res<UsfActiveContentProfile>,
    phenomenon_definitions: Res<PhenomenonDefinitionRegistry>,
    execution_plan: Res<UsfExecutionPlan>,
    mut dpt_store: ResMut<DptStore>,
    zlm_registry: Res<ZlmRegistry>,
    scale_content_registry: Res<ScaleContentRegistry>,
    zone_behavior_registry: Res<ZoneBehaviorRegistry>,
    chunk_visual_query: Query<Option<&UsfDemoChunkVisual>, With<Chunk>>,
    chunk_query: Query<(&Chunk, Option<&UsfDemoChunkVisual>)>,
    mut hydration_state: ResMut<UsfDemoHydrationWorkflowState>,
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
    let mut tasks = Vec::<ChunkDemoHydrationTask>::new();

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
                "USF placeholder gameplay skipped: missing execution route for scale index {}",
                chunk_scale.index_from_top()
            );
            continue;
        };
        if !route
            .content_package_ids
            .iter()
            .any(|content_package_id| content_package_id == PLACEHOLDER_GAMEPLAY_CONTENT_PACKAGE_ID)
        {
            continue;
        }

        let Some(schema) = active_content_profile.schema_for_scale(chunk_scale) else {
            warn!(
                "USF demo chunk hydration skipped: missing DPT schema for chunk {:?} at scale index {}",
                chunk.coord,
                chunk_scale.index_from_top()
            );
            continue;
        };

        let canonical_coord = canonical_grid_coord(&chunk.coord);
        let chunk_key = DptChunkKey {
            scale: chunk_scale,
            coord: canonical_coord.clone(),
        };
        let zone_type = {
            let chunk_record = dpt_store.ensure_chunk_with_scale_binding(chunk_key, schema, &scale_content_registry);
            zlm_registry.classify_with_scale_binding(chunk_scale, schema, &chunk_record.metrics, &scale_content_registry)
        };
        let zone_density_profile = zone_behavior_registry
            .density_profile_for_zone(&zone_type)
            .unwrap_or_else(|| panic!("USF demo chunk hydration failed: missing zone density profile for zone '{}'.", zone_type.0));
        let supports = zone_behavior_registry
            .supports_for_zone(&zone_type)
            .unwrap_or_else(|| panic!("USF demo chunk hydration failed: missing supported phenomena for zone '{}'.", zone_type.0));
        // Placeholder gameplay path currently materializes one phenomenon contract per zone for chunk meshing.
        // Full zone realization policy selection remains in usf::zone runtime systems.
        let selected_support = supports
            .first()
            .unwrap_or_else(|| panic!("USF demo chunk hydration failed: zone '{}' has an empty supported phenomena list.", zone_type.0));
        let phenomenon_script_id = selected_support.phenomenon_id.clone();
        let phenomenon_kind = phenomenon_definitions
            .kind_for(phenomenon_script_id.as_str())
            .unwrap_or_else(|| panic!("USF demo chunk hydration failed: unknown phenomenon '{}'.", phenomenon_script_id));
        if phenomenon_kind != PhenomenonKind::MetricSurfaceDebug {
            panic!(
                "USF demo chunk hydration failed: phenomenon '{}' has unsupported kind '{:?}'. \
                 Expected '{:?}' for metric-surface debug meshing.",
                phenomenon_script_id,
                phenomenon_kind,
                PhenomenonKind::MetricSurfaceDebug
            );
        }
        let metric_surface_debug_field = phenomenon_definitions
            .metric_surface_debug_for(phenomenon_script_id.as_str())
            .unwrap_or_else(|| {
                panic!(
                    "USF demo chunk hydration failed: phenomenon '{}' is missing metric_surface_debug field definition.",
                    phenomenon_script_id
                )
            });
        let zone_density_signature = zone_density_profile.signature();
        let chunk_store_key = route.chunk_store_key.as_str();

        in_flight_entities.push(entity);
        tasks.push(ChunkDemoHydrationTask {
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

    let hydrate_async_input = crate::chunk::workflows::external::hydrate_chunk_visuals::AsyncInput {
        settings: settings.clone(),
        tasks,
        build_workers: settings.hydration_build_workers.max(1),
        commit_budget: settings.hydration_commit_budget.max(1),
    };

    let handle = composite_workflow!(
        HydrateChunkVisualsBatch,
        move in hydrate_async_input: crate::chunk::workflows::external::hydrate_chunk_visuals::AsyncInput,
    {
        let _ = run_workflow_ioe_with_timeout_control::<crate::chunk::workflows::chunk::hydrate_chunk_visuals::TypeIOE, _>(
            Duration::from_secs_f64(5.0),
            WorkflowTimeoutMode::VirtualTime,
            crate::chunk::workflows::chunk::hydrate_chunk_visuals::stages::build_artifacts::core_types::Input {
                inner: crate::chunk::workflows::external::hydrate_chunk_visuals::Input {
                    inner: hydrate_async_input,
                },
            },
            |ctx| chunk_demo_hydration_timeout_decision(ctx.module_name, ctx.workflow_name, ctx.timeout_count),
        )
        .await;
    });
    hydration_state.handle = Some(handle);
}

fn chunk_demo_hydration_timeout_decision(module_name: &'static str, workflow_name: &'static str, timeout_count: usize) -> WorkflowTimeoutControlDecision {
    if timeout_count == 1 {
        warn!(
            "Chunk demo hydration timeout request: {}::{}, timeout_count={}, decision=Retry",
            module_name, workflow_name, timeout_count
        );
        return WorkflowTimeoutControlDecision::Retry;
    }

    warn!(
        "Chunk demo hydration timeout escalation: {}::{}, timeout_count={}, decision=Panic",
        module_name, workflow_name, timeout_count
    );
    WorkflowTimeoutControlDecision::Panic
}

pub(crate) fn prepare_chunk_demo_hydration_artifact(settings: &UsfDemoSettings, task: ChunkDemoHydrationTask) -> ChunkDemoHydrationArtifact {
    let chunk_file = chunk_file_path(settings, task.chunk_scale, &task.canonical_coord, task.chunk_store_key.as_str());
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
            settings,
            task.chunk_scale,
            &task.canonical_coord,
            &task.zone_type,
            task.zone_density_profile,
            task.zone_density_signature,
            task.phenomenon_script_id.as_str(),
            task.metric_surface_debug_field,
        );
        if let Err(error) = save_chunk_record(&chunk_file, &generated) {
            warn!("USF demo persistence write failed for {:?}: {}", chunk_file, error);
        }
        record = Some(generated);
    }

    let record = record.expect("USF demo record should exist after generate/load");
    let mesh = build_chunk_mesh(&record);

    ChunkDemoHydrationArtifact {
        chunk_entity: task.chunk_entity,
        chunk_coord: task.chunk_coord,
        canonical_coord: task.canonical_coord,
        record,
        mesh,
    }
}

pub(crate) fn apply_chunk_demo_hydration_artifact(
    artifact: ChunkDemoHydrationArtifact,
    commands: &mut Commands,
    chunk_store: &mut UsfDemoChunkStore,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let ChunkDemoHydrationArtifact {
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
                "USF demo collider build failed for chunk {:?}; mesh will render without collision.",
                chunk_coord
            );
            entity_commands.remove::<Collider>();
        }
    } else {
        commands.entity(chunk_entity).remove::<Collider>();
    }

    commands.entity(chunk_entity).insert(UsfDemoChunkVisual {
        chunk_seed: record.chunk_seed,
        sample_step: record.sample_step,
    });

    chunk_store.records.insert(canonical_coord, record);
}

pub(crate) fn sync_chunk_demo_visual_transforms_system(
    settings: Res<UsfDemoSettings>,
    mut params: ParamSet<(
        Query<(&ChunkLoader, &Transform), With<Player>>,
        Query<&Transform, (With<MainCamera>, Without<Player>, Without<UsfDemoChunkVisual>)>,
        Single<&Transform, (With<WorldPresentationRoot>, Without<Player>, Without<UsfDemoChunkVisual>)>,
        Query<
            (Entity, &Chunk, &mut Transform, &mut Visibility),
            (With<UsfDemoChunkVisual>, Without<Player>, Without<MainCamera>, Without<WorldPresentationRoot>),
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

#[derive(Debug, Clone, Copy)]
struct ChunkVisualCullCandidate {
    entity: Entity,
    coarse_depth: u8,
    distance_sq: f32,
    front_dot: f32,
}

#[inline]
fn chunk_visual_band_budget(coarse_depth: u8, active_budget: usize) -> usize {
    match coarse_depth {
        0 => active_budget.max(1),
        1 => (active_budget / 3).max(6),
        2 => (active_budget / 6).max(2),
        3 => (active_budget / 12).max(1),
        _ => 1,
    }
}

fn select_visible_chunk_visual_entities(candidates: &[ChunkVisualCullCandidate], active_budget: usize) -> HashSet<Entity> {
    let mut grouped = BTreeMap::<u8, Vec<ChunkVisualCullCandidate>>::new();
    for candidate in candidates.iter().copied() {
        grouped.entry(candidate.coarse_depth).or_default().push(candidate);
    }

    let mut selected = HashSet::<Entity>::new();
    for (depth, mut entries) in grouped {
        entries.sort_by(|a, b| {
            b.front_dot
                .total_cmp(&a.front_dot)
                .then_with(|| a.distance_sq.total_cmp(&b.distance_sq))
                .then_with(|| a.entity.to_bits().cmp(&b.entity.to_bits()))
        });
        let budget = chunk_visual_band_budget(depth, active_budget).min(entries.len());
        for candidate in entries.into_iter().take(budget) {
            selected.insert(candidate.entity);
        }
    }

    selected
}

#[inline]
fn wrap_root_native_delta_axis(delta: f64) -> f64 {
    let period = ROOT_AXIS_PERIOD_UNITS as f64;
    if !delta.is_finite() || period <= f64::EPSILON {
        return 0.0;
    }
    let half = period * 0.5;
    ((delta + half).rem_euclid(period)) - half
}

#[inline]
fn chunk_center_in_active_native_space(chunk_coord: &GridVec, player_root_native: (f64, f64, f64), active_scale_index: i16) -> Vec3 {
    let chunk_canonical = canonical_grid_coord(chunk_coord);
    let chunk_root_native = sample_root_native_position(&chunk_canonical, Vec3::ZERO);
    let root_to_active = 10.0_f64.powi(active_scale_index as i32);

    let dx = wrap_root_native_delta_axis(chunk_root_native.0 - player_root_native.0) * root_to_active;
    let dy = wrap_root_native_delta_axis(chunk_root_native.1 - player_root_native.1) * root_to_active;
    let dz = wrap_root_native_delta_axis(chunk_root_native.2 - player_root_native.2) * root_to_active;
    Vec3::new(dx as f32, dy as f32, dz as f32)
}

#[inline]
fn chunk_visual_scale_in_active_native_space(chunk_scale: Scale, active_scale_index: i16) -> f32 {
    let exponent = active_scale_index - chunk_scale.index_from_top() as i16;
    let scale = 10.0_f64.powi(exponent as i32);
    if !scale.is_finite() || scale <= 0.0 {
        return f32::MIN_POSITIVE;
    }
    scale.clamp(f32::MIN_POSITIVE as f64, f32::MAX as f64) as f32
}

pub(crate) fn bind_chunk_demo_visuals_to_world_presentation_root_system(
    mut commands: Commands,
    root_query: Single<Entity, With<WorldPresentationRoot>>,
    chunk_query: Query<(Entity, Option<&ChildOf>), (With<UsfDemoChunkVisual>, Without<Player>)>,
) {
    let root = *root_query;
    for (entity, child_of) in chunk_query.iter() {
        if child_of.is_some_and(|relation| relation.parent() == root) {
            continue;
        }
        commands.entity(entity).insert(ChildOf(root));
    }
}

pub(crate) fn prune_chunk_demo_store_system(settings: Res<UsfDemoSettings>, loaded_chunks: Query<&Chunk>, mut chunk_store: ResMut<UsfDemoChunkStore>) {
    if !settings.enabled {
        chunk_store.records.clear();
        return;
    }

    let loaded = loaded_chunks.iter().map(|chunk| canonical_grid_coord(&chunk.coord)).collect::<HashSet<_>>();
    chunk_store.records.retain(|coord, _| loaded.contains(coord));
}

fn generate_chunk_record(
    settings: &UsfDemoSettings,
    chunk_scale: Scale,
    canonical_coord: &GridVec,
    zone_type: &ZoneTypeId,
    zone_density_profile: ZoneDensityProfile,
    zone_density_signature: u64,
    phenomenon_script_id: &str,
    metric_surface_debug_field: MetricSurfaceDebugFieldDefinition,
) -> PersistedChunkRecord {
    let axis_samples = build_axis_samples(settings.sample_step);
    let axis_points = axis_samples.len();
    let total_points = axis_points * axis_points * axis_points;

    let chunk_seed = derive_chunk_seed(settings.world_seed, canonical_coord);

    let rho_values = sample_density_field_values(
        settings,
        chunk_scale,
        canonical_coord,
        &axis_samples,
        zone_density_profile,
        metric_surface_debug_field,
    );
    let zone_id = zone_numeric_id(zone_type);
    let zone_values = vec![zone_id; total_points];

    PersistedChunkRecord {
        world_seed: settings.world_seed,
        active_scale_index: chunk_scale.index_from_top(),
        chunk_coord: SerializableGridCoord::from_grid(canonical_coord),
        zone_type: zone_type.0.clone(),
        zone_density_signature,
        density_field_signature: density_field_signature(metric_surface_debug_field),
        phenomenon_script_id: phenomenon_script_id.to_ascii_lowercase(),
        chunk_seed,
        sample_step: settings.sample_step,
        iso_level: settings.iso_level,
        axis_samples,
        rho_field: MixedMetricFieldU8::from_values(rho_values),
        zone_field: MixedMetricFieldU32::from_values(zone_values),
    }
}

fn sample_density_field_values(
    settings: &UsfDemoSettings,
    _chunk_scale: Scale,
    canonical_coord: &GridVec,
    axis_samples: &[u16],
    _zone_density_profile: ZoneDensityProfile,
    metric_surface_debug_field: MetricSurfaceDebugFieldDefinition,
) -> Vec<u8> {
    #[cfg(test)]
    {
        return sample_density_field_values_cpu(
            settings.world_seed,
            canonical_coord,
            axis_samples,
            _zone_density_profile,
            metric_surface_debug_field,
        );
    }

    #[cfg(not(test))]
    {
        match gpu_density::sample_density_field(settings.world_seed, _chunk_scale, canonical_coord, axis_samples) {
            Ok(values) => values,
            Err(error) => panic!(
                "USF demo GPU density sampling failed (scale_index={}, coord={:?}): {}",
                _chunk_scale.index_from_top(),
                canonical_coord,
                error
            ),
        }
    }
}

#[cfg(test)]
fn sample_density_field_values_cpu(
    world_seed: u64,
    canonical_coord: &GridVec,
    axis_samples: &[u16],
    zone_density_profile: ZoneDensityProfile,
    metric_surface_debug_field: MetricSurfaceDebugFieldDefinition,
) -> Vec<u8> {
    let axis_points = axis_samples.len();
    let total_points = axis_points * axis_points * axis_points;
    let mut rho_values = Vec::with_capacity(total_points);

    for iz in 0..axis_points {
        for iy in 0..axis_points {
            for ix in 0..axis_points {
                let local_offset = Vec3::new(
                    axis_samples[ix] as f32 - HALF_CHUNK_SPAN_F32,
                    axis_samples[iy] as f32 - HALF_CHUNK_SPAN_F32,
                    axis_samples[iz] as f32 - HALF_CHUNK_SPAN_F32,
                );
                let root_native = sample_root_native_position(canonical_coord, local_offset);
                rho_values.push(hash_density_u8(world_seed, root_native, zone_density_profile, metric_surface_debug_field));
            }
        }
    }

    rho_values
}

fn build_chunk_mesh(record: &PersistedChunkRecord) -> Option<Mesh> {
    let axis_points = record.axis_samples.len();
    if axis_points < 2 {
        return None;
    }

    let total_points = axis_points * axis_points * axis_points;
    let rho_values = record.rho_field.expand(total_points)?;

    let mut points = vec![Vec3::ZERO; total_points];
    let mut signed_field = vec![0.0_f32; total_points];

    for iz in 0..axis_points {
        for iy in 0..axis_points {
            for ix in 0..axis_points {
                let idx = grid_index(ix, iy, iz, axis_points);
                let x = record.axis_samples[ix] as f32 - HALF_CHUNK_SPAN_F32;
                let y = record.axis_samples[iy] as f32 - HALF_CHUNK_SPAN_F32;
                let z = record.axis_samples[iz] as f32 - HALF_CHUNK_SPAN_F32;
                points[idx] = Vec3::new(x, y, z);
                // Treat high rho as solid so we get "blobs in empty space" instead of caves in a filled volume.
                signed_field[idx] = record.iso_level as f32 - rho_values[idx] as f32;
            }
        }
    }

    let cube_corners: [[usize; 3]; 8] = [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0], [0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]];
    let tetrahedra: [[usize; 4]; 6] = [[0, 5, 1, 6], [0, 1, 2, 6], [0, 2, 3, 6], [0, 3, 7, 6], [0, 7, 4, 6], [0, 4, 5, 6]];

    let mut out_positions = Vec::<[f32; 3]>::new();
    let mut out_normals = Vec::<[f32; 3]>::new();
    let mut out_uvs = Vec::<[f32; 2]>::new();

    for iz in 0..(axis_points - 1) {
        for iy in 0..(axis_points - 1) {
            for ix in 0..(axis_points - 1) {
                let mut cube_points = [Vec3::ZERO; 8];
                let mut cube_values = [0.0_f32; 8];
                for (corner_i, [ox, oy, oz]) in cube_corners.iter().copied().enumerate() {
                    let sx = ix + ox;
                    let sy = iy + oy;
                    let sz = iz + oz;
                    let idx = grid_index(sx, sy, sz, axis_points);
                    cube_points[corner_i] = points[idx];
                    cube_values[corner_i] = signed_field[idx];
                }

                for tet in tetrahedra {
                    let p = [cube_points[tet[0]], cube_points[tet[1]], cube_points[tet[2]], cube_points[tet[3]]];
                    let s = [cube_values[tet[0]], cube_values[tet[1]], cube_values[tet[2]], cube_values[tet[3]]];
                    emit_tetra_surface(p, s, &mut out_positions, &mut out_normals, &mut out_uvs);
                }
            }
        }
    }

    if out_positions.is_empty() {
        return None;
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    let triangle_indices = (0..out_positions.len() as u32).collect::<Vec<_>>();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, out_positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, out_normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, out_uvs);
    mesh.insert_indices(Indices::U32(triangle_indices));
    Some(mesh)
}

fn emit_tetra_surface(points: [Vec3; 4], values: [f32; 4], out_positions: &mut Vec<[f32; 3]>, out_normals: &mut Vec<[f32; 3]>, out_uvs: &mut Vec<[f32; 2]>) {
    let mut inside = [0usize; 4];
    let mut outside = [0usize; 4];
    let mut inside_count = 0usize;
    let mut outside_count = 0usize;

    for i in 0..4 {
        if values[i] <= 0.0 {
            inside[inside_count] = i;
            inside_count += 1;
        } else {
            outside[outside_count] = i;
            outside_count += 1;
        }
    }

    if inside_count == 0 || inside_count == 4 {
        return;
    }

    let edge_point = |a_i: usize, b_i: usize| interpolate_iso(points[a_i], values[a_i], points[b_i], values[b_i]);

    match inside_count {
        1 => {
            let i = inside[0];
            let a = outside[0];
            let b = outside[1];
            let c = outside[2];
            let inside_ref = points[i];
            let outside_ref = (points[a] + points[b] + points[c]) / 3.0;
            push_oriented_triangle(
                edge_point(i, a),
                edge_point(i, b),
                edge_point(i, c),
                inside_ref,
                outside_ref,
                out_positions,
                out_normals,
                out_uvs,
            );
        }
        3 => {
            let o = outside[0];
            let a = inside[0];
            let b = inside[1];
            let c = inside[2];
            let inside_ref = (points[a] + points[b] + points[c]) / 3.0;
            let outside_ref = points[o];
            push_oriented_triangle(
                edge_point(o, a),
                edge_point(o, c),
                edge_point(o, b),
                inside_ref,
                outside_ref,
                out_positions,
                out_normals,
                out_uvs,
            );
        }
        2 => {
            let a = inside[0];
            let b = inside[1];
            let c = outside[0];
            let d = outside[1];
            let inside_ref = (points[a] + points[b]) * 0.5;
            let outside_ref = (points[c] + points[d]) * 0.5;

            let p0 = edge_point(a, c);
            let p1 = edge_point(b, c);
            let p2 = edge_point(b, d);
            let p3 = edge_point(a, d);
            push_oriented_triangle(p0, p1, p2, inside_ref, outside_ref, out_positions, out_normals, out_uvs);
            push_oriented_triangle(p0, p2, p3, inside_ref, outside_ref, out_positions, out_normals, out_uvs);
        }
        _ => {}
    }
}

fn push_oriented_triangle(
    a: Vec3,
    mut b: Vec3,
    mut c: Vec3,
    inside_ref: Vec3,
    outside_ref: Vec3,
    out_positions: &mut Vec<[f32; 3]>,
    out_normals: &mut Vec<[f32; 3]>,
    out_uvs: &mut Vec<[f32; 2]>,
) {
    let mut normal = (b - a).cross(c - a);
    let mut len_sq = normal.length_squared();
    if len_sq <= 1e-10 {
        return;
    }

    let expected_outward = outside_ref - inside_ref;
    if expected_outward.length_squared() > 1e-10 && normal.dot(expected_outward) < 0.0 {
        std::mem::swap(&mut b, &mut c);
        normal = (b - a).cross(c - a);
        len_sq = normal.length_squared();
        if len_sq <= 1e-10 {
            return;
        }
    }

    let n = normal / len_sq.sqrt();
    for p in [a, b, c] {
        out_positions.push([p.x, p.y, p.z]);
        out_normals.push([n.x, n.y, n.z]);
        out_uvs.push([
            ((p.x + HALF_CHUNK_SPAN_F32) / CHUNK_SPAN_UNITS_I64 as f32).clamp(0.0, 1.0),
            ((p.y + HALF_CHUNK_SPAN_F32) / CHUNK_SPAN_UNITS_I64 as f32).clamp(0.0, 1.0),
        ]);
    }
}

#[inline]
fn interpolate_iso(a_pos: Vec3, a_val: f32, b_pos: Vec3, b_val: f32) -> Vec3 {
    let denom = a_val - b_val;
    let t = if denom.abs() <= 1e-6 { 0.5 } else { (a_val / denom).clamp(0.0, 1.0) };
    a_pos + (b_pos - a_pos) * t
}

#[inline]
fn grid_index(ix: usize, iy: usize, iz: usize, axis_points: usize) -> usize {
    ix + axis_points * (iy + axis_points * iz)
}

fn canonical_grid_coord(coord: &GridVec) -> GridVec {
    let mut canonical = coord.clone();
    canonical.normalize();
    canonical
}

fn build_axis_samples(step: u16) -> Vec<u16> {
    let step = step.clamp(1, 1_000);
    let mut samples = Vec::new();
    let mut cursor = 0_u16;
    while cursor < 1_000 {
        samples.push(cursor);
        let next = cursor.saturating_add(step);
        if next == cursor {
            break;
        }
        cursor = next.min(1_000);
    }
    if samples.last().copied() != Some(1_000) {
        samples.push(1_000);
    }
    samples
}

fn derive_chunk_seed(world_seed: u64, canonical_coord: &GridVec) -> u64 {
    let mut hasher = DefaultHasher::new();
    world_seed.hash(&mut hasher);
    canonical_coord.hash(&mut hasher);
    let raw = hasher.finish();
    if raw == 0 { 0x9e37_79b9_7f4a_7c15 } else { raw }
}

fn chunk_file_path(settings: &UsfDemoSettings, chunk_scale: Scale, canonical_coord: &GridVec, chunk_store_key: &str) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    canonical_coord.hash(&mut hasher);
    let coord_hash = hasher.finish();
    let normalized_store_key = chunk_store_key.trim().to_ascii_lowercase();
    let sanitized_store_key = if normalized_store_key.is_empty() {
        "chunk_store.default".to_string()
    } else {
        normalized_store_key
            .chars()
            .map(|char| {
                if char.is_ascii_alphanumeric() || char == '_' || char == '-' || char == '.' {
                    char
                } else {
                    '_'
                }
            })
            .collect::<String>()
    };
    Path::new(&settings.persistence_dir).join(sanitized_store_key).join(format!(
        "ws_{:016x}_as_{:02}_coord_{:016x}.json",
        settings.world_seed,
        chunk_scale.index_from_top(),
        coord_hash
    ))
}

fn load_chunk_record(path: &Path) -> Option<PersistedChunkRecord> {
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice::<PersistedChunkRecord>(&bytes).ok()
}

fn save_chunk_record(path: &Path, record: &PersistedChunkRecord) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("create_dir_all failed: {error}"))?;
    }
    let encoded = serde_json::to_vec_pretty(record).map_err(|error| format!("serialize failed: {error}"))?;
    fs::write(path, encoded).map_err(|error| format!("write failed: {error}"))?;
    Ok(())
}

fn color_from_seed(seed: u64) -> Color {
    let r = ((seed & 0xff) as f32) / 255.0;
    let g = (((seed >> 8) & 0xff) as f32) / 255.0;
    let b = (((seed >> 16) & 0xff) as f32) / 255.0;
    Color::srgb(0.2 + 0.6 * r, 0.2 + 0.6 * g, 0.2 + 0.6 * b)
}

fn hash_density_u8(
    world_seed: u64,
    root_native: (f64, f64, f64),
    _zone_density_profile: ZoneDensityProfile,
    metric_surface_debug_field: MetricSurfaceDebugFieldDefinition,
) -> u8 {
    let (wx, wy, wz) = wrap_root_native_position(root_native);

    // Phenomenon owns field function parameters; meshing/collider are engine kernels.
    let seed = mix64(world_seed ^ metric_surface_debug_field.seed_salt_primary);
    let base = value_noise_3d(seed, wx, wy, wz, metric_surface_debug_field.coarse_span_units);
    let detail = value_noise_3d(
        seed ^ metric_surface_debug_field.seed_salt_detail,
        wx,
        wy,
        wz,
        metric_surface_debug_field.detail_span_units,
    );
    let weight_sum = (metric_surface_debug_field.coarse_weight + metric_surface_debug_field.detail_weight).max(f32::MIN_POSITIVE);
    let combined = ((base * metric_surface_debug_field.coarse_weight) + (detail * metric_surface_debug_field.detail_weight)) / weight_sum;

    // Bias and shape to "mostly empty with occasional coherent surfaces".
    let shaped = ((combined - metric_surface_debug_field.bias) * metric_surface_debug_field.gain + metric_surface_debug_field.center).clamp(0.0, 1.0);
    // v2 contract: one shared field function across all scales.
    // Zone behavior remains metadata for later phenomena/interaction logic, not geometry warping.
    (shaped * 255.0).round() as u8
}

fn zone_numeric_id(zone_type: &ZoneTypeId) -> u32 {
    let mut state = 0x13d7_4b29_11f2_7a67_u64;
    for byte in zone_type.0.as_bytes() {
        state = mix64(state ^ (*byte as u64));
    }
    (state & 0x0000_ffff) as u32
}

#[inline]
fn density_field_signature(metric_surface_debug_field: MetricSurfaceDebugFieldDefinition) -> u64 {
    const DENSITY_ALGO_REVISION: u64 = 4;
    let mut signature_seed = 0xa3f1_1a89_5d4c_2be7_u64 ^ DENSITY_ALGO_REVISION;
    signature_seed ^= CHUNK_SPAN_UNITS_I64 as u64;
    signature_seed ^= (ROOT_AXIS_CELL_COUNT as u64) << 8;
    signature_seed ^= (ROOT_AXIS_PERIOD_UNITS as u64) << 16;
    signature_seed ^= 0x4750_555f_4445_4d4f_u64; // "GPU_DEMO"
    signature_seed ^= metric_surface_debug_field.coarse_span_units.to_bits();
    signature_seed ^= metric_surface_debug_field.detail_span_units.to_bits();
    signature_seed ^= (metric_surface_debug_field.coarse_weight.to_bits() as u64) << 1;
    signature_seed ^= (metric_surface_debug_field.detail_weight.to_bits() as u64) << 2;
    signature_seed ^= (metric_surface_debug_field.bias.to_bits() as u64) << 3;
    signature_seed ^= (metric_surface_debug_field.gain.to_bits() as u64) << 4;
    signature_seed ^= (metric_surface_debug_field.center.to_bits() as u64) << 5;
    signature_seed ^= metric_surface_debug_field.seed_salt_primary;
    signature_seed ^= metric_surface_debug_field.seed_salt_detail;
    mix64(signature_seed)
}

#[inline]
fn fold_signed(value: i64) -> u64 {
    value as u64
}

#[inline]
fn mix64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[inline]
fn sample_root_native_position(canonical_coord: &GridVec, local_offset: Vec3) -> (f64, f64, f64) {
    let mut sample = UnitVec::new(canonical_coord.clone(), local_offset);
    while sample.grid_offset.scale != Scale::MAX {
        sample.zoom_out();
    }
    let root = sample.grid_offset.xyz;
    (
        root.x as f64 * CHUNK_SPAN_UNITS_I64 as f64 + sample.unit_offset.x as f64,
        root.y as f64 * CHUNK_SPAN_UNITS_I64 as f64 + sample.unit_offset.y as f64,
        root.z as f64 * CHUNK_SPAN_UNITS_I64 as f64 + sample.unit_offset.z as f64,
    )
}

#[inline]
fn wrap_root_native_axis(value: f64) -> f64 {
    let period = ROOT_AXIS_PERIOD_UNITS as f64;
    if !value.is_finite() || period <= 0.0 {
        return 0.0;
    }
    value.rem_euclid(period)
}

#[inline]
fn wrap_root_native_position((x, y, z): (f64, f64, f64)) -> (f64, f64, f64) {
    (wrap_root_native_axis(x), wrap_root_native_axis(y), wrap_root_native_axis(z))
}

fn value_noise_3d(seed: u64, gx: f64, gy: f64, gz: f64, cell_size: f64) -> f32 {
    let cell_size = cell_size.max(f64::EPSILON);
    let sx = gx / cell_size;
    let sy = gy / cell_size;
    let sz = gz / cell_size;

    let cx0 = sx.floor() as i64;
    let cy0 = sy.floor() as i64;
    let cz0 = sz.floor() as i64;
    let cx1 = cx0 + 1;
    let cy1 = cy0 + 1;
    let cz1 = cz0 + 1;

    let tx = smoothstep01((sx - cx0 as f64) as f32);
    let ty = smoothstep01((sy - cy0 as f64) as f32);
    let tz = smoothstep01((sz - cz0 as f64) as f32);

    let c000 = lattice_noise01(seed, cx0, cy0, cz0);
    let c100 = lattice_noise01(seed, cx1, cy0, cz0);
    let c010 = lattice_noise01(seed, cx0, cy1, cz0);
    let c110 = lattice_noise01(seed, cx1, cy1, cz0);
    let c001 = lattice_noise01(seed, cx0, cy0, cz1);
    let c101 = lattice_noise01(seed, cx1, cy0, cz1);
    let c011 = lattice_noise01(seed, cx0, cy1, cz1);
    let c111 = lattice_noise01(seed, cx1, cy1, cz1);

    let x00 = lerp(c000, c100, tx);
    let x10 = lerp(c010, c110, tx);
    let x01 = lerp(c001, c101, tx);
    let x11 = lerp(c011, c111, tx);
    let y0 = lerp(x00, x10, ty);
    let y1 = lerp(x01, x11, ty);
    lerp(y0, y1, tz)
}

#[inline]
fn lattice_noise01(seed: u64, x: i64, y: i64, z: i64) -> f32 {
    let mut state = mix64(seed ^ 0x5f35_d3a1_c9b4_e227_u64);
    state = mix64(state ^ fold_signed(x));
    state = mix64(state ^ fold_signed(y));
    state = mix64(state ^ fold_signed(z));
    ((state >> 40) as u32) as f32 / ((1_u32 << 24) - 1) as f32
}

#[inline]
fn smoothstep01(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::pos::types::GridXyz;

    fn test_settings() -> UsfDemoSettings {
        UsfDemoSettings {
            enabled: true,
            world_seed: 42,
            sample_step: 64,
            iso_level: 128,
            hydration_batch_size: 64,
            hydration_commit_budget: 4,
            hydration_build_workers: 4,
            persistence_dir: std::env::temp_dir().join("usf_demo_chunk_tests").to_string_lossy().to_string(),
        }
    }

    fn rho_values(record: &PersistedChunkRecord) -> Vec<u8> {
        let axis_points = record.axis_samples.len();
        let total = axis_points * axis_points * axis_points;
        record.rho_field.expand(total).expect("rho field should expand")
    }

    fn test_zone() -> (ZoneTypeId, ZoneDensityProfile) {
        (
            ZoneTypeId::new("forest"),
            ZoneDensityProfile {
                density_multiplier: 0.72,
                density_offset: 0.14,
                density_floor: 0.05,
                density_ceil: 0.88,
            },
        )
    }

    fn test_phenomenon_id() -> &'static str {
        "phenomenon.placeholder.metric_surface_debug"
    }

    fn test_metric_surface_debug_field() -> MetricSurfaceDebugFieldDefinition {
        MetricSurfaceDebugFieldDefinition::default()
    }

    #[test]
    fn rho_sampling_matches_across_adjacent_chunk_borders() {
        let settings = test_settings();
        let left = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let right = left.clone() + IVec3::new(1, 0, 0);
        let (zone_type, zone_density_profile) = test_zone();
        let zone_density_signature = zone_density_profile.signature();

        let left_record = generate_chunk_record(
            &settings,
            Scale::MAX,
            &left,
            &zone_type,
            zone_density_profile,
            zone_density_signature,
            test_phenomenon_id(),
            test_metric_surface_debug_field(),
        );
        let right_record = generate_chunk_record(
            &settings,
            Scale::MAX,
            &right,
            &zone_type,
            zone_density_profile,
            zone_density_signature,
            test_phenomenon_id(),
            test_metric_surface_debug_field(),
        );

        let axis_points = left_record.axis_samples.len();
        assert_eq!(axis_points, right_record.axis_samples.len());

        let left_values = rho_values(&left_record);
        let right_values = rho_values(&right_record);
        let lx = axis_points - 1;
        let rx = 0;

        for iz in 0..axis_points {
            for iy in 0..axis_points {
                let left_idx = grid_index(lx, iy, iz, axis_points);
                let right_idx = grid_index(rx, iy, iz, axis_points);
                assert_eq!(left_values[left_idx], right_values[right_idx], "border seam mismatch at (y={}, z={})", iy, iz);
            }
        }
    }

    #[test]
    fn rho_sampling_loops_across_top_level_wrap_boundary() {
        let settings = test_settings();
        let left = GridVec::new_root(GridXyz::new_local(4, 0, 0));
        let right = GridVec::new_root(GridXyz::new_local(-5, 0, 0));
        let (zone_type, zone_density_profile) = test_zone();
        let zone_density_signature = zone_density_profile.signature();

        let left_record = generate_chunk_record(
            &settings,
            Scale::MAX,
            &left,
            &zone_type,
            zone_density_profile,
            zone_density_signature,
            test_phenomenon_id(),
            test_metric_surface_debug_field(),
        );
        let right_record = generate_chunk_record(
            &settings,
            Scale::MAX,
            &right,
            &zone_type,
            zone_density_profile,
            zone_density_signature,
            test_phenomenon_id(),
            test_metric_surface_debug_field(),
        );

        let axis_points = left_record.axis_samples.len();
        assert_eq!(axis_points, right_record.axis_samples.len());

        let left_values = rho_values(&left_record);
        let right_values = rho_values(&right_record);
        let lx = axis_points - 1;
        let rx = 0;

        for iz in 0..axis_points {
            for iy in 0..axis_points {
                let left_idx = grid_index(lx, iy, iz, axis_points);
                let right_idx = grid_index(rx, iy, iz, axis_points);
                assert_eq!(
                    left_values[left_idx], right_values[right_idx],
                    "top-level wrap seam mismatch at (y={}, z={})",
                    iy, iz
                );
            }
        }
    }

    #[test]
    fn persistence_roundtrip_is_stable() {
        let settings = test_settings();
        let coord = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let (zone_type, zone_density_profile) = test_zone();
        let record = generate_chunk_record(
            &settings,
            Scale::MAX,
            &coord,
            &zone_type,
            zone_density_profile,
            zone_density_profile.signature(),
            test_phenomenon_id(),
            test_metric_surface_debug_field(),
        );

        let path = Path::new(&settings.persistence_dir).join("roundtrip_chunk.json");
        save_chunk_record(&path, &record).expect("save should succeed");
        let loaded = load_chunk_record(&path).expect("load should succeed");
        assert_eq!(record, loaded);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn density_sampling_is_scale_invariant_for_shared_root_space_points() {
        let settings = test_settings();
        let parent = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let child = GridVec::new(parent.clone(), GridXyz::new_local(3, 4, 4));
        let (_zone_type, zone_density_profile) = test_zone();
        let child_digit = child.xyz;

        // Parent/child samples are equivalent when parent local offset = child_origin + child_offset/10.
        // Choose offsets divisible by 10 so the parent offset remains integral in local units.
        for child_local in [-500_i32, -300, 0, 300, 500] {
            let child_offset = Vec3::splat(child_local as f32);
            let parent_offset = Vec3::new(
                child_digit.x as f32 * 100.0 + child_offset.x / 10.0,
                child_digit.y as f32 * 100.0 + child_offset.y / 10.0,
                child_digit.z as f32 * 100.0 + child_offset.z / 10.0,
            );

            let child_root_native = sample_root_native_position(&child, child_offset);
            let parent_root_native = sample_root_native_position(&parent, parent_offset);
            let wrapped_child = wrap_root_native_position(child_root_native);
            let wrapped_parent = wrap_root_native_position(parent_root_native);
            let abs_diff = (
                (wrapped_child.0 - wrapped_parent.0).abs(),
                (wrapped_child.1 - wrapped_parent.1).abs(),
                (wrapped_child.2 - wrapped_parent.2).abs(),
            );

            assert!(
                abs_diff.0 <= 1e-6 && abs_diff.1 <= 1e-6 && abs_diff.2 <= 1e-6,
                "root-native mismatch for child_local={child_local}: child={wrapped_child:?}, parent={wrapped_parent:?}"
            );

            let child_density = hash_density_u8(settings.world_seed, child_root_native, zone_density_profile, test_metric_surface_debug_field());
            let parent_density = hash_density_u8(settings.world_seed, parent_root_native, zone_density_profile, test_metric_surface_debug_field());
            assert_eq!(
                child_density, parent_density,
                "density mismatch for child_local={child_local}: child={child_density}, parent={parent_density}"
            );
        }
    }

    #[test]
    fn density_sampling_is_scale_invariant_for_fractional_shared_points() {
        let settings = test_settings();
        let parent = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let child = GridVec::new(parent.clone(), GridXyz::new_local(-2, 1, 3));
        let (_zone_type, zone_density_profile) = test_zone();
        let child_digit = child.xyz;

        let child_offsets = [
            Vec3::new(-437.25, 118.125, -42.5),
            Vec3::new(-221.75, -339.875, 271.625),
            Vec3::new(19.375, -75.625, 413.5),
            Vec3::new(348.5, 244.25, -196.125),
        ];

        for child_offset in child_offsets {
            let parent_offset = Vec3::new(
                child_digit.x as f32 * 100.0 + child_offset.x / 10.0,
                child_digit.y as f32 * 100.0 + child_offset.y / 10.0,
                child_digit.z as f32 * 100.0 + child_offset.z / 10.0,
            );

            let child_root_native = sample_root_native_position(&child, child_offset);
            let parent_root_native = sample_root_native_position(&parent, parent_offset);
            let wrapped_child = wrap_root_native_position(child_root_native);
            let wrapped_parent = wrap_root_native_position(parent_root_native);
            let abs_diff = (
                (wrapped_child.0 - wrapped_parent.0).abs(),
                (wrapped_child.1 - wrapped_parent.1).abs(),
                (wrapped_child.2 - wrapped_parent.2).abs(),
            );

            assert!(
                abs_diff.0 <= 1e-3 && abs_diff.1 <= 1e-3 && abs_diff.2 <= 1e-3,
                "root-native mismatch for child_offset={child_offset:?}: child={wrapped_child:?}, parent={wrapped_parent:?}"
            );

            let child_density = hash_density_u8(settings.world_seed, child_root_native, zone_density_profile, test_metric_surface_debug_field());
            let parent_density = hash_density_u8(settings.world_seed, parent_root_native, zone_density_profile, test_metric_surface_debug_field());
            assert_eq!(
                child_density, parent_density,
                "fractional density mismatch for child_offset={child_offset:?}: child={child_density}, parent={parent_density}"
            );
        }
    }
}
