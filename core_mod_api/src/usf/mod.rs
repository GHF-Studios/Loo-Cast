pub mod aspects;
pub mod authority;
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
use crate::core::orchestration::AppSet;
use authority::{
    UsfAuthorityDiagnostics, UsfAuthorityDiagnosticsEvent, UsfAuthorityDiagnosticsExportSettings, UsfAuthorityDiagnosticsExportState,
    UsfAuthorityViolationMode, UsfWorldAuthorityContract, export_usf_authority_diagnostics_events_system, report_usf_authority_diagnostics_system,
    validate_usf_world_authority_contract_system,
};
use content::{UsfActiveModpack, UsfConfiguredMod, UsfExecutionPlan, UsfScaleDefinition, UsfScaleExecutionRoute};
use definition::{DptMetricDefinition, DptMetricId, DptSchema, ZoneTypeId};
use dpt::{DptChunkKey, DptChunkRecord};
use phenomenon::{
    ManifestationMaterialProfileDefinition, PartitionedPhenomenonModelMember, PartitionedPhenomenonModelRoot, PhenomenonId, PhenomenonKind, PhenomenonLineage,
    PhenomenonManifestationFieldContract, PhenomenonMeshWindow, PhenomenonNodeKey, PhenomenonNodeSeed, PhenomenonStateSnapshot,
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

        app.init_resource::<UsfWorldAuthorityContract>()
            .init_resource::<UsfAuthorityDiagnostics>()
            .init_resource::<UsfAuthorityDiagnosticsExportSettings>()
            .init_resource::<UsfAuthorityDiagnosticsExportState>()
            .add_message::<UsfAuthorityDiagnosticsEvent>()
            .add_systems(Startup, validate_usf_world_authority_contract_system.in_set(AppSet::Diagnostics))
            .add_systems(Update, report_usf_authority_diagnostics_system.in_set(AppSet::Diagnostics))
            .add_systems(
                Update,
                export_usf_authority_diagnostics_events_system
                    .after(report_usf_authority_diagnostics_system)
                    .in_set(AppSet::Diagnostics),
            )
            .add_plugins(pos::PosPlugin)
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
            .register_type::<UsfAuthorityDiagnostics>()
            .register_type::<UsfAuthorityDiagnosticsEvent>()
            .register_type::<UsfAuthorityDiagnosticsExportSettings>()
            .register_type::<UsfAuthorityDiagnosticsExportState>()
            .register_type::<UsfAuthorityViolationMode>()
            .register_type::<UsfWorldAuthorityContract>()
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
            .register_type::<ManifestationMaterialProfileDefinition>()
            .register_type::<PartitionedPhenomenonModelRoot>()
            .register_type::<PartitionedPhenomenonModelMember>()
            .register_type::<SubstrateChunkEdge>()
            .register_type::<ChunkEdgeInterface>()
            .register_type::<SubstrateLeafContainer>()
            .register_type::<SubstrateTransitionDecision>()
            .register_type::<AdaptiveSubstrateOctreeNode>()
            .register_type::<SubstrateChunkSummary>()
            .register_type::<AdaptiveChunkSubstrate>();
    }
}
