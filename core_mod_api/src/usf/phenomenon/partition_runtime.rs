use std::collections::{HashMap, HashSet};

use crate::bevy::prelude::*;
use crate::config::statics::CONFIG;
use crate::usf::authority::{
    USF_DOMAIN_PARTIAL_PHENOMENON_MODEL, USF_DOMAIN_PHENOMENON_MODEL, UsfAuthorityDiagnostics, UsfWorldAuthorityContract,
    guard_canonical_domain_with_diagnostics,
};

use super::components::{
    PartialPhenomenonModel, PartitionedPhenomenonModelMember, PartitionedPhenomenonModelRoot, PhenomenonModel, PhenomenonModelProjectionContract,
    PhenomenonModelScriptDefinitionRef, PhenomenonModelState, PhenomenonModelSupport, PhenomenonModelTopology,
};
use super::persistence::{load_partial_phenomenon_model_record, partial_record_path};
use super::systems::PhenomenonPersistenceRuntimeSettings;
use crate::usf::pos::grid::types::GridVec;

#[derive(Resource, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Resource)]
pub struct PartitionRuntimeSettings {
    pub max_support_radius: u16,
    pub max_chunks_per_root: usize,
    pub generation_budget_per_frame: usize,
    pub member_mutations_per_frame: usize,
}
impl Default for PartitionRuntimeSettings {
    fn default() -> Self {
        let max_support_radius = CONFIG().get::<u16>("usf/runtime/phenomenon_partition/max_support_radius");
        let max_chunks_per_root = CONFIG().get::<usize>("usf/runtime/phenomenon_partition/max_chunks_per_root");
        let generation_budget_per_frame = CONFIG().get::<usize>("usf/runtime/phenomenon_partition/generation_budget_per_frame");
        let member_mutations_per_frame = CONFIG().get::<usize>("usf/runtime/phenomenon_partition/member_mutations_per_frame");
        if max_support_radius == 0 {
            panic!("USF phenomenon partition config is invalid: max_support_radius must be >= 1.");
        }
        if max_chunks_per_root == 0 {
            panic!("USF phenomenon partition config is invalid: max_chunks_per_root must be >= 1.");
        }
        if generation_budget_per_frame == 0 {
            panic!("USF phenomenon partition config is invalid: generation_budget_per_frame must be >= 1.");
        }
        if member_mutations_per_frame == 0 {
            panic!("USF phenomenon partition config is invalid: member_mutations_per_frame must be >= 1.");
        }
        Self {
            max_support_radius,
            max_chunks_per_root,
            generation_budget_per_frame,
            member_mutations_per_frame,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct PartitionSyncRuntimeState {
    roots: HashMap<Entity, RootPartitionSyncCursor>,
}

#[derive(Debug, Clone)]
struct RootPartitionSyncCursor {
    support_signature: u64,
    model_id: String,
    anchor_chunk: GridVec,
    radius: i32,
    side_len: usize,
    volume: usize,
    generation_cursor: usize,
    desired_chunks: Vec<GridVec>,
    desired_chunk_set: HashSet<GridVec>,
    generation_complete: bool,
    sync_cursor: usize,
    despawn_cursor: usize,
    despawn_scan_complete: bool,
    warned_cap: bool,
}
impl RootPartitionSyncCursor {
    fn new(support_signature: u64, model_id: String, anchor_chunk: GridVec, radius: i32) -> Self {
        let side_len = ((radius * 2) + 1).max(1) as usize;
        let volume = side_len * side_len * side_len;
        Self {
            support_signature,
            model_id,
            anchor_chunk,
            radius,
            side_len,
            volume,
            generation_cursor: 0,
            desired_chunks: Vec::new(),
            desired_chunk_set: HashSet::new(),
            generation_complete: false,
            sync_cursor: 0,
            despawn_cursor: 0,
            despawn_scan_complete: false,
            warned_cap: false,
        }
    }

    fn reset(&mut self, support_signature: u64, model_id: String, anchor_chunk: GridVec, radius: i32) {
        *self = Self::new(support_signature, model_id, anchor_chunk, radius);
    }
}

pub(super) fn sync_partitioned_model_members_system(
    authority_contract: Res<UsfWorldAuthorityContract>,
    mut authority_diagnostics: Option<ResMut<UsfAuthorityDiagnostics>>,
    mut commands: Commands,
    partition_runtime_settings: Res<PartitionRuntimeSettings>,
    mut runtime_state: ResMut<PartitionSyncRuntimeState>,
    settings: Res<PhenomenonPersistenceRuntimeSettings>,
    root_dirty_query: Query<
        (),
        (
            With<PartitionedPhenomenonModelRoot>,
            Or<(
                Added<PartitionedPhenomenonModelRoot>,
                Added<PhenomenonModel>,
                Changed<PhenomenonModel>,
                Added<PhenomenonModelScriptDefinitionRef>,
                Changed<PhenomenonModelScriptDefinitionRef>,
                Added<PhenomenonModelSupport>,
                Changed<PhenomenonModelSupport>,
                Added<PhenomenonModelProjectionContract>,
                Changed<PhenomenonModelProjectionContract>,
            )>,
        ),
    >,
    member_dirty_query: Query<
        (),
        (
            With<PartitionedPhenomenonModelMember>,
            Or<(
                Added<PartitionedPhenomenonModelMember>,
                Changed<PartitionedPhenomenonModelMember>,
                Added<PartialPhenomenonModel>,
                Changed<PartialPhenomenonModel>,
            )>,
        ),
    >,
    mut removed_partition_members: RemovedComponents<PartitionedPhenomenonModelMember>,
    mut removed_partition_roots: RemovedComponents<PartitionedPhenomenonModelRoot>,
    root_query: Query<
        (
            Entity,
            &PhenomenonModel,
            &PhenomenonModelScriptDefinitionRef,
            &PhenomenonModelSupport,
            &PhenomenonModelProjectionContract,
            &PhenomenonModelState,
        ),
        With<PartitionedPhenomenonModelRoot>,
    >,
    member_query: Query<
        (
            Entity,
            &PhenomenonModel,
            &PhenomenonModelScriptDefinitionRef,
            &PhenomenonModelSupport,
            &PhenomenonModelProjectionContract,
            &PartitionedPhenomenonModelMember,
            &PartialPhenomenonModel,
        ),
        With<PartitionedPhenomenonModelMember>,
    >,
) {
    if !guard_canonical_domain_with_diagnostics(authority_contract.as_ref(), authority_diagnostics.as_deref_mut(), USF_DOMAIN_PHENOMENON_MODEL) {
        return;
    }
    if !guard_canonical_domain_with_diagnostics(
        authority_contract.as_ref(),
        authority_diagnostics.as_deref_mut(),
        USF_DOMAIN_PARTIAL_PHENOMENON_MODEL,
    ) {
        return;
    }

    let has_removed_partition_members = removed_partition_members.read().next().is_some();
    let has_removed_partition_roots = removed_partition_roots.read().next().is_some();
    let has_dirty_roots = !root_dirty_query.is_empty();
    let has_dirty_members = !member_dirty_query.is_empty();
    let settings_changed = partition_runtime_settings.is_changed();
    let has_pending_runtime_work = runtime_state
        .roots
        .values()
        .any(|cursor| !cursor.generation_complete || cursor.sync_cursor < cursor.desired_chunks.len() || !cursor.despawn_scan_complete);
    if !has_dirty_roots
        && !has_dirty_members
        && !has_removed_partition_members
        && !has_removed_partition_roots
        && !settings_changed
        && !has_pending_runtime_work
    {
        return;
    }

    #[derive(Clone)]
    struct ExistingPartitionMemberSnapshot {
        entity: Entity,
        root_model_entity: Entity,
        canonical_chunk: GridVec,
        model: PhenomenonModel,
        model_script_ref: PhenomenonModelScriptDefinitionRef,
        support: PhenomenonModelSupport,
        projection: PhenomenonModelProjectionContract,
        partial: PartialPhenomenonModel,
    }

    #[derive(Clone)]
    struct RootPartitionSnapshot {
        root_entity: Entity,
        root_model: PhenomenonModel,
        model_script_ref: PhenomenonModelScriptDefinitionRef,
        support: PhenomenonModelSupport,
        projection: PhenomenonModelProjectionContract,
        state: PhenomenonModelState,
        anchor_chunk: GridVec,
        radius: i32,
        support_signature: u64,
    }

    let mut existing_by_key = HashMap::<(Entity, GridVec), ExistingPartitionMemberSnapshot>::new();
    for (member_entity, model, model_script_ref, support, projection, partition_member, partial) in member_query.iter() {
        let mut canonical_chunk = partial.chunk_coord.clone();
        canonical_chunk.normalize();
        let snapshot = ExistingPartitionMemberSnapshot {
            entity: member_entity,
            root_model_entity: partition_member.root_model_entity,
            canonical_chunk: canonical_chunk.clone(),
            model: *model,
            model_script_ref: model_script_ref.clone(),
            support: support.clone(),
            projection: projection.clone(),
            partial: partial.clone(),
        };
        if let Some(previous_member) = existing_by_key.insert((partition_member.root_model_entity, canonical_chunk.clone()), snapshot) {
            // Keep exactly one member entity per (root, chunk) key; duplicates are legacy drift.
            commands.entity(previous_member.entity).despawn();
        }
    }

    let mut existing_entries_by_root = HashMap::<Entity, Vec<(Entity, GridVec)>>::new();
    for snapshot in existing_by_key.values() {
        existing_entries_by_root
            .entry(snapshot.root_model_entity)
            .or_default()
            .push((snapshot.entity, snapshot.canonical_chunk.clone()));
    }
    for entries in existing_entries_by_root.values_mut() {
        entries.sort_by(|left, right| grid_coord_sort_key(&left.1).cmp(&grid_coord_sort_key(&right.1)));
    }

    let mut root_snapshots = Vec::<RootPartitionSnapshot>::new();
    let mut live_roots = HashSet::<Entity>::new();
    for (root_entity, root_model, model_script_ref, support, projection, state) in root_query.iter() {
        if root_model.topology != PhenomenonModelTopology::PartitionedByChunk {
            continue;
        }
        live_roots.insert(root_entity);
        let mut anchor_chunk = support.support.anchor_chunk.clone();
        anchor_chunk.normalize();
        let radius = support.support.chunk_radius.max(1).min(partition_runtime_settings.max_support_radius) as i32;
        root_snapshots.push(RootPartitionSnapshot {
            root_entity,
            root_model: *root_model,
            model_script_ref: model_script_ref.clone(),
            support: support.clone(),
            projection: projection.clone(),
            state: state.clone(),
            support_signature: partition_support_signature(&anchor_chunk, radius),
            anchor_chunk,
            radius,
        });
    }
    root_snapshots.sort_by_key(|snapshot| snapshot.root_entity.index());

    runtime_state.roots.retain(|root_entity, _| live_roots.contains(root_entity));
    for snapshot in &root_snapshots {
        runtime_state
            .roots
            .entry(snapshot.root_entity)
            .and_modify(|cursor| {
                let model_id_changed = !cursor.model_id.eq_ignore_ascii_case(snapshot.model_script_ref.model_id.as_str());
                if settings_changed || cursor.support_signature != snapshot.support_signature || model_id_changed {
                    cursor.reset(
                        snapshot.support_signature,
                        snapshot.model_script_ref.model_id.clone(),
                        snapshot.anchor_chunk.clone(),
                        snapshot.radius,
                    );
                }
            })
            .or_insert_with(|| {
                RootPartitionSyncCursor::new(
                    snapshot.support_signature,
                    snapshot.model_script_ref.model_id.clone(),
                    snapshot.anchor_chunk.clone(),
                    snapshot.radius,
                )
            });
    }

    let mut generation_budget = partition_runtime_settings.generation_budget_per_frame.max(1);
    for snapshot in &root_snapshots {
        if generation_budget == 0 {
            break;
        }
        let Some(cursor) = runtime_state.roots.get_mut(&snapshot.root_entity) else {
            continue;
        };
        advance_root_generation_cursor(
            cursor,
            &mut generation_budget,
            partition_runtime_settings.max_chunks_per_root,
            snapshot.root_entity,
        );
    }

    let mut mutation_budget = partition_runtime_settings.member_mutations_per_frame.max(1);
    for snapshot in &root_snapshots {
        if mutation_budget == 0 {
            break;
        }
        let Some(cursor) = runtime_state.roots.get_mut(&snapshot.root_entity) else {
            continue;
        };
        while mutation_budget > 0 && cursor.sync_cursor < cursor.desired_chunks.len() {
            let chunk_coord = cursor.desired_chunks[cursor.sync_cursor].clone();
            cursor.sync_cursor += 1;
            let partition_key = PartialPhenomenonModel::deterministic_partition_key(snapshot.root_model.phenomenon_id, snapshot.root_model.scale, &chunk_coord);
            let updated_partial = PartialPhenomenonModel {
                phenomenon_id: snapshot.root_model.phenomenon_id,
                scale: snapshot.root_model.scale,
                chunk_coord: chunk_coord.clone(),
                partition_key,
            };

            if let Some(existing_member) = existing_by_key.remove(&(snapshot.root_entity, chunk_coord.clone())) {
                let desired_member = PartitionedPhenomenonModelMember {
                    root_model_entity: snapshot.root_entity,
                };
                let requires_update = existing_member.model != snapshot.root_model
                    || existing_member.model_script_ref != snapshot.model_script_ref
                    || existing_member.support != snapshot.support
                    || existing_member.projection != snapshot.projection
                    || existing_member.partial != updated_partial
                    || existing_member.root_model_entity != desired_member.root_model_entity
                    || existing_member.canonical_chunk != chunk_coord;

                if requires_update {
                    commands.entity(existing_member.entity).insert((
                        Name::new(format!(
                            "phenomena_partition_scale{}_{}_part_{:016x}",
                            snapshot.root_model.scale.index_from_top(),
                            snapshot.model_script_ref.model_id,
                            partition_key
                        )),
                        snapshot.root_model,
                        snapshot.model_script_ref.clone(),
                        snapshot.support.clone(),
                        snapshot.projection.clone(),
                        updated_partial,
                        desired_member,
                    ));
                    mutation_budget = mutation_budget.saturating_sub(1);
                }
                continue;
            }

            let mut member_state = snapshot.state.clone();
            if settings.enabled {
                let partial_path = partial_record_path(
                    settings.persistence_dir.as_str(),
                    snapshot.root_model.phenomenon_id,
                    snapshot.root_model.scale,
                    snapshot.model_script_ref.model_id.as_str(),
                    partition_key,
                );
                match load_partial_phenomenon_model_record(&partial_path) {
                    Ok(Some(record)) => {
                        member_state.scalar_channels = record.scalar_channels;
                    }
                    Ok(None) => {}
                    Err(error) => {
                        warn!(
                            "USF phenomenon partition hydrate skipped: could not load partial record '{:?}': {}",
                            partial_path, error
                        );
                    }
                }
            }

            commands.spawn((
                Name::new(format!(
                    "phenomena_partition_scale{}_{}_part_{:016x}",
                    snapshot.root_model.scale.index_from_top(),
                    snapshot.model_script_ref.model_id,
                    partition_key
                )),
                snapshot.root_model,
                snapshot.model_script_ref.clone(),
                snapshot.support.clone(),
                snapshot.projection.clone(),
                member_state,
                updated_partial,
                PartitionedPhenomenonModelMember {
                    root_model_entity: snapshot.root_entity,
                },
            ));
            mutation_budget = mutation_budget.saturating_sub(1);
        }

        if mutation_budget == 0 {
            break;
        }
        if !cursor.generation_complete || cursor.sync_cursor < cursor.desired_chunks.len() {
            continue;
        }
        let root_entries = existing_entries_by_root.get(&snapshot.root_entity).cloned().unwrap_or_default();
        while mutation_budget > 0 && cursor.despawn_cursor < root_entries.len() {
            let (member_entity, canonical_chunk) = &root_entries[cursor.despawn_cursor];
            cursor.despawn_cursor += 1;
            if !cursor.desired_chunk_set.contains(canonical_chunk) {
                commands.entity(*member_entity).despawn();
                mutation_budget = mutation_budget.saturating_sub(1);
            }
        }
        cursor.despawn_scan_complete = cursor.despawn_cursor >= root_entries.len();
    }

    for (root_entity, entries) in existing_entries_by_root {
        if live_roots.contains(&root_entity) {
            continue;
        }
        for (member_entity, _) in entries {
            commands.entity(member_entity).despawn();
        }
    }
}

fn advance_root_generation_cursor(cursor: &mut RootPartitionSyncCursor, generation_budget: &mut usize, max_chunks_per_root: usize, root_model_entity: Entity) {
    while *generation_budget > 0 && !cursor.generation_complete {
        if cursor.generation_cursor >= cursor.volume {
            cursor.generation_complete = true;
            break;
        }

        let offset = partition_offset_from_linear_index(cursor.generation_cursor, cursor.side_len, cursor.radius);
        cursor.generation_cursor += 1;
        *generation_budget = generation_budget.saturating_sub(1);

        let mut chunk = cursor.anchor_chunk.clone() + offset;
        chunk.normalize();
        if cursor.desired_chunk_set.insert(chunk.clone()) {
            cursor.desired_chunks.push(chunk);
            cursor.despawn_cursor = 0;
            cursor.despawn_scan_complete = false;
        }

        if cursor.desired_chunks.len() >= max_chunks_per_root {
            cursor.generation_complete = true;
            if !cursor.warned_cap {
                warn!(
                    "USF partition expansion capped for root entity {} model '{}': generated={} capped={}.",
                    root_model_entity.index(),
                    cursor.model_id,
                    cursor.desired_chunks.len(),
                    max_chunks_per_root
                );
                cursor.warned_cap = true;
            }
            break;
        }
    }
}

fn partition_offset_from_linear_index(linear_index: usize, side_len: usize, radius: i32) -> IVec3 {
    let layer_area = side_len * side_len;
    let z_index = linear_index / layer_area;
    let remainder = linear_index % layer_area;
    let y_index = remainder / side_len;
    let x_index = remainder % side_len;
    IVec3::new(x_index as i32 - radius, y_index as i32 - radius, z_index as i32 - radius)
}

fn partition_support_signature(anchor_chunk: &GridVec, radius: i32) -> u64 {
    let mut canonical = anchor_chunk.clone();
    canonical.normalize();

    let mut state = mix64(0x9e37_79b9_7f4a_7c15_u64 ^ canonical.scale.index_from_top() as u64);
    state = mix64(state ^ radius.max(0) as u64);
    for xyz in canonical.to_raw_vec_3d() {
        state = mix64(state ^ fold_signed(xyz.x));
        state = mix64(state ^ fold_signed(xyz.y));
        state = mix64(state ^ fold_signed(xyz.z));
    }
    if state == 0 {
        return 1;
    }
    state
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

fn grid_coord_sort_key(coord: &GridVec) -> (u8, Vec<(i32, i32, i32)>) {
    let mut canonical = coord.clone();
    canonical.normalize();
    let digits = canonical.to_raw_vec_3d().into_iter().map(|xyz| (xyz.x, xyz.y, xyz.z)).collect::<Vec<_>>();
    (canonical.scale.index_from_top(), digits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::pos::types::GridXyz;

    #[test]
    fn partition_linear_offsets_are_stable() {
        let radius = 1;
        let side_len = 3;
        assert_eq!(partition_offset_from_linear_index(0, side_len, radius), IVec3::new(-1, -1, -1));
        assert_eq!(partition_offset_from_linear_index(1, side_len, radius), IVec3::new(0, -1, -1));
        assert_eq!(partition_offset_from_linear_index(3, side_len, radius), IVec3::new(-1, 0, -1));
        assert_eq!(partition_offset_from_linear_index(13, side_len, radius), IVec3::ZERO);
        assert_eq!(partition_offset_from_linear_index(26, side_len, radius), IVec3::new(1, 1, 1));
    }

    #[test]
    fn incremental_partition_generation_honors_budget_and_cap() {
        let anchor = GridVec::new_root(GridXyz::ZERO);
        let mut cursor = RootPartitionSyncCursor::new(7, "demo".to_string(), anchor.clone(), 1);
        let mut budget = 5;
        advance_root_generation_cursor(&mut cursor, &mut budget, 27, Entity::from_raw_u32(3).expect("entity index should be valid"));
        assert_eq!(cursor.desired_chunks.len(), 5);
        assert_eq!(cursor.generation_cursor, 5);
        assert!(!cursor.generation_complete);

        let mut capped_cursor = RootPartitionSyncCursor::new(11, "demo".to_string(), anchor, 2);
        let mut big_budget = 512;
        advance_root_generation_cursor(
            &mut capped_cursor,
            &mut big_budget,
            8,
            Entity::from_raw_u32(4).expect("entity index should be valid"),
        );
        assert_eq!(capped_cursor.desired_chunks.len(), 8);
        assert!(capped_cursor.generation_complete);
        assert!(capped_cursor.warned_cap);
    }
}
