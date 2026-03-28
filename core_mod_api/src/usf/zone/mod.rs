mod resources;
mod systems;
mod types;

use crate::bevy::prelude::*;
use crate::usf::schedule::{UsfSimulationSet, UsfZoneSet};

pub use resources::{
    ZoneBehaviorRegistry, ZoneDensityProfile, ZonePhenomenonSelectionStrategy, ZonePhenomenonSpawnPolicy, ZonePhenomenonSupport, ZoneRealizationState,
    ZoneRealizedPhenomenon, ZoneRuntimeState, ZoneSelectionPolicy, ZoneTemporalContext, time_scale_for_levels_above, time_scale_for_scale,
    time_scale_for_scale_indices,
};
pub use types::{StableRegionId, ZoneAnchor, ZoneExtent, ZoneId, ZonePhenomenon, ZoneRealizationEvent, ZoneTimeFactor};

use systems::{reconcile_zone_realization_system, reconcile_zone_runtime_system, sync_zone_temporal_context_system};

pub(crate) struct ZonePlugin;
impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ZoneRuntimeState>()
            .init_resource::<ZoneRealizationState>()
            .init_resource::<ZoneBehaviorRegistry>()
            .init_resource::<ZoneTemporalContext>()
            .add_message::<ZoneRealizationEvent>()
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
