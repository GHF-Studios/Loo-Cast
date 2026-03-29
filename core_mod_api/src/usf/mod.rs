pub mod aspects;
pub mod content;
pub mod definition;
pub mod dpt;
pub mod math;
pub mod phenomenon;
pub mod pos;
pub mod scale;
pub mod schedule;
pub mod transform;
pub mod zlm;
pub mod zone;

use crate::bevy::prelude::*;
use content::{
    ScaleContentBinding, UsfContentPackageActivation, UsfContentPackageDefinition, UsfContentProfileDefinition, UsfExecutionPlan, UsfScaleExecutionRoute,
};
use definition::{DptMetricDefinition, DptMetricId, DptSchema, ZoneTypeId};
use dpt::{DptChunkKey, DptChunkRecord};
use phenomenon::{PhenomenonId, PhenomenonKind, PhenomenonLineage, PhenomenonMeshWindow, PhenomenonNodeKey, PhenomenonNodeSeed, PhenomenonStateSnapshot};
use zlm::{ZlmMetricBand, ZlmScaleDefinition, ZlmZoneRule};
use zone::{
    StableRegionId, ZoneAnchor, ZoneBehaviorRegistry, ZoneDensityProfile, ZoneExtent, ZoneId, ZonePhenomenon, ZonePhenomenonSelectionStrategy,
    ZonePhenomenonSpawnPolicy, ZonePhenomenonSupport, ZoneRealizationEvent, ZoneSelectionPolicy, ZoneTimeFactor,
};

pub(crate) struct UsfPlugin;
impl Plugin for UsfPlugin {
    fn build(&self, app: &mut App) {
        schedule::configure_usf_simulation_sets(app);

        app.add_plugins(pos::PosPlugin)
            .add_plugins(transform::TransformPlugin)
            .add_plugins(definition::DefinitionPlugin)
            .add_plugins(content::ContentPlugin)
            .add_plugins(dpt::DptPlugin)
            .add_plugins(zlm::ZlmPlugin)
            .add_plugins(zone::ZonePlugin)
            .add_plugins(phenomenon::PhenomenonPlugin)
            .register_type::<DptMetricId>()
            .register_type::<ZoneTypeId>()
            .register_type::<DptMetricDefinition>()
            .register_type::<DptSchema>()
            .register_type::<ScaleContentBinding>()
            .register_type::<UsfContentPackageDefinition>()
            .register_type::<UsfContentProfileDefinition>()
            .register_type::<UsfScaleExecutionRoute>()
            .register_type::<UsfExecutionPlan>()
            .register_type::<UsfContentPackageActivation>()
            .register_type::<DptChunkKey>()
            .register_type::<DptChunkRecord>()
            .register_type::<ZlmMetricBand>()
            .register_type::<ZlmZoneRule>()
            .register_type::<ZlmScaleDefinition>()
            .register_type::<StableRegionId>()
            .register_type::<ZoneId>()
            .register_type::<ZoneExtent>()
            .register_type::<ZoneAnchor>()
            .register_type::<ZonePhenomenon>()
            .register_type::<ZoneTimeFactor>()
            .register_type::<ZoneBehaviorRegistry>()
            .register_type::<ZoneDensityProfile>()
            .register_type::<ZonePhenomenonSelectionStrategy>()
            .register_type::<ZonePhenomenonSpawnPolicy>()
            .register_type::<ZoneSelectionPolicy>()
            .register_type::<ZonePhenomenonSupport>()
            .register_type::<ZoneRealizationEvent>()
            .register_type::<PhenomenonKind>()
            .register_type::<PhenomenonId>()
            .register_type::<PhenomenonNodeSeed>()
            .register_type::<PhenomenonLineage>()
            .register_type::<PhenomenonNodeKey>()
            .register_type::<PhenomenonStateSnapshot>()
            .register_type::<PhenomenonMeshWindow>();
    }
}
