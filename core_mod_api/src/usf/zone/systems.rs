use crate::bevy::prelude::*;
use crate::chunk::components::{Chunk, ChunkLoader};
use crate::player::components::Player;
use crate::usf::definition::ZoneTypeId;
use crate::usf::phenomenon::{Phenomenon, PhenomenonId, PhenomenonModel, PhenomenonScriptDefinitionRef};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use crate::usf::substrate::AdaptiveSubstrateStore;
use std::collections::{HashMap, HashSet};

use super::resources::{ZoneBehaviorRegistry, ZonePhenomenonSpawnPolicy, ZoneRealizationState, ZoneRealizedPhenomenon, ZoneRuntimeState, ZoneTemporalContext};
use super::types::{StableRegionId, ZoneAnchor, ZoneExtent, ZoneId, ZonePhenomenon, ZoneRealizationEvent, ZoneTimeFactor};
use super::{select_supported_phenomenon_for_zone, support_count_key};

pub(super) fn sync_zone_temporal_context_system(player_loader_query: Query<&ChunkLoader, With<Player>>, mut temporal_context: ResMut<ZoneTemporalContext>) {
    let Ok(chunk_loader) = player_loader_query.single() else {
        return;
    };
    temporal_context.active_scale = chunk_loader.scale;
}

pub(super) fn reconcile_zone_runtime_system(
    mut commands: Commands,
    substrate_store: Res<AdaptiveSubstrateStore>,
    temporal_context: Res<ZoneTemporalContext>,
    loaded_chunks: Query<&Chunk>,
    mut runtime_state: ResMut<ZoneRuntimeState>,
    mut zone_anchor_query: Query<(&mut ZoneAnchor, &mut ZoneTimeFactor)>,
) {
    let mut classified_chunks = HashMap::<(Scale, ZoneTypeId), Vec<GridVec>>::new();
    for chunk in loaded_chunks.iter() {
        let mut canonical_coord = chunk.coord.clone();
        canonical_coord.normalize();
        let Some(chunk_summary) = substrate_store.summary_for_chunk(&canonical_coord) else {
            continue;
        };
        let zone_type = chunk_summary.zone_type.clone();
        classified_chunks.entry((canonical_coord.scale, zone_type)).or_default().push(canonical_coord);
    }

    let mut components = Vec::<ClassifiedZoneComponent>::new();
    for ((scale, zone_type), coords) in classified_chunks {
        for mut chunk_coords in connected_chunk_components(coords) {
            sort_grid_coords(&mut chunk_coords);
            components.push(ClassifiedZoneComponent {
                scale,
                zone_type: zone_type.clone(),
                chunk_coords,
            });
        }
    }
    let next_records = assign_zone_records_from_components(components, &runtime_state.records);

    let next_chunk_to_zone = build_chunk_to_zone_index(&next_records);
    let next_parent_by_zone = compute_zone_parent_map(&next_records, &next_chunk_to_zone);

    let stale_ids = runtime_state
        .records
        .keys()
        .filter(|zone_id| !next_records.contains_key(*zone_id))
        .cloned()
        .collect::<Vec<_>>();
    for zone_id in stale_ids {
        runtime_state.records.remove(&zone_id);
        if let Some(entity) = runtime_state.entities.remove(&zone_id) {
            commands.entity(entity).despawn();
        }
    }

    for (zone_id, extent) in &next_records {
        let time_factor = temporal_context.time_factor_for_scale(zone_id.scale);
        let parent = next_parent_by_zone.get(zone_id).cloned();
        if let Some(entity) = runtime_state.entities.get(zone_id).copied() {
            if let Ok((mut anchor, mut zone_time_factor)) = zone_anchor_query.get_mut(entity) {
                anchor.chunk_count = extent.chunk_coords.len() as u32;
                anchor.parent = parent;
                zone_time_factor.value = time_factor;
            } else {
                let entity = spawn_zone_anchor(&mut commands, zone_id, extent, parent, time_factor);
                runtime_state.entities.insert(zone_id.clone(), entity);
            }
            continue;
        }

        let entity = spawn_zone_anchor(&mut commands, zone_id, extent, parent, time_factor);
        runtime_state.entities.insert(zone_id.clone(), entity);
    }

    runtime_state.records = next_records;
    runtime_state.chunk_to_zone = next_chunk_to_zone;
    runtime_state.parent_by_zone = next_parent_by_zone;
}

pub(super) fn reconcile_zone_realization_system(
    mut commands: Commands,
    runtime_state: Res<ZoneRuntimeState>,
    zone_behavior_registry: Res<ZoneBehaviorRegistry>,
    temporal_context: Res<ZoneTemporalContext>,
    mut realization_state: ResMut<ZoneRealizationState>,
    zone_phenomenon_query: Query<(Entity, &ZonePhenomenon, &PhenomenonScriptDefinitionRef), With<Phenomenon>>,
    phenomenon_model_query: Query<(Entity, &PhenomenonModel)>,
    mut zone_realization_event_writer: MessageWriter<ZoneRealizationEvent>,
) {
    // Active-scale authority: zone realizations follow the player's currently active scale.
    // This keeps spawned phenomena coherent through USF zoom transitions.
    let active_scale = temporal_context.active_scale;
    let desired_zone_ids = runtime_state
        .records
        .keys()
        .filter(|zone_id| zone_id.scale == active_scale)
        .cloned()
        .collect::<HashSet<_>>();
    let live_zone_realizations = zone_phenomenon_query
        .iter()
        .map(|(phenomenon_entity, marker, script_ref)| {
            (
                marker.zone_id.clone(),
                ZoneRealizedPhenomenon {
                    phenomenon_entity,
                    phenomenon_script_id: script_ref.phenomenon_id.clone(),
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let mut model_entities_by_phenomenon = HashMap::<Entity, Vec<Entity>>::new();
    for (model_entity, model) in phenomenon_model_query.iter() {
        model_entities_by_phenomenon.entry(model.phenomenon_entity).or_default().push(model_entity);
    }

    let stale_zone_ids = realization_state
        .zone_to_phenomenon
        .keys()
        .filter(|zone_id| !desired_zone_ids.contains(*zone_id))
        .cloned()
        .collect::<Vec<_>>();
    let stale_zone_id_set = stale_zone_ids.iter().cloned().collect::<HashSet<_>>();
    for zone_id in stale_zone_ids {
        if let Some(realization) = realization_state.zone_to_phenomenon.remove(&zone_id) {
            zone_realization_event_writer.write(ZoneRealizationEvent::Despawned {
                zone_id: zone_id.clone(),
                phenomenon_entity: realization.phenomenon_entity,
            });
            if let Some(model_entities) = model_entities_by_phenomenon.get(&realization.phenomenon_entity) {
                for model_entity in model_entities {
                    commands.entity(*model_entity).despawn();
                }
            }
            commands.entity(realization.phenomenon_entity).despawn();
        }
    }

    for (zone_id, realization) in &live_zone_realizations {
        if desired_zone_ids.contains(zone_id) || stale_zone_id_set.contains(zone_id) {
            continue;
        }
        zone_realization_event_writer.write(ZoneRealizationEvent::Despawned {
            zone_id: zone_id.clone(),
            phenomenon_entity: realization.phenomenon_entity,
        });
        if let Some(model_entities) = model_entities_by_phenomenon.get(&realization.phenomenon_entity) {
            for model_entity in model_entities {
                commands.entity(*model_entity).despawn();
            }
        }
        commands.entity(realization.phenomenon_entity).despawn();
        realization_state.zone_to_phenomenon.remove(zone_id);
    }

    let mut support_active_counts = HashMap::<(ZoneTypeId, String), u32>::new();
    for zone_id in &desired_zone_ids {
        let Some(realization) = realization_state
            .zone_to_phenomenon
            .get(zone_id)
            .cloned()
            .or_else(|| live_zone_realizations.get(zone_id).cloned())
        else {
            continue;
        };
        *support_active_counts
            .entry(support_count_key(&zone_id.zone_type, &realization.phenomenon_script_id))
            .or_default() += 1;
    }

    let mut desired_zone_ids_sorted = desired_zone_ids.into_iter().collect::<Vec<_>>();
    desired_zone_ids_sorted.sort_by_key(zone_id_sort_key);
    for zone_id in desired_zone_ids_sorted {
        let Some(realization) = realization_state
            .zone_to_phenomenon
            .get(&zone_id)
            .cloned()
            .or_else(|| live_zone_realizations.get(&zone_id).cloned())
        else {
            let Some(selected_support) =
                select_supported_phenomenon_for_zone(&zone_id, &zone_behavior_registry, &support_active_counts, temporal_context.active_scale)
            else {
                continue;
            };
            let phenomenon_id = deterministic_phenomenon_id_for_zone(&zone_id);
            if selected_support.spawn_policy != ZonePhenomenonSpawnPolicy::SinglePerZone {
                panic!(
                    "USF zone realization failed: unsupported spawn policy '{:?}' for zone '{}'.",
                    selected_support.spawn_policy, zone_id.zone_type.0
                );
            }
            let selected_phenomenon_script_id = selected_support.phenomenon_id.clone();
            let phenomenon_entity = commands
                .spawn((
                    Name::new(format!(
                        "zone_phenomenon_container_scale{}_{}_{}",
                        zone_id.scale.index_from_top(),
                        zone_id.zone_type.0,
                        zone_id.stable_region_id.0
                    )),
                    Phenomenon {
                        id: phenomenon_id,
                        kind: selected_support.kind,
                    },
                    PhenomenonScriptDefinitionRef {
                        phenomenon_id: selected_phenomenon_script_id.clone(),
                    },
                    ZonePhenomenon { zone_id: zone_id.clone() },
                ))
                .id();
            let realization = ZoneRealizedPhenomenon {
                phenomenon_entity,
                phenomenon_script_id: selected_phenomenon_script_id.clone(),
            };
            zone_realization_event_writer.write(ZoneRealizationEvent::Spawned {
                zone_id: zone_id.clone(),
                phenomenon_entity,
                phenomenon_id,
            });
            *support_active_counts
                .entry(support_count_key(&zone_id.zone_type, &selected_phenomenon_script_id))
                .or_default() += 1;
            realization_state.zone_to_phenomenon.insert(zone_id, realization);
            continue;
        };

        realization_state.zone_to_phenomenon.insert(zone_id, realization);
    }
}

fn spawn_zone_anchor(commands: &mut Commands, zone_id: &ZoneId, extent: &ZoneExtent, parent: Option<ZoneId>, time_factor: f32) -> Entity {
    commands
        .spawn((
            Name::new(format!(
                "zone_anchor_scale{}_{}_{}",
                zone_id.scale.index_from_top(),
                zone_id.zone_type.0,
                zone_id.stable_region_id.0
            )),
            ZoneAnchor {
                id: zone_id.clone(),
                chunk_count: extent.chunk_coords.len() as u32,
                parent,
            },
            ZoneTimeFactor { value: time_factor },
        ))
        .id()
}

fn connected_chunk_components(coords: Vec<GridVec>) -> Vec<Vec<GridVec>> {
    let mut remaining = coords.into_iter().collect::<HashSet<_>>();
    let mut components = Vec::<Vec<GridVec>>::new();
    const NEIGHBOR_OFFSETS: [IVec3; 6] = [
        IVec3::new(1, 0, 0),
        IVec3::new(-1, 0, 0),
        IVec3::new(0, 1, 0),
        IVec3::new(0, -1, 0),
        IVec3::new(0, 0, 1),
        IVec3::new(0, 0, -1),
    ];

    while let Some(start) = remaining.iter().next().cloned() {
        remaining.remove(&start);
        let mut stack = vec![start.clone()];
        let mut component = vec![start];

        while let Some(cursor) = stack.pop() {
            for offset in NEIGHBOR_OFFSETS {
                let neighbor = cursor.clone() + offset;
                if !remaining.remove(&neighbor) {
                    continue;
                }
                stack.push(neighbor.clone());
                component.push(neighbor);
            }
        }

        components.push(component);
    }

    components
}

#[derive(Debug, Clone)]
struct ClassifiedZoneComponent {
    scale: Scale,
    zone_type: ZoneTypeId,
    chunk_coords: Vec<GridVec>,
}

fn assign_zone_records_from_components(
    mut components: Vec<ClassifiedZoneComponent>,
    previous_records: &HashMap<ZoneId, ZoneExtent>,
) -> HashMap<ZoneId, ZoneExtent> {
    components.sort_by(|a, b| component_sort_key(a).cmp(&component_sort_key(b)));

    let mut previous_by_signature = HashMap::<(Scale, ZoneTypeId), Vec<(ZoneId, HashSet<GridVec>)>>::new();
    for (zone_id, extent) in previous_records {
        previous_by_signature
            .entry((zone_id.scale, zone_id.zone_type.clone()))
            .or_default()
            .push((zone_id.clone(), extent.chunk_coords.iter().cloned().collect()));
    }
    for entries in previous_by_signature.values_mut() {
        entries.sort_by(|(a_zone_id, _), (b_zone_id, _)| zone_id_sort_key(a_zone_id).cmp(&zone_id_sort_key(b_zone_id)));
    }

    let mut reused_zone_ids = HashSet::<ZoneId>::new();
    let mut next_records = HashMap::<ZoneId, ZoneExtent>::new();

    for component in components {
        let signature = (component.scale, component.zone_type.clone());
        let reused_zone_id = previous_by_signature.get(&signature).and_then(|candidates| {
            candidates
                .iter()
                .filter(|(zone_id, _)| !reused_zone_ids.contains(zone_id))
                .filter_map(|(zone_id, previous_coords)| {
                    let overlap = component.chunk_coords.iter().filter(|coord| previous_coords.contains(*coord)).count();
                    (overlap > 0).then_some((overlap, zone_id.clone()))
                })
                .max_by(|(a_overlap, a_zone_id), (b_overlap, b_zone_id)| {
                    a_overlap
                        .cmp(b_overlap)
                        .then_with(|| zone_id_sort_key(b_zone_id).cmp(&zone_id_sort_key(a_zone_id)))
                })
                .map(|(_, zone_id)| zone_id)
        });

        let zone_id = if let Some(zone_id) = reused_zone_id {
            reused_zone_ids.insert(zone_id.clone());
            zone_id
        } else {
            allocate_new_zone_id(&component, &next_records)
        };
        next_records.insert(
            zone_id,
            ZoneExtent {
                chunk_coords: component.chunk_coords,
            },
        );
    }

    next_records
}

fn allocate_new_zone_id(component: &ClassifiedZoneComponent, existing_records: &HashMap<ZoneId, ZoneExtent>) -> ZoneId {
    let base = compute_stable_region_id(component.scale, &component.zone_type, &component.chunk_coords);
    let mut candidate_stable_id = base.0;
    let mut salt = 0_u64;
    loop {
        let zone_id = ZoneId {
            scale: component.scale,
            zone_type: component.zone_type.clone(),
            stable_region_id: StableRegionId(candidate_stable_id),
        };
        if !existing_records.contains_key(&zone_id) {
            return zone_id;
        }
        salt = salt.wrapping_add(1);
        candidate_stable_id = mix64(base.0 ^ salt.wrapping_mul(0x9e37_79b9_7f4a_7c15));
        if candidate_stable_id == 0 {
            candidate_stable_id = 1;
        }
    }
}

fn component_sort_key(component: &ClassifiedZoneComponent) -> (u8, String, Vec<Vec<(i32, i32, i32)>>) {
    (
        component.scale.index_from_top(),
        component.zone_type.0.to_ascii_lowercase(),
        component.chunk_coords.iter().map(grid_coord_sort_key).collect(),
    )
}

fn build_chunk_to_zone_index(records: &HashMap<ZoneId, ZoneExtent>) -> HashMap<GridVec, ZoneId> {
    let mut index = HashMap::new();
    for (zone_id, extent) in records {
        for coord in &extent.chunk_coords {
            index.insert(coord.clone(), zone_id.clone());
        }
    }
    index
}

fn compute_zone_parent_map(records: &HashMap<ZoneId, ZoneExtent>, chunk_to_zone: &HashMap<GridVec, ZoneId>) -> HashMap<ZoneId, ZoneId> {
    let mut parent_by_zone = HashMap::new();

    for (zone_id, extent) in records {
        if zone_id.scale == Scale::MAX {
            continue;
        }

        let expected_parent_scale = zone_id.scale.zoomed_out();
        let mut parent_votes = HashMap::<ZoneId, usize>::new();

        for coord in &extent.chunk_coords {
            let Some(parent_coord) = coord.parent.as_ref().map(|parent| parent.as_ref().clone()) else {
                continue;
            };
            let Some(parent_zone_id) = chunk_to_zone.get(&parent_coord) else {
                continue;
            };
            if parent_zone_id.scale != expected_parent_scale {
                continue;
            }
            *parent_votes.entry(parent_zone_id.clone()).or_default() += 1;
        }

        let selected_parent = parent_votes
            .into_iter()
            .min_by(|(a_zone, a_votes), (b_zone, b_votes)| b_votes.cmp(a_votes).then_with(|| zone_id_sort_key(a_zone).cmp(&zone_id_sort_key(b_zone))));
        if let Some((parent_zone_id, _)) = selected_parent {
            parent_by_zone.insert(zone_id.clone(), parent_zone_id);
        }
    }

    parent_by_zone
}

fn sort_grid_coords(coords: &mut [GridVec]) {
    coords.sort_by(|a, b| grid_coord_sort_key(a).cmp(&grid_coord_sort_key(b)));
}

fn grid_coord_sort_key(coord: &GridVec) -> Vec<(i32, i32, i32)> {
    coord.to_raw_vec_3d().into_iter().map(|xyz| (xyz.x, xyz.y, xyz.z)).collect()
}

fn zone_id_sort_key(zone_id: &ZoneId) -> (u8, String, u64) {
    (
        zone_id.scale.index_from_top(),
        zone_id.zone_type.0.to_ascii_lowercase(),
        zone_id.stable_region_id.0,
    )
}

fn deterministic_phenomenon_id_for_zone(zone_id: &ZoneId) -> PhenomenonId {
    let mut state = mix64(0x9e37_79b9_7f4a_7c15 ^ zone_id.scale.index_from_top() as u64);
    for byte in zone_id.zone_type.0.as_bytes() {
        state = mix64(state ^ *byte as u64);
    }
    state = mix64(state ^ zone_id.stable_region_id.0);

    if state == 0 {
        return PhenomenonId(1);
    }
    PhenomenonId(state)
}

fn compute_stable_region_id(scale: Scale, zone_type: &ZoneTypeId, coords: &[GridVec]) -> StableRegionId {
    let mut state = mix64(0xc6a4_a793_5bd1_e995 ^ scale.index_from_top() as u64);
    for byte in zone_type.0.as_bytes() {
        state = mix64(state ^ *byte as u64);
    }

    for coord in coords {
        for xyz in coord.to_raw_vec_3d() {
            state = mix64(state ^ fold_signed(xyz.x));
            state = mix64(state ^ fold_signed(xyz.y));
            state = mix64(state ^ fold_signed(xyz.z));
        }
    }

    if state == 0 {
        return StableRegionId(0x9e37_79b9_7f4a_7c15);
    }
    StableRegionId(state)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::pos::types::GridXyz;
    use crate::usf::zone::ZonePhenomenonSupport;

    fn zone_id(scale: Scale, zone_type: &str, stable_region_id: u64) -> ZoneId {
        ZoneId {
            scale,
            zone_type: ZoneTypeId::new(zone_type),
            stable_region_id: StableRegionId(stable_region_id),
        }
    }

    fn one_level_coord(root_xyz: GridXyz, child_xyz: GridXyz) -> GridVec {
        GridVec::new(GridVec::new_root(root_xyz), child_xyz)
    }

    #[test]
    fn zone_parent_selection_uses_majority_vote_and_deterministic_tie_break() {
        let root_scale = Scale::MAX;
        let child_scale = root_scale.zoomed_in();
        let parent_a = zone_id(root_scale, "forest", 1);
        let parent_b = zone_id(root_scale, "arid", 2);
        let child = zone_id(child_scale, "wetland", 3);

        let child_coords = vec![
            GridVec::new(GridVec::new_root(GridXyz::new_local(0, 0, 0)), GridXyz::new_local(0, 0, 0)),
            GridVec::new(GridVec::new_root(GridXyz::new_local(0, 0, 0)), GridXyz::new_local(1, 0, 0)),
            GridVec::new(GridVec::new_root(GridXyz::new_local(1, 0, 0)), GridXyz::new_local(0, 0, 0)),
        ];

        let mut records = HashMap::new();
        records.insert(
            parent_a.clone(),
            ZoneExtent {
                chunk_coords: vec![GridVec::new_root(GridXyz::new_local(0, 0, 0))],
            },
        );
        records.insert(
            parent_b.clone(),
            ZoneExtent {
                chunk_coords: vec![GridVec::new_root(GridXyz::new_local(1, 0, 0))],
            },
        );
        records.insert(child.clone(), ZoneExtent { chunk_coords: child_coords });

        let chunk_to_zone = build_chunk_to_zone_index(&records);
        let parent_map = compute_zone_parent_map(&records, &chunk_to_zone);

        assert_eq!(parent_map.get(&child), Some(&parent_a));
    }

    #[test]
    fn zone_phenomenon_id_is_deterministic() {
        let zone = zone_id(Scale::MAX.zoomed_in(), "forest", 99);
        let a = deterministic_phenomenon_id_for_zone(&zone);
        let b = deterministic_phenomenon_id_for_zone(&zone);
        assert_eq!(a, b);
    }

    #[test]
    fn phenomenon_kind_uses_registry_mapping_when_present() {
        let mut registry = ZoneBehaviorRegistry {
            phenomenon_support_by_zone: HashMap::new(),
            selection_policy_by_zone: HashMap::new(),
            density_profile_by_zone: HashMap::new(),
        };
        registry.phenomenon_support_by_zone.insert(
            ZoneTypeId::new("mystic"),
            vec![ZonePhenomenonSupport {
                phenomenon_id: "phenomenon.demo.manifestation_density".to_string(),
                kind: crate::usf::phenomenon::PhenomenonKind::ManifestationDensityDebug,
                priority: 100,
                weight: 1.0,
                spawn_policy: ZonePhenomenonSpawnPolicy::SinglePerZone,
                max_active: 1,
            }],
        );
        registry.selection_policy_by_zone.insert(
            ZoneTypeId::new("mystic"),
            crate::usf::zone::ZoneSelectionPolicy {
                strategy: crate::usf::zone::ZonePhenomenonSelectionStrategy::WeightedTopPriority,
            },
        );

        let kind = select_supported_phenomenon_for_zone(&zone_id(Scale::MAX, "mystic", 1234), &registry, &HashMap::new(), Scale::MAX)
            .expect("expected support selection")
            .kind;
        assert_eq!(kind, crate::usf::phenomenon::PhenomenonKind::ManifestationDensityDebug);
    }

    #[test]
    fn support_selection_respects_max_active_limit() {
        let mut registry = ZoneBehaviorRegistry {
            phenomenon_support_by_zone: HashMap::new(),
            selection_policy_by_zone: HashMap::new(),
            density_profile_by_zone: HashMap::new(),
        };
        registry.phenomenon_support_by_zone.insert(
            ZoneTypeId::new("mystic"),
            vec![
                ZonePhenomenonSupport {
                    phenomenon_id: "phenomenon.debug.alpha".to_string(),
                    kind: crate::usf::phenomenon::PhenomenonKind::ManifestationDensityDebug,
                    priority: 100,
                    weight: 1.0,
                    spawn_policy: ZonePhenomenonSpawnPolicy::SinglePerZone,
                    max_active: 1,
                },
                ZonePhenomenonSupport {
                    phenomenon_id: "phenomenon.debug.beta".to_string(),
                    kind: crate::usf::phenomenon::PhenomenonKind::ManifestationDensityDebug,
                    priority: 100,
                    weight: 1.0,
                    spawn_policy: ZonePhenomenonSpawnPolicy::SinglePerZone,
                    max_active: 1,
                },
            ],
        );
        registry.selection_policy_by_zone.insert(
            ZoneTypeId::new("mystic"),
            crate::usf::zone::ZoneSelectionPolicy {
                strategy: crate::usf::zone::ZonePhenomenonSelectionStrategy::HighestWeightTopPriority,
            },
        );

        let mut active_counts = HashMap::new();
        active_counts.insert((ZoneTypeId::new("mystic"), "phenomenon.debug.alpha".to_string()), 1);
        let selected = select_supported_phenomenon_for_zone(&zone_id(Scale::MAX, "mystic", 7), &registry, &active_counts, Scale::MAX)
            .expect("expected fallback support when highest-priority support is saturated");

        assert_eq!(selected.phenomenon_id, "phenomenon.debug.beta");
    }

    #[test]
    fn support_selection_returns_none_when_all_supports_saturated() {
        let mut registry = ZoneBehaviorRegistry {
            phenomenon_support_by_zone: HashMap::new(),
            selection_policy_by_zone: HashMap::new(),
            density_profile_by_zone: HashMap::new(),
        };
        registry.phenomenon_support_by_zone.insert(
            ZoneTypeId::new("mystic"),
            vec![ZonePhenomenonSupport {
                phenomenon_id: "phenomenon.debug.alpha".to_string(),
                kind: crate::usf::phenomenon::PhenomenonKind::ManifestationDensityDebug,
                priority: 100,
                weight: 1.0,
                spawn_policy: ZonePhenomenonSpawnPolicy::SinglePerZone,
                max_active: 1,
            }],
        );
        registry.selection_policy_by_zone.insert(
            ZoneTypeId::new("mystic"),
            crate::usf::zone::ZoneSelectionPolicy {
                strategy: crate::usf::zone::ZonePhenomenonSelectionStrategy::WeightedTopPriority,
            },
        );

        let mut active_counts = HashMap::new();
        active_counts.insert((ZoneTypeId::new("mystic"), "phenomenon.debug.alpha".to_string()), 1);

        let selected = select_supported_phenomenon_for_zone(&zone_id(Scale::MAX, "mystic", 8), &registry, &active_counts, Scale::MAX);

        assert!(selected.is_none());
    }

    #[test]
    fn zone_record_assignment_reuses_zone_id_when_component_expands() {
        let scale = Scale::MAX.zoomed_in();
        let prior_zone = zone_id(scale, "forest", 42);
        let chunk_a = one_level_coord(GridXyz::new_local(0, 0, 0), GridXyz::new_local(0, 0, 0));
        let chunk_b = one_level_coord(GridXyz::new_local(0, 0, 0), GridXyz::new_local(1, 0, 0));

        let mut previous_records = HashMap::new();
        previous_records.insert(
            prior_zone.clone(),
            ZoneExtent {
                chunk_coords: vec![chunk_a.clone()],
            },
        );

        let next_records = assign_zone_records_from_components(
            vec![ClassifiedZoneComponent {
                scale,
                zone_type: ZoneTypeId::new("forest"),
                chunk_coords: vec![chunk_a, chunk_b],
            }],
            &previous_records,
        );

        let Some(extent) = next_records.get(&prior_zone) else {
            panic!("expected prior zone id to be reused");
        };
        assert_eq!(extent.chunk_coords.len(), 2);
    }

    #[test]
    fn zone_record_assignment_uses_deterministic_tie_break_on_equal_overlap() {
        let scale = Scale::MAX.zoomed_in();
        let zone_low = zone_id(scale, "forest", 1);
        let zone_high = zone_id(scale, "forest", 2);
        let chunk_a = one_level_coord(GridXyz::new_local(0, 0, 0), GridXyz::new_local(0, 0, 0));
        let chunk_b = one_level_coord(GridXyz::new_local(0, 0, 0), GridXyz::new_local(2, 0, 0));

        let mut previous_records = HashMap::new();
        previous_records.insert(
            zone_low.clone(),
            ZoneExtent {
                chunk_coords: vec![chunk_a.clone()],
            },
        );
        previous_records.insert(
            zone_high.clone(),
            ZoneExtent {
                chunk_coords: vec![chunk_b.clone()],
            },
        );

        let next_records = assign_zone_records_from_components(
            vec![ClassifiedZoneComponent {
                scale,
                zone_type: ZoneTypeId::new("forest"),
                chunk_coords: vec![chunk_a, chunk_b],
            }],
            &previous_records,
        );

        assert!(next_records.contains_key(&zone_low));
        assert!(!next_records.contains_key(&zone_high));
    }
}
