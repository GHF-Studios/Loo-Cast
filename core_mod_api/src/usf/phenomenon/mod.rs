use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;
use crate::usf::schedule::{UsfPhenomenonSet, UsfSimulationSet};

pub mod components;
pub mod generator;
pub mod generators;
pub mod meshing;
pub mod resources;
pub mod systems;
pub mod types;

pub use components::{
    Phenomenon, PhenomenonModel, PhenomenonModelScriptDefinitionRef, PhenomenonNode, PhenomenonNodeLifecycle, PhenomenonNodeState, PhenomenonRootNodeRef,
    PhenomenonScriptDefinitionRef,
};
pub use generator::{
    BuildStateInput, MeshWindowInput, PhenomenonChildPlan, PhenomenonGenerator, PhenomenonMeshWindow, PhenomenonStateSnapshot, PlanChildrenInput,
};
pub use generators::layer_echo::LayerEchoGenerator;
pub use meshing::{PHENOMENON_SEAM_LATTICE_DENOM, PhenomenonLatticeWindow, seam_safe_lattice_window};
pub use resources::PhenomenonDefinitionRegistry;
pub use systems::{PhenomenonDebugStats, PhenomenonGeneratorState, PhenomenonLifecyclePolicy};
pub use types::{MetricSurfaceDebugFieldDefinition, PhenomenonId, PhenomenonKind, PhenomenonLineage, PhenomenonNodeKey, PhenomenonNodeSeed};

use systems::{
    despawn_invalid_nodes_system, ensure_primary_models_system, ensure_root_nodes_system, expand_phenomenon_frontier_system, prune_orphan_models_system,
    refresh_active_node_stats_system, sync_policy_depth_to_frontier_scale_system,
};

pub(crate) struct PhenomenonPlugin;
impl Plugin for PhenomenonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhenomenonLifecyclePolicy>()
            .init_resource::<PhenomenonDefinitionRegistry>()
            .init_resource::<PhenomenonGeneratorState>()
            .init_resource::<PhenomenonDebugStats>()
            .add_systems(
                Update,
                (
                    sync_policy_depth_to_frontier_scale_system,
                    ensure_primary_models_system,
                    prune_orphan_models_system.after(ensure_primary_models_system),
                    ensure_root_nodes_system,
                    expand_phenomenon_frontier_system.after(ensure_root_nodes_system),
                    despawn_invalid_nodes_system.after(expand_phenomenon_frontier_system),
                )
                    .in_set(UsfPhenomenonSet::Runtime)
                    .in_set(UsfSimulationSet::Phenomenon),
            )
            .add_systems(Update, refresh_active_node_stats_system.in_set(AppSet::Diagnostics))
            .register_type::<Phenomenon>()
            .register_type::<PhenomenonModel>()
            .register_type::<PhenomenonScriptDefinitionRef>()
            .register_type::<PhenomenonModelScriptDefinitionRef>()
            .register_type::<PhenomenonRootNodeRef>()
            .register_type::<PhenomenonNode>()
            .register_type::<PhenomenonNodeState>()
            .register_type::<PhenomenonNodeLifecycle>()
            .register_type::<PhenomenonDefinitionRegistry>()
            .register_type::<PhenomenonLifecyclePolicy>()
            .register_type::<PhenomenonDebugStats>()
            .register_type::<MetricSurfaceDebugFieldDefinition>()
            .register_type::<PhenomenonLatticeWindow>();
    }
}
