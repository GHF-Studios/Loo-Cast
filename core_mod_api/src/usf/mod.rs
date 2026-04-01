pub mod aspects;
pub mod capability;
pub mod content;
pub mod definition;
pub mod dpt;
pub mod math;
pub mod phenomenon;
pub mod pos;
pub mod runtime;
pub mod scale;
pub mod schedule;
pub mod substrate;
pub mod transform;
pub mod zlm;
pub mod zone;

use crate::bevy::prelude::*;
use content::{UsfActiveModpack, UsfConfiguredMod, UsfExecutionPlan, UsfScaleDefinition, UsfScaleExecutionRoute};
use definition::{DptMetricDefinition, DptMetricId, DptSchema, ZoneTypeId};
use dpt::{DptChunkKey, DptChunkRecord};
use phenomenon::{
    PartitionedPhenomenaModelMember, PartitionedPhenomenaModelRoot, PhenomenonId, PhenomenonKind, PhenomenonLineage, PhenomenonManifestationFieldContract,
    PhenomenonMeshWindow, PhenomenonNodeKey, PhenomenonNodeSeed, PhenomenonStateSnapshot,
};
use substrate::{
    AdaptiveChunkSubstrate, AdaptiveSubstrateOctreeNode, ChunkEdgeInterface, SubstrateChunkEdge, SubstrateChunkSummary, SubstrateLeafContainer,
    SubstrateTransitionDecision,
};
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
            .add_plugins(capability::CapabilityPlugin)
            .add_plugins(content::ContentPlugin)
            .add_plugins(dpt::DptPlugin)
            .add_plugins(zlm::ZlmPlugin)
            .add_plugins(substrate::SubstratePlugin)
            .add_plugins(runtime::UsfRuntimePlugin)
            .add_plugins(zone::ZonePlugin)
            .add_plugins(phenomenon::PhenomenonPlugin)
            .register_type::<DptMetricId>()
            .register_type::<ZoneTypeId>()
            .register_type::<DptMetricDefinition>()
            .register_type::<DptSchema>()
            .register_type::<UsfScaleDefinition>()
            .register_type::<UsfConfiguredMod>()
            .register_type::<UsfScaleExecutionRoute>()
            .register_type::<UsfExecutionPlan>()
            .register_type::<UsfActiveModpack>()
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
            .register_type::<PhenomenonMeshWindow>()
            .register_type::<PhenomenonManifestationFieldContract>()
            .register_type::<PartitionedPhenomenaModelRoot>()
            .register_type::<PartitionedPhenomenaModelMember>()
            .register_type::<SubstrateChunkEdge>()
            .register_type::<ChunkEdgeInterface>()
            .register_type::<SubstrateLeafContainer>()
            .register_type::<SubstrateTransitionDecision>()
            .register_type::<AdaptiveSubstrateOctreeNode>()
            .register_type::<SubstrateChunkSummary>()
            .register_type::<AdaptiveChunkSubstrate>();
    }
}
