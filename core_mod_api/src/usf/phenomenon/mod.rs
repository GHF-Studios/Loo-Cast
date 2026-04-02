use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;
use crate::usf::schedule::{UsfPhenomenonSet, UsfSimulationSet};

pub mod components;
pub mod generator;
pub mod generators;
pub mod meshing;
pub mod partition_runtime;
pub mod persistence;
pub mod persistence_runtime;
pub mod resources;
pub mod systems;
pub mod types;

pub use components::{
    MonolithicPhenomenaModel, PartialPhenomenaModel, PartitionedPhenomenaModelMember, PartitionedPhenomenaModelRoot, PhenomenaModelState,
    PhenomenaModelSupport, PhenomenaModelTopology, PhenomenaProjectionContract, Phenomenon, PhenomenonModel, PhenomenonModelProjectionContract,
    PhenomenonModelScriptDefinitionRef, PhenomenonModelSupport, PhenomenonNode, PhenomenonNodeLifecycle, PhenomenonNodeState, PhenomenonRootNodeRef,
    PhenomenonScriptDefinitionRef,
};
pub use generator::{
    BuildStateInput, MeshWindowInput, PhenomenonChildPlan, PhenomenonGenerator, PhenomenonMeshWindow, PhenomenonStateSnapshot, PlanChildrenInput,
};
pub use generators::layer_echo::LayerEchoGenerator;
pub use meshing::{PHENOMENON_SEAM_LATTICE_DENOM, PhenomenonLatticeWindow, seam_safe_lattice_window};
pub use partition_runtime::PartitionRuntimeSettings;
pub use persistence::{
    PARTIAL_PHENOMENA_MODEL_SCHEMA_VERSION, PHENOMENA_MODEL_SCHEMA_VERSION, PHENOMENON_SCHEMA_VERSION, PersistedPartialPhenomenaModelRecord,
    PersistedPhenomenaModelRecord, PersistedPhenomenonRecord, PhenomenonPersistenceDurability,
};
pub use resources::PhenomenonDefinitionRegistry;
pub use systems::{
    PhenomenonDebugStats, PhenomenonGeneratorState, PhenomenonLifecyclePolicy, PhenomenonPersistenceHydrationState, PhenomenonPersistenceRuntimeSettings,
};
pub use types::{
    ManifestationDensityFieldDefinition, ManifestationMaterialProfileDefinition, PhenomenonCapability, PhenomenonId, PhenomenonKind, PhenomenonLineage,
    PhenomenonManifestationFieldContract, PhenomenonNodeKey, PhenomenonNodeSeed,
};

use partition_runtime::sync_partitioned_model_members_system;
use persistence_runtime::{
    PhenomenonPersistenceJournalRecoveryState, PhenomenonPersistenceWriteRuntimeState, PhenomenonPersistenceWriteStats,
    enqueue_authoritative_phenomena_persistence_writes_system, flush_authoritative_phenomena_persistence_writes_system,
    recover_authoritative_phenomena_persistence_journal_system,
};
use systems::{
    apply_zone_realization_startup_hooks_system, despawn_invalid_nodes_system, enforce_model_topology_component_contracts_system, ensure_root_nodes_system,
    ensure_scale_models_system, expand_phenomenon_frontier_system, hydrate_persisted_phenomena_state_system, prune_orphan_models_system,
    refresh_active_node_stats_system, sync_policy_depth_to_frontier_scale_system,
};

pub(crate) struct PhenomenonPlugin;
impl Plugin for PhenomenonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhenomenonLifecyclePolicy>()
            .init_resource::<PhenomenonDefinitionRegistry>()
            .init_resource::<PhenomenonGeneratorState>()
            .init_resource::<PhenomenonDebugStats>()
            .init_resource::<PhenomenonPersistenceRuntimeSettings>()
            .init_resource::<PhenomenonPersistenceHydrationState>()
            .init_resource::<PhenomenonPersistenceWriteRuntimeState>()
            .init_resource::<PhenomenonPersistenceWriteStats>()
            .init_resource::<PhenomenonPersistenceJournalRecoveryState>()
            .init_resource::<PartitionRuntimeSettings>()
            .add_systems(
                Update,
                (
                    sync_policy_depth_to_frontier_scale_system,
                    ensure_scale_models_system,
                    enforce_model_topology_component_contracts_system.after(ensure_scale_models_system),
                    apply_zone_realization_startup_hooks_system.after(enforce_model_topology_component_contracts_system),
                    hydrate_persisted_phenomena_state_system.after(apply_zone_realization_startup_hooks_system),
                    sync_partitioned_model_members_system.after(hydrate_persisted_phenomena_state_system),
                    prune_orphan_models_system.after(sync_partitioned_model_members_system),
                    ensure_root_nodes_system,
                    expand_phenomenon_frontier_system.after(ensure_root_nodes_system),
                    despawn_invalid_nodes_system.after(expand_phenomenon_frontier_system),
                    recover_authoritative_phenomena_persistence_journal_system.after(despawn_invalid_nodes_system),
                    enqueue_authoritative_phenomena_persistence_writes_system.after(recover_authoritative_phenomena_persistence_journal_system),
                    flush_authoritative_phenomena_persistence_writes_system.after(enqueue_authoritative_phenomena_persistence_writes_system),
                )
                    .in_set(UsfPhenomenonSet::Runtime)
                    .in_set(UsfSimulationSet::Phenomenon),
            )
            .add_systems(Update, refresh_active_node_stats_system.in_set(AppSet::Diagnostics))
            .register_type::<Phenomenon>()
            .register_type::<PhenomenonModel>()
            .register_type::<PhenomenaModelTopology>()
            .register_type::<PhenomenaModelSupport>()
            .register_type::<PhenomenaProjectionContract>()
            .register_type::<PhenomenonModelSupport>()
            .register_type::<PhenomenonModelProjectionContract>()
            .register_type::<MonolithicPhenomenaModel>()
            .register_type::<PartitionedPhenomenaModelRoot>()
            .register_type::<PartitionedPhenomenaModelMember>()
            .register_type::<PartialPhenomenaModel>()
            .register_type::<PhenomenaModelState>()
            .register_type::<PhenomenonScriptDefinitionRef>()
            .register_type::<PhenomenonModelScriptDefinitionRef>()
            .register_type::<PhenomenonRootNodeRef>()
            .register_type::<PhenomenonNode>()
            .register_type::<PhenomenonNodeState>()
            .register_type::<PhenomenonNodeLifecycle>()
            .register_type::<PhenomenonDefinitionRegistry>()
            .register_type::<PhenomenonLifecyclePolicy>()
            .register_type::<PhenomenonDebugStats>()
            .register_type::<PhenomenonPersistenceRuntimeSettings>()
            .register_type::<PhenomenonPersistenceDurability>()
            .register_type::<PhenomenonPersistenceWriteStats>()
            .register_type::<PhenomenonPersistenceJournalRecoveryState>()
            .register_type::<PartitionRuntimeSettings>()
            .register_type::<ManifestationDensityFieldDefinition>()
            .register_type::<ManifestationMaterialProfileDefinition>()
            .register_type::<PhenomenonManifestationFieldContract>()
            .register_type::<PhenomenonLatticeWindow>();
    }
}
