mod resources;
mod systems;
mod types;

use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;
use crate::usf::content::UsfActiveModpack;
use crate::usf::schedule::{UsfSimulationSet, UsfZoneSet};

pub use resources::{
    ZoneBehaviorRegistry, ZoneDensityProfile, ZonePhenomenonSelectionStrategy, ZonePhenomenonSpawnPolicy, ZonePhenomenonSupport, ZoneRealizationState,
    ZoneRealizedPhenomenon, ZoneRuntimeState, ZoneSelectionPolicy, ZoneTemporalContext, time_scale_for_levels_above, time_scale_for_scale,
    time_scale_for_scale_indices,
};
pub use types::{StableRegionId, ZoneAnchor, ZoneExtent, ZoneId, ZonePhenomenon, ZoneRealizationEvent, ZoneTimeFactor};

use systems::{reconcile_zone_realization_system, reconcile_zone_runtime_system, sync_zone_temporal_context_system};

fn validate_zone_behavior_registry_system(active_modpack: Res<UsfActiveModpack>, zone_behavior_registry: Res<ZoneBehaviorRegistry>) {
    for zone_type in &active_modpack.known_zone_types {
        if zone_behavior_registry.supports_for_zone(zone_type).is_none() {
            panic!(
                "USF zone behavior validation failed: missing supported phenomena for zone '{}' declared by active modpack",
                zone_type.0
            );
        }
        if zone_behavior_registry.selection_policy_for_zone(zone_type).is_none() {
            panic!(
                "USF zone behavior validation failed: missing selection policy for zone '{}' declared by active modpack",
                zone_type.0
            );
        }
        if zone_behavior_registry.density_profile_for_zone(zone_type).is_none() {
            panic!(
                "USF zone behavior validation failed: missing density profile for zone '{}' declared by active modpack",
                zone_type.0
            );
        }
    }

    for zone_type in zone_behavior_registry.phenomenon_support_by_zone.keys() {
        if !active_modpack.known_zone_types.contains(zone_type) {
            panic!(
                "USF zone behavior validation failed: zone '{}' has behavior but is not declared in active modpack zone types",
                zone_type.0
            );
        }
    }
}

pub(crate) struct ZonePlugin;
impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ZoneRuntimeState>()
            .init_resource::<ZoneRealizationState>()
            .init_resource::<ZoneBehaviorRegistry>()
            .init_resource::<ZoneTemporalContext>()
            .add_message::<ZoneRealizationEvent>()
            .add_systems(Startup, validate_zone_behavior_registry_system.in_set(AppSet::Diagnostics))
            .add_systems(
                Update,
                (
                    sync_zone_temporal_context_system,
                    reconcile_zone_runtime_system,
                    reconcile_zone_realization_system,
                )
                    .chain()
                    .in_set(UsfZoneSet::Runtime)
                    .in_set(UsfSimulationSet::Zone),
            )
            .register_type::<ZoneRealizationEvent>()
            .register_type::<ZoneDensityProfile>()
            .register_type::<ZonePhenomenonSelectionStrategy>()
            .register_type::<ZonePhenomenonSpawnPolicy>()
            .register_type::<ZoneSelectionPolicy>()
            .register_type::<ZonePhenomenonSupport>();
    }
}
