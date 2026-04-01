use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::bevy::prelude::*;
use crate::chunk::components::ChunkLoader;
use crate::player::components::Player;

use crate::usf::phenomenon::components::{
    MonolithicPhenomenaModel, PartialPhenomenaModel, PhenomenaModelState, PhenomenaModelSupport, PhenomenaModelTopology, PhenomenaProjectionContract,
    Phenomenon, PhenomenonModel, PhenomenonModelProjectionContract, PhenomenonModelScriptDefinitionRef, PhenomenonModelSupport, PhenomenonNode,
    PhenomenonNodeLifecycle, PhenomenonNodeState, PhenomenonRootNodeRef, PhenomenonScriptDefinitionRef,
};
use crate::usf::phenomenon::generator::{BuildStateInput, PhenomenonGenerator, PlanChildrenInput};
use crate::usf::phenomenon::persistence::{
    load_partial_phenomena_model_record, load_phenomena_model_record, load_phenomenon_record, model_record_from_runtime, monolithic_model_record_from_runtime,
    partial_model_record_from_runtime, phenomenon_record_from_runtime, save_partial_phenomena_model_record, save_phenomena_model_record,
    save_phenomenon_record, topology_from_tag,
};
use crate::usf::phenomenon::resources::PhenomenonDefinitionRegistry;
use crate::usf::phenomenon::types::{PhenomenonId, PhenomenonLineage, PhenomenonNodeKey, PhenomenonNodeSeed};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::types::GridXyz;
use crate::usf::pos::types::LocalCell3;
use crate::usf::scale::Scale;

use super::generators::layer_echo::LayerEchoGenerator;

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
    pub frontier_primary_seed: u64,
    pub frontier_primary_scale_index: u32,
    pub frontier_primary_window_size_milli: u32,
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
}
impl Default for PhenomenonPersistenceRuntimeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            persistence_dir: "target/usf_demo/phenomena_authority".to_string(),
        }
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
    mut commands: Commands,
    definitions: Res<PhenomenonDefinitionRegistry>,
    phenomenon_query: Query<(Entity, &Phenomenon, Option<&PhenomenonScriptDefinitionRef>)>,
    model_query: Query<(Entity, &PhenomenonModel, Option<&PhenomenonModelScriptDefinitionRef>)>,
) {
    let mut typed_models_by_phenomenon_scale = HashMap::<(Entity, u8, String), Entity>::new();
    for (model_entity, model, model_definition_ref) in model_query.iter() {
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

        for (scale, selected_model_id) in definitions.model_selector_all(&definition_ref.phenomenon_id) {
            let lookup_key = (phenomenon_entity, scale.index_from_top(), selected_model_id.to_ascii_lowercase());
            if typed_models_by_phenomenon_scale.contains_key(&lookup_key) {
                continue;
            }

            let topology = if scale.index_from_top() <= 8 {
                PhenomenaModelTopology::PartitionedByChunk
            } else {
                PhenomenaModelTopology::MonolithicChunk
            };
            let anchor_chunk = GridVec::new_splat(scale, GridXyz::ZERO);
            let chunk_radius = match topology {
                PhenomenaModelTopology::MonolithicChunk => 0,
                PhenomenaModelTopology::PartitionedByChunk => 2,
            };
            let support = PhenomenonModelSupport {
                support: PhenomenaModelSupport {
                    anchor_chunk: anchor_chunk.clone(),
                    chunk_radius,
                },
            };
            let projection = PhenomenonModelProjectionContract {
                contract: PhenomenaProjectionContract::default(),
            };
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
                    entity_commands.insert(PartialPhenomenaModel {
                        phenomenon_id: phenomenon.id,
                        scale,
                        partition_key: PartialPhenomenaModel::deterministic_partition_key(phenomenon.id, scale, &anchor_chunk),
                        chunk_coord: anchor_chunk,
                    });
                }
            }
            typed_models_by_phenomenon_scale.insert(lookup_key, entity_commands.id());
        }
    }
}

pub(super) fn prune_orphan_models_system(
    mut commands: Commands,
    definitions: Res<PhenomenonDefinitionRegistry>,
    phenomenon_query: Query<(Entity, Option<&PhenomenonScriptDefinitionRef>), With<Phenomenon>>,
    model_query: Query<(Entity, &PhenomenonModel, Option<&PhenomenonModelScriptDefinitionRef>)>,
) {
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

pub(super) fn hydrate_persisted_phenomena_state_system(
    settings: Res<PhenomenonPersistenceRuntimeSettings>,
    mut hydration_state: ResMut<PhenomenonPersistenceHydrationState>,
    phenomenon_query: Query<(&Phenomenon, &PhenomenonScriptDefinitionRef)>,
    mut model_query: Query<(
        &PhenomenonModel,
        &PhenomenonModelScriptDefinitionRef,
        &mut PhenomenonModelSupport,
        &mut PhenomenonModelProjectionContract,
        &mut PhenomenaModelState,
        Option<&PartialPhenomenaModel>,
    )>,
) {
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

    for (model, model_script_ref, mut support, mut projection, mut state, partial) in model_query.iter_mut() {
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

        let Some(partial) = partial else {
            continue;
        };
        let partial_path = partial_record_path(
            settings.persistence_dir.as_str(),
            partial.phenomenon_id,
            partial.scale,
            model_script_ref.model_id.as_str(),
            partial.partition_key,
        );
        let Some(partial_record) = load_partial_phenomena_model_record(&partial_path)
            .unwrap_or_else(|error| panic!("USF phenomenon hydrate failed: could not load partial record '{partial_path:?}': {error}"))
        else {
            continue;
        };
        state.scalar_channels = partial_record.scalar_channels;
    }

    hydration_state.hydrated = true;
}

pub(super) fn persist_authoritative_phenomena_state_system(
    settings: Res<PhenomenonPersistenceRuntimeSettings>,
    phenomenon_query: Query<(Entity, &Phenomenon, &PhenomenonScriptDefinitionRef)>,
    model_query: Query<(
        &PhenomenonModel,
        &PhenomenonModelScriptDefinitionRef,
        &PhenomenonModelSupport,
        &PhenomenonModelProjectionContract,
        &PhenomenaModelState,
        Option<&MonolithicPhenomenaModel>,
        Option<&PartialPhenomenaModel>,
    )>,
) {
    if !settings.enabled {
        return;
    }

    let mut script_id_by_entity = HashMap::<Entity, String>::new();
    for (entity, phenomenon, script_ref) in phenomenon_query.iter() {
        let record = phenomenon_record_from_runtime(phenomenon.id, phenomenon.kind, script_ref.phenomenon_id.as_str());
        let record_path = phenomenon_record_path(settings.persistence_dir.as_str(), phenomenon.id);
        if let Err(error) = save_phenomenon_record(&record_path, &record) {
            panic!(
                "USF phenomenon persistence failed: could not save phenomenon record '{}' ({:?}): {}",
                script_ref.phenomenon_id, record_path, error
            );
        }
        script_id_by_entity.insert(entity, script_ref.phenomenon_id.clone());
    }

    for (model, model_script_ref, support, projection, state, monolithic, partial) in model_query.iter() {
        let Some(script_id) = script_id_by_entity.get(&model.phenomenon_entity) else {
            continue;
        };
        let model_record = if let Some(monolithic) = monolithic {
            monolithic_model_record_from_runtime(model_script_ref.model_id.as_str(), monolithic, support, projection, state)
        } else {
            model_record_from_runtime(
                model.phenomenon_id,
                model_script_ref.model_id.as_str(),
                model.scale,
                model.topology,
                support,
                projection,
                state,
            )
        };
        let model_record_path = model_record_path(
            settings.persistence_dir.as_str(),
            model.phenomenon_id,
            model.scale,
            model_script_ref.model_id.as_str(),
        );
        if let Err(error) = save_phenomena_model_record(&model_record_path, &model_record) {
            panic!(
                "USF phenomenon persistence failed: could not save model record '{}' for phenomenon '{}' ({:?}): {}",
                model_script_ref.model_id, script_id, model_record_path, error
            );
        }

        if let Some(partial) = partial {
            let partial_record = partial_model_record_from_runtime(model_script_ref.model_id.as_str(), partial, state);
            let partial_record_path = partial_record_path(
                settings.persistence_dir.as_str(),
                partial.phenomenon_id,
                partial.scale,
                model_script_ref.model_id.as_str(),
                partial.partition_key,
            );
            if let Err(error) = save_partial_phenomena_model_record(&partial_record_path, &partial_record) {
                panic!(
                    "USF phenomenon persistence failed: could not save partial model record '{}' for phenomenon '{}' ({:?}): {}",
                    model_script_ref.model_id, script_id, partial_record_path, error
                );
            }
        }
    }
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

fn sanitize_id_for_path(id: &str) -> String {
    let normalized = id.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return "_".to_string();
    }
    normalized
        .chars()
        .map(|value| {
            if value.is_ascii_alphanumeric() || value == '_' || value == '-' || value == '.' {
                value
            } else {
                '_'
            }
        })
        .collect::<String>()
}

fn phenomenon_record_path(root: &str, phenomenon_id: PhenomenonId) -> PathBuf {
    Path::new(root).join(format!("phenomenon_{:016x}.json", phenomenon_id.0))
}

fn model_record_path(root: &str, phenomenon_id: PhenomenonId, scale: Scale, model_id: &str) -> PathBuf {
    Path::new(root).join(format!(
        "model_{:016x}_scale_{:02}_{}.json",
        phenomenon_id.0,
        scale.index_from_top(),
        sanitize_id_for_path(model_id),
    ))
}

fn partial_record_path(root: &str, phenomenon_id: PhenomenonId, scale: Scale, model_id: &str, partition_key: u64) -> PathBuf {
    Path::new(root).join(format!(
        "partial_{:016x}_scale_{:02}_{}_part_{:016x}.json",
        phenomenon_id.0,
        scale.index_from_top(),
        sanitize_id_for_path(model_id),
        partition_key,
    ))
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
            kind: PhenomenonKind::MetricSurfaceDebug,
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
            kind: PhenomenonKind::MetricSurfaceDebug,
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
            kind: PhenomenonKind::MetricSurfaceDebug,
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
                kind: PhenomenonKind::MetricSurfaceDebug,
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
}
