use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use crate::bevy::prelude::*;
use crate::config::statics::CONFIG;
use crate::usf::authority::{USF_DOMAIN_SUBSTRATE, UsfAuthorityDiagnostics, UsfWorldAuthorityContract, guard_runtime_state_domain_with_diagnostics};
use crate::usf::chunk::components::Chunk;
use crate::usf::metric_container::{MetricContainerChunkKey, MetricContainerLayout, deterministic_metric_vector};
use crate::usf::mod_packs::UsfActiveModPack;
use crate::usf::phenomenon::{
    PartialPhenomenonModel, PersistedPartialPhenomenonModelRecord, PersistedPhenomenonModelRecord, PhenomenonId, PhenomenonModel, PhenomenonModelProjection,
    PhenomenonModelScriptDefinitionRef, PhenomenonModelState, PhenomenonModelSupport, PhenomenonModelTopology,
};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use crate::usf::schedule::{UsfSimulationSet, UsfSubstrateSet};
use crate::usf::zlm::ZlmRegistry;
use crate::usf::zone::ZoneTypeId;

mod policy;
use policy::{
    SubstrateTransitionPolicy, build_octree_from_leaf, choose_leaf_representation, compute_refinement_state, derive_transition_decision_from_state,
    leaf_kind_tag,
};

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubstrateChunkEdge {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct ChunkEdgeInterface {
    pub edge: SubstrateChunkEdge,
    pub projected_metric_vector: Vec<f32>,
    pub coupling_signature: u64,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub enum SubstrateLeafContainer {
    Uniform {
        value: f32,
    },
    DenseBrick {
        values: Vec<f32>,
        axis_resolution: u8,
    },
    PaletteBrick {
        palette: Vec<f32>,
        indices: Vec<u8>,
        axis_resolution: u8,
    },
    Gradient {
        origin: [f32; 3],
        gradient: [f32; 3],
        base: f32,
    },
    Statistical {
        mean: f32,
        variance: f32,
        min: f32,
        max: f32,
    },
    Heightfield {
        heights: Vec<f32>,
        axis_resolution: u8,
        min: f32,
        max: f32,
    },
    DelegatedToPhenomenon {
        phenomenon_id: PhenomenonId,
        model_id: String,
        scale_index: u8,
    },
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq)]
pub struct SubstrateRefinementState {
    pub energy: f32,
    pub instability: f32,
    pub gradient: f32,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct SubstrateTransitionDecision {
    pub refine: bool,
    pub coarsen: bool,
    pub target_leaf: Option<SubstrateLeafContainer>,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub enum AdaptiveSubstrateOctreeNode {
    Leaf {
        state: SubstrateRefinementState,
        leaf: SubstrateLeafContainer,
    },
    Branch {
        state: SubstrateRefinementState,
        children: Vec<AdaptiveSubstrateOctreeNode>,
    },
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct SubstrateChunkSummary {
    pub scale: Scale,
    pub zone_type: ZoneTypeId,
    pub metric_vector: Vec<f32>,
    pub projection_signature: u64,
    pub dominant_leaf_kind: String,
    pub edge_interfaces: Vec<ChunkEdgeInterface>,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct AdaptiveChunkSubstrate {
    pub chunk_coord: GridVec,
    pub octree: AdaptiveSubstrateOctreeNode,
    pub summary: SubstrateChunkSummary,
}

#[derive(Resource, Debug, Default)]
pub struct AdaptiveSubstrateStore {
    pub chunks: HashMap<GridVec, AdaptiveChunkSubstrate>,
}
impl AdaptiveSubstrateStore {
    pub fn summary_for_chunk(&self, chunk_coord: &GridVec) -> Option<&SubstrateChunkSummary> {
        self.chunks.get(chunk_coord).map(|chunk| &chunk.summary)
    }

    pub fn chunk_for_coord(&self, chunk_coord: &GridVec) -> Option<&AdaptiveChunkSubstrate> {
        self.chunks.get(chunk_coord)
    }

    pub fn edge_interfaces_for_chunk(&self, chunk_coord: &GridVec) -> Option<&[ChunkEdgeInterface]> {
        self.chunks.get(chunk_coord).map(|chunk| chunk.summary.edge_interfaces.as_slice())
    }

    pub fn edge_interface_for_chunk(&self, chunk_coord: &GridVec, edge: SubstrateChunkEdge) -> Option<&ChunkEdgeInterface> {
        self.chunks
            .get(chunk_coord)
            .and_then(|chunk| chunk.summary.edge_interfaces.iter().find(|interface| interface.edge == edge))
    }

    pub fn upsert_chunk(&mut self, chunk: AdaptiveChunkSubstrate) {
        self.chunks.insert(chunk.chunk_coord.clone(), chunk);
    }
}

#[derive(Resource, Debug, Default)]
pub struct AdaptiveSubstrateRuntimeState {
    initialized: bool,
    pending_rebuild_chunks: HashSet<GridVec>,
    probe: SubstrateRebuildProbe,
}

#[derive(Resource, Debug, Default)]
pub struct SubstratePlannedUpdateQueue {
    live_chunk_coords: HashSet<GridVec>,
    prune_to_live: bool,
    planned_chunks: Vec<AdaptiveChunkSubstrate>,
}
impl SubstratePlannedUpdateQueue {
    fn clear(&mut self) {
        self.live_chunk_coords.clear();
        self.prune_to_live = false;
        self.planned_chunks.clear();
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct SubstrateChunkDeltaState {
    pub revision: u64,
    pub changed_chunks: HashSet<GridVec>,
    pub removed_chunks: HashSet<GridVec>,
}
impl SubstrateChunkDeltaState {
    fn clear_frame(&mut self) {
        self.changed_chunks.clear();
        self.removed_chunks.clear();
    }

    fn mark_changed(&mut self, coord: GridVec) {
        self.changed_chunks.insert(coord);
    }

    fn mark_removed(&mut self, coord: GridVec) {
        self.removed_chunks.insert(coord);
    }

    fn bump_revision_if_dirty(&mut self) {
        if !self.changed_chunks.is_empty() || !self.removed_chunks.is_empty() {
            self.revision = self.revision.wrapping_add(1);
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Resource)]
pub struct SubstrateRebuildSettings {
    pub max_chunks_per_frame: usize,
}
impl Default for SubstrateRebuildSettings {
    fn default() -> Self {
        Self {
            max_chunks_per_frame: CONFIG().get::<usize>("usf/substrate/runtime_state/max_chunks_per_frame"),
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone, Copy, PartialEq)]
#[reflect(Resource)]
pub struct SubstratePartitionCouplingPolicy {
    pub existing_metric_weight: f32,
    pub neighbor_metric_weight: f32,
}
impl Default for SubstratePartitionCouplingPolicy {
    fn default() -> Self {
        Self {
            existing_metric_weight: 0.70,
            neighbor_metric_weight: 0.30,
        }
    }
}
impl SubstratePartitionCouplingPolicy {
    pub fn from_config() -> Self {
        let existing_metric_weight = CONFIG().get::<f32>("usf/substrate/runtime_state/partition_edge_existing_weight");
        let neighbor_metric_weight = CONFIG().get::<f32>("usf/substrate/runtime_state/partition_edge_neighbor_weight");
        let weights_are_valid = existing_metric_weight.is_finite()
            && neighbor_metric_weight.is_finite()
            && existing_metric_weight >= 0.0
            && neighbor_metric_weight >= 0.0
            && (existing_metric_weight + neighbor_metric_weight) > 0.0;
        if !weights_are_valid {
            panic!(
                "USF substrate partition coupling config is invalid: existing_weight={} neighbor_weight={} (expected finite values >= 0 with positive sum).",
                existing_metric_weight, neighbor_metric_weight
            );
        }
        Self {
            existing_metric_weight,
            neighbor_metric_weight,
        }
    }
}

#[derive(Debug, Default)]
pub(super) struct SubstrateRebuildProbe {
    window_start: Option<Instant>,
    runs: u32,
    full_rebuild_runs: u32,
    runs_with_dirty_chunks: u32,
    runs_with_dirty_models: u32,
    runs_with_removed_chunks: u32,
    runs_with_removed_models: u32,
    rebuilt_chunks: u64,
    live_chunks: u64,
    queued_before_total: u64,
    queued_after_total: u64,
    budget_total: u64,
    elapsed_ms_total: f64,
}
impl SubstrateRebuildProbe {
    fn observe(
        &mut self,
        full_rebuild: bool,
        has_dirty_chunks: bool,
        has_dirty_models: bool,
        has_removed_chunks: bool,
        has_removed_models: bool,
        rebuilt_chunks: usize,
        live_chunks: usize,
        queued_before: usize,
        queued_after: usize,
        frame_budget: usize,
        elapsed: Duration,
    ) {
        if !usf_hotpath_probe_enabled() {
            return;
        }

        let now = Instant::now();
        let window_start = self.window_start.get_or_insert(now);
        self.runs += 1;
        if full_rebuild {
            self.full_rebuild_runs += 1;
        }
        if has_dirty_chunks {
            self.runs_with_dirty_chunks += 1;
        }
        if has_dirty_models {
            self.runs_with_dirty_models += 1;
        }
        if has_removed_chunks {
            self.runs_with_removed_chunks += 1;
        }
        if has_removed_models {
            self.runs_with_removed_models += 1;
        }
        self.rebuilt_chunks += rebuilt_chunks as u64;
        self.live_chunks += live_chunks as u64;
        self.queued_before_total += queued_before as u64;
        self.queued_after_total += queued_after as u64;
        self.budget_total += frame_budget as u64;
        self.elapsed_ms_total += elapsed.as_secs_f64() * 1000.0;

        if now.duration_since(*window_start) < Duration::from_secs(1) {
            return;
        }

        warn!(
            "USF hotpath probe [substrate_rebuild]: runs={}, full_rebuild_runs={}, dirty_chunk_runs={}, dirty_model_runs={}, removed_chunk_runs={}, removed_model_runs={}, rebuilt_chunks={}, avg_live_chunks_per_run={:.1}, avg_queued_before={:.1}, avg_queued_after={:.1}, avg_budget={:.1}, avg_ms_per_run={:.3}",
            self.runs,
            self.full_rebuild_runs,
            self.runs_with_dirty_chunks,
            self.runs_with_dirty_models,
            self.runs_with_removed_chunks,
            self.runs_with_removed_models,
            self.rebuilt_chunks,
            (self.live_chunks as f64) / (self.runs as f64),
            (self.queued_before_total as f64) / (self.runs as f64),
            (self.queued_after_total as f64) / (self.runs as f64),
            (self.budget_total as f64) / (self.runs as f64),
            self.elapsed_ms_total / (self.runs as f64)
        );

        self.window_start = Some(now);
        self.runs = 0;
        self.full_rebuild_runs = 0;
        self.runs_with_dirty_chunks = 0;
        self.runs_with_dirty_models = 0;
        self.runs_with_removed_chunks = 0;
        self.runs_with_removed_models = 0;
        self.rebuilt_chunks = 0;
        self.live_chunks = 0;
        self.queued_before_total = 0;
        self.queued_after_total = 0;
        self.budget_total = 0;
        self.elapsed_ms_total = 0.0;
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
struct ModelProjectionContribution {
    phenomenon_id: PhenomenonId,
    model_id: String,
    topology: PhenomenonModelTopology,
    partition_key: Option<u64>,
    metric_index: usize,
    value: f32,
}

#[derive(Debug, Clone)]
struct PartitionProjectionSample {
    phenomenon_id: PhenomenonId,
    model_id: String,
    scale: Scale,
    chunk_coord: GridVec,
    partition_key: u64,
    metric_index: usize,
    value: f32,
}

#[derive(Debug, Clone, Default)]
struct PartitionProjectionIndex {
    by_chunk: HashMap<GridVec, Vec<PartitionProjectionSample>>,
}
impl PartitionProjectionIndex {
    fn insert(&mut self, sample: PartitionProjectionSample) {
        self.by_chunk.entry(sample.chunk_coord.clone()).or_default().push(sample);
    }

    fn samples_for_chunk(&self, chunk_coord: &GridVec) -> Option<&[PartitionProjectionSample]> {
        self.by_chunk.get(chunk_coord).map(Vec::as_slice)
    }
}

#[derive(Debug, Clone)]
struct RuntimeModelProjectionSnapshot {
    phenomenon_id: PhenomenonId,
    topology: PhenomenonModelTopology,
    model_id: String,
    support: Option<crate::usf::phenomenon::PhenomenonModelSupportBounds>,
    partial_chunk: Option<GridVec>,
    partition_key: Option<u64>,
    projection_metric_name: String,
    projection_bias: f32,
    projection_gain: f32,
    scalar_channels: Vec<(String, f32)>,
}

#[derive(Debug, Clone, Default)]
struct RuntimeModelProjectionIndex {
    by_scale: HashMap<u8, Vec<RuntimeModelProjectionSnapshot>>,
}
impl RuntimeModelProjectionIndex {
    fn insert(&mut self, scale: Scale, snapshot: RuntimeModelProjectionSnapshot) {
        self.by_scale
            .entry(scale.index_from_top())
            .or_default()
            .push(snapshot);
    }

    fn snapshots_for_scale(&self, scale: Scale) -> Option<&[RuntimeModelProjectionSnapshot]> {
        self.by_scale
            .get(&scale.index_from_top())
            .map(Vec::as_slice)
    }
}

pub(super) fn plan_chunk_substrate_rebuilds_system(
    authority_contract: Res<UsfWorldAuthorityContract>,
    mut authority_diagnostics: Option<ResMut<UsfAuthorityDiagnostics>>,
    active_modpack: Res<UsfActiveModPack>,
    zlm_registry: Res<ZlmRegistry>,
    rebuild_settings: Res<SubstrateRebuildSettings>,
    partition_coupling_policy: Res<SubstratePartitionCouplingPolicy>,
    transition_policy: Res<SubstrateTransitionPolicy>,
    loaded_chunks: Query<&Chunk>,
    dirty_chunks: Query<&Chunk, (With<Chunk>, Or<(Added<Chunk>, Changed<Chunk>)>)>,
    mut removed_chunks: RemovedComponents<Chunk>,
    model_query: Query<(
        &PhenomenonModel,
        &PhenomenonModelScriptDefinitionRef,
        &PhenomenonModelProjection,
        Option<&PhenomenonModelSupport>,
        Option<&PhenomenonModelState>,
        Option<&PartialPhenomenonModel>,
    )>,
    dirty_models: Query<
        (&PhenomenonModel, Option<&PhenomenonModelSupport>, Option<&PartialPhenomenonModel>),
        (
            With<PhenomenonModel>,
            Or<(
                Added<PhenomenonModel>,
                Changed<PhenomenonModel>,
                Added<PhenomenonModelScriptDefinitionRef>,
                Changed<PhenomenonModelScriptDefinitionRef>,
                Added<PhenomenonModelProjection>,
                Changed<PhenomenonModelProjection>,
                Added<PhenomenonModelSupport>,
                Changed<PhenomenonModelSupport>,
                Added<PhenomenonModelState>,
                Changed<PhenomenonModelState>,
                Added<PartialPhenomenonModel>,
                Changed<PartialPhenomenonModel>,
            )>,
        ),
    >,
    mut removed_models: RemovedComponents<PhenomenonModel>,
    mut runtime_state: ResMut<AdaptiveSubstrateRuntimeState>,
    substrate_store: Res<AdaptiveSubstrateStore>,
    mut plan_queue: ResMut<SubstratePlannedUpdateQueue>,
) {
    if !guard_runtime_state_domain_with_diagnostics(authority_contract.as_ref(), authority_diagnostics.as_deref_mut(), USF_DOMAIN_SUBSTRATE) {
        return;
    }

    let has_removed_chunks = removed_chunks.read().next().is_some();
    let has_removed_models = removed_models.read().next().is_some();
    let has_dirty_chunks = !dirty_chunks.is_empty();
    let has_dirty_models = !dirty_models.is_empty();
    let full_rebuild_requested = !runtime_state.initialized || active_modpack.is_changed() || zlm_registry.is_changed() || has_removed_models;
    let has_pending_rebuild_chunks = !runtime_state.pending_rebuild_chunks.is_empty();
    let should_rebuild = full_rebuild_requested || has_dirty_chunks || has_dirty_models || has_removed_chunks || has_pending_rebuild_chunks;
    if !should_rebuild {
        return;
    }
    plan_queue.clear();
    let started_at = Instant::now();

    let mut live_chunk_coords = HashSet::<GridVec>::new();
    for chunk in loaded_chunks.iter() {
        let mut canonical_coord = chunk.coord.clone();
        canonical_coord.normalize();
        live_chunk_coords.insert(canonical_coord);
    }
    runtime_state.pending_rebuild_chunks.retain(|coord| live_chunk_coords.contains(coord));

    if full_rebuild_requested {
        runtime_state.pending_rebuild_chunks = live_chunk_coords.clone();
        runtime_state.initialized = true;
    } else {
        let mut affected = HashSet::<GridVec>::new();
        for dirty_chunk in dirty_chunks.iter() {
            let mut canonical_coord = dirty_chunk.coord.clone();
            canonical_coord.normalize();
            if live_chunk_coords.contains(&canonical_coord) {
                affected.insert(canonical_coord);
            }
        }

        for (model, support, partial) in dirty_models.iter() {
            collect_dirty_model_affected_chunks(model, support, partial, &live_chunk_coords, &mut affected);
        }
        expand_affected_chunk_descendants(&mut affected, &live_chunk_coords);
        runtime_state.pending_rebuild_chunks.extend(affected);
    }

    let frame_budget = rebuild_settings.max_chunks_per_frame.max(1);
    let queued_before = runtime_state.pending_rebuild_chunks.len();
    let rebuild_targets_sorted = drain_rebuild_batch(&mut runtime_state.pending_rebuild_chunks, frame_budget);
    let rebuild_chunk_count = rebuild_targets_sorted.len();
    let should_prune_store = has_removed_chunks || has_removed_models || full_rebuild_requested;

    let mut planned_summaries = HashMap::<GridVec, SubstrateChunkSummary>::new();
    let mut planned_chunks = Vec::<AdaptiveChunkSubstrate>::with_capacity(rebuild_targets_sorted.len());
    if !rebuild_targets_sorted.is_empty() {
        let runtime_model_projection_index = build_runtime_model_projection_index(&model_query);
        let partition_projection_index = build_partition_projection_index_runtime(&active_modpack, &model_query);
        for canonical_coord in rebuild_targets_sorted {
            let Some(schema) = active_modpack.schema_for_scale(canonical_coord.scale) else {
                continue;
            };
            let chunk_substrate = build_chunk_substrate_runtime(
                &canonical_coord,
                schema,
                &active_modpack,
                &zlm_registry,
                *transition_policy,
                &runtime_model_projection_index,
                &partition_projection_index,
                *partition_coupling_policy,
                &substrate_store,
                &planned_summaries,
            );
            planned_summaries.insert(chunk_substrate.chunk_coord.clone(), chunk_substrate.summary.clone());
            planned_chunks.push(chunk_substrate);
        }
    }
    plan_queue.live_chunk_coords = live_chunk_coords.clone();
    plan_queue.prune_to_live = should_prune_store;
    plan_queue.planned_chunks = planned_chunks;
    let queued_after = runtime_state.pending_rebuild_chunks.len();
    runtime_state.probe.observe(
        full_rebuild_requested,
        has_dirty_chunks,
        has_dirty_models,
        has_removed_chunks,
        has_removed_models,
        rebuild_chunk_count,
        live_chunk_coords.len(),
        queued_before,
        queued_after,
        frame_budget,
        started_at.elapsed(),
    );
}

pub(super) fn apply_planned_chunk_substrates_system(
    mut substrate_store: ResMut<AdaptiveSubstrateStore>,
    mut plan_queue: ResMut<SubstratePlannedUpdateQueue>,
    mut delta_state: ResMut<SubstrateChunkDeltaState>,
) {
    delta_state.clear_frame();
    if !plan_queue.prune_to_live && plan_queue.planned_chunks.is_empty() {
        return;
    }

    if plan_queue.prune_to_live {
        let removed = substrate_store
            .chunks
            .keys()
            .filter(|coord| !plan_queue.live_chunk_coords.contains(*coord))
            .cloned()
            .collect::<Vec<_>>();
        substrate_store
            .chunks
            .retain(|coord, _| plan_queue.live_chunk_coords.contains(coord));
        for coord in removed {
            delta_state.mark_removed(coord);
        }
    }
    for planned_chunk in plan_queue.planned_chunks.drain(..) {
        delta_state.mark_changed(planned_chunk.chunk_coord.clone());
        substrate_store.upsert_chunk(planned_chunk);
    }
    let removed_after_apply = substrate_store
        .chunks
        .keys()
        .filter(|coord| !plan_queue.live_chunk_coords.contains(*coord))
        .cloned()
        .collect::<Vec<_>>();
    substrate_store
        .chunks
        .retain(|coord, _| plan_queue.live_chunk_coords.contains(coord));
    for coord in removed_after_apply {
        delta_state.mark_removed(coord);
    }
    delta_state.bump_revision_if_dirty();
    plan_queue.clear();
}

fn collect_dirty_model_affected_chunks(
    model: &PhenomenonModel,
    support: Option<&PhenomenonModelSupport>,
    partial: Option<&PartialPhenomenonModel>,
    live_chunk_coords: &HashSet<GridVec>,
    affected_chunks: &mut HashSet<GridVec>,
) {
    if let Some(partial) = partial {
        let mut canonical_chunk = partial.chunk_coord.clone();
        canonical_chunk.normalize();
        if live_chunk_coords.contains(&canonical_chunk) {
            affected_chunks.insert(canonical_chunk);
        }
        return;
    }

    if let Some(support) = support {
        let mut anchor = support.support.anchor_chunk.clone();
        anchor.normalize();
        let radius = support.support.chunk_radius as u32;
        for mut candidate in anchor.query_grid_radius(radius) {
            candidate.normalize();
            if candidate.scale != model.scale {
                continue;
            }
            if live_chunk_coords.contains(&candidate) {
                affected_chunks.insert(candidate);
            }
        }
        return;
    }

    for live_chunk in live_chunk_coords {
        if live_chunk.scale == model.scale {
            affected_chunks.insert(live_chunk.clone());
        }
    }
}

fn expand_affected_chunk_descendants(affected_chunks: &mut HashSet<GridVec>, live_chunk_coords: &HashSet<GridVec>) {
    if affected_chunks.is_empty() {
        return;
    }

    let Some(max_live_scale_index) = live_chunk_coords.iter().map(|coord| coord.scale.index_from_top()).max() else {
        return;
    };
    let mut seeds = affected_chunks
        .iter()
        .cloned()
        .map(|mut seed| {
            seed.normalize();
            seed
        })
        .filter(|seed| seed.scale.index_from_top() < max_live_scale_index)
        .collect::<Vec<_>>();
    if seeds.is_empty() {
        return;
    }
    seeds = compress_descendant_seeds(seeds);
    if seeds.is_empty() {
        return;
    }

    let descendant_additions = live_chunk_coords
        .iter()
        .filter(|candidate| {
            let candidate_scale_index = candidate.scale.index_from_top();
            seeds
                .iter()
                .any(|seed| seed.scale.index_from_top() < candidate_scale_index && is_chunk_ancestor(seed, candidate))
        })
        .cloned()
        .collect::<Vec<_>>();
    affected_chunks.extend(descendant_additions);
}

fn compress_descendant_seeds(mut seeds: Vec<GridVec>) -> Vec<GridVec> {
    if seeds.len() <= 1 {
        return seeds;
    }
    seeds.sort_by_key(substrate_coord_sort_key);
    let mut minimal = Vec::<GridVec>::new();
    for seed in seeds {
        if minimal.iter().any(|ancestor| is_chunk_ancestor(ancestor, &seed)) {
            continue;
        }
        minimal.push(seed);
    }
    minimal
}

fn is_chunk_ancestor(ancestor: &GridVec, candidate: &GridVec) -> bool {
    let mut ancestor_canonical = ancestor.clone();
    ancestor_canonical.normalize();
    let mut candidate_canonical = candidate.clone();
    candidate_canonical.normalize();

    if ancestor_canonical.scale.index_from_top() > candidate_canonical.scale.index_from_top() {
        return false;
    }

    let ancestor_digits = ancestor_canonical.to_raw_vec_3d();
    let candidate_digits = candidate_canonical.to_raw_vec_3d();
    if ancestor_digits.len() > candidate_digits.len() {
        return false;
    }
    ancestor_digits
        .iter()
        .zip(candidate_digits.iter())
        .all(|(ancestor_digit, candidate_digit)| ancestor_digit == candidate_digit)
}

fn substrate_coord_sort_key(coord: &GridVec) -> (u8, Vec<(i32, i32, i32)>) {
    let mut canonical = coord.clone();
    canonical.normalize();
    let digits = canonical.to_raw_vec_3d().into_iter().map(|xyz| (xyz.x, xyz.y, xyz.z)).collect::<Vec<_>>();
    (canonical.scale.index_from_top(), digits)
}

fn drain_rebuild_batch(pending_rebuild_chunks: &mut HashSet<GridVec>, frame_budget: usize) -> Vec<GridVec> {
    if pending_rebuild_chunks.is_empty() {
        return Vec::new();
    }
    let mut sorted = pending_rebuild_chunks.iter().cloned().collect::<Vec<_>>();
    sorted.sort_by_key(substrate_coord_sort_key);
    let budget = frame_budget.max(1);
    let drained = sorted.into_iter().take(budget).collect::<Vec<_>>();
    for coord in &drained {
        pending_rebuild_chunks.remove(coord);
    }
    drained
}

pub fn rebuild_substrate_from_persisted_models(
    chunk_coord: &GridVec,
    schema: &MetricContainerLayout,
    fallback_zone: ZoneTypeId,
    model_records: &[PersistedPhenomenonModelRecord],
    partial_records: &[PersistedPartialPhenomenonModelRecord],
) -> AdaptiveChunkSubstrate {
    let transition_policy = SubstrateTransitionPolicy::default();
    let partition_coupling_policy = SubstratePartitionCouplingPolicy::default();
    let mut canonical = chunk_coord.clone();
    canonical.normalize();
    let canonical_persisted_coord = crate::usf::phenomenon::persistence::PersistedGridCoord::from_grid(&canonical);
    let partition_projection_index = build_partition_projection_index_persisted(schema, model_records, partial_records);

    let mut metric_vector = vec![0.5; schema.metrics.len()];
    let mut contributions = Vec::<ModelProjectionContribution>::new();
    for model in model_records {
        let topology = topology_from_record_tag(model.topology.as_str());
        let model_metric_index = metric_index_for_projection_metric(schema, model.projection_metric_name.as_str()).unwrap_or_default();
        let model_support_anchor = match model.support_anchor_chunk.to_grid() {
            Ok(coord) => coord,
            Err(_) => continue,
        };
        let support = crate::usf::phenomenon::PhenomenonModelSupportBounds {
            anchor_chunk: model_support_anchor,
            chunk_radius: model.support_chunk_radius,
        };
        if !support.contains_chunk(&canonical) {
            continue;
        }

        let partial_match = if topology == PhenomenonModelTopology::PartitionedByChunk {
            select_matching_partial_record(partial_records, model, &canonical_persisted_coord)
        } else {
            None
        };
        if topology == PhenomenonModelTopology::PartitionedByChunk && partial_match.is_none() {
            continue;
        }
        let scalar_channels = partial_match
            .map(|partial| partial.scalar_channels.as_slice())
            .unwrap_or_else(|| model.scalar_channels.as_slice());
        let value = scalar_channels
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(model.projection_metric_name.as_str()))
            .or_else(|| scalar_channels.first())
            .map(|(_, value)| *value)
            .unwrap_or(0.5);
        let jitter = deterministic_jitter(
            &canonical,
            mix64(hash_string(model.model_id.as_str()) ^ hash_string(model.projection_metric_name.as_str())),
        );
        let projected = (value * model.projection_gain + model.projection_bias + jitter).clamp(0.0, 1.0);
        contributions.push(ModelProjectionContribution {
            phenomenon_id: PhenomenonId(model.phenomenon_id),
            model_id: model.model_id.clone(),
            topology,
            partition_key: partial_match.map(|partial| partial.partition_key),
            metric_index: model_metric_index,
            value: projected,
        });
    }

    if contributions.is_empty() && canonical.scale == Scale::MAX {
        metric_vector = deterministic_metric_vector(
            &MetricContainerChunkKey {
                scale: canonical.scale,
                coord: canonical.clone(),
            },
            schema,
        );
    }

    apply_projection_contributions_to_metric_vector(&mut metric_vector, &contributions);
    sort_projection_contributions(&mut contributions);

    let refinement_state = compute_refinement_state(&metric_vector, &contributions, None);
    let projection_signature = projection_signature(&metric_vector, &contributions);
    let leaf = choose_leaf_representation(&canonical, &metric_vector, &contributions, refinement_state, transition_policy);
    let transition_decision = derive_transition_decision_from_state(refinement_state, &leaf, transition_policy);
    let octree = build_octree_from_leaf(
        refinement_state,
        &leaf,
        &metric_vector,
        &contributions,
        transition_decision,
        canonical.scale.index_from_top(),
    );
    let dominant_leaf_kind = leaf_kind_tag(&leaf).to_string();
    let zone_type = fallback_zone;
    let edge_interfaces = build_chunk_edge_interfaces(
        &canonical,
        &metric_vector,
        projection_signature,
        &contributions,
        Some(&partition_projection_index),
        partition_coupling_policy,
    );

    AdaptiveChunkSubstrate {
        chunk_coord: canonical.clone(),
        octree,
        summary: SubstrateChunkSummary {
            scale: canonical.scale,
            zone_type,
            metric_vector,
            projection_signature,
            dominant_leaf_kind,
            edge_interfaces,
        },
    }
}

fn build_chunk_substrate_runtime(
    canonical_coord: &GridVec,
    schema: &MetricContainerLayout,
    active_modpack: &UsfActiveModPack,
    zlm_registry: &ZlmRegistry,
    transition_policy: SubstrateTransitionPolicy,
    runtime_model_projection_index: &RuntimeModelProjectionIndex,
    partition_projection_index: &PartitionProjectionIndex,
    partition_coupling_policy: SubstratePartitionCouplingPolicy,
    substrate_store: &AdaptiveSubstrateStore,
    planned_summaries: &HashMap<GridVec, SubstrateChunkSummary>,
) -> AdaptiveChunkSubstrate {
    let mut metric_vector = parent_seed_metrics(canonical_coord, schema, substrate_store, planned_summaries).unwrap_or_else(|| vec![0.5; schema.metrics.len()]);
    if metric_vector.len() != schema.metrics.len() {
        metric_vector.resize(schema.metrics.len(), 0.5);
    }

    let mut contributions = Vec::<ModelProjectionContribution>::new();
    if let Some(snapshots) = runtime_model_projection_index.snapshots_for_scale(canonical_coord.scale) {
        for snapshot in snapshots {
            if snapshot.topology == PhenomenonModelTopology::PartitionedByChunk
                && snapshot.partial_chunk.as_ref().is_none_or(|chunk| chunk != canonical_coord)
            {
                continue;
            }
            if snapshot
                .support
                .as_ref()
                .map(|support| support.contains_chunk(canonical_coord))
                .is_some_and(|supports| !supports)
            {
                continue;
            }

            let metric_index =
                metric_index_for_projection_metric(schema, snapshot.projection_metric_name.as_str()).unwrap_or_default();
            let projected_value = projection_value_for_snapshot(canonical_coord, snapshot);
            contributions.push(ModelProjectionContribution {
                phenomenon_id: snapshot.phenomenon_id,
                model_id: snapshot.model_id.clone(),
                topology: snapshot.topology,
                partition_key: snapshot.partition_key,
                metric_index,
                value: projected_value,
            });
        }
    }

    if contributions.is_empty() && canonical_coord.scale == Scale::MAX {
        metric_vector = deterministic_metric_vector(
            &MetricContainerChunkKey {
                scale: canonical_coord.scale,
                coord: canonical_coord.clone(),
            },
            schema,
        );
    }

    apply_projection_contributions_to_metric_vector(&mut metric_vector, &contributions);
    let parent_metrics = canonical_coord
        .parent
        .as_ref()
        .and_then(|parent| summary_for_chunk_with_overlay(parent.as_ref(), planned_summaries, substrate_store))
        .map(|summary| summary.metric_vector.as_slice());

    sort_projection_contributions(&mut contributions);

    let refinement_state = compute_refinement_state(&metric_vector, &contributions, parent_metrics);
    let projection_signature = projection_signature(&metric_vector, &contributions);
    let leaf = choose_leaf_representation(canonical_coord, &metric_vector, &contributions, refinement_state, transition_policy);
    let transition_decision = derive_transition_decision_from_state(refinement_state, &leaf, transition_policy);
    let octree = build_octree_from_leaf(
        refinement_state,
        &leaf,
        &metric_vector,
        &contributions,
        transition_decision,
        canonical_coord.scale.index_from_top(),
    );
    let dominant_leaf_kind = leaf_kind_tag(&leaf).to_string();
    let zone_type = zlm_registry.classify_for_scale(canonical_coord.scale, schema, &metric_vector, active_modpack);
    let edge_interfaces = build_chunk_edge_interfaces(
        canonical_coord,
        &metric_vector,
        projection_signature,
        &contributions,
        Some(partition_projection_index),
        partition_coupling_policy,
    );

    AdaptiveChunkSubstrate {
        chunk_coord: canonical_coord.clone(),
        octree,
        summary: SubstrateChunkSummary {
            scale: canonical_coord.scale,
            zone_type,
            metric_vector,
            projection_signature,
            dominant_leaf_kind,
            edge_interfaces,
        },
    }
}

fn parent_seed_metrics(
    canonical_coord: &GridVec,
    schema: &MetricContainerLayout,
    substrate_store: &AdaptiveSubstrateStore,
    planned_summaries: &HashMap<GridVec, SubstrateChunkSummary>,
) -> Option<Vec<f32>> {
    let parent_summary = canonical_coord
        .parent
        .as_ref()
        .and_then(|parent| summary_for_chunk_with_overlay(parent.as_ref(), planned_summaries, substrate_store))?;

    let mut metrics = vec![0.5; schema.metrics.len()];
    for (index, value) in parent_summary.metric_vector.iter().copied().enumerate() {
        let Some(slot) = metrics.get_mut(index) else {
            break;
        };
        *slot = ((value * 0.85) + 0.075).clamp(0.0, 1.0);
    }
    Some(metrics)
}

fn summary_for_chunk_with_overlay<'a>(
    chunk_coord: &GridVec,
    planned_summaries: &'a HashMap<GridVec, SubstrateChunkSummary>,
    substrate_store: &'a AdaptiveSubstrateStore,
) -> Option<&'a SubstrateChunkSummary> {
    planned_summaries.get(chunk_coord).or_else(|| substrate_store.summary_for_chunk(chunk_coord))
}

fn build_runtime_model_projection_index(
    model_query: &Query<(
        &PhenomenonModel,
        &PhenomenonModelScriptDefinitionRef,
        &PhenomenonModelProjection,
        Option<&PhenomenonModelSupport>,
        Option<&PhenomenonModelState>,
        Option<&PartialPhenomenonModel>,
    )>,
) -> RuntimeModelProjectionIndex {
    let mut index = RuntimeModelProjectionIndex::default();
    for (model, model_script_ref, projection, support, model_state, partial) in model_query.iter() {
        let mut partial_chunk = partial.map(|partial| partial.chunk_coord.clone());
        if let Some(chunk) = partial_chunk.as_mut() {
            chunk.normalize();
        }
        let scalar_channels = model_state
            .map(|state| state.scalar_channels.clone())
            .unwrap_or_default();

        index.insert(
            model.scale,
            RuntimeModelProjectionSnapshot {
                phenomenon_id: model.phenomenon_id,
                topology: model.topology,
                model_id: model_script_ref.model_id.clone(),
                support: support.map(|value| value.support.clone()),
                partial_chunk,
                partition_key: partial.map(|value| value.partition_key),
                projection_metric_name: projection.spec.metric_name.clone(),
                projection_bias: projection.spec.projection_bias,
                projection_gain: projection.spec.projection_gain,
                scalar_channels,
            },
        );
    }
    for snapshots in index.by_scale.values_mut() {
        snapshots.sort_by(|a, b| {
            a.phenomenon_id
                .0
                .cmp(&b.phenomenon_id.0)
                .then_with(|| a.model_id.cmp(&b.model_id))
                .then_with(|| a.partition_key.unwrap_or(0).cmp(&b.partition_key.unwrap_or(0)))
                .then_with(|| a.projection_metric_name.cmp(&b.projection_metric_name))
        });
    }
    index
}

fn projection_value_for_model(
    canonical_coord: &GridVec,
    model_script_ref: &PhenomenonModelScriptDefinitionRef,
    projection: &PhenomenonModelProjection,
    state: &PhenomenonModelState,
) -> f32 {
    let channel_value = state
        .scalar_channels
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(projection.spec.metric_name.as_str()))
        .or_else(|| state.scalar_channels.first())
        .map(|(_, value)| *value)
        .unwrap_or(0.5);
    let jitter = deterministic_jitter(
        canonical_coord,
        mix64(hash_string(model_script_ref.model_id.as_str()) ^ hash_string(projection.spec.metric_name.as_str())),
    );
    (channel_value * projection.spec.projection_gain + projection.spec.projection_bias + jitter).clamp(0.0, 1.0)
}

fn projection_value_for_snapshot(canonical_coord: &GridVec, snapshot: &RuntimeModelProjectionSnapshot) -> f32 {
    let channel_value = snapshot
        .scalar_channels
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(snapshot.projection_metric_name.as_str()))
        .or_else(|| snapshot.scalar_channels.first())
        .map(|(_, value)| *value)
        .unwrap_or(0.5);
    let jitter = deterministic_jitter(
        canonical_coord,
        mix64(hash_string(snapshot.model_id.as_str()) ^ hash_string(snapshot.projection_metric_name.as_str())),
    );
    (channel_value * snapshot.projection_gain + snapshot.projection_bias + jitter).clamp(0.0, 1.0)
}

fn metric_index_for_projection_metric(schema: &MetricContainerLayout, metric_name: &str) -> Option<usize> {
    let normalized = metric_name.trim().to_ascii_lowercase();
    schema
        .metrics
        .iter()
        .position(|metric| metric.name.trim().eq_ignore_ascii_case(normalized.as_str()))
}

fn sort_projection_contributions(contributions: &mut Vec<ModelProjectionContribution>) {
    contributions.sort_by(|a, b| b.value.total_cmp(&a.value).then_with(|| compare_projection_identity(a, b)));
}

fn compare_projection_identity(a: &ModelProjectionContribution, b: &ModelProjectionContribution) -> Ordering {
    a.phenomenon_id
        .0
        .cmp(&b.phenomenon_id.0)
        .then_with(|| a.model_id.cmp(&b.model_id))
        .then_with(|| a.metric_index.cmp(&b.metric_index))
        .then_with(|| a.partition_key.unwrap_or(0).cmp(&b.partition_key.unwrap_or(0)))
}

fn apply_projection_contributions_to_metric_vector(metric_vector: &mut [f32], contributions: &[ModelProjectionContribution]) {
    if metric_vector.is_empty() || contributions.is_empty() {
        return;
    }
    let mut totals = vec![0.0_f32; metric_vector.len()];
    for contribution in contributions {
        let Some(total) = totals.get_mut(contribution.metric_index) else {
            continue;
        };
        *total += contribution.value;
    }
    for (index, total) in totals.into_iter().enumerate() {
        if total == 0.0 {
            continue;
        }
        metric_vector[index] = (metric_vector[index] + total).clamp(0.0, 1.0);
    }
}

fn select_matching_partial_record<'a>(
    partial_records: &'a [PersistedPartialPhenomenonModelRecord],
    model: &PersistedPhenomenonModelRecord,
    canonical_persisted_coord: &crate::usf::phenomenon::persistence::PersistedGridCoord,
) -> Option<&'a PersistedPartialPhenomenonModelRecord> {
    partial_records
        .iter()
        .filter(|partial| {
            partial.phenomenon_id == model.phenomenon_id
                && partial.model_id.eq_ignore_ascii_case(model.model_id.as_str())
                && partial.scale_index == model.scale_index
                && partial.chunk_coord == *canonical_persisted_coord
        })
        .min_by(|left, right| {
            left.partition_key
                .cmp(&right.partition_key)
                .then_with(|| left.scalar_channels.len().cmp(&right.scalar_channels.len()))
                .then_with(|| left.model_id.cmp(&right.model_id))
        })
}

fn build_chunk_edge_interfaces(
    canonical_coord: &GridVec,
    metric_vector: &[f32],
    projection_signature: u64,
    contributions: &[ModelProjectionContribution],
    partition_projection_index: Option<&PartitionProjectionIndex>,
    partition_coupling_policy: SubstratePartitionCouplingPolicy,
) -> Vec<ChunkEdgeInterface> {
    substrate_edge_definitions()
        .iter()
        .map(|(edge, offset)| {
            let mut neighbor_coord = canonical_coord.clone() + *offset;
            neighbor_coord.normalize();
            let uncoupled_signature = mix64(projection_signature ^ edge_tag(*edge) ^ 0x47a1_58f2_1cb4_9d7e);

            let (projected_metric_vector, coupling_signature) = partition_projection_index
                .and_then(|index| {
                    partition_coupled_edge_projection(
                        *edge,
                        canonical_coord,
                        &neighbor_coord,
                        metric_vector,
                        projection_signature,
                        contributions,
                        index,
                        partition_coupling_policy,
                    )
                })
                .unwrap_or_else(|| (uncoupled_edge_projection(metric_vector), uncoupled_signature));
            ChunkEdgeInterface {
                edge: *edge,
                projected_metric_vector,
                coupling_signature,
            }
        })
        .collect()
}

fn substrate_edge_definitions() -> [(SubstrateChunkEdge, IVec3); 6] {
    [
        (SubstrateChunkEdge::PosX, IVec3::new(1, 0, 0)),
        (SubstrateChunkEdge::NegX, IVec3::new(-1, 0, 0)),
        (SubstrateChunkEdge::PosY, IVec3::new(0, 1, 0)),
        (SubstrateChunkEdge::NegY, IVec3::new(0, -1, 0)),
        (SubstrateChunkEdge::PosZ, IVec3::new(0, 0, 1)),
        (SubstrateChunkEdge::NegZ, IVec3::new(0, 0, -1)),
    ]
}

fn uncoupled_edge_projection(metric_vector: &[f32]) -> Vec<f32> {
    metric_vector.to_vec()
}

fn partition_coupled_edge_projection(
    edge: SubstrateChunkEdge,
    canonical_coord: &GridVec,
    neighbor_coord: &GridVec,
    metric_vector: &[f32],
    projection_signature: u64,
    contributions: &[ModelProjectionContribution],
    partition_projection_index: &PartitionProjectionIndex,
    partition_coupling_policy: SubstratePartitionCouplingPolicy,
) -> Option<(Vec<f32>, u64)> {
    let neighbor_samples = partition_projection_index.samples_for_chunk(neighbor_coord)?;
    if neighbor_samples.is_empty() {
        return None;
    }

    let mut projected_metric_vector = metric_vector.to_vec();
    let mut coupling_signature =
        mix64(projection_signature ^ edge_tag(edge) ^ hash_grid_coord(canonical_coord) ^ hash_grid_coord(neighbor_coord) ^ 0x6e90_3f2a_7c94_9be1);
    let mut matched_pair_count = 0usize;

    let total_weight = (partition_coupling_policy.existing_metric_weight + partition_coupling_policy.neighbor_metric_weight).max(1e-6);
    let existing_metric_weight = partition_coupling_policy.existing_metric_weight / total_weight;
    let neighbor_metric_weight = partition_coupling_policy.neighbor_metric_weight / total_weight;

    for current in contributions
        .iter()
        .filter(|entry| entry.topology == PhenomenonModelTopology::PartitionedByChunk && entry.partition_key.is_some())
    {
        let Some(current_partition_key) = current.partition_key else {
            continue;
        };
        let Some(neighbor) = neighbor_samples.iter().find(|sample| {
            sample.scale == canonical_coord.scale
                && sample.metric_index == current.metric_index
                && sample.phenomenon_id == current.phenomenon_id
                && sample.model_id.eq_ignore_ascii_case(current.model_id.as_str())
        }) else {
            continue;
        };

        let blended = ((current.value + neighbor.value) * 0.5).clamp(0.0, 1.0);
        if let Some(slot) = projected_metric_vector.get_mut(current.metric_index) {
            *slot = ((*slot * existing_metric_weight) + (blended * neighbor_metric_weight)).clamp(0.0, 1.0);
        }

        coupling_signature = mix64(coupling_signature ^ current.phenomenon_id.0);
        coupling_signature = mix64(coupling_signature ^ hash_string(current.model_id.as_str()));
        coupling_signature = mix64(coupling_signature ^ current_partition_key);
        coupling_signature = mix64(coupling_signature ^ neighbor.partition_key);
        coupling_signature = mix64(coupling_signature ^ current.value.to_bits() as u64);
        coupling_signature = mix64(coupling_signature ^ neighbor.value.to_bits() as u64);
        matched_pair_count += 1;
    }

    if matched_pair_count == 0 {
        return None;
    }

    Some((projected_metric_vector, coupling_signature))
}

fn edge_tag(edge: SubstrateChunkEdge) -> u64 {
    match edge {
        SubstrateChunkEdge::PosX => 0x31d2_6d59_51f7_a117,
        SubstrateChunkEdge::NegX => 0x4a7b_3c2f_8b0d_1191,
        SubstrateChunkEdge::PosY => 0x1f90_8261_2bb9_cc17,
        SubstrateChunkEdge::NegY => 0x9ce4_7084_8f3a_2105,
        SubstrateChunkEdge::PosZ => 0x0ad3_4b6c_f1a2_733b,
        SubstrateChunkEdge::NegZ => 0xe3c5_91d7_147e_52d1,
    }
}

fn topology_from_record_tag(raw: &str) -> PhenomenonModelTopology {
    let normalized = raw.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "partitioned_by_chunk" | "partitioned-by-chunk" | "partitioned" => PhenomenonModelTopology::PartitionedByChunk,
        _ => PhenomenonModelTopology::MonolithicChunk,
    }
}

fn build_partition_projection_index_runtime(
    active_modpack: &UsfActiveModPack,
    model_query: &Query<(
        &PhenomenonModel,
        &PhenomenonModelScriptDefinitionRef,
        &PhenomenonModelProjection,
        Option<&PhenomenonModelSupport>,
        Option<&PhenomenonModelState>,
        Option<&PartialPhenomenonModel>,
    )>,
) -> PartitionProjectionIndex {
    let mut index = PartitionProjectionIndex::default();
    for (model, model_script_ref, projection, support, model_state, partial) in model_query.iter() {
        if model.topology != PhenomenonModelTopology::PartitionedByChunk {
            continue;
        }
        let Some(partial) = partial else {
            continue;
        };
        let Some(schema) = active_modpack.schema_for_scale(model.scale) else {
            continue;
        };

        let mut canonical_chunk = partial.chunk_coord.clone();
        canonical_chunk.normalize();
        if support
            .map(|support| support.support.contains_chunk(&canonical_chunk))
            .is_some_and(|supports| !supports)
        {
            continue;
        }

        let metric_index = metric_index_for_projection_metric(schema, projection.spec.metric_name.as_str()).unwrap_or_default();
        let fallback_state = PhenomenonModelState::default();
        let state = model_state.unwrap_or(&fallback_state);
        let value = projection_value_for_model(&canonical_chunk, model_script_ref, projection, state);
        index.insert(PartitionProjectionSample {
            phenomenon_id: model.phenomenon_id,
            model_id: model_script_ref.model_id.clone(),
            scale: model.scale,
            chunk_coord: canonical_chunk,
            partition_key: partial.partition_key,
            metric_index,
            value,
        });
    }
    for samples in index.by_chunk.values_mut() {
        sort_partition_projection_samples(samples);
    }
    index
}

fn build_partition_projection_index_persisted(
    schema: &MetricContainerLayout,
    model_records: &[PersistedPhenomenonModelRecord],
    partial_records: &[PersistedPartialPhenomenonModelRecord],
) -> PartitionProjectionIndex {
    let mut index = PartitionProjectionIndex::default();
    for model in model_records {
        if topology_from_record_tag(model.topology.as_str()) != PhenomenonModelTopology::PartitionedByChunk {
            continue;
        }
        let Some(scale) = Scale::from_index_from_top(model.scale_index) else {
            continue;
        };
        let metric_index = metric_index_for_projection_metric(schema, model.projection_metric_name.as_str()).unwrap_or_default();
        let model_support_anchor = match model.support_anchor_chunk.to_grid() {
            Ok(coord) => coord,
            Err(_) => continue,
        };
        let support = crate::usf::phenomenon::PhenomenonModelSupportBounds {
            anchor_chunk: model_support_anchor,
            chunk_radius: model.support_chunk_radius,
        };

        for partial in partial_records.iter().filter(|partial| {
            partial.phenomenon_id == model.phenomenon_id
                && partial.scale_index == model.scale_index
                && partial.model_id.eq_ignore_ascii_case(model.model_id.as_str())
        }) {
            let Ok(mut chunk_coord) = partial.chunk_coord.to_grid() else {
                continue;
            };
            chunk_coord.normalize();
            if !support.contains_chunk(&chunk_coord) {
                continue;
            }

            let scalar_channels = if partial.scalar_channels.is_empty() {
                model.scalar_channels.as_slice()
            } else {
                partial.scalar_channels.as_slice()
            };
            let value = scalar_channels
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(model.projection_metric_name.as_str()))
                .or_else(|| scalar_channels.first())
                .map(|(_, value)| *value)
                .unwrap_or(0.5);
            let jitter = deterministic_jitter(
                &chunk_coord,
                mix64(hash_string(model.model_id.as_str()) ^ hash_string(model.projection_metric_name.as_str())),
            );
            let projected = (value * model.projection_gain + model.projection_bias + jitter).clamp(0.0, 1.0);
            index.insert(PartitionProjectionSample {
                phenomenon_id: PhenomenonId(model.phenomenon_id),
                model_id: model.model_id.clone(),
                scale,
                chunk_coord,
                partition_key: partial.partition_key,
                metric_index,
                value: projected,
            });
        }
    }
    for samples in index.by_chunk.values_mut() {
        sort_partition_projection_samples(samples);
    }
    index
}

fn sort_partition_projection_samples(samples: &mut [PartitionProjectionSample]) {
    samples.sort_by(|a, b| {
        a.phenomenon_id
            .0
            .cmp(&b.phenomenon_id.0)
            .then_with(|| a.model_id.cmp(&b.model_id))
            .then_with(|| a.metric_index.cmp(&b.metric_index))
            .then_with(|| a.partition_key.cmp(&b.partition_key))
            .then_with(|| a.value.total_cmp(&b.value))
    });
}

fn projection_signature(metric_vector: &[f32], contributions: &[ModelProjectionContribution]) -> u64 {
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    for value in metric_vector {
        state = mix64(state ^ value.to_bits() as u64);
    }
    for contribution in contributions {
        state = mix64(state ^ contribution.phenomenon_id.0);
        state = mix64(state ^ hash_string(contribution.model_id.as_str()));
        state = mix64(state ^ contribution.metric_index as u64);
        state = mix64(state ^ contribution.value.to_bits() as u64);
        if let Some(partition_key) = contribution.partition_key {
            state = mix64(state ^ partition_key);
        }
    }
    state
}

fn deterministic_jitter(canonical_coord: &GridVec, seed: u64) -> f32 {
    let mut state = mix64(seed ^ hash_grid_coord(canonical_coord));
    state = mix64(state ^ canonical_coord.scale.index_from_top() as u64);
    (((state >> 44) as f32) / ((1_u32 << 20) as f32) - 0.5) * 0.08
}

fn hash_grid_coord(canonical_coord: &GridVec) -> u64 {
    let mut state = 0xc6a4_a793_5bd1_e995_u64;
    for xyz in canonical_coord.to_raw_vec_3d() {
        state = mix64(state ^ fold_signed(xyz.x));
        state = mix64(state ^ fold_signed(xyz.y));
        state = mix64(state ^ fold_signed(xyz.z));
    }
    state
}

fn hash_string(value: &str) -> u64 {
    let mut state = 0xcbf2_9ce4_8422_2325_u64;
    for byte in value.as_bytes() {
        state ^= *byte as u64;
        state = state.wrapping_mul(0x1000_0000_01b3);
    }
    state
}

#[inline]
fn fold_signed(value: i32) -> u64 {
    value as i64 as u64
}

#[inline]
fn mix64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

pub(crate) struct SubstratePlugin;
impl Plugin for SubstratePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AdaptiveSubstrateStore>()
            .init_resource::<AdaptiveSubstrateRuntimeState>()
            .init_resource::<SubstratePlannedUpdateQueue>()
            .init_resource::<SubstrateChunkDeltaState>()
            .init_resource::<SubstrateRebuildSettings>()
            .insert_resource(SubstratePartitionCouplingPolicy::from_config())
            .init_resource::<SubstrateTransitionPolicy>()
            .add_systems(
                Update,
                plan_chunk_substrate_rebuilds_system
                    .in_set(UsfSubstrateSet::Pre)
                    .in_set(UsfSimulationSet::Substrate),
            )
            .add_systems(
                Update,
                apply_planned_chunk_substrates_system
                    .in_set(UsfSubstrateSet::Runtime)
                    .in_set(UsfSimulationSet::Substrate),
            )
            .register_type::<SubstrateChunkEdge>()
            .register_type::<ChunkEdgeInterface>()
            .register_type::<SubstrateLeafContainer>()
            .register_type::<SubstrateRefinementState>()
            .register_type::<SubstrateRebuildSettings>()
            .register_type::<SubstratePartitionCouplingPolicy>()
            .register_type::<SubstrateTransitionPolicy>()
            .register_type::<SubstrateTransitionDecision>()
            .register_type::<AdaptiveSubstrateOctreeNode>()
            .register_type::<SubstrateChunkSummary>()
            .register_type::<AdaptiveChunkSubstrate>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::metric::{MetricDefinition, MetricId, MetricStorageClass, MetricValueType};
    use crate::usf::pos::types::GridXyz;

    fn test_schema() -> MetricContainerLayout {
        MetricContainerLayout {
            revision: 1,
            metrics: vec![MetricDefinition {
                id: MetricId(0),
                name: "demo_mass_density".to_string(),
                value_type: MetricValueType::F32,
                semantics_tag: "matter.density.normalized".to_string(),
                storage_class: MetricStorageClass::Brick,
                derived: false,
                min_scale_index: 0,
                max_scale_index: Scale::SCALE_LEVEL_COUNT.saturating_sub(1),
            }],
            fallback_zone: ZoneTypeId::new("empty"),
        }
    }

    fn test_coord() -> GridVec {
        GridVec::new(GridVec::new_root(GridXyz::new_local(0, 0, 0)), GridXyz::new_local(1, 0, 0))
    }

    fn coord_at(x: i32, y: i32, z: i32) -> GridVec {
        GridVec::new(GridVec::new_root(GridXyz::new_local(0, 0, 0)), GridXyz::new_local(x, y, z))
    }

    fn test_schema_two_metrics() -> MetricContainerLayout {
        MetricContainerLayout {
            revision: 1,
            metrics: vec![
                MetricDefinition {
                    id: MetricId(0),
                    name: "metric_a".to_string(),
                    value_type: MetricValueType::F32,
                    semantics_tag: "metric.a".to_string(),
                    storage_class: MetricStorageClass::Brick,
                    derived: false,
                    min_scale_index: 0,
                    max_scale_index: Scale::SCALE_LEVEL_COUNT.saturating_sub(1),
                },
                MetricDefinition {
                    id: MetricId(1),
                    name: "metric_b".to_string(),
                    value_type: MetricValueType::F32,
                    semantics_tag: "metric.b".to_string(),
                    storage_class: MetricStorageClass::Brick,
                    derived: false,
                    min_scale_index: 0,
                    max_scale_index: Scale::SCALE_LEVEL_COUNT.saturating_sub(1),
                },
            ],
            fallback_zone: ZoneTypeId::new("empty"),
        }
    }

    #[test]
    fn substrate_rebuild_from_persisted_models_is_deterministic() {
        let schema = test_schema();
        let coord = test_coord();
        let support_coord = super::super::phenomenon::persistence::PersistedGridCoord::from_grid(&coord);
        let model = PersistedPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 11,
            model_id: "demo_mass_density.default".to_string(),
            scale_index: coord.scale.index_from_top(),
            topology: "partitioned_by_chunk".to_string(),
            support_anchor_chunk: support_coord.clone(),
            support_chunk_radius: 2,
            projection_metric_name: "demo_mass_density".to_string(),
            projection_bias: 0.0,
            projection_gain: 1.0,
            scalar_channels: vec![("demo_mass_density".to_string(), 0.72)],
        };
        let partial = PersistedPartialPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 11,
            model_id: "demo_mass_density.default".to_string(),
            scale_index: coord.scale.index_from_top(),
            chunk_coord: support_coord,
            partition_key: 778811,
            scalar_channels: vec![("demo_mass_density".to_string(), 0.72)],
        };

        let a = rebuild_substrate_from_persisted_models(&coord, &schema, ZoneTypeId::new("empty"), &[model.clone()], &[partial.clone()]);
        let b = rebuild_substrate_from_persisted_models(&coord, &schema, ZoneTypeId::new("empty"), &[model], &[partial]);

        assert_eq!(a.summary.metric_vector, b.summary.metric_vector);
        assert_eq!(a.summary.projection_signature, b.summary.projection_signature);
        assert_eq!(a.summary.zone_type, b.summary.zone_type);
        assert_eq!(a.octree, b.octree);
    }

    #[test]
    fn substrate_rebuild_is_stable_under_model_record_order_permutation() {
        let schema = test_schema_two_metrics();
        let coord = test_coord();
        let support_coord = super::super::phenomenon::persistence::PersistedGridCoord::from_grid(&coord);
        let model_a = PersistedPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 41,
            model_id: "demo_mass_density.model_a".to_string(),
            scale_index: coord.scale.index_from_top(),
            topology: "monolithic_chunk".to_string(),
            support_anchor_chunk: support_coord.clone(),
            support_chunk_radius: 0,
            projection_metric_name: "metric_a".to_string(),
            projection_bias: 0.1,
            projection_gain: 1.0,
            scalar_channels: vec![("metric_a".to_string(), 0.3)],
        };
        let model_b = PersistedPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 42,
            model_id: "demo_mass_density.model_b".to_string(),
            scale_index: coord.scale.index_from_top(),
            topology: "monolithic_chunk".to_string(),
            support_anchor_chunk: support_coord,
            support_chunk_radius: 0,
            projection_metric_name: "metric_b".to_string(),
            projection_bias: 0.2,
            projection_gain: 1.0,
            scalar_channels: vec![("metric_b".to_string(), 0.4)],
        };

        let ordered = rebuild_substrate_from_persisted_models(
            &coord,
            &schema,
            ZoneTypeId::new("empty"),
            &[model_a.clone(), model_b.clone()],
            &[],
        );
        let reversed = rebuild_substrate_from_persisted_models(&coord, &schema, ZoneTypeId::new("empty"), &[model_b, model_a], &[]);
        assert_eq!(ordered.summary.metric_vector, reversed.summary.metric_vector);
        assert_eq!(ordered.summary.projection_signature, reversed.summary.projection_signature);
        assert_eq!(ordered.summary.edge_interfaces, reversed.summary.edge_interfaces);
    }

    #[test]
    fn persisted_partial_matching_is_scoped_by_phenomenon_id() {
        let schema = test_schema_two_metrics();
        let coord = test_coord();
        let support_coord = super::super::phenomenon::persistence::PersistedGridCoord::from_grid(&coord);
        let model_a = PersistedPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 111,
            model_id: "demo_mass_density.default".to_string(),
            scale_index: coord.scale.index_from_top(),
            topology: "partitioned_by_chunk".to_string(),
            support_anchor_chunk: support_coord.clone(),
            support_chunk_radius: 2,
            projection_metric_name: "metric_a".to_string(),
            projection_bias: 0.0,
            projection_gain: 1.0,
            scalar_channels: vec![("metric_a".to_string(), 0.05)],
        };
        let model_b = PersistedPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 222,
            model_id: "demo_mass_density.default".to_string(),
            scale_index: coord.scale.index_from_top(),
            topology: "partitioned_by_chunk".to_string(),
            support_anchor_chunk: support_coord.clone(),
            support_chunk_radius: 2,
            projection_metric_name: "metric_b".to_string(),
            projection_bias: 0.0,
            projection_gain: 1.0,
            scalar_channels: vec![("metric_b".to_string(), 0.95)],
        };
        let partial_a = PersistedPartialPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 111,
            model_id: "demo_mass_density.default".to_string(),
            scale_index: coord.scale.index_from_top(),
            chunk_coord: support_coord.clone(),
            partition_key: 0x1111,
            scalar_channels: vec![("metric_a".to_string(), 0.05)],
        };
        let partial_b = PersistedPartialPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 222,
            model_id: "demo_mass_density.default".to_string(),
            scale_index: coord.scale.index_from_top(),
            chunk_coord: support_coord,
            partition_key: 0x2222,
            scalar_channels: vec![("metric_b".to_string(), 0.95)],
        };

        let rebuilt = rebuild_substrate_from_persisted_models(&coord, &schema, ZoneTypeId::new("empty"), &[model_a, model_b], &[partial_a, partial_b]);
        let metric_a = rebuilt.summary.metric_vector.first().copied().unwrap_or_default();
        let metric_b = rebuilt.summary.metric_vector.get(1).copied().unwrap_or_default();

        assert!(
            metric_b > metric_a,
            "metric_b should remain strongly influenced by phenomenon_id=222 partial record, got metric_a={metric_a} metric_b={metric_b}"
        );
    }

    #[test]
    fn duplicate_partition_partials_select_deterministic_lowest_partition_key() {
        let schema = test_schema();
        let coord = test_coord();
        let support_coord = super::super::phenomenon::persistence::PersistedGridCoord::from_grid(&coord);
        let model = PersistedPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 99,
            model_id: "demo_mass_density.default".to_string(),
            scale_index: coord.scale.index_from_top(),
            topology: "partitioned_by_chunk".to_string(),
            support_anchor_chunk: support_coord.clone(),
            support_chunk_radius: 1,
            projection_metric_name: "demo_mass_density".to_string(),
            projection_bias: 0.0,
            projection_gain: 1.0,
            scalar_channels: vec![("demo_mass_density".to_string(), 0.1)],
        };
        let low_key = PersistedPartialPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 99,
            model_id: "demo_mass_density.default".to_string(),
            scale_index: coord.scale.index_from_top(),
            chunk_coord: support_coord.clone(),
            partition_key: 1,
            scalar_channels: vec![("demo_mass_density".to_string(), 0.2)],
        };
        let high_key = PersistedPartialPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 99,
            model_id: "demo_mass_density.default".to_string(),
            scale_index: coord.scale.index_from_top(),
            chunk_coord: support_coord,
            partition_key: 200,
            scalar_channels: vec![("demo_mass_density".to_string(), 0.9)],
        };

        let first_low = rebuild_substrate_from_persisted_models(
            &coord,
            &schema,
            ZoneTypeId::new("empty"),
            std::slice::from_ref(&model),
            &[low_key.clone(), high_key.clone()],
        );
        let first_high = rebuild_substrate_from_persisted_models(
            &coord,
            &schema,
            ZoneTypeId::new("empty"),
            std::slice::from_ref(&model),
            &[high_key, low_key],
        );

        assert_eq!(first_low.summary.metric_vector, first_high.summary.metric_vector);
        assert_eq!(first_low.summary.projection_signature, first_high.summary.projection_signature);
    }

    #[test]
    fn partition_neighbor_coupling_changes_edge_projection_when_neighbor_exists() {
        let schema = test_schema();
        let coord = test_coord();
        let mut neighbor = coord.clone() + IVec3::new(1, 0, 0);
        neighbor.normalize();

        let support_coord = super::super::phenomenon::persistence::PersistedGridCoord::from_grid(&coord);
        let neighbor_support_coord = super::super::phenomenon::persistence::PersistedGridCoord::from_grid(&neighbor);
        let model = PersistedPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 11,
            model_id: "demo_mass_density.default".to_string(),
            scale_index: coord.scale.index_from_top(),
            topology: "partitioned_by_chunk".to_string(),
            support_anchor_chunk: support_coord.clone(),
            support_chunk_radius: 2,
            projection_metric_name: "demo_mass_density".to_string(),
            projection_bias: 0.0,
            projection_gain: 1.0,
            scalar_channels: vec![("demo_mass_density".to_string(), 0.4)],
        };
        let current_partial = PersistedPartialPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 11,
            model_id: "demo_mass_density.default".to_string(),
            scale_index: coord.scale.index_from_top(),
            chunk_coord: support_coord,
            partition_key: 0xaaaa,
            scalar_channels: vec![("demo_mass_density".to_string(), 0.4)],
        };
        let neighbor_partial = PersistedPartialPhenomenonModelRecord {
            schema_version: 2,
            phenomenon_id: 11,
            model_id: "demo_mass_density.default".to_string(),
            scale_index: coord.scale.index_from_top(),
            chunk_coord: neighbor_support_coord,
            partition_key: 0xbbbb,
            scalar_channels: vec![("demo_mass_density".to_string(), 0.9)],
        };

        let with_neighbor = rebuild_substrate_from_persisted_models(
            &coord,
            &schema,
            ZoneTypeId::new("empty"),
            std::slice::from_ref(&model),
            &[current_partial.clone(), neighbor_partial],
        );
        let without_neighbor =
            rebuild_substrate_from_persisted_models(&coord, &schema, ZoneTypeId::new("empty"), std::slice::from_ref(&model), &[current_partial]);

        let with_edge = with_neighbor
            .summary
            .edge_interfaces
            .iter()
            .find(|edge| edge.edge == SubstrateChunkEdge::PosX)
            .expect("posx edge should exist");
        let without_edge = without_neighbor
            .summary
            .edge_interfaces
            .iter()
            .find(|edge| edge.edge == SubstrateChunkEdge::PosX)
            .expect("posx edge should exist");

        assert_ne!(with_edge.coupling_signature, without_edge.coupling_signature);
        assert_ne!(with_edge.projected_metric_vector, without_edge.projected_metric_vector);
    }

    #[test]
    fn drain_rebuild_batch_is_deterministic_and_respects_budget() {
        let mut pending = HashSet::<GridVec>::new();
        pending.insert(coord_at(1, 0, 0));
        pending.insert(coord_at(-1, 0, 0));
        pending.insert(coord_at(0, 0, 0));

        let drained = drain_rebuild_batch(&mut pending, 2);
        assert_eq!(drained.len(), 2);
        let drained_tail_x = drained
            .iter()
            .map(|coord| coord.to_raw_vec_3d().last().map(|xyz| xyz.x).unwrap_or_default())
            .collect::<Vec<_>>();
        assert_eq!(drained_tail_x, vec![-1, 0]);
        assert_eq!(pending.len(), 1);
    }

    #[test]
    fn drain_rebuild_batch_zero_budget_still_drains_one_chunk() {
        let mut pending = HashSet::<GridVec>::new();
        pending.insert(coord_at(4, 0, 0));
        pending.insert(coord_at(3, 0, 0));

        let drained = drain_rebuild_batch(&mut pending, 0);
        assert_eq!(drained.len(), 1);
        assert_eq!(pending.len(), 1);
    }

    #[test]
    fn compress_descendant_seeds_keeps_minimal_ancestors_only() {
        let root = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let child = GridVec::new(root.clone(), GridXyz::new_local(1, 0, 0));
        let peer_root = GridVec::new_root(GridXyz::new_local(3, 0, 0));

        let compressed = compress_descendant_seeds(vec![child.clone(), root.clone(), peer_root.clone()]);
        assert_eq!(compressed.len(), 2);
        assert!(compressed.iter().any(|seed| seed == &root));
        assert!(compressed.iter().any(|seed| seed == &peer_root));
        assert!(!compressed.iter().any(|seed| seed == &child));
    }

    #[test]
    fn expand_affected_chunk_descendants_adds_live_descendants_from_coarse_seed() {
        let coarse_root = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let fine_child = GridVec::new(coarse_root.clone(), GridXyz::new_local(1, 0, 0));
        let finer_grandchild = GridVec::new(fine_child.clone(), GridXyz::new_local(0, 1, 0));

        let mut affected = HashSet::<GridVec>::new();
        affected.insert(coarse_root.clone());
        let live = HashSet::<GridVec>::from_iter([coarse_root.clone(), fine_child, finer_grandchild.clone()]);
        expand_affected_chunk_descendants(&mut affected, &live);

        assert!(affected.contains(&finer_grandchild));
    }

    #[test]
    fn expand_affected_chunk_descendants_is_noop_when_live_chunks_have_no_deeper_scales() {
        let a = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let b = GridVec::new_root(GridXyz::new_local(2, 0, 0));
        let mut affected = HashSet::<GridVec>::from_iter([a.clone()]);
        let live = HashSet::<GridVec>::from_iter([a, b]);

        expand_affected_chunk_descendants(&mut affected, &live);
        assert_eq!(affected.len(), 1);
    }

    #[test]
    fn partition_coupling_policy_controls_edge_blend_weights() {
        let canonical = coord_at(0, 0, 0);
        let mut neighbor = canonical.clone() + IVec3::new(1, 0, 0);
        neighbor.normalize();

        let contributions = vec![ModelProjectionContribution {
            phenomenon_id: PhenomenonId(1),
            model_id: "demo_mass_density.default".to_string(),
            topology: PhenomenonModelTopology::PartitionedByChunk,
            partition_key: Some(0xaaaa),
            metric_index: 0,
            value: 0.2,
        }];
        let mut partition_index = PartitionProjectionIndex::default();
        partition_index.insert(PartitionProjectionSample {
            phenomenon_id: PhenomenonId(1),
            model_id: "demo_mass_density.default".to_string(),
            scale: canonical.scale,
            chunk_coord: neighbor.clone(),
            partition_key: 0xbbbb,
            metric_index: 0,
            value: 0.2,
        });

        let prefer_existing = partition_coupled_edge_projection(
            SubstrateChunkEdge::PosX,
            &canonical,
            &neighbor,
            &[0.9],
            0x1234,
            &contributions,
            &partition_index,
            SubstratePartitionCouplingPolicy {
                existing_metric_weight: 1.0,
                neighbor_metric_weight: 0.0,
            },
        )
        .expect("partition-coupled projection should exist");
        let prefer_neighbor = partition_coupled_edge_projection(
            SubstrateChunkEdge::PosX,
            &canonical,
            &neighbor,
            &[0.9],
            0x1234,
            &contributions,
            &partition_index,
            SubstratePartitionCouplingPolicy {
                existing_metric_weight: 0.0,
                neighbor_metric_weight: 1.0,
            },
        )
        .expect("partition-coupled projection should exist");

        assert!(prefer_existing.0[0] > prefer_neighbor.0[0]);
        assert!((prefer_existing.0[0] - 0.9).abs() < 0.0001);
        assert!((prefer_neighbor.0[0] - 0.2).abs() < 0.0001);
    }
}
