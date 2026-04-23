use super::field::canonical_grid_coord;
use super::runtime::{ChunkRealizationInstance, ChunkRealizationIntent, UsfChunkRealizationRuntimeSettings};
use crate::bevy::prelude::*;
use crate::bevy_rapier3d::prelude::Collider;
use crate::rhai_binding::bridges::domains::core_mod_api::usf::output_channels::{
    ChunkRealizationAudioEmitter, ChunkRealizationInteractionTrigger, ChunkRealizationParticleEmitter, ChunkRealizationSimulationService, OutputChannelPayload,
};
use crate::usf::authority::{
    USF_DOMAIN_CHUNK_REALIZATION_STATE, UsfAuthorityDiagnostics, UsfWorldAuthorityContract, guard_runtime_state_domain_with_diagnostics,
};
use crate::usf::chunk::components::Chunk;
use crate::usf::mod_packs::UsfExecutionPlan;
use crate::usf::phenomenon::{PhenomenonDefinitionRegistry, PhenomenonModel, PhenomenonModelScriptDefinitionRef, PhenomenonModelSupport};
use crate::usf::substrate::{AdaptiveSubstrateStore, SubstrateChunkDeltaState};
use crate::usf::zone::{ZoneBehaviorRegistry, ZoneId, ZoneRealizationState, ZoneRealizedPhenomenon, ZoneRuntimeState};
use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct ChunkRealizationIntentGrace {
    pub missing_frames: u32,
}

#[derive(Default)]
struct IntentSyncCounters {
    scanned_chunks: u64,
    upserted: u64,
    removed: u64,
    unchanged: u64,
    none_to_none: u64,
}
impl IntentSyncCounters {
    fn record(&mut self, action: IntentSyncAction) {
        self.scanned_chunks += 1;
        match action {
            IntentSyncAction::Unchanged => self.unchanged += 1,
            IntentSyncAction::Upserted => self.upserted += 1,
            IntentSyncAction::Removed => self.removed += 1,
            IntentSyncAction::NoneToNone => self.none_to_none += 1,
        }
    }
}

#[derive(Default)]
pub(crate) struct IntentSyncProbe {
    window_start: Option<Instant>,
    calls: u32,
    full_sync_calls: u32,
    settings_changed_calls: u32,
    execution_plan_changed_calls: u32,
    substrate_changed_calls: u32,
    definitions_changed_calls: u32,
    zone_runtime_changed_calls: u32,
    zone_realization_changed_calls: u32,
    zone_behavior_changed_calls: u32,
    dirty_chunk_only_calls: u32,
    counters: IntentSyncCounters,
}
impl IntentSyncProbe {
    fn observe(&mut self, full_sync: bool, reason_flags: IntentSyncReasonFlags, counters: IntentSyncCounters) {
        if !usf_hotpath_probe_enabled() {
            return;
        }

        let now = Instant::now();
        let window_start = self.window_start.get_or_insert(now);
        self.calls += 1;
        if full_sync {
            self.full_sync_calls += 1;
        } else if reason_flags.has_dirty_chunks {
            self.dirty_chunk_only_calls += 1;
        }
        if reason_flags.settings_changed {
            self.settings_changed_calls += 1;
        }
        if reason_flags.execution_plan_changed {
            self.execution_plan_changed_calls += 1;
        }
        if reason_flags.substrate_changed {
            self.substrate_changed_calls += 1;
        }
        if reason_flags.definitions_changed {
            self.definitions_changed_calls += 1;
        }
        if reason_flags.zone_runtime_changed {
            self.zone_runtime_changed_calls += 1;
        }
        if reason_flags.zone_realization_changed {
            self.zone_realization_changed_calls += 1;
        }
        if reason_flags.zone_behavior_changed {
            self.zone_behavior_changed_calls += 1;
        }
        self.counters.scanned_chunks += counters.scanned_chunks;
        self.counters.upserted += counters.upserted;
        self.counters.removed += counters.removed;
        self.counters.unchanged += counters.unchanged;
        self.counters.none_to_none += counters.none_to_none;

        if now.duration_since(*window_start) < Duration::from_secs(1) {
            return;
        }

        warn!(
            "USF hotpath probe [chunk_realization_intent]: calls={}, full_sync_calls={}, dirty_only_calls={}, reasons(settings={}, execution_plan={}, substrate={}, definitions={}, zone_runtime={}, zone_realization={}, zone_behavior={}), scanned_chunks={}, upserted={}, removed={}, unchanged={}, none_to_none={}",
            self.calls,
            self.full_sync_calls,
            self.dirty_chunk_only_calls,
            self.settings_changed_calls,
            self.execution_plan_changed_calls,
            self.substrate_changed_calls,
            self.definitions_changed_calls,
            self.zone_runtime_changed_calls,
            self.zone_realization_changed_calls,
            self.zone_behavior_changed_calls,
            self.counters.scanned_chunks,
            self.counters.upserted,
            self.counters.removed,
            self.counters.unchanged,
            self.counters.none_to_none
        );

        self.window_start = Some(now);
        self.calls = 0;
        self.full_sync_calls = 0;
        self.settings_changed_calls = 0;
        self.execution_plan_changed_calls = 0;
        self.substrate_changed_calls = 0;
        self.definitions_changed_calls = 0;
        self.zone_runtime_changed_calls = 0;
        self.zone_realization_changed_calls = 0;
        self.zone_behavior_changed_calls = 0;
        self.dirty_chunk_only_calls = 0;
        self.counters = IntentSyncCounters::default();
    }
}

#[derive(Default, Clone, Copy)]
struct IntentSyncReasonFlags {
    settings_changed: bool,
    execution_plan_changed: bool,
    substrate_changed: bool,
    definitions_changed: bool,
    zone_runtime_changed: bool,
    zone_realization_changed: bool,
    zone_behavior_changed: bool,
    has_dirty_chunks: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntentSyncAction {
    Unchanged,
    Upserted,
    Removed,
    NoneToNone,
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
struct RuntimeModelIntentCandidate {
    model_entity_bits: u64,
    model_id: String,
    support: Option<crate::usf::phenomenon::PhenomenonModelSupportBounds>,
}

pub(crate) fn sync_chunk_realization_intents_system(
    authority_contract: Res<UsfWorldAuthorityContract>,
    mut authority_diagnostics: Option<ResMut<UsfAuthorityDiagnostics>>,
    settings: Res<UsfChunkRealizationRuntimeSettings>,
    execution_plan: Res<UsfExecutionPlan>,
    substrate_store: Res<AdaptiveSubstrateStore>,
    substrate_delta_state: Res<SubstrateChunkDeltaState>,
    phenomenon_definitions: Res<PhenomenonDefinitionRegistry>,
    phenomenon_model_query: Query<(Entity, &PhenomenonModel, &PhenomenonModelScriptDefinitionRef, Option<&PhenomenonModelSupport>)>,
    zone_runtime_state: Res<ZoneRuntimeState>,
    zone_realization_state: Res<ZoneRealizationState>,
    zone_behavior_registry: Res<ZoneBehaviorRegistry>,
    dirty_chunks: Query<Entity, (With<Chunk>, Or<(Added<Chunk>, Changed<Chunk>)>)>,
    chunk_query: Query<(Entity, &Chunk, Option<&ChunkRealizationIntent>, Option<&ChunkRealizationIntentGrace>)>,
    mut commands: Commands,
    mut probe: Local<IntentSyncProbe>,
    mut last_seen_substrate_delta_revision: Local<u64>,
) {
    if !guard_runtime_state_domain_with_diagnostics(
        authority_contract.as_ref(),
        authority_diagnostics.as_deref_mut(),
        USF_DOMAIN_CHUNK_REALIZATION_STATE,
    ) {
        return;
    }

    let substrate_delta_changed = substrate_delta_state.revision != *last_seen_substrate_delta_revision;
    if substrate_delta_changed {
        *last_seen_substrate_delta_revision = substrate_delta_state.revision;
    }
    let has_substrate_chunk_deltas =
        substrate_delta_changed && (!substrate_delta_state.changed_chunks.is_empty() || !substrate_delta_state.removed_chunks.is_empty());
    let reason_flags = IntentSyncReasonFlags {
        settings_changed: settings.is_changed(),
        execution_plan_changed: execution_plan.is_changed(),
        substrate_changed: has_substrate_chunk_deltas,
        definitions_changed: phenomenon_definitions.is_changed(),
        zone_runtime_changed: zone_runtime_state.is_changed(),
        zone_realization_changed: zone_realization_state.is_changed(),
        zone_behavior_changed: zone_behavior_registry.is_changed(),
        has_dirty_chunks: !dirty_chunks.is_empty() || has_substrate_chunk_deltas,
    };
    let full_sync = reason_flags.settings_changed
        || reason_flags.execution_plan_changed
        || reason_flags.definitions_changed
        || reason_flags.zone_runtime_changed
        || reason_flags.zone_realization_changed
        || reason_flags.zone_behavior_changed;
    if !full_sync && !reason_flags.has_dirty_chunks {
        return;
    }
    let mut counters = IntentSyncCounters::default();
    let mut runtime_models_by_phenomenon_scale = HashMap::<(Entity, u8), Vec<RuntimeModelIntentCandidate>>::new();
    for (model_entity, model, model_ref, support) in phenomenon_model_query.iter() {
        runtime_models_by_phenomenon_scale
            .entry((model.phenomenon_entity, model.scale.index_from_top()))
            .or_default()
            .push(RuntimeModelIntentCandidate {
                model_entity_bits: model_entity.to_bits(),
                model_id: model_ref.model_id.clone(),
                support: support.map(|value| value.support.clone()),
            });
    }
    for candidates in runtime_models_by_phenomenon_scale.values_mut() {
        candidates.sort_by(|left, right| {
            left.model_id
                .cmp(&right.model_id)
                .then_with(|| left.model_entity_bits.cmp(&right.model_entity_bits))
        });
    }

    if full_sync {
        for (entity, chunk, existing_intent, existing_grace) in chunk_query.iter() {
            let action = sync_chunk_realization_intent_for_entity(
                entity,
                chunk,
                existing_intent.cloned(),
                existing_grace.copied(),
                settings.enabled,
                settings.intent_grace_frames,
                &execution_plan,
                &substrate_store,
                &phenomenon_definitions,
                &runtime_models_by_phenomenon_scale,
                &zone_runtime_state,
                &zone_realization_state,
                &zone_behavior_registry,
                &mut commands,
            );
            counters.record(action);
        }
        probe.observe(true, reason_flags, counters);
        return;
    }

    let mut dirty_entity_set = HashMap::<Entity, ()>::new();
    for entity in dirty_chunks.iter() {
        dirty_entity_set.insert(entity, ());
    }
    if has_substrate_chunk_deltas {
        for (entity, chunk, _existing_intent, _existing_grace) in chunk_query.iter() {
            let canonical = canonical_grid_coord(&chunk.coord);
            if substrate_delta_state.changed_chunks.contains(&canonical) || substrate_delta_state.removed_chunks.contains(&canonical) {
                dirty_entity_set.insert(entity, ());
            }
        }
    }

    for (entity, _) in dirty_entity_set {
        let Ok((_entity, chunk, existing_intent, existing_grace)) = chunk_query.get(entity) else {
            continue;
        };
        let action = sync_chunk_realization_intent_for_entity(
            entity,
            chunk,
            existing_intent.cloned(),
            existing_grace.copied(),
            settings.enabled,
            settings.intent_grace_frames,
            &execution_plan,
            &substrate_store,
            &phenomenon_definitions,
            &runtime_models_by_phenomenon_scale,
            &zone_runtime_state,
            &zone_realization_state,
            &zone_behavior_registry,
            &mut commands,
        );
        counters.record(action);
    }
    probe.observe(false, reason_flags, counters);
}

fn sync_chunk_realization_intent_for_entity(
    entity: Entity,
    chunk: &Chunk,
    existing_intent: Option<ChunkRealizationIntent>,
    existing_grace: Option<ChunkRealizationIntentGrace>,
    chunk_realization_runtime_enabled: bool,
    intent_grace_frames: u32,
    execution_plan: &UsfExecutionPlan,
    substrate_store: &AdaptiveSubstrateStore,
    phenomenon_definitions: &PhenomenonDefinitionRegistry,
    runtime_models_by_phenomenon_scale: &HashMap<(Entity, u8), Vec<RuntimeModelIntentCandidate>>,
    zone_runtime_state: &ZoneRuntimeState,
    zone_realization_state: &ZoneRealizationState,
    zone_behavior_registry: &ZoneBehaviorRegistry,
    commands: &mut Commands,
) -> IntentSyncAction {
    if !chunk_realization_runtime_enabled {
        if existing_intent.is_some() || existing_grace.is_some() {
            commands.entity(entity).remove::<ChunkRealizationIntent>();
            commands.entity(entity).remove::<ChunkRealizationIntentGrace>();
            commands.entity(entity).remove::<ChunkRealizationInstance>();
            commands.entity(entity).remove::<Mesh3d>();
            commands.entity(entity).remove::<MeshMaterial3d<StandardMaterial>>();
            commands.entity(entity).remove::<Collider>();
            commands.entity(entity).remove::<ChunkRealizationAudioEmitter>();
            commands.entity(entity).remove::<ChunkRealizationParticleEmitter>();
            commands.entity(entity).remove::<ChunkRealizationInteractionTrigger>();
            commands.entity(entity).remove::<ChunkRealizationSimulationService>();
            return IntentSyncAction::Removed;
        }
        return IntentSyncAction::NoneToNone;
    }

    let desired = if chunk_realization_runtime_enabled {
        desired_chunk_realization_intent(
            chunk,
            execution_plan,
            substrate_store,
            phenomenon_definitions,
            runtime_models_by_phenomenon_scale,
            zone_runtime_state,
            zone_realization_state,
            zone_behavior_registry,
        )
    } else {
        None
    };

    match (existing_intent, desired) {
        (Some(current), Some(next)) if current == next => {
            if existing_grace.is_some() {
                commands.entity(entity).remove::<ChunkRealizationIntentGrace>();
            }
            IntentSyncAction::Unchanged
        }
        (_, Some(next)) => {
            // Intent changes invalidate the current realization artifact and force deterministic rebuild.
            commands.entity(entity).insert(next);
            commands.entity(entity).remove::<ChunkRealizationIntentGrace>();
            commands.entity(entity).remove::<ChunkRealizationInstance>();
            commands.entity(entity).remove::<Mesh3d>();
            commands.entity(entity).remove::<MeshMaterial3d<StandardMaterial>>();
            commands.entity(entity).remove::<Collider>();
            commands.entity(entity).remove::<ChunkRealizationAudioEmitter>();
            commands.entity(entity).remove::<ChunkRealizationParticleEmitter>();
            commands.entity(entity).remove::<ChunkRealizationInteractionTrigger>();
            commands.entity(entity).remove::<ChunkRealizationSimulationService>();
            IntentSyncAction::Upserted
        }
        // Keep the previous intent only for a bounded grace window while authority settles.
        (Some(_), None) => {
            let grace_limit = intent_grace_frames.max(1);
            let next_missing_frames = existing_grace.map(|grace| grace.missing_frames.saturating_add(1)).unwrap_or(1);
            if next_missing_frames < grace_limit {
                commands.entity(entity).insert(ChunkRealizationIntentGrace {
                    missing_frames: next_missing_frames,
                });
                return IntentSyncAction::Unchanged;
            }

            commands.entity(entity).remove::<ChunkRealizationIntent>();
            commands.entity(entity).remove::<ChunkRealizationIntentGrace>();
            commands.entity(entity).remove::<ChunkRealizationInstance>();
            commands.entity(entity).remove::<Mesh3d>();
            commands.entity(entity).remove::<MeshMaterial3d<StandardMaterial>>();
            commands.entity(entity).remove::<Collider>();
            commands.entity(entity).remove::<ChunkRealizationAudioEmitter>();
            commands.entity(entity).remove::<ChunkRealizationParticleEmitter>();
            commands.entity(entity).remove::<ChunkRealizationInteractionTrigger>();
            commands.entity(entity).remove::<ChunkRealizationSimulationService>();
            IntentSyncAction::Removed
        }
        (None, None) => {
            if existing_grace.is_some() {
                commands.entity(entity).remove::<ChunkRealizationIntentGrace>();
            }
            IntentSyncAction::NoneToNone
        }
    }
}

fn desired_chunk_realization_intent(
    chunk: &Chunk,
    execution_plan: &UsfExecutionPlan,
    substrate_store: &AdaptiveSubstrateStore,
    phenomenon_definitions: &PhenomenonDefinitionRegistry,
    runtime_models_by_phenomenon_scale: &HashMap<(Entity, u8), Vec<RuntimeModelIntentCandidate>>,
    zone_runtime_state: &ZoneRuntimeState,
    zone_realization_state: &ZoneRealizationState,
    zone_behavior_registry: &ZoneBehaviorRegistry,
) -> Option<ChunkRealizationIntent> {
    let chunk_scale = chunk.coord.scale;
    let route = execution_plan.route_for_scale(chunk_scale)?;
    let canonical_coord = canonical_grid_coord(&chunk.coord);
    let chunk_summary = substrate_store.summary_for_chunk(&canonical_coord)?;
    let zone_type = chunk_summary.zone_type.clone();
    let zone_id = zone_runtime_state.chunk_to_zone.get(&canonical_coord)?;
    if zone_id.zone_type != zone_type {
        panic!(
            "USF chunk realization intent failed: chunk {:?} classified as zone '{}' in substrate summary, but zone runtime mapped it to '{}'.",
            canonical_coord, zone_type.0, zone_id.zone_type.0
        );
    }
    let zone_density_profile = zone_behavior_registry
        .density_profile_for_zone(&zone_type)
        .unwrap_or_else(|| panic!("USF chunk realization intent failed: missing zone density profile for zone '{}'.", zone_type.0));
    let realization = realized_phenomenon_for_zone(zone_id, zone_realization_state)?;
    let phenomenon_script_id = realization.phenomenon_script_id.clone();
    let supports_zone_phenomenon = zone_behavior_registry.supports_for_zone(&zone_type).is_some_and(|supports| {
        supports
            .iter()
            .any(|support| support.phenomenon_id.eq_ignore_ascii_case(phenomenon_script_id.as_str()))
    });
    if !supports_zone_phenomenon {
        panic!(
            "USF chunk realization intent failed: realized phenomenon '{}' is not declared in zone support policy for zone '{}'.",
            phenomenon_script_id, zone_type.0
        );
    }
    let Some(selected_model_id) =
        select_runtime_model_id_for_chunk(realization.phenomenon_entity, chunk_scale, &canonical_coord, runtime_models_by_phenomenon_scale)
    else {
        return None;
    };
    if !phenomenon_definitions.model_belongs_to_phenomenon(selected_model_id.as_str(), phenomenon_script_id.as_str()) {
        panic!(
            "USF chunk realization intent failed: selected model '{}' does not belong to phenomenon '{}' at scale {}.",
            selected_model_id,
            phenomenon_script_id,
            chunk_scale.index_from_top()
        );
    }
    let Some(output_density_field) = phenomenon_definitions.output_density_field_for_model(selected_model_id.as_str()) else {
        return None;
    };
    let output_field_spec = crate::usf::phenomenon::PhenomenonOutputFieldSpec::DensityField(output_density_field);
    let mut channel_payloads = HashMap::<String, OutputChannelPayload>::new();
    channel_payloads.insert("mesh".to_string(), OutputChannelPayload::Mesh);
    if let Some(material_profile) = phenomenon_definitions.output_material_profile_for_model(selected_model_id.as_str()) {
        channel_payloads.insert("material".to_string(), OutputChannelPayload::Material(material_profile));
    }
    if phenomenon_definitions.output_collider_enabled_for_model(selected_model_id.as_str()) {
        channel_payloads.insert("collider".to_string(), OutputChannelPayload::Collider);
    }
    if let Some(audio_emitter) = phenomenon_definitions.output_audio_emitter_for_model(selected_model_id.as_str()) {
        channel_payloads.insert("audio".to_string(), OutputChannelPayload::Audio(audio_emitter));
    }
    if let Some(particle_emitter) = phenomenon_definitions.output_particle_emitter_for_model(selected_model_id.as_str()) {
        channel_payloads.insert("particles".to_string(), OutputChannelPayload::Particles(particle_emitter));
    }
    if let Some(trigger) = phenomenon_definitions.output_interaction_trigger_for_model(selected_model_id.as_str()) {
        channel_payloads.insert("trigger".to_string(), OutputChannelPayload::Trigger(trigger));
    }
    if let Some(simulation_service) = phenomenon_definitions.simulation_service_for_model(selected_model_id.as_str()) {
        channel_payloads.insert("simulation_service".to_string(), OutputChannelPayload::SimulationService(simulation_service));
    }

    Some(ChunkRealizationIntent {
        zone_type,
        zone_density_profile,
        zone_density_signature: zone_density_profile.signature(),
        phenomenon_script_id,
        selected_model_id,
        output_field_spec,
        channel_payloads,
        chunk_store_key: route.chunk_store_key.to_string(),
    })
}

fn realized_phenomenon_for_zone<'a>(zone_id: &ZoneId, realization_state: &'a ZoneRealizationState) -> Option<&'a ZoneRealizedPhenomenon> {
    realization_state.zone_to_phenomenon.get(zone_id)
}

fn select_runtime_model_id_for_chunk(
    phenomenon_entity: Entity,
    chunk_scale: crate::usf::scale::Scale,
    canonical_coord: &crate::usf::pos::grid::types::GridVec,
    runtime_models_by_phenomenon_scale: &HashMap<(Entity, u8), Vec<RuntimeModelIntentCandidate>>,
) -> Option<String> {
    let mut matching = runtime_models_by_phenomenon_scale
        .get(&(phenomenon_entity, chunk_scale.index_from_top()))?
        .iter()
        .filter(|candidate| candidate.support.as_ref().map_or(true, |support| support.contains_chunk(canonical_coord)))
        .collect::<Vec<_>>();
    if matching.is_empty() {
        return None;
    }
    matching.sort_by(|left, right| {
        let left_radius = left.support.as_ref().map(|support| support.chunk_radius).unwrap_or(u16::MAX);
        let right_radius = right.support.as_ref().map(|support| support.chunk_radius).unwrap_or(u16::MAX);
        left_radius
            .cmp(&right_radius)
            .then_with(|| left.model_id.cmp(&right.model_id))
            .then_with(|| left.model_entity_bits.cmp(&right.model_entity_bits))
    });
    matching.first().map(|candidate| candidate.model_id.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::phenomenon::PhenomenonModelSupportBounds;
    use crate::usf::pos::grid::types::GridVec;
    use crate::usf::pos::types::GridXyz;
    use crate::usf::scale::Scale;
    use crate::usf::zone::ZoneTypeId;
    use crate::usf::zone::{StableRegionId, ZoneRealizedPhenomenon};

    fn test_zone_id() -> ZoneId {
        ZoneId {
            scale: Scale::MAX,
            zone_type: ZoneTypeId::new("mystic"),
            stable_region_id: StableRegionId(77),
        }
    }

    #[test]
    fn zone_realization_is_chunk_realization_intent_authority() {
        let zone_id = test_zone_id();
        let mut realization_state = ZoneRealizationState::default();
        realization_state.zone_to_phenomenon.insert(
            zone_id.clone(),
            ZoneRealizedPhenomenon {
                phenomenon_entity: Entity::PLACEHOLDER,
                phenomenon_script_id: "phenomenon.debug.authoritative".to_string(),
            },
        );

        let selected = realized_phenomenon_for_zone(&zone_id, &realization_state)
            .expect("expected realized phenomenon mapping")
            .phenomenon_script_id
            .clone();
        assert_eq!(selected, "phenomenon.debug.authoritative");
    }

    #[test]
    fn runtime_model_selection_prefers_more_specific_support() {
        let phenomenon_entity = Entity::from_bits(11);
        let chunk_scale = Scale::MAX;
        let chunk_coord = GridVec::new_splat(chunk_scale, GridXyz::ZERO);
        let mut runtime_models_by_phenomenon_scale = HashMap::<(Entity, u8), Vec<RuntimeModelIntentCandidate>>::new();
        runtime_models_by_phenomenon_scale.insert(
            (phenomenon_entity, chunk_scale.index_from_top()),
            vec![
                RuntimeModelIntentCandidate {
                    model_entity_bits: 100,
                    model_id: "model_fallback".to_string(),
                    support: None,
                },
                RuntimeModelIntentCandidate {
                    model_entity_bits: 200,
                    model_id: "model_specific".to_string(),
                    support: Some(PhenomenonModelSupportBounds {
                        anchor_chunk: chunk_coord.clone(),
                        chunk_radius: 0,
                    }),
                },
            ],
        );

        let selected = select_runtime_model_id_for_chunk(phenomenon_entity, chunk_scale, &chunk_coord, &runtime_models_by_phenomenon_scale)
            .expect("expected runtime model selection");
        assert_eq!(selected, "model_specific");
    }
}
