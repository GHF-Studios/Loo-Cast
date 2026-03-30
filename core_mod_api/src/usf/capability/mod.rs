use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityId(pub String);
impl CapabilityId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into().trim().to_ascii_lowercase())
    }
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityAuthorityMode {
    ProjectionOnly,
    LeasedLocalAuthority,
    AuthoritativeRuntime,
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct CapabilityExecutionContract {
    pub id: CapabilityId,
    pub owner_path: String,
    pub authority_mode: CapabilityAuthorityMode,
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfCapabilityGraph {
    pub world_capabilities: Vec<CapabilityId>,
    pub presentation_capabilities: Vec<CapabilityId>,
    pub simulation_capabilities: Vec<CapabilityId>,
    pub contracts_by_capability: HashMap<CapabilityId, CapabilityExecutionContract>,
}
impl Default for UsfCapabilityGraph {
    fn default() -> Self {
        let world_capabilities = vec![CapabilityId::new("world.chunk_surface")];
        let presentation_capabilities = vec![CapabilityId::new("presentation.render.chunk_surface_mesh")];
        let simulation_capabilities = vec![CapabilityId::new("simulation.physics.rapier.local")];

        let mut contracts_by_capability = HashMap::new();
        contracts_by_capability.insert(
            CapabilityId::new("world.chunk_surface"),
            CapabilityExecutionContract {
                id: CapabilityId::new("world.chunk_surface"),
                owner_path: "usf.mod_runtime.chunk_surface".to_string(),
                authority_mode: CapabilityAuthorityMode::AuthoritativeRuntime,
            },
        );
        contracts_by_capability.insert(
            CapabilityId::new("presentation.render.chunk_surface_mesh"),
            CapabilityExecutionContract {
                id: CapabilityId::new("presentation.render.chunk_surface_mesh"),
                owner_path: "usf.mod_runtime.chunk_surface".to_string(),
                authority_mode: CapabilityAuthorityMode::ProjectionOnly,
            },
        );
        contracts_by_capability.insert(
            CapabilityId::new("simulation.physics.rapier.local"),
            CapabilityExecutionContract {
                id: CapabilityId::new("simulation.physics.rapier.local"),
                owner_path: "simulation.physics.rapier".to_string(),
                authority_mode: CapabilityAuthorityMode::LeasedLocalAuthority,
            },
        );

        Self {
            world_capabilities,
            presentation_capabilities,
            simulation_capabilities,
            contracts_by_capability,
        }
    }
}

fn validate_capability_graph_system(graph: Res<UsfCapabilityGraph>) {
    if graph.world_capabilities.is_empty() {
        panic!("USF capability graph validation failed: world.* capability bucket must not be empty");
    }
    if graph.presentation_capabilities.is_empty() {
        panic!("USF capability graph validation failed: presentation.* capability bucket must not be empty");
    }
    if graph.simulation_capabilities.is_empty() {
        panic!("USF capability graph validation failed: simulation.* capability bucket must not be empty");
    }

    for capability in graph
        .world_capabilities
        .iter()
        .chain(graph.presentation_capabilities.iter())
        .chain(graph.simulation_capabilities.iter())
    {
        if !graph.contracts_by_capability.contains_key(capability) {
            panic!(
                "USF capability graph validation failed: missing execution contract for capability '{}'",
                capability.0
            );
        }
    }
}

pub(crate) struct CapabilityPlugin;
impl Plugin for CapabilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfCapabilityGraph>()
            .add_systems(Startup, validate_capability_graph_system.in_set(AppSet::Diagnostics))
            .register_type::<CapabilityId>()
            .register_type::<CapabilityAuthorityMode>()
            .register_type::<CapabilityExecutionContract>()
            .register_type::<UsfCapabilityGraph>();
    }
}
