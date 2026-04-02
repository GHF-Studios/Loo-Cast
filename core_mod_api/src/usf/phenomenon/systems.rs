use std::collections::{HashMap, HashSet};

use crate::bevy::prelude::*;
use crate::chunk::components::{Chunk, ChunkLoader};
use crate::config::statics::CONFIG;
use crate::player::components::Player;
use crate::usf::authority::{
    USF_DOMAIN_PARTIAL_PHENOMENON_MODEL, USF_DOMAIN_PHENOMENON, USF_DOMAIN_PHENOMENON_MODEL, UsfAuthorityDiagnostics, UsfWorldAuthorityContract,
    guard_canonical_domain_with_diagnostics,
};

use crate::usf::phenomenon::components::{
    MonolithicPhenomenaModel, PartialPhenomenaModel, PartitionedPhenomenaModelMember, PartitionedPhenomenaModelRoot, PhenomenaModelState,
    PhenomenaModelSupport, PhenomenaModelTopology, Phenomenon, PhenomenonModel, PhenomenonModelProjectionContract, PhenomenonModelScriptDefinitionRef,
    PhenomenonModelSupport, PhenomenonNode, PhenomenonNodeLifecycle, PhenomenonNodeState, PhenomenonRootNodeRef, PhenomenonScriptDefinitionRef,
};
use crate::usf::phenomenon::generator::{BuildStateInput, PhenomenonGenerator, PlanChildrenInput};
use crate::usf::phenomenon::persistence::{
    PhenomenonPersistenceDurability, load_phenomena_model_record, load_phenomenon_record, model_record_path, phenomenon_record_path, topology_from_tag,
};
use crate::usf::phenomenon::resources::PhenomenonDefinitionRegistry;
use crate::usf::phenomenon::types::{PhenomenonId, PhenomenonLineage, PhenomenonNodeKey, PhenomenonNodeSeed};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::types::GridXyz;
use crate::usf::pos::types::LocalCell3;
use crate::usf::scale::Scale;
use crate::usf::zone::{ZoneId, ZonePhenomenon, ZoneRealizationEvent, ZoneRuntimeState};

use super::generators::layer_echo::LayerEchoGenerator;

#[inline]
fn guard_phenomenon_authority_contract(authority_contract: &UsfWorldAuthorityContract, diagnostics: Option<&mut UsfAuthorityDiagnostics>) -> bool {
    if let Some(diagnostics) = diagnostics {
        if !guard_canonical_domain_with_diagnostics(authority_contract, Some(&mut *diagnostics), USF_DOMAIN_PHENOMENON) {
            return false;
        }
        if !guard_canonical_domain_with_diagnostics(authority_contract, Some(&mut *diagnostics), USF_DOMAIN_PHENOMENON_MODEL) {
            return false;
        }
        return guard_canonical_domain_with_diagnostics(authority_contract, Some(&mut *diagnostics), USF_DOMAIN_PARTIAL_PHENOMENON_MODEL);
    }
    authority_contract.guard_canonical_domain(USF_DOMAIN_PHENOMENON)
        && authority_contract.guard_canonical_domain(USF_DOMAIN_PHENOMENON_MODEL)
        && authority_contract.guard_canonical_domain(USF_DOMAIN_PARTIAL_PHENOMENON_MODEL)
}

#[derive(Resource, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Resource)]
pub struct PhenomenonLifecyclePolicy {
    pub max_depth: u32,
    pub max_children_per_node: u32,
    pub frontier_margin: u32,
}

impl Default for PhenomenonLifecyclePolicy {
    fn default() -> Self {
        Self {
            max_depth: (Scale::SCALE_LEVEL_COUNT - 1) as u32,
            max_children_per_node: 1,
            frontier_margin: 2,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct PhenomenonGeneratorState {
    pub layer_echo: LayerEchoGenerator,
}

#[derive(Resource, Reflect, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[reflect(Resource)]
pub struct PhenomenonDebugStats {
    pub active_nodes: u32,
    pub active_frontier_proxies: u32,
    pub frontier_focus_seed: u64,
    pub frontier_focus_scale_index: u32,
    pub frontier_focus_window_size_milli: u32,
    pub frontier_proxy_spawns_frame: u32,
    pub frontier_proxy_despawns_frame: u32,
    pub generated_meshes_total: u64,
    pub generated_meshes_frame: u32,
    pub mesh_cache_hits_total: u64,
    pub mesh_cache_hits_frame: u32,
}

#[derive(Resource, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Resource)]
pub struct PhenomenonPersistenceRuntimeSettings {
    pub enabled: bool,
    pub persistence_dir: String,
    pub async_write_enabled: bool,
    pub async_write_batch_size: usize,
    pub max_queued_records_soft: usize,
    pub durability: PhenomenonPersistenceDurability,
    pub journal_enabled: bool,
    pub journal_dir: String,
    pub retain_successful_journal_batches: bool,
}
impl Default for PhenomenonPersistenceRuntimeSettings {
    fn default() -> Self {
        let persistence_dir = CONFIG().get::<String>("usf/runtime/phenomenon_persistence/persistence_dir");
        let durability_tag = CONFIG().get::<String>("usf/runtime/phenomenon_persistence/durability");
        Self {
            enabled: CONFIG().get::<bool>("usf/runtime/phenomenon_persistence/enabled"),
            persistence_dir,
            async_write_enabled: CONFIG().get::<bool>("usf/runtime/phenomenon_persistence/async_write_enabled"),
            async_write_batch_size: CONFIG().get::<usize>("usf/runtime/phenomenon_persistence/async_write_batch_size"),
            max_queued_records_soft: CONFIG().get::<usize>("usf/runtime/phenomenon_persistence/max_queued_records_soft"),
            durability: parse_persistence_durability(durability_tag.as_str()),
            journal_enabled: CONFIG().get::<bool>("usf/runtime/phenomenon_persistence/journal_enabled"),
            journal_dir: CONFIG().get::<String>("usf/runtime/phenomenon_persistence/journal_dir"),
            retain_successful_journal_batches: CONFIG().get::<bool>("usf/runtime/phenomenon_persistence/retain_successful_journal_batches"),
        }
    }
}

#[inline]
fn parse_persistence_durability(raw: &str) -> PhenomenonPersistenceDurability {
    let normalized = raw.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "atomic_replace" | "atomic-replace" | "atomic" => PhenomenonPersistenceDurability::AtomicReplace,
        "atomic_replace_and_fsync" | "atomic-replace-and-fsync" | "atomic_fsync" | "atomic-fsync" => PhenomenonPersistenceDurability::AtomicReplaceAndFsync,
        _ => panic!(
            "USF phenomenon persistence config is invalid: durability='{}'; expected atomic_replace or atomic_replace_and_fsync.",
            normalized
        ),
    }
}

#[derive(Resource, Debug, Default)]
pub struct PhenomenonPersistenceHydrationState {
    pub hydrated: bool,
}

#[inline]
fn is_canonical_root_node(node: &PhenomenonNode) -> bool {
    node.parent.is_none()
        && node.scale == Scale::MAX
        && node.local_cell == LocalCell3::ZERO
        && node.local_index == 0
        && node.lineage.cells.len() == 1
        && node.lineage.leaf() == Some(LocalCell3::ZERO)
}

pub(super) fn ensure_scale_models_system(
    authority_contract: Res<UsfWorldAuthorityContract>,
    mut authority_diagnostics: Option<ResMut<UsfAuthorityDiagnostics>>,
    mut commands: Commands,
    definitions: Res<PhenomenonDefinitionRegistry>,
    loaded_chunks: Query<&Chunk>,
    dirty_chunks: Query<(), (With<Chunk>, Or<(Added<Chunk>, Changed<Chunk>)>)>,
    mut removed_chunks: RemovedComponents<Chunk>,
    dirty_phenomena_query: Query<
        (),
        (
            With<Phenomenon>,
            Or<(
                Added<Phenomenon>,
                Changed<Phenomenon>,
                Added<PhenomenonScriptDefinitionRef>,
                Changed<PhenomenonScriptDefinitionRef>,
            )>,
        ),
    >,
    dirty_model_query: Query<
        (),
        (
            With<PhenomenonModel>,
            Or<(
                Added<PhenomenonModel>,
                Changed<PhenomenonModel>,
                Added<PhenomenonModelScriptDefinitionRef>,
                Changed<PhenomenonModelScriptDefinitionRef>,
                Added<PartitionedPhenomenaModelMember>,
                Changed<PartitionedPhenomenaModelMember>,
            )>,
        ),
    >,
    phenomenon_query: Query<(Entity, &Phenomenon, Option<&PhenomenonScriptDefinitionRef>)>,
    model_query: Query<(
        Entity,
        &PhenomenonModel,
        Option<&PhenomenonModelScriptDefinitionRef>,
        Option<&PartitionedPhenomenaModelMember>,
    )>,
) {
    if !guard_phenomenon_authority_contract(authority_contract.as_ref(), authority_diagnostics.as_deref_mut()) {
        return;
    }

    let chunk_topology_changed = !dirty_chunks.is_empty() || removed_chunks.read().next().is_some();
    let should_sync = definitions.is_changed() || !dirty_phenomena_query.is_empty() || !dirty_model_query.is_empty() || chunk_topology_changed;
    if !should_sync {
        return;
    }
    let live_scales = live_chunk_scales(&loaded_chunks);
    if live_scales.is_empty() {
        return;
    }
    let mut live_scales_sorted = live_scales.into_iter().collect::<Vec<_>>();
    live_scales_sorted.sort_by_key(Scale::index_from_top);

    let mut typed_models_by_phenomenon_scale = HashMap::<(Entity, u8, String), Entity>::new();
    for (model_entity, model, model_definition_ref, partition_member) in model_query.iter() {
        if partition_member.is_some() {
            continue;
        }
        let Some(model_definition_ref) = model_definition_ref else {
            continue;
        };
        typed_models_by_phenomenon_scale.insert(
            (
                model.phenomenon_entity,
                model.scale.index_from_top(),
                model_definition_ref.model_id.to_ascii_lowercase(),
            ),
            model_entity,
        );
    }

    for (phenomenon_entity, phenomenon, definition_ref) in phenomenon_query.iter() {
        let Some(definition_ref) = definition_ref else {
            continue;
        };
        let Some(expected_kind) = definitions.kind_for(&definition_ref.phenomenon_id) else {
            panic!(
                "USF phenomenon runtime failed: phenomenon entity {} references unknown definition '{}'.",
                phenomenon_entity.index(),
                definition_ref.phenomenon_id
            );
        };
        if expected_kind != phenomenon.kind {
            panic!(
                "USF phenomenon runtime failed: phenomenon entity {} has kind '{:?}' but definition '{}' requires '{:?}'.",
                phenomenon_entity.index(),
                phenomenon.kind,
                definition_ref.phenomenon_id,
                expected_kind
            );
        }

        for scale in live_scales_sorted.iter().copied() {
            let Some(selected_model_id) = definitions.model_for_scale(&definition_ref.phenomenon_id, scale) else {
                continue;
            };
            let lookup_key = (phenomenon_entity, scale.index_from_top(), selected_model_id.to_ascii_lowercase());
            if typed_models_by_phenomenon_scale.contains_key(&lookup_key) {
                continue;
            }

            let topology = definitions.topology_for_model(selected_model_id).unwrap_or_else(|| {
                panic!(
                    "USF phenomenon runtime failed: selected model '{}' is missing topology metadata.",
                    selected_model_id
                )
            });
            let configured_support_radius = definitions.support_chunk_radius_for_model(selected_model_id).unwrap_or_else(|| {
                panic!(
                    "USF phenomenon runtime failed: selected model '{}' is missing support radius metadata.",
                    selected_model_id
                )
            });
            let anchor_chunk = GridVec::new_splat(scale, GridXyz::ZERO);
            let chunk_radius = match topology {
                PhenomenaModelTopology::MonolithicChunk => 0,
                PhenomenaModelTopology::PartitionedByChunk => configured_support_radius.max(1),
            };
            let support = PhenomenonModelSupport {
                support: PhenomenaModelSupport {
                    anchor_chunk: anchor_chunk.clone(),
                    chunk_radius,
                },
            };
            let projection_contract = definitions.projection_contract_for_model(selected_model_id).unwrap_or_else(|| {
                panic!(
                    "USF phenomenon runtime failed: selected model '{}' is missing projection contract metadata.",
                    selected_model_id
                )
            });
            let projection = PhenomenonModelProjectionContract { contract: projection_contract };
            let model_name = format!(
                "phenomena_model_scale{}_{}_{}",
                scale.index_from_top(),
                definition_ref.phenomenon_id,
                phenomenon.id.0
            );
            let mut entity_commands = commands.spawn((
                Name::new(model_name),
                PhenomenonModel {
                    phenomenon_entity,
                    phenomenon_id: phenomenon.id,
                    scale,
                    topology,
                },
                PhenomenonModelScriptDefinitionRef {
                    model_id: selected_model_id.to_string(),
                    phenomenon_id: definition_ref.phenomenon_id.clone(),
                },
                support.clone(),
                projection,
                PhenomenaModelState::default(),
            ));
            match topology {
                PhenomenaModelTopology::MonolithicChunk => {
                    entity_commands.insert(MonolithicPhenomenaModel {
                        phenomenon_id: phenomenon.id,
                        scale,
                        chunk_coord: anchor_chunk,
                    });
                }
                PhenomenaModelTopology::PartitionedByChunk => {
                    entity_commands.insert(PartitionedPhenomenaModelRoot);
                }
            }
            typed_models_by_phenomenon_scale.insert(lookup_key, entity_commands.id());
        }
    }
}

pub(super) fn enforce_model_topology_component_contracts_system(
    authority_contract: Res<UsfWorldAuthorityContract>,
    mut authority_diagnostics: Option<ResMut<UsfAuthorityDiagnostics>>,
    mut commands: Commands,
    dirty_models: Query<
        (),
        (
            With<PhenomenonModel>,
            Or<(
                Added<PhenomenonModel>,
                Changed<PhenomenonModel>,
                Added<MonolithicPhenomenaModel>,
                Changed<MonolithicPhenomenaModel>,
                Added<PartialPhenomenaModel>,
                Changed<PartialPhenomenaModel>,
                Added<PartitionedPhenomenaModelRoot>,
                Changed<PartitionedPhenomenaModelRoot>,
                Added<PartitionedPhenomenaModelMember>,
                Changed<PartitionedPhenomenaModelMember>,
            )>,
        ),
    >,
    model_query: Query<(
        Entity,
        &PhenomenonModel,
        Option<&PhenomenonModelSupport>,
        Option<&MonolithicPhenomenaModel>,
        Option<&PartialPhenomenaModel>,
        Option<&PartitionedPhenomenaModelRoot>,
        Option<&PartitionedPhenomenaModelMember>,
    )>,
) {
    if !guard_phenomenon_authority_contract(authority_contract.as_ref(), authority_diagnostics.as_deref_mut()) {
        return;
    }

    if dirty_models.is_empty() {
        return;
    }

    for (entity, model, support, monolithic, partial, partition_root, partition_member) in model_query.iter() {
        let mut entity_commands = commands.entity(entity);
        if partition_member.is_some() {
            if model.topology != PhenomenaModelTopology::PartitionedByChunk {
                panic!(
                    "USF phenomenon runtime failed: partition member model entity {} has non-partitioned topology '{:?}'.",
                    entity.index(),
                    model.topology
                );
            }
            if monolithic.is_some() {
                entity_commands.remove::<MonolithicPhenomenaModel>();
            }
            if partial.is_some() {
                entity_commands.remove::<PartialPhenomenaModel>();
            }
            if partition_root.is_some() {
                entity_commands.remove::<PartitionedPhenomenaModelRoot>();
            }
            continue;
        }

        match model.topology {
            PhenomenaModelTopology::MonolithicChunk => {
                if monolithic.is_none() {
                    let chunk_coord = support
                        .map(|support| support.support.anchor_chunk.clone())
                        .unwrap_or_else(|| GridVec::new_splat(model.scale, GridXyz::ZERO));
                    entity_commands.insert(MonolithicPhenomenaModel {
                        phenomenon_id: model.phenomenon_id,
                        scale: model.scale,
                        chunk_coord,
                    });
                }
                if partial.is_some() {
                    entity_commands.remove::<PartialPhenomenaModel>();
                }
                if partition_root.is_some() {
                    entity_commands.remove::<PartitionedPhenomenaModelRoot>();
                }
            }
            PhenomenaModelTopology::PartitionedByChunk => {
                if partition_root.is_none() {
                    entity_commands.insert(PartitionedPhenomenaModelRoot);
                }
                if monolithic.is_some() {
                    entity_commands.remove::<MonolithicPhenomenaModel>();
                }
                if partial.is_some() {
                    entity_commands.remove::<PartialPhenomenaModel>();
                }
            }
        }
    }
}

pub(super) fn apply_zone_realization_startup_hooks_system(
    authority_contract: Res<UsfWorldAuthorityContract>,
    mut authority_diagnostics: Option<ResMut<UsfAuthorityDiagnostics>>,
    mut zone_realization_events: MessageReader<ZoneRealizationEvent>,
    zone_runtime_state: Res<ZoneRuntimeState>,
    zone_phenomenon_query: Query<(Entity, &ZonePhenomenon), With<Phenomenon>>,
    dirty_models: Query<
        (),
        (
            With<PhenomenonModel>,
            Or<(
                Added<PhenomenonModel>,
                Changed<PhenomenonModel>,
                Added<PhenomenonModelSupport>,
                Changed<PhenomenonModelSupport>,
                Added<PhenomenaModelState>,
                Changed<PhenomenaModelState>,
                Added<MonolithicPhenomenaModel>,
                Changed<MonolithicPhenomenaModel>,
                Added<PartialPhenomenaModel>,
                Changed<PartialPhenomenaModel>,
            )>,
        ),
    >,
    mut model_query: Query<(
        &PhenomenonModel,
        &mut PhenomenonModelSupport,
        &mut PhenomenaModelState,
        Option<&mut MonolithicPhenomenaModel>,
        Option<&mut PartialPhenomenaModel>,
        Option<&PartitionedPhenomenaModelMember>,
    )>,
) {
    if !guard_phenomenon_authority_contract(authority_contract.as_ref(), authority_diagnostics.as_deref_mut()) {
        return;
    }

    let spawned = zone_realization_events
        .read()
        .filter_map(|event| match event {
            ZoneRealizationEvent::Spawned {
                zone_id, phenomenon_entity, ..
            } => Some((*phenomenon_entity, zone_id.clone())),
            ZoneRealizationEvent::Despawned { .. } => None,
        })
        .collect::<Vec<_>>();
    let reconcile_all_realized = zone_runtime_state.is_changed() || !dirty_models.is_empty();
    if spawned.is_empty() && !reconcile_all_realized {
        return;
    }

    let mut zone_by_phenomenon_entity = HashMap::<Entity, ZoneId>::new();
    for (phenomenon_entity, zone_id) in spawned {
        zone_by_phenomenon_entity.insert(phenomenon_entity, zone_id);
    }
    if reconcile_all_realized {
        for (phenomenon_entity, zone_phenomenon) in zone_phenomenon_query.iter() {
            zone_by_phenomenon_entity
                .entry(phenomenon_entity)
                .or_insert_with(|| zone_phenomenon.zone_id.clone());
        }
    }

    if zone_by_phenomenon_entity.is_empty() {
        return;
    }

    for (phenomenon_entity, zone_id) in zone_by_phenomenon_entity {
        let Some((anchor_chunk, chunk_radius)) = zone_support_seed(&zone_runtime_state, &zone_id) else {
            continue;
        };
        let zone_seed = zone_seed_scalar(&zone_id, &anchor_chunk);
        for (model, mut support, mut state, monolithic, partial, partition_member) in model_query.iter_mut() {
            if model.phenomenon_entity != phenomenon_entity {
                continue;
            }

            let desired_chunk_radius = match model.topology {
                PhenomenaModelTopology::MonolithicChunk => 0,
                PhenomenaModelTopology::PartitionedByChunk => chunk_radius.max(1),
            };
            if support.support.anchor_chunk != anchor_chunk {
                support.support.anchor_chunk = anchor_chunk.clone();
            }
            if support.support.chunk_radius != desired_chunk_radius {
                support.support.chunk_radius = desired_chunk_radius;
            }

            if let Some(mut monolithic) = monolithic {
                if monolithic.chunk_coord != anchor_chunk {
                    monolithic.chunk_coord = anchor_chunk.clone();
                }
            }
            if let Some(mut partial) = partial {
                if partition_member.is_some() {
                    let _ = upsert_scalar_channel(&mut state.scalar_channels, "zone.seed", zone_seed);
                    continue;
                }
                if partial.chunk_coord != anchor_chunk {
                    partial.chunk_coord = anchor_chunk.clone();
                }
                let desired_partition_key = PartialPhenomenaModel::deterministic_partition_key(partial.phenomenon_id, partial.scale, &anchor_chunk);
                if partial.partition_key != desired_partition_key {
                    partial.partition_key = desired_partition_key;
                }
            }

            let _ = upsert_scalar_channel(&mut state.scalar_channels, "zone.seed", zone_seed);
        }
    }
}

pub(super) fn prune_orphan_models_system(
    authority_contract: Res<UsfWorldAuthorityContract>,
    mut authority_diagnostics: Option<ResMut<UsfAuthorityDiagnostics>>,
    mut commands: Commands,
    definitions: Res<PhenomenonDefinitionRegistry>,
    loaded_chunks: Query<&Chunk>,
    dirty_chunks: Query<(), (With<Chunk>, Or<(Added<Chunk>, Changed<Chunk>)>)>,
    mut removed_chunks: RemovedComponents<Chunk>,
    dirty_phenomena_query: Query<
        (),
        (
            With<Phenomenon>,
            Or<(
                Added<Phenomenon>,
                Changed<Phenomenon>,
                Added<PhenomenonScriptDefinitionRef>,
                Changed<PhenomenonScriptDefinitionRef>,
            )>,
        ),
    >,
    dirty_model_query: Query<
        (),
        (
            With<PhenomenonModel>,
            Or<(
                Added<PhenomenonModel>,
                Changed<PhenomenonModel>,
                Added<PhenomenonModelScriptDefinitionRef>,
                Changed<PhenomenonModelScriptDefinitionRef>,
            )>,
        ),
    >,
    mut removed_phenomena: RemovedComponents<Phenomenon>,
    mut removed_models: RemovedComponents<PhenomenonModel>,
    phenomenon_query: Query<(Entity, Option<&PhenomenonScriptDefinitionRef>), With<Phenomenon>>,
    model_query: Query<(Entity, &PhenomenonModel, Option<&PhenomenonModelScriptDefinitionRef>)>,
) {
    if !guard_phenomenon_authority_contract(authority_contract.as_ref(), authority_diagnostics.as_deref_mut()) {
        return;
    }

    let chunk_topology_changed = !dirty_chunks.is_empty() || removed_chunks.read().next().is_some();
    let should_prune = definitions.is_changed()
        || !dirty_phenomena_query.is_empty()
        || !dirty_model_query.is_empty()
        || chunk_topology_changed
        || removed_phenomena.read().next().is_some()
        || removed_models.read().next().is_some();
    if !should_prune {
        return;
    }
    let live_scales = live_chunk_scales(&loaded_chunks);

    let mut live_phenomenon_entities = HashSet::<Entity>::new();
    let mut script_id_by_entity = HashMap::<Entity, String>::new();
    for (phenomenon_entity, definition_ref) in phenomenon_query.iter() {
        live_phenomenon_entities.insert(phenomenon_entity);
        if let Some(definition_ref) = definition_ref {
            script_id_by_entity.insert(phenomenon_entity, definition_ref.phenomenon_id.clone());
        }
    }

    for (model_entity, model, model_definition_ref) in model_query.iter() {
        if !live_phenomenon_entities.contains(&model.phenomenon_entity) {
            commands.entity(model_entity).despawn();
            continue;
        }
        if !live_scales.contains(&model.scale) {
            commands.entity(model_entity).despawn();
            continue;
        }
        let Some(model_definition_ref) = model_definition_ref else {
            continue;
        };
        let Some(phenomenon_definition_id) = script_id_by_entity.get(&model.phenomenon_entity) else {
            commands.entity(model_entity).despawn();
            continue;
        };
        let Some(expected_model_for_scale) = definitions.model_for_scale(phenomenon_definition_id, model.scale) else {
            commands.entity(model_entity).despawn();
            continue;
        };
        if &model_definition_ref.phenomenon_id != phenomenon_definition_id
            || !definitions.model_belongs_to_phenomenon(&model_definition_ref.model_id, &model_definition_ref.phenomenon_id)
            || !model_definition_ref.model_id.eq_ignore_ascii_case(expected_model_for_scale)
        {
            commands.entity(model_entity).despawn();
        }
    }
}

fn live_chunk_scales(loaded_chunks: &Query<&Chunk>) -> HashSet<Scale> {
    loaded_chunks.iter().map(|chunk| chunk.coord.scale).collect::<HashSet<_>>()
}

fn zone_support_seed(zone_runtime_state: &ZoneRuntimeState, zone_id: &ZoneId) -> Option<(GridVec, u16)> {
    let extent = zone_runtime_state.records.get(zone_id)?;
    if extent.chunk_coords.is_empty() {
        return None;
    }

    let mut sorted_chunk_coords = extent.chunk_coords.clone();
    sorted_chunk_coords.sort_by(|left, right| grid_coord_sort_key(left).cmp(&grid_coord_sort_key(right)));
    let anchor_chunk = sorted_chunk_coords.first().cloned()?;
    let chunk_radius = estimate_support_radius_from_chunk_count(sorted_chunk_coords.len());
    Some((anchor_chunk, chunk_radius))
}

#[inline]
fn estimate_support_radius_from_chunk_count(chunk_count: usize) -> u16 {
    if chunk_count <= 1 {
        return 0;
    }
    ((chunk_count as f64).cbrt().ceil() as u16).saturating_sub(1).max(1)
}

fn grid_coord_sort_key(coord: &GridVec) -> (u8, Vec<(i32, i32, i32)>) {
    let mut canonical = coord.clone();
    canonical.normalize();
    let digits = canonical.to_raw_vec_3d().into_iter().map(|xyz| (xyz.x, xyz.y, xyz.z)).collect::<Vec<_>>();
    (canonical.scale.index_from_top(), digits)
}

fn zone_seed_scalar(zone_id: &ZoneId, anchor_chunk: &GridVec) -> f32 {
    let mut state = mix64(0x9e37_79b9_7f4a_7c15 ^ zone_id.stable_region_id.0);
    for byte in zone_id.zone_type.0.as_bytes() {
        state = mix64(state ^ *byte as u64);
    }
    state = mix64(state ^ zone_id.scale.index_from_top() as u64);
    for xyz in anchor_chunk.to_raw_vec_3d() {
        state = mix64(state ^ fold_signed(xyz.x));
        state = mix64(state ^ fold_signed(xyz.y));
        state = mix64(state ^ fold_signed(xyz.z));
    }
    ((state >> 40) as f32) / ((1_u32 << 24) as f32)
}

fn upsert_scalar_channel(channels: &mut Vec<(String, f32)>, channel_name: &str, value: f32) -> bool {
    for (name, channel_value) in channels.iter_mut() {
        if name.eq_ignore_ascii_case(channel_name) {
            if (*channel_value - value).abs() <= f32::EPSILON {
                return false;
            }
            *channel_value = value;
            return true;
        }
    }
    channels.push((channel_name.to_string(), value));
    true
}

#[inline]
fn fold_signed(value: i32) -> u64 {
    value as i64 as u64
}

#[inline]
fn mix64(mut state: u64) -> u64 {
    state ^= state >> 30;
    state = state.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    state ^= state >> 27;
    state = state.wrapping_mul(0x94d0_49bb_1331_11eb);
    state ^ (state >> 31)
}

pub(super) fn hydrate_persisted_phenomena_state_system(
    authority_contract: Res<UsfWorldAuthorityContract>,
    mut authority_diagnostics: Option<ResMut<UsfAuthorityDiagnostics>>,
    settings: Res<PhenomenonPersistenceRuntimeSettings>,
    mut hydration_state: ResMut<PhenomenonPersistenceHydrationState>,
    phenomenon_query: Query<(&Phenomenon, &PhenomenonScriptDefinitionRef)>,
    mut model_query: Query<(
        &PhenomenonModel,
        &PhenomenonModelScriptDefinitionRef,
        &mut PhenomenonModelSupport,
        &mut PhenomenonModelProjectionContract,
        &mut PhenomenaModelState,
        Option<&PartitionedPhenomenaModelMember>,
    )>,
) {
    if !guard_phenomenon_authority_contract(authority_contract.as_ref(), authority_diagnostics.as_deref_mut()) {
        return;
    }

    if !settings.enabled || hydration_state.hydrated {
        return;
    }

    for (phenomenon, script_ref) in phenomenon_query.iter() {
        let record_path = phenomenon_record_path(settings.persistence_dir.as_str(), phenomenon.id);
        let Some(record) = load_phenomenon_record(&record_path)
            .unwrap_or_else(|error| panic!("USF phenomenon hydrate failed: could not load phenomenon record '{record_path:?}': {error}"))
        else {
            continue;
        };
        if record.phenomenon_id != phenomenon.id.0 {
            panic!(
                "USF phenomenon hydrate failed: persisted phenomenon id mismatch for '{}': runtime={} persisted={}",
                script_ref.phenomenon_id, phenomenon.id.0, record.phenomenon_id
            );
        }
    }

    for (model, model_script_ref, mut support, mut projection, mut state, partition_member) in model_query.iter_mut() {
        if partition_member.is_some() {
            continue;
        }
        let model_path = model_record_path(
            settings.persistence_dir.as_str(),
            model.phenomenon_id,
            model.scale,
            model_script_ref.model_id.as_str(),
        );
        let Some(record) = load_phenomena_model_record(&model_path)
            .unwrap_or_else(|error| panic!("USF phenomenon hydrate failed: could not load model record '{model_path:?}': {error}"))
        else {
            continue;
        };
        if let Ok(topology) = topology_from_tag(record.topology.as_str()) {
            if topology != model.topology {
                warn!(
                    "USF phenomenon hydrate: persisted topology '{}' does not match runtime topology '{:?}' for model '{}'.",
                    record.topology, model.topology, model_script_ref.model_id
                );
            }
        }
        if let Ok(anchor_chunk) = record.support_anchor_chunk.to_grid() {
            support.support.anchor_chunk = anchor_chunk;
        }
        support.support.chunk_radius = record.support_chunk_radius;
        projection.contract.metric_name = record.projection_metric_name;
        projection.contract.projection_bias = record.projection_bias;
        projection.contract.projection_gain = record.projection_gain;
        state.scalar_channels = record.scalar_channels;
    }

    hydration_state.hydrated = true;
}

pub(super) fn sync_policy_depth_to_frontier_scale_system(
    player_loader_query: Query<&ChunkLoader, With<Player>>,
    mut policy: ResMut<PhenomenonLifecyclePolicy>,
) {
    let Ok(chunk_loader) = player_loader_query.single() else {
        return;
    };
    let frontier_depth = chunk_loader.phenomenon_frontier_view().scale.index_from_top() as u32;
    let max_allowed_depth = (Scale::SCALE_LEVEL_COUNT - 1) as u32;
    policy.max_depth = frontier_depth.saturating_add(policy.frontier_margin).min(max_allowed_depth);
}

pub(super) fn ensure_root_nodes_system(
    mut commands: Commands,
    phenomenon_query: Query<(Entity, &Phenomenon, Option<&PhenomenonRootNodeRef>)>,
    node_query: Query<(Entity, &PhenomenonNode), With<PhenomenonNodeLifecycle>>,
    generator_state: Res<PhenomenonGeneratorState>,
) {
    let mut roots_by_phenomenon: HashMap<PhenomenonId, Entity> = HashMap::new();
    for (entity, node) in node_query.iter() {
        if is_canonical_root_node(node) {
            roots_by_phenomenon.entry(node.phenomenon_id).or_insert(entity);
        }
    }

    for (phenomenon_entity, phenomenon, root_ref) in phenomenon_query.iter() {
        if let Some(root_entity) = roots_by_phenomenon.get(&phenomenon.id).copied() {
            if root_ref.map(|root| root.node) != Some(root_entity) {
                commands.entity(phenomenon_entity).insert(PhenomenonRootNodeRef { node: root_entity });
            }
            continue;
        }

        let key = PhenomenonNodeKey {
            phenomenon_id: phenomenon.id,
            scale: Scale::MAX,
            lineage: PhenomenonLineage::root(),
            parent: None,
            local_index: 0,
        };
        let snapshot = generator_state.layer_echo.build_state(BuildStateInput {
            key: key.clone(),
            parent_state: None,
        });
        let root_entity = commands
            .spawn((
                Name::new(format!("phenomenon_node_root_{}", phenomenon.id.0)),
                PhenomenonNode::from_key(key),
                PhenomenonNodeState { snapshot },
                PhenomenonNodeLifecycle { depth: 0 },
            ))
            .id();
        commands.entity(phenomenon_entity).insert(PhenomenonRootNodeRef { node: root_entity });
        roots_by_phenomenon.insert(phenomenon.id, root_entity);
    }
}

pub(super) fn expand_phenomenon_frontier_system(
    mut commands: Commands,
    policy: Res<PhenomenonLifecyclePolicy>,
    generator_state: Res<PhenomenonGeneratorState>,
    node_query: Query<(Entity, &PhenomenonNode, &PhenomenonNodeState, &PhenomenonNodeLifecycle)>,
) {
    let mut existing_seeds: HashSet<PhenomenonNodeSeed> = node_query.iter().map(|(_, node, _, _)| node.seed).collect();

    for (_entity, node, node_state, lifecycle) in node_query.iter() {
        if lifecycle.depth >= policy.max_depth {
            continue;
        }

        let plan = generator_state.layer_echo.plan_children(PlanChildrenInput {
            key: node.key(),
            state: &node_state.snapshot,
            max_children: policy.max_children_per_node,
        });

        for child in plan {
            let child_key = PhenomenonNodeKey {
                phenomenon_id: node.phenomenon_id,
                scale: child.scale,
                lineage: node.lineage.pushed(child.local_cell),
                parent: Some(node.seed),
                local_index: child.local_index,
            };
            let child_seed = child_key.clone().deterministic_seed();
            if existing_seeds.contains(&child_seed) {
                continue;
            }

            let child_snapshot = generator_state.layer_echo.build_state(BuildStateInput {
                key: child_key.clone(),
                parent_state: Some(&node_state.snapshot),
            });
            let child_node = PhenomenonNode::from_key(child_key);
            commands.spawn((
                Name::new(format!("phenomenon_node_{}_{}", child_node.scale.index_from_top(), child_node.local_index)),
                child_node,
                PhenomenonNodeState { snapshot: child_snapshot },
                PhenomenonNodeLifecycle {
                    depth: lifecycle.depth.saturating_add(1),
                },
            ));
            existing_seeds.insert(child_seed);
        }
    }
}

pub(super) fn despawn_invalid_nodes_system(
    mut commands: Commands,
    policy: Res<PhenomenonLifecyclePolicy>,
    phenomenon_query: Query<&Phenomenon>,
    node_query: Query<(Entity, &PhenomenonNode, &PhenomenonNodeLifecycle)>,
) {
    let live_phenomena: HashSet<PhenomenonId> = phenomenon_query.iter().map(|phenomenon| phenomenon.id).collect();
    let live_seeds: HashSet<PhenomenonNodeSeed> = node_query.iter().map(|(_, node, _)| node.seed).collect();

    for (entity, node, lifecycle) in node_query.iter() {
        let detached_root = node.parent.is_none() && !live_phenomena.contains(&node.phenomenon_id);
        let invalid_root_contract = node.parent.is_none() && !is_canonical_root_node(node);
        let missing_parent = node.parent.is_some_and(|parent_seed| !live_seeds.contains(&parent_seed));
        let out_of_policy_depth = lifecycle.depth > policy.max_depth;
        if detached_root || invalid_root_contract || missing_parent || out_of_policy_depth {
            commands.entity(entity).despawn();
        }
    }
}

pub(super) fn refresh_active_node_stats_system(node_query: Query<&PhenomenonNode>, mut stats: ResMut<PhenomenonDebugStats>) {
    stats.active_nodes = node_query.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::phenomenon::generator::PhenomenonStateSnapshot;
    use crate::usf::phenomenon::types::PhenomenonKind;

    fn setup_lifecycle_test_app(max_depth: u32, max_children_per_node: u32) -> App {
        let mut app = App::new();
        app.init_resource::<PhenomenonLifecyclePolicy>();
        app.init_resource::<PhenomenonGeneratorState>();
        {
            let mut policy = app.world_mut().resource_mut::<PhenomenonLifecyclePolicy>();
            policy.max_depth = max_depth;
            policy.max_children_per_node = max_children_per_node;
        }
        app.add_systems(
            Update,
            (
                ensure_root_nodes_system,
                expand_phenomenon_frontier_system.after(ensure_root_nodes_system),
                despawn_invalid_nodes_system.after(expand_phenomenon_frontier_system),
            ),
        );
        app
    }

    #[test]
    fn decentralized_lifecycle_spawns_root_children_and_grandchildren() {
        let mut app = setup_lifecycle_test_app(3, 2);
        app.world_mut().spawn(Phenomenon {
            id: PhenomenonId(1),
            kind: PhenomenonKind::ManifestationDensityDebug,
        });

        for _ in 0..3 {
            app.update();
        }

        let mut has_depth_0 = false;
        let mut has_depth_1 = false;
        let mut has_depth_2 = false;
        let mut query = app.world_mut().query::<(&PhenomenonNode, &PhenomenonNodeLifecycle)>();
        for (_node, lifecycle) in query.iter(app.world()) {
            match lifecycle.depth {
                0 => has_depth_0 = true,
                1 => has_depth_1 = true,
                2 => has_depth_2 = true,
                _ => {}
            }
        }

        assert!(has_depth_0, "missing root node");
        assert!(has_depth_1, "missing child nodes");
        assert!(has_depth_2, "missing grandchildren nodes");
    }

    #[test]
    fn lifecycle_despawns_nodes_when_depth_policy_shrinks() {
        let mut app = setup_lifecycle_test_app(3, 2);
        app.world_mut().spawn(Phenomenon {
            id: PhenomenonId(2),
            kind: PhenomenonKind::ManifestationDensityDebug,
        });
        for _ in 0..3 {
            app.update();
        }

        {
            let mut policy = app.world_mut().resource_mut::<PhenomenonLifecyclePolicy>();
            policy.max_depth = 0;
        }
        for _ in 0..3 {
            app.update();
        }

        let mut max_observed_depth = 0_u32;
        let mut query = app.world_mut().query::<&PhenomenonNodeLifecycle>();
        for lifecycle in query.iter(app.world()) {
            max_observed_depth = max_observed_depth.max(lifecycle.depth);
        }
        assert_eq!(max_observed_depth, 0);
    }

    #[test]
    fn debug_stats_report_active_nodes() {
        let mut app = setup_lifecycle_test_app(2, 2);
        app.init_resource::<PhenomenonDebugStats>();
        app.add_systems(Update, refresh_active_node_stats_system);
        app.world_mut().spawn(Phenomenon {
            id: PhenomenonId(3),
            kind: PhenomenonKind::ManifestationDensityDebug,
        });
        app.update();
        app.update();

        let stats = app.world().resource::<PhenomenonDebugStats>();
        assert!(stats.active_nodes > 0);
    }

    #[test]
    fn lifecycle_replaces_noncanonical_root_with_pinned_root_contract() {
        let mut app = setup_lifecycle_test_app(2, 2);
        let phenomenon_entity = app
            .world_mut()
            .spawn(Phenomenon {
                id: PhenomenonId(77),
                kind: PhenomenonKind::ManifestationDensityDebug,
            })
            .id();

        let bad_root_key = PhenomenonNodeKey {
            phenomenon_id: PhenomenonId(77),
            scale: Scale::MAX.zoomed_in(),
            lineage: PhenomenonLineage::from_cells(vec![LocalCell3::new_local(0, 0, 0), LocalCell3::new_local(1, 0, 0)]),
            parent: None,
            local_index: 5,
        };
        app.world_mut().spawn((
            PhenomenonNode::from_key(bad_root_key.clone()),
            PhenomenonNodeState {
                snapshot: PhenomenonStateSnapshot {
                    seed: bad_root_key.deterministic_seed(),
                    root_seed: bad_root_key.deterministic_seed(),
                    lineage_depth: 0,
                    metric_phase: Vec3::ZERO,
                    channels: Vec4::ZERO,
                },
            },
            PhenomenonNodeLifecycle { depth: 0 },
        ));

        app.update();

        let root_ref = app
            .world()
            .get::<PhenomenonRootNodeRef>(phenomenon_entity)
            .expect("phenomenon should have a root ref");
        let root_node = app.world().get::<PhenomenonNode>(root_ref.node).expect("canonical root node should exist");

        assert!(is_canonical_root_node(root_node));

        let mut root_count = 0usize;
        let mut query = app.world_mut().query::<&PhenomenonNode>();
        for node in query.iter(app.world()) {
            if node.phenomenon_id == PhenomenonId(77) && node.parent.is_none() {
                root_count += 1;
            }
        }
        assert_eq!(root_count, 1, "expected exactly one canonical root for phenomenon 77");
    }

    #[test]
    fn persistence_durability_parser_accepts_supported_aliases() {
        assert_eq!(parse_persistence_durability("atomic_replace"), PhenomenonPersistenceDurability::AtomicReplace);
        assert_eq!(
            parse_persistence_durability("atomic-replace-and-fsync"),
            PhenomenonPersistenceDurability::AtomicReplaceAndFsync
        );
    }
}
