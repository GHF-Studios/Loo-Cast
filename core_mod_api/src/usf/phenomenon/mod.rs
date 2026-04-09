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
    MonolithicPhenomenonModel, PartialPhenomenonModel, PartitionedPhenomenonModelMember, PartitionedPhenomenonModelRoot, Phenomenon, PhenomenonModel,
    PhenomenonModelProjection, PhenomenonModelProjectionSpec, PhenomenonModelScriptDefinitionRef, PhenomenonModelState, PhenomenonModelSupport,
    PhenomenonModelSupportBounds, PhenomenonModelTopology, PhenomenonNode, PhenomenonNodeLifecycle, PhenomenonNodeState, PhenomenonRootNodeRef,
    PhenomenonScriptDefinitionRef,
};
pub use generator::{
    BuildStateInput, MeshWindowInput, PhenomenonChildPlan, PhenomenonGenerator, PhenomenonMeshWindow, PhenomenonStateSnapshot, PlanChildrenInput,
};
pub use generators::layer_echo::LayerEchoGenerator;
pub use meshing::{PHENOMENON_SEAM_LATTICE_DENOM, PhenomenonLatticeWindow, seam_safe_lattice_window};
pub use partition_runtime::{PartitionRuntimeSettings, PartitionSyncRuntimeState};
pub use persistence::{
    PARTIAL_PHENOMENON_MODEL_SCHEMA_VERSION, PHENOMENON_MODEL_SCHEMA_VERSION, PHENOMENON_SCHEMA_VERSION, PersistedPartialPhenomenonModelRecord,
    PersistedPhenomenonModelRecord, PersistedPhenomenonRecord, PhenomenonPersistenceDurability,
};
pub use resources::{PhenomenonDefinitionRegistry, PhenomenonModelRegistry};
pub use systems::{
    PhenomenonChildScaleModelRequest, PhenomenonChildScaleRequestSettings, PhenomenonDebugStats, PhenomenonGeneratorState, PhenomenonLifecyclePolicy,
    PhenomenonPersistenceRestoreState, PhenomenonPersistenceRuntimeSettings,
};
pub use types::{
    InteractionTriggerDefinition, OutputAudioEmitterDefinition, OutputDensityFieldDefinition, OutputMaterialProfileDefinition, OutputParticleEmitterDefinition,
    PhenomenonId, PhenomenonKind, PhenomenonLineage, PhenomenonNodeKey, PhenomenonNodeSeed, PhenomenonOutputFieldSpec, PhenomenonSimulationServiceDefinition,
};

use partition_runtime::sync_partitioned_model_members_system;
use persistence_runtime::{
    PhenomenonPersistenceJournalRecoveryState, PhenomenonPersistenceWriteRuntimeState, PhenomenonPersistenceWriteStats,
    enqueue_authoritative_phenomena_persistence_writes_system, flush_authoritative_phenomena_persistence_writes_system,
    recover_authoritative_phenomena_persistence_journal_system,
};
use systems::{
    apply_child_scale_model_requests_system, despawn_invalid_nodes_system, emit_child_scale_model_requests_system,
    enforce_model_topology_component_invariants_system, ensure_root_nodes_system, ensure_scale_models_system, expand_phenomenon_frontier_system,
    prune_orphan_models_system, reconcile_zone_realization_model_state_system, refresh_active_node_stats_system, restore_persisted_phenomena_state_system,
    sync_policy_depth_to_frontier_scale_system,
};

pub(crate) struct PhenomenonPlugin;
fn validate_phenomenon_model_registry_system(
    definitions: Res<PhenomenonDefinitionRegistry>,
    model_registry: Res<PhenomenonModelRegistry>,
) {
    if model_registry.phenomenon_by_model_id.is_empty() {
        panic!("USF phenomenon model registry validation failed: no phenomenon models are registered.");
    }
    if model_registry.model_selection_by_phenomenon_scale.is_empty() {
        panic!("USF phenomenon model registry validation failed: no model selection entries are registered.");
    }

    for (model_id, phenomenon_id) in &model_registry.phenomenon_by_model_id {
        if !definitions.model_belongs_to_phenomenon(model_id, phenomenon_id) {
            panic!(
                "USF phenomenon model registry validation failed: model '{}' is not linked to phenomenon '{}'.",
                model_id, phenomenon_id
            );
        }
        if !model_registry.topology_by_model_id.contains_key(model_id) {
            panic!(
                "USF phenomenon model registry validation failed: model '{}' has no topology entry.",
                model_id
            );
        }
        if !model_registry.support_chunk_radius_by_model_id.contains_key(model_id) {
            panic!(
                "USF phenomenon model registry validation failed: model '{}' has no support_chunk_radius entry.",
                model_id
            );
        }
        if !model_registry.projection_spec_by_model_id.contains_key(model_id) {
            panic!(
                "USF phenomenon model registry validation failed: model '{}' has no projection spec entry.",
                model_id
            );
        }
    }
}

impl Plugin for PhenomenonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhenomenonLifecyclePolicy>()
            .init_resource::<PhenomenonDefinitionRegistry>()
            .init_resource::<PhenomenonModelRegistry>()
            .init_resource::<PhenomenonGeneratorState>()
            .init_resource::<PhenomenonDebugStats>()
            .init_resource::<PhenomenonPersistenceRuntimeSettings>()
            .init_resource::<PhenomenonPersistenceRestoreState>()
            .init_resource::<PhenomenonPersistenceWriteRuntimeState>()
            .init_resource::<PhenomenonPersistenceWriteStats>()
            .init_resource::<PhenomenonPersistenceJournalRecoveryState>()
            .init_resource::<PartitionRuntimeSettings>()
            .init_resource::<PartitionSyncRuntimeState>()
            .init_resource::<PhenomenonChildScaleRequestSettings>()
            .add_message::<PhenomenonChildScaleModelRequest>()
            .add_systems(Startup, validate_phenomenon_model_registry_system.in_set(AppSet::Diagnostics))
            .add_systems(
                Update,
                (
                    sync_policy_depth_to_frontier_scale_system,
                    ensure_scale_models_system,
                    enforce_model_topology_component_invariants_system.after(ensure_scale_models_system),
                    reconcile_zone_realization_model_state_system.after(enforce_model_topology_component_invariants_system),
                    restore_persisted_phenomena_state_system.after(reconcile_zone_realization_model_state_system),
                    emit_child_scale_model_requests_system.after(restore_persisted_phenomena_state_system),
                    apply_child_scale_model_requests_system.after(emit_child_scale_model_requests_system),
                    sync_partitioned_model_members_system.after(apply_child_scale_model_requests_system),
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
            .register_type::<PhenomenonModelTopology>()
            .register_type::<PhenomenonModelSupportBounds>()
            .register_type::<PhenomenonModelProjectionSpec>()
            .register_type::<PhenomenonModelSupport>()
            .register_type::<PhenomenonModelProjection>()
            .register_type::<MonolithicPhenomenonModel>()
            .register_type::<PartitionedPhenomenonModelRoot>()
            .register_type::<PartitionedPhenomenonModelMember>()
            .register_type::<PartialPhenomenonModel>()
            .register_type::<PhenomenonModelState>()
            .register_type::<PhenomenonScriptDefinitionRef>()
            .register_type::<PhenomenonModelScriptDefinitionRef>()
            .register_type::<PhenomenonRootNodeRef>()
            .register_type::<PhenomenonNode>()
            .register_type::<PhenomenonNodeState>()
            .register_type::<PhenomenonNodeLifecycle>()
            .register_type::<PhenomenonDefinitionRegistry>()
            .register_type::<PhenomenonModelRegistry>()
            .register_type::<PhenomenonLifecyclePolicy>()
            .register_type::<PhenomenonDebugStats>()
            .register_type::<PhenomenonPersistenceRuntimeSettings>()
            .register_type::<PhenomenonPersistenceDurability>()
            .register_type::<PhenomenonPersistenceWriteStats>()
            .register_type::<PhenomenonPersistenceJournalRecoveryState>()
            .register_type::<PartitionRuntimeSettings>()
            .register_type::<PhenomenonChildScaleRequestSettings>()
            .register_type::<PhenomenonChildScaleModelRequest>()
            .register_type::<OutputDensityFieldDefinition>()
            .register_type::<OutputMaterialProfileDefinition>()
            .register_type::<PhenomenonOutputFieldSpec>()
            .register_type::<PhenomenonSimulationServiceDefinition>()
            .register_type::<OutputAudioEmitterDefinition>()
            .register_type::<OutputParticleEmitterDefinition>()
            .register_type::<InteractionTriggerDefinition>()
            .register_type::<PhenomenonLatticeWindow>();
    }
}
