use std::collections::{HashMap, HashSet};

use crate::bevy::prelude::*;
use crate::chunk::components::ChunkLoader;
use crate::player::components::Player;

use crate::usf::phenomenon::components::{Phenomenon, PhenomenonNode, PhenomenonNodeLifecycle, PhenomenonNodeState, PhenomenonRootNodeRef};
use crate::usf::phenomenon::generator::{BuildStateInput, PhenomenonGenerator, PlanChildrenInput};
use crate::usf::phenomenon::types::{PhenomenonId, PhenomenonLineage, PhenomenonNodeKey, PhenomenonNodeSeed};
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
    pub generated_meshes_total: u64,
    pub generated_meshes_frame: u32,
    pub mesh_cache_hits_total: u64,
    pub mesh_cache_hits_frame: u32,
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

pub(super) fn sync_policy_depth_to_frontier_scale_system(
    player_loader_query: Query<&ChunkLoader, With<Player>>,
    mut policy: ResMut<PhenomenonLifecyclePolicy>,
) {
    let Ok(chunk_loader) = player_loader_query.single() else {
        return;
    };
    let frontier_depth = chunk_loader.phenomenon_frontier_view().scale.index_from_top() as u32;
    let max_allowed_depth = (Scale::SCALE_LEVEL_COUNT - 1) as u32;
    policy.max_depth = frontier_depth
        .saturating_add(policy.frontier_margin)
        .min(max_allowed_depth);
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
                Name::new(format!(
                    "phenomenon_node_{}_{}",
                    child_node.scale.index_from_top(),
                    child_node.local_index
                )),
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
            kind: PhenomenonKind::Mandelbulb,
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
            kind: PhenomenonKind::SierpinskiSponge,
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
            kind: PhenomenonKind::Mandelbulb,
        });
        app.update();
        app.update();

        let stats = app.world().resource::<PhenomenonDebugStats>();
        assert!(stats.active_nodes > 0);
    }

    #[test]
    fn lifecycle_replaces_noncanonical_root_with_pinned_root_contract() {
        let mut app = setup_lifecycle_test_app(2, 2);
        let phenomenon_entity = app.world_mut().spawn(Phenomenon {
            id: PhenomenonId(77),
            kind: PhenomenonKind::SierpinskiSponge,
        }).id();

        let bad_root_key = PhenomenonNodeKey {
            phenomenon_id: PhenomenonId(77),
            scale: Scale::MAX.zoomed_in(),
            lineage: PhenomenonLineage::from_cells(vec![
                LocalCell3::new_local(0, 0, 0),
                LocalCell3::new_local(1, 0, 0),
            ]),
            parent: None,
            local_index: 5,
        };
        app.world_mut().spawn((
            PhenomenonNode::from_key(bad_root_key.clone()),
            PhenomenonNodeState {
                snapshot: PhenomenonStateSnapshot {
                    seed: bad_root_key.deterministic_seed(),
                    lineage_depth: 0,
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
        let root_node = app
            .world()
            .get::<PhenomenonNode>(root_ref.node)
            .expect("canonical root node should exist");

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
