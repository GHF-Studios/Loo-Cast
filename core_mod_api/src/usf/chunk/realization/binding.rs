use super::field::canonical_grid_coord;
use super::runtime::{ChunkRealizationIntent, ChunkRealizationInstance, UsfChunkRealizationRuntimeSettings};
use crate::bevy::prelude::*;
use crate::bevy_rapier3d::prelude::Collider;
use crate::usf::chunk::components::Chunk;
use crate::usf::authority::{USF_DOMAIN_CHUNK_REALIZATION_RUNTIME, UsfAuthorityDiagnostics, UsfWorldAuthorityContract, guard_runtime_state_domain_with_diagnostics};
use crate::usf::mod_packs::UsfExecutionPlan;
use crate::usf::phenomenon::PhenomenonDefinitionRegistry;
use crate::rhai_binding::bridges::domains::core_mod_api::usf::realization_channels::{
    ChunkRealizationAudioEmitter, ChunkRealizationInteractionTrigger, ChunkRealizationParticleEmitter, RealizationChannelPayload,
};
use crate::usf::substrate::AdaptiveSubstrateStore;
use crate::usf::zone::{ZoneBehaviorRegistry, ZoneId, ZoneRealizationState, ZoneRuntimeState};
use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct ChunkRealizationAuthorityGrace {
    pub missing_frames: u32,
}

#[derive(Default)]
struct BindingSyncCounters {
    scanned_chunks: u64,
    upserted: u64,
    removed: u64,
    unchanged: u64,
    none_to_none: u64,
}
impl BindingSyncCounters {
    fn record(&mut self, action: BindingSyncAction) {
        self.scanned_chunks += 1;
        match action {
            BindingSyncAction::Unchanged => self.unchanged += 1,
            BindingSyncAction::Upserted => self.upserted += 1,
            BindingSyncAction::Removed => self.removed += 1,
            BindingSyncAction::NoneToNone => self.none_to_none += 1,
        }
    }
}

#[derive(Default)]
pub(crate) struct BindingSyncProbe {
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
    counters: BindingSyncCounters,
}
impl BindingSyncProbe {
    fn observe(&mut self, full_sync: bool, reason_flags: BindingSyncReasonFlags, counters: BindingSyncCounters) {
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
        self.counters = BindingSyncCounters::default();
    }
}

#[derive(Default, Clone, Copy)]
struct BindingSyncReasonFlags {
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
enum BindingSyncAction {
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

pub(crate) fn sync_chunk_realization_intents_system(
    authority_contract: Res<UsfWorldAuthorityContract>,
    mut authority_diagnostics: Option<ResMut<UsfAuthorityDiagnostics>>,
    settings: Res<UsfChunkRealizationRuntimeSettings>,
    execution_plan: Res<UsfExecutionPlan>,
    substrate_store: Res<AdaptiveSubstrateStore>,
    phenomenon_definitions: Res<PhenomenonDefinitionRegistry>,
    zone_runtime_state: Res<ZoneRuntimeState>,
    zone_realization_state: Res<ZoneRealizationState>,
    zone_behavior_registry: Res<ZoneBehaviorRegistry>,
    dirty_chunks: Query<Entity, (With<Chunk>, Or<(Added<Chunk>, Changed<Chunk>)>)>,
    chunk_query: Query<(Entity, &Chunk, Option<&ChunkRealizationIntent>, Option<&ChunkRealizationAuthorityGrace>)>,
    mut commands: Commands,
    mut probe: Local<BindingSyncProbe>,
) {
    if !guard_runtime_state_domain_with_diagnostics(
        authority_contract.as_ref(),
        authority_diagnostics.as_deref_mut(),
        USF_DOMAIN_CHUNK_REALIZATION_RUNTIME,
    ) {
        return;
    }

    let reason_flags = BindingSyncReasonFlags {
        settings_changed: settings.is_changed(),
        execution_plan_changed: execution_plan.is_changed(),
        substrate_changed: substrate_store.is_changed(),
        definitions_changed: phenomenon_definitions.is_changed(),
        zone_runtime_changed: zone_runtime_state.is_changed(),
        zone_realization_changed: zone_realization_state.is_changed(),
        zone_behavior_changed: zone_behavior_registry.is_changed(),
        has_dirty_chunks: !dirty_chunks.is_empty(),
    };
    let full_sync = reason_flags.settings_changed
        || reason_flags.execution_plan_changed
        || reason_flags.substrate_changed
        || reason_flags.definitions_changed
        || reason_flags.zone_runtime_changed
        || reason_flags.zone_realization_changed
        || reason_flags.zone_behavior_changed;
    if !full_sync && !reason_flags.has_dirty_chunks {
        return;
    }
    let mut counters = BindingSyncCounters::default();

    if full_sync {
        for (entity, chunk, existing_binding, existing_grace) in chunk_query.iter() {
            let action = sync_chunk_realization_intent_for_entity(
                entity,
                chunk,
                existing_binding.cloned(),
                existing_grace.copied(),
                settings.enabled,
                settings.binding_grace_frames,
                &execution_plan,
                &substrate_store,
                &phenomenon_definitions,
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

    for entity in dirty_chunks.iter() {
        let Ok((_entity, chunk, existing_binding, existing_grace)) = chunk_query.get(entity) else {
            continue;
        };
        let action = sync_chunk_realization_intent_for_entity(
            entity,
            chunk,
            existing_binding.cloned(),
            existing_grace.copied(),
            settings.enabled,
            settings.binding_grace_frames,
            &execution_plan,
            &substrate_store,
            &phenomenon_definitions,
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
    existing_binding: Option<ChunkRealizationIntent>,
    existing_grace: Option<ChunkRealizationAuthorityGrace>,
    chunk_realization_runtime_enabled: bool,
    binding_grace_frames: u32,
    execution_plan: &UsfExecutionPlan,
    substrate_store: &AdaptiveSubstrateStore,
    phenomenon_definitions: &PhenomenonDefinitionRegistry,
    zone_runtime_state: &ZoneRuntimeState,
    zone_realization_state: &ZoneRealizationState,
    zone_behavior_registry: &ZoneBehaviorRegistry,
    commands: &mut Commands,
) -> BindingSyncAction {
    if !chunk_realization_runtime_enabled {
        if existing_binding.is_some() || existing_grace.is_some() {
            commands.entity(entity).remove::<ChunkRealizationIntent>();
            commands.entity(entity).remove::<ChunkRealizationAuthorityGrace>();
            commands.entity(entity).remove::<ChunkRealizationInstance>();
            commands.entity(entity).remove::<Mesh3d>();
            commands.entity(entity).remove::<MeshMaterial3d<StandardMaterial>>();
            commands.entity(entity).remove::<Collider>();
            commands.entity(entity).remove::<ChunkRealizationAudioEmitter>();
            commands.entity(entity).remove::<ChunkRealizationParticleEmitter>();
            commands.entity(entity).remove::<ChunkRealizationInteractionTrigger>();
            return BindingSyncAction::Removed;
        }
        return BindingSyncAction::NoneToNone;
    }

    let desired = if chunk_realization_runtime_enabled {
        desired_chunk_realization_intent(
            chunk,
            execution_plan,
            substrate_store,
            phenomenon_definitions,
            zone_runtime_state,
            zone_realization_state,
            zone_behavior_registry,
        )
    } else {
        None
    };

    match (existing_binding, desired) {
        (Some(current), Some(next)) if current == next => {
            if existing_grace.is_some() {
                commands.entity(entity).remove::<ChunkRealizationAuthorityGrace>();
            }
            BindingSyncAction::Unchanged
        }
        (_, Some(next)) => {
            // Intent changes invalidate the current realization artifact and force deterministic rebuild.
            commands.entity(entity).insert(next);
            commands.entity(entity).remove::<ChunkRealizationAuthorityGrace>();
            commands.entity(entity).remove::<ChunkRealizationInstance>();
            commands.entity(entity).remove::<Mesh3d>();
            commands.entity(entity).remove::<MeshMaterial3d<StandardMaterial>>();
            commands.entity(entity).remove::<Collider>();
            commands.entity(entity).remove::<ChunkRealizationAudioEmitter>();
            commands.entity(entity).remove::<ChunkRealizationParticleEmitter>();
            commands.entity(entity).remove::<ChunkRealizationInteractionTrigger>();
            BindingSyncAction::Upserted
        }
        // Keep the previous binding only for a bounded grace window while authority settles.
        (Some(_), None) => {
            let grace_limit = binding_grace_frames.max(1);
            let next_missing_frames = existing_grace.map(|grace| grace.missing_frames.saturating_add(1)).unwrap_or(1);
            if next_missing_frames < grace_limit {
                commands.entity(entity).insert(ChunkRealizationAuthorityGrace {
                    missing_frames: next_missing_frames,
                });
                return BindingSyncAction::Unchanged;
            }

            commands.entity(entity).remove::<ChunkRealizationIntent>();
            commands.entity(entity).remove::<ChunkRealizationAuthorityGrace>();
            commands.entity(entity).remove::<ChunkRealizationInstance>();
            commands.entity(entity).remove::<Mesh3d>();
            commands.entity(entity).remove::<MeshMaterial3d<StandardMaterial>>();
            commands.entity(entity).remove::<Collider>();
            commands.entity(entity).remove::<ChunkRealizationAudioEmitter>();
            commands.entity(entity).remove::<ChunkRealizationParticleEmitter>();
            commands.entity(entity).remove::<ChunkRealizationInteractionTrigger>();
            BindingSyncAction::Removed
        }
        (None, None) => {
            if existing_grace.is_some() {
                commands.entity(entity).remove::<ChunkRealizationAuthorityGrace>();
            }
            BindingSyncAction::NoneToNone
        }
    }
}

fn desired_chunk_realization_intent(
    chunk: &Chunk,
    execution_plan: &UsfExecutionPlan,
    substrate_store: &AdaptiveSubstrateStore,
    phenomenon_definitions: &PhenomenonDefinitionRegistry,
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
    let phenomenon_script_id = realized_phenomenon_script_id_for_zone(zone_id, zone_realization_state)?.to_string();
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
    let Some(realization_field_contract) = phenomenon_definitions.realization_field_contract_for_scale(phenomenon_script_id.as_str(), chunk_scale) else {
        return None;
    };
    let mut channel_payloads = HashMap::<String, RealizationChannelPayload>::new();
    channel_payloads.insert(
        "mesh".to_string(),
        RealizationChannelPayload::Mesh {
            material_profile: phenomenon_definitions.realization_material_for_scale(phenomenon_script_id.as_str(), chunk_scale),
        },
    );
    if phenomenon_definitions.realization_collider_enabled_for_scale(phenomenon_script_id.as_str(), chunk_scale) {
        channel_payloads.insert("collider".to_string(), RealizationChannelPayload::Collider);
    }
    if let Some(audio_emitter) = phenomenon_definitions.realization_audio_emitter_for_scale(phenomenon_script_id.as_str(), chunk_scale) {
        channel_payloads.insert("audio".to_string(), RealizationChannelPayload::Audio(audio_emitter));
    }
    if let Some(particle_emitter) = phenomenon_definitions.realization_particle_emitter_for_scale(phenomenon_script_id.as_str(), chunk_scale) {
        channel_payloads.insert("particles".to_string(), RealizationChannelPayload::Particles(particle_emitter));
    }
    if let Some(trigger) = phenomenon_definitions.interaction_trigger_for_scale(phenomenon_script_id.as_str(), chunk_scale) {
        channel_payloads.insert("trigger".to_string(), RealizationChannelPayload::Trigger(trigger));
    }

    Some(ChunkRealizationIntent {
        zone_type,
        zone_density_profile,
        zone_density_signature: zone_density_profile.signature(),
        phenomenon_script_id,
        realization_field_contract,
        channel_payloads,
        chunk_store_key: route.chunk_store_key.to_string(),
    })
}

fn realized_phenomenon_script_id_for_zone<'a>(zone_id: &ZoneId, realization_state: &'a ZoneRealizationState) -> Option<&'a str> {
    realization_state
        .zone_to_phenomenon
        .get(zone_id)
        .map(|realization| realization.phenomenon_script_id.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let selected = realized_phenomenon_script_id_for_zone(&zone_id, &realization_state).expect("expected realized phenomenon mapping");
        assert_eq!(selected, "phenomenon.debug.authoritative");
    }
}
