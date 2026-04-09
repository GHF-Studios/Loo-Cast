use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;
use crate::core::run_conditions::run_after_startup_finished;
use crate::player::components::Player;
use crate::usf::chunk::components::ChunkLoader;
use crate::usf::chunk::resources::{ChunkBatchTracker, ChunkLoadGate, ChunkManager};
use crate::usf::mod_packs::UsfRuntimeConceptView;
use crate::usf::phenomenon::PhenomenonModel;
use crate::usf::scale::Scale;
use crate::usf::zone::{ZoneBehaviorRegistry, ZoneRealizationState, ZoneRuntimeState};
use std::collections::HashSet;

fn env_bool(name: &str, default: bool) -> bool {
    match std::env::var(name) {
        Ok(raw) => match raw.trim().to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => true,
            "0" | "false" | "no" | "off" => false,
            _ => default,
        },
        Err(_) => default,
    }
}

fn env_u8(name: &str, default: u8) -> u8 {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<u8>().ok())
        .unwrap_or(default)
}

fn env_u32(name: &str, default: u32) -> u32 {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<u32>().ok())
        .unwrap_or(default)
}

fn env_f32(name: &str, default: f32) -> f32 {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<f32>().ok())
        .unwrap_or(default)
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsfBootstrapWorldgenPhase {
    Disabled,
    Stabilizing,
    Descending,
    Completed,
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfBootstrapWorldgenSettings {
    pub enabled: bool,
    pub start_scale_index: u8,
    pub target_scale_index: u8,
    pub settle_frames_per_scale: u32,
    pub zoom_step_multiplier: f32,
}
impl Default for UsfBootstrapWorldgenSettings {
    fn default() -> Self {
        let max_index = Scale::SCALE_LEVEL_COUNT.saturating_sub(1);
        let start_scale_index = env_u8("LOOCAST_USF_BOOTSTRAP_START_SCALE_INDEX", 0).min(max_index);
        let default_target = Scale::ScaleMeter1.index_from_top();
        let target_scale_index = env_u8("LOOCAST_USF_BOOTSTRAP_TARGET_SCALE_INDEX", default_target).clamp(start_scale_index, max_index);

        Self {
            enabled: env_bool("LOOCAST_USF_BOOTSTRAP_ENABLED", false),
            start_scale_index,
            target_scale_index,
            settle_frames_per_scale: env_u32("LOOCAST_USF_BOOTSTRAP_SETTLE_FRAMES", 16).max(1),
            zoom_step_multiplier: env_f32("LOOCAST_USF_BOOTSTRAP_ZOOM_STEP_MULTIPLIER", 1.08).max(1.001),
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone, Copy)]
#[reflect(Resource)]
pub struct UsfBootstrapZoomDirective {
    pub active: bool,
    pub zoom_step_multiplier: f32,
}
impl Default for UsfBootstrapZoomDirective {
    fn default() -> Self {
        Self {
            active: false,
            zoom_step_multiplier: 1.08,
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfBootstrapWorldgenState {
    pub phase: UsfBootstrapWorldgenPhase,
    pub input_locked: bool,
    pub active_scale_index: Option<u8>,
    pub target_scale_index: u8,
    pub stable_frames: u32,
    pub settle_frames_per_scale: u32,
    pub last_scale_index: Option<u8>,
}
impl Default for UsfBootstrapWorldgenState {
    fn default() -> Self {
        Self {
            phase: UsfBootstrapWorldgenPhase::Disabled,
            input_locked: false,
            active_scale_index: None,
            target_scale_index: 0,
            stable_frames: 0,
            settle_frames_per_scale: 1,
            last_scale_index: None,
        }
    }
}

fn current_slice_is_stable(
    runtime_concepts: &UsfRuntimeConceptView,
    loader: &ChunkLoader,
    chunk_manager: Option<&ChunkManager>,
    chunk_batch_tracker: Option<&ChunkBatchTracker>,
    chunk_load_gate: Option<&ChunkLoadGate>,
    zone_runtime_state: Option<&ZoneRuntimeState>,
    zone_behavior_registry: Option<&ZoneBehaviorRegistry>,
    zone_realization_state: Option<&ZoneRealizationState>,
    phenomenon_model_query: &Query<&PhenomenonModel>,
) -> bool {
    if runtime_concepts.scale_definition_for_scale(loader.scale).is_none() {
        return false;
    }
    if runtime_concepts.schema_for_scale(loader.scale).is_none() {
        return false;
    }

    if let Some(chunk_load_gate) = chunk_load_gate {
        if chunk_load_gate.is_locked() {
            return false;
        }
    }
    if let Some(chunk_batch_tracker) = chunk_batch_tracker {
        if chunk_batch_tracker.is_batch_running() || chunk_batch_tracker.is_batch_planned() {
            return false;
        }
    }
    if let Some(chunk_manager) = chunk_manager {
        if chunk_manager.active_scale != loader.scale {
            return false;
        }
        if chunk_manager.chunks.is_empty() {
            return false;
        }
    }
    if let Some(zone_runtime_state) = zone_runtime_state {
        if zone_runtime_state.records.is_empty() {
            return false;
        }
        let mut canonical_loader_coord = loader.coord.clone();
        canonical_loader_coord.normalize();
        if !zone_runtime_state.chunk_to_zone.contains_key(&canonical_loader_coord) {
            return false;
        }

        let mut current_scale_zone_ids = zone_runtime_state
            .records
            .keys()
            .filter(|zone_id| zone_id.scale == loader.scale)
            .cloned()
            .collect::<Vec<_>>();
        if current_scale_zone_ids.is_empty() {
            return false;
        }
        current_scale_zone_ids.sort_by_key(|zone_id| (zone_id.scale.index_from_top(), zone_id.zone_type.0.clone(), zone_id.stable_region_id.0));

        let Some(zone_behavior_registry) = zone_behavior_registry else {
            return false;
        };
        let Some(zone_realization_state) = zone_realization_state else {
            return false;
        };
        let ready_phenomena_at_scale = phenomenon_model_query
            .iter()
            .filter(|model| model.scale == loader.scale)
            .map(|model| model.phenomenon_entity)
            .collect::<HashSet<_>>();

        for zone_id in current_scale_zone_ids {
            let requires_phenomenon_realization = zone_behavior_registry
                .supports_for_zone(&zone_id.zone_type)
                .is_some_and(|supports| !supports.is_empty());
            if !requires_phenomenon_realization {
                continue;
            }
            let Some(realization) = zone_realization_state.zone_to_phenomenon.get(&zone_id) else {
                return false;
            };
            if !ready_phenomena_at_scale.contains(&realization.phenomenon_entity) {
                return false;
            }
        }
    }

    true
}

fn sync_usf_bootstrap_worldgen_system(
    settings: Res<UsfBootstrapWorldgenSettings>,
    runtime_concepts: Res<UsfRuntimeConceptView>,
    mut state: ResMut<UsfBootstrapWorldgenState>,
    mut zoom_directive: ResMut<UsfBootstrapZoomDirective>,
    player_loader_query: Query<&ChunkLoader, With<Player>>,
    chunk_manager: Option<Res<ChunkManager>>,
    chunk_batch_tracker: Option<Res<ChunkBatchTracker>>,
    chunk_load_gate: Option<Res<ChunkLoadGate>>,
    zone_runtime_state: Option<Res<ZoneRuntimeState>>,
    zone_behavior_registry: Option<Res<ZoneBehaviorRegistry>>,
    zone_realization_state: Option<Res<ZoneRealizationState>>,
    phenomenon_model_query: Query<&PhenomenonModel>,
) {
    let max_index = Scale::SCALE_LEVEL_COUNT.saturating_sub(1);

    if !settings.enabled {
        state.phase = UsfBootstrapWorldgenPhase::Disabled;
        state.input_locked = false;
        state.active_scale_index = None;
        state.stable_frames = 0;
        state.last_scale_index = None;
        zoom_directive.active = false;
        return;
    }

    let Ok(chunk_loader) = player_loader_query.single() else {
        zoom_directive.active = false;
        return;
    };

    let start_scale_index = settings.start_scale_index.min(max_index);
    let target_scale_index = settings.target_scale_index.clamp(start_scale_index, max_index);
    let settle_frames_per_scale = settings.settle_frames_per_scale.max(1);
    let current_scale_index = chunk_loader.scale.index_from_top();

    if state.phase == UsfBootstrapWorldgenPhase::Disabled {
        state.phase = UsfBootstrapWorldgenPhase::Stabilizing;
        state.stable_frames = 0;
        state.last_scale_index = Some(current_scale_index);
        info!(
            "USF bootstrap worldgen: enabled (start_scale={}, target_scale={}, settle_frames={})",
            start_scale_index, target_scale_index, settle_frames_per_scale
        );
    }

    state.input_locked = state.phase != UsfBootstrapWorldgenPhase::Completed;
    state.active_scale_index = Some(current_scale_index);
    state.target_scale_index = target_scale_index;
    state.settle_frames_per_scale = settle_frames_per_scale;

    if current_scale_index < start_scale_index {
        state.phase = UsfBootstrapWorldgenPhase::Stabilizing;
        state.stable_frames = 0;
        zoom_directive.active = false;
        state.last_scale_index = Some(current_scale_index);
        return;
    }

    if current_scale_index >= target_scale_index {
        if current_slice_is_stable(
            runtime_concepts.as_ref(),
            chunk_loader,
            chunk_manager.as_deref(),
            chunk_batch_tracker.as_deref(),
            chunk_load_gate.as_deref(),
            zone_runtime_state.as_deref(),
            zone_behavior_registry.as_deref(),
            zone_realization_state.as_deref(),
            &phenomenon_model_query,
        ) {
            state.stable_frames = state.stable_frames.saturating_add(1);
        } else {
            state.stable_frames = 0;
        }

        if state.stable_frames >= settle_frames_per_scale {
            state.phase = UsfBootstrapWorldgenPhase::Completed;
            state.input_locked = false;
            zoom_directive.active = false;
            info!(
                "USF bootstrap worldgen: completed at scale {} (target {}).",
                current_scale_index, target_scale_index
            );
        }
        state.last_scale_index = Some(current_scale_index);
        return;
    }

    match state.phase {
        UsfBootstrapWorldgenPhase::Stabilizing => {
            zoom_directive.active = false;
            if current_slice_is_stable(
                runtime_concepts.as_ref(),
                chunk_loader,
                chunk_manager.as_deref(),
                chunk_batch_tracker.as_deref(),
                chunk_load_gate.as_deref(),
                zone_runtime_state.as_deref(),
                zone_behavior_registry.as_deref(),
                zone_realization_state.as_deref(),
                &phenomenon_model_query,
            ) {
                state.stable_frames = state.stable_frames.saturating_add(1);
            } else {
                state.stable_frames = 0;
            }

            if state.stable_frames >= settle_frames_per_scale {
                state.phase = UsfBootstrapWorldgenPhase::Descending;
                state.stable_frames = 0;
                zoom_directive.active = true;
                info!(
                    "USF bootstrap worldgen: descending from scale {} toward target {}.",
                    current_scale_index, target_scale_index
                );
            }
        }
        UsfBootstrapWorldgenPhase::Descending => {
            zoom_directive.active = true;
            if state.last_scale_index.is_some_and(|last_scale_index| current_scale_index > last_scale_index) {
                state.phase = UsfBootstrapWorldgenPhase::Stabilizing;
                state.stable_frames = 0;
                zoom_directive.active = false;
                info!("USF bootstrap worldgen: reached next finer scale {}.", current_scale_index);
            }
        }
        UsfBootstrapWorldgenPhase::Completed => {
            zoom_directive.active = false;
            state.input_locked = false;
        }
        UsfBootstrapWorldgenPhase::Disabled => {
            zoom_directive.active = false;
        }
    }

    zoom_directive.zoom_step_multiplier = settings.zoom_step_multiplier;
    state.last_scale_index = Some(current_scale_index);
}

pub(crate) struct WorldgenPlugin;
impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfBootstrapWorldgenSettings>()
            .init_resource::<UsfBootstrapWorldgenState>()
            .init_resource::<UsfBootstrapZoomDirective>()
            .add_systems(
                Update,
                sync_usf_bootstrap_worldgen_system
                    .in_set(AppSet::Intent)
                    .run_if(run_after_startup_finished),
            )
            .register_type::<UsfBootstrapWorldgenPhase>()
            .register_type::<UsfBootstrapWorldgenSettings>()
            .register_type::<UsfBootstrapWorldgenState>()
            .register_type::<UsfBootstrapZoomDirective>();
    }
}
